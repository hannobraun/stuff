use std::thread;

use crossbeam_channel::{select, RecvError, SendError, Sender};

use crate::{
    audio::{Buffer, BUFFER_SIZE, SAMPLE_RATE},
    synth::components::SynthComponent,
};

use super::{
    clock::Clock,
    components::{oscillator::Oscillator, scaler::Scaler},
    interface::{Note, UserInput},
    signal::Signal,
    wave::Wave,
};

pub fn start(output: Sender<Buffer>) -> Sender<UserInput> {
    let (input_tx, input) = crossbeam_channel::bounded::<UserInput>(0);

    thread::spawn(move || {
        let mut clock = Clock {
            time: 0,
            sample_rate: SAMPLE_RATE as u64,
        };

        let (note, mut note_writer) = Signal::variable();
        let (volume, mut volume_writer) = Signal::variable();
        volume_writer.update(|_| Some(0.5));

        let osc = Signal::new(Oscillator {
            frequency: note,
            wave: Wave::sawtooth(),
            ..Default::default()
        });
        let mut scaler = Scaler {
            input: osc,
            scale: volume,
            ..Default::default()
        };

        let volume_increment = 0.1;

        let mut buffer = [0.; BUFFER_SIZE];
        let mut octave = 0;

        loop {
            let input = select! {
                send(output, buffer) -> res => {
                    if let Err(SendError(_)) = res {
                        break;
                    }

                    for value in &mut buffer {
                        clock.advance();
                        scaler.update(&clock);
                        *value = scaler.output.get().unwrap_or(0.);
                    }

                    continue;
                }
                recv(input) -> input => {
                    match input {
                        Ok(input) => input,
                        Err(RecvError) =>  {
                            break;
                        }
                    }
                }
            };

            match input {
                UserInput::OctaveDec => {
                    octave -= 1;
                    continue;
                }
                UserInput::OctaveInc => {
                    octave += 1;
                    continue;
                }

                UserInput::VolumeDec => {
                    volume_writer.update(|volume| {
                        Some(volume.unwrap() - volume_increment)
                    });
                    continue;
                }
                UserInput::VolumeInc => {
                    volume_writer.update(|volume| {
                        Some(volume.unwrap() + volume_increment)
                    });
                    continue;
                }

                UserInput::PlayNote(note) => {
                    let number = match note {
                        Note::C => 4,
                        Note::D => 6,
                        Note::E => 8,
                        Note::F => 9,
                        Note::G => 11,
                        Note::A => 13,
                        Note::B => 15,
                    };
                    let number = number + octave * 12;

                    let frequency =
                        2f32.powf((number - 49) as f32 / 12.) * 440.;

                    note_writer.update(|_| Some(frequency));
                }
                UserInput::ReleaseNote => {
                    note_writer.update(|_| None);
                }
            }
        }
    });

    input_tx
}

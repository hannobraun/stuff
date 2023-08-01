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
};

pub fn start(output: Sender<Buffer>) -> Sender<UserInput> {
    let (input_tx, input) = crossbeam_channel::bounded::<UserInput>(0);

    thread::spawn(move || {
        let mut clock = Clock {
            time: 0,
            sample_rate: SAMPLE_RATE as u64,
        };

        let mut oscillator = Oscillator::default();

        let mut scaler = Scaler {
            ..Default::default()
        };
        scaler.input.connect(&oscillator.output);
        scaler.scale.set(Some(0.5));

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

                        oscillator.update(&clock);
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
                    scaler.scale.set(Some(
                        scaler.scale.get().unwrap() - volume_increment,
                    ));
                    continue;
                }
                UserInput::VolumeInc => {
                    scaler.scale.set(Some(
                        scaler.scale.get().unwrap() + volume_increment,
                    ));
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

                    oscillator.frequency.set(Some(frequency));
                }
                UserInput::ReleaseNote(_) => {
                    oscillator.frequency.set(None);
                }
            }
        }
    });

    input_tx
}

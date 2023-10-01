use std::thread;

use crossbeam_channel::{select, RecvError, SendError, Sender};

use crate::{
    audio::{Buffer, BUFFER_SIZE, SAMPLE_RATE},
    synth::components::SynthComponent,
};

use super::{
    clock::Clock,
    components::{mixer::Mixer, oscillator::Oscillator, scaler::Scaler},
    interface::{Note, UserInput},
};

pub fn start(output: Sender<Buffer>) -> Sender<UserInput> {
    let (input_tx, input) = crossbeam_channel::bounded::<UserInput>(0);

    thread::spawn(move || {
        let mut clock = Clock {
            time: 0,
            sample_rate: SAMPLE_RATE as u64,
        };

        let mut osc_c = Oscillator::default();
        let mut osc_d = Oscillator::default();
        let mut osc_e = Oscillator::default();
        let mut osc_f = Oscillator::default();
        let mut osc_g = Oscillator::default();
        let mut osc_a = Oscillator::default();
        let mut osc_b = Oscillator::default();

        let mut mixer = Mixer::default();
        mixer.connect(&osc_c.output);
        mixer.connect(&osc_d.output);
        mixer.connect(&osc_e.output);
        mixer.connect(&osc_f.output);
        mixer.connect(&osc_g.output);
        mixer.connect(&osc_a.output);
        mixer.connect(&osc_b.output);

        let mut scaler = Scaler::default();
        scaler.input.connect(&mixer.output);
        scaler.scale.set(Some(0.5));

        let volume_increment = 0.1;

        let mut buffer = [0.; BUFFER_SIZE];
        let mut octave = 2;

        loop {
            let input = select! {
                send(output, buffer) -> res => {
                    if let Err(SendError(_)) = res {
                        break;
                    }

                    for value in &mut buffer {
                        clock.advance();

                        osc_c.update(&clock);
                        osc_d.update(&clock);
                        osc_e.update(&clock);
                        osc_f.update(&clock);
                        osc_g.update(&clock);
                        osc_a.update(&clock);
                        osc_b.update(&clock);
                        mixer.update(&clock);
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

                    let oscillator = match note {
                        Note::C => &mut osc_c,
                        Note::D => &mut osc_d,
                        Note::E => &mut osc_e,
                        Note::F => &mut osc_f,
                        Note::G => &mut osc_g,
                        Note::A => &mut osc_a,
                        Note::B => &mut osc_b,
                    };

                    oscillator.frequency.set(Some(frequency));
                }
                UserInput::ReleaseNote(note) => {
                    let oscillator = match note {
                        Note::C => &mut osc_c,
                        Note::D => &mut osc_d,
                        Note::E => &mut osc_e,
                        Note::F => &mut osc_f,
                        Note::G => &mut osc_g,
                        Note::A => &mut osc_a,
                        Note::B => &mut osc_b,
                    };

                    oscillator.frequency.set(None);
                }
            }
        }
    });

    input_tx
}

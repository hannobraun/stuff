pub enum UserInput {
    OctaveDec,
    OctaveInc,

    VolumeDec,
    VolumeInc,

    PlayNote(Note),
    ReleaseNote(Note),
}

pub enum Note {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

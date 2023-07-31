pub enum Input {
    OctaveDec,
    OctaveInc,

    VolumeDec,
    VolumeInc,

    PlayNote(Note),
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

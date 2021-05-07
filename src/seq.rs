// These are the structs that hold the garlic_crust sequence information.
// They are not necessarily identical to the garlic_crust types, but their Display needs to be what garlic_crust expects.

pub struct SeqEvent {
    pub time: TimeFloat,
    pub message: SeqMsg,
}

pub enum SeqMsg {
    NoteOn(SeqParameter),
    NoteOff,
    SetVel,
    SetSlide,
    SetPan,
    // ...?
}
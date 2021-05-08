// These are the structs that hold the garlic_crust sequence information.
// They are not necessarily identical to the garlic_crust types, but their Display needs to be what garlic_crust expects.

pub type TimeFloat = f32;

pub type SeqParameter = f32; // check whether we have enough withi half::f16

pub struct Seq {
    events: Vec<SeqEvent>,
    open_note: Option<midly::num::u7>,
    channel: midly::num::u4,
}

pub struct SeqEvent {
    pub time: TimeFloat,
    pub message: SeqMsg,
}

pub enum SeqMsg {
    NoteOn(SeqParameter, SeqParameter),
    NoteOff,
    SetVel,
    SetSlide,
    SetPan,
    // ...?
}
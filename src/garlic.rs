// These are the structs that hold the garlic_crust sequence information.
// They are not necessarily identical to the garlic_crust types, but their Display needs to be what garlic_crust expects.

pub type TimeFloat = f32;

pub type SeqParameter = f32; // check whether we have enough withi half::f16

pub struct SeqEvent {
    pub time: TimeFloat,
    pub message: NoteMessage,
}

pub type Sequence = Vec::<SeqEvent>;

/*
pub enum SeqMsg {
    NoteOn(SeqParameter, SeqParameter),
    NoteOff,
    SetVel,
    SetSlide,
    SetPan,
    // ...?
}
*/

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum SeqMsg {
    NoteOff,
    NoteOn,
}

#[derive(Debug, Copy, Clone)]
pub struct NoteMessage {
    pub msg: SeqMsg,
    pub key: usize,
    pub vel: usize,
    pub channel: usize,
}

impl NoteMessage {
    pub fn from(message: &midly::MidiMessage, channel: &midly::num::u4) -> Option<NoteMessage> {
        let channel = channel.as_int() as usize;
        if let midly::MidiMessage::NoteOn {key, vel} = message {
            return Some(NoteMessage {
                channel: channel,
                key: key.as_int() as usize,
                vel: vel.as_int() as usize,
                msg: SeqMsg::NoteOn,
            });
        }
        if let midly::MidiMessage::NoteOff {key, vel} = message {
            return Some(NoteMessage {
                channel: channel,
                key: key.as_int() as usize,
                vel: vel.as_int() as usize,
                msg: SeqMsg::NoteOff,
            });
        }

        None
    }
}

impl std::fmt::Display for NoteMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.msg {
            SeqMsg::NoteOn =>
                write!(f, "SeqMsg::NoteOn({}, {})", self.key as SeqParameter, self.vel as SeqParameter),
            SeqMsg::NoteOff =>
                write!(f, "SeqMsg::NoteOff")
        }
    }
}
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

#[derive(Debug)]
pub enum SeqMsg {
    NoteOn,
    NoteOff
}

#[derive(Debug)]
pub struct NoteMessage {
    channel: usize,
    key: usize,
    vel: usize,
    msg: SeqMsg,
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
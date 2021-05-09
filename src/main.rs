/*
* garlic_extract
* QM / Team210
*
* this extracts the garlic_crust Sequence from a midi file, given as command line argument
*
*/

mod garlic;

type GroupedMessageMap = std::collections::BTreeMap::<usize, Vec<garlic::NoteMessage>>;

// to be clear: channel is ignored right now. I don't even know where to put this comment, so little do I care about it.

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("cli arguments: {:?}", args);

    if args.len() < 2 {
        println!("give midi file as argument!");
        return;
    }

    let midi_bytes = std::fs::read(&args[1]).unwrap();
    let smf = midly::Smf::parse(&midi_bytes).unwrap();
    println!("HEADER: {:?}", smf.header);

    let mut track_iter = smf.tracks.iter();
    // expectation: first track holds the tempo information (which we need if the header is just giving us the Metrical information PPQ, i.e. how many ticks per beat)
    let meta_track = track_iter.next().unwrap();
    let secs_per_tick = calculate_secs_per_tick(&smf.header.timing, &meta_track);

    let mut time_grouped_messages = GroupedMessageMap::new();
    let mut total_time: f32 = 0.;

    for track in track_iter {
        //println!("------ track {} has {} events", t, track.len());

        let mut current_track_iter = track.iter();
        let mut current_tick = 0;
        while let Some(&event) = current_track_iter.next() { // does this
            // println!("-- event: {:?}", event);
            let delta = event.delta.as_int() as usize;
            if delta > 0 {
                current_tick += delta;
            }
            if let midly::TrackEventKind::Midi{message, channel} = event.kind {
                match message {
                    midly::MidiMessage::NoteOn{..} | midly::MidiMessage::NoteOff{..} => {
                        sort_into_map(&mut time_grouped_messages, current_tick, garlic::NoteMessage::from(&message, &channel).unwrap());
                    },
                    _ => ()
                }
            }
        }

        total_time = total_time.max(current_tick as f32 * secs_per_tick)
    }

    let mut sequences = Vec::<garlic::Sequence>::new();

    let group_iterator = time_grouped_messages.iter();
    for (tick, group) in group_iterator {
        let time = (*tick as f32) * secs_per_tick;
        println!("Note group at {} -- {:?}", time, group);

        for note in group.iter() {
            let note_event = garlic::SeqEvent { time, message: *note };
            match note.msg {
                garlic::SeqMsg::NoteOn => {
                    //match sequences.iter_mut().find(find_free_sequence) {
                    match sequences.iter_mut().find(|seq| find_free_sequence(seq)) {
                        Some(lowest_free_sequence) => {
                            lowest_free_sequence.push(note_event);
                        },
                        None => {
                            sequences.push(vec![note_event]);
                        }
                    }
                },
                garlic::SeqMsg::NoteOff => {
                    match sequences.iter_mut().find(|seq| find_sequence_with_open_note(seq, &note.key)) {
                        Some(lowest_open_sequence) => {
                            lowest_open_sequence.push(note_event);
                        },
                        None => (),
                    }
                }
            }
        }
    }

    for (index, seq) in sequences.iter().enumerate() {
        println!();
        println!("{}", format_sequence(&seq, &index));
    }

    println!();
    println!("pub const SECONDS: TimeFloat = {:.3};", total_time + 0.5e-3);
}

fn calculate_secs_per_tick(timing: &midly::Timing, track: &midly::Track) -> f32 {
    let ppq;
    if let midly::Timing::Metrical(u15_value) = &timing {
        ppq = u15_value.as_int();
    } else {
        println!("ja wellwell. need Metrical timing for now.");
        std::process::exit(1);
    }

    let mut tempo = 0;
    // garlic_extract can not deal with tempo changes yet.
    // good question: how does FL studio export tempo changes in MIDI ?
    for event in track.iter() {
        if let midly::TrackEventKind::Meta(midly::MetaMessage::Tempo(u24_value)) = event.kind {
            tempo = u24_value.as_int();
        } else {
            println!("don't process this thing: {:?}", event);
        }
    }
    if tempo == 0 {
        println!("Tempo was not correctly initialized. Fix that shit! Is the tempo information in the first track, where garlic_extract expects it?");
        std::process::exit(2);
    }
    // Tempo is [µs/beat], ppq is [ticks/beat], so... (beat = quarter note)
    let secs_per_tick: f32 = 1.0e-6 * tempo as f32 / ppq as f32;
    println!("ppq, tempo, secs_per_tick = {:?}, {:?}, {:?}", ppq, tempo, secs_per_tick);

    secs_per_tick
}

fn sort_into_map(map: &mut GroupedMessageMap, current_tick: usize, message: garlic::NoteMessage) {
    if let Some(current_events) = map.get_mut(&current_tick) {
        let position = current_events.iter().position(|it| it.msg > message.msg || (it.msg == message.msg && it.key > message.key)).unwrap_or(current_events.len());
        current_events.insert(position, message);
    } else {
        map.insert(current_tick, vec![message]);
    }
}

// guess I can put this into Iterator trait's position():
// pub fn position<P>(&mut self, predicate: P) -> Option<usize> where P: FnMut(Self::Item) -> bool
fn find_free_sequence (seq: &&mut Vec::<garlic::SeqEvent>) -> bool {
    match seq.last() {
        Some(garlic::SeqEvent {
            message: garlic::NoteMessage {
                msg: garlic::SeqMsg::NoteOn,
                ..
            },
            ..
        }) => false,
        _ => true
    }
}

fn find_sequence_with_open_note (seq: &&mut Vec::<garlic::SeqEvent>, key: &usize) -> bool {
    match seq.last() {
        Some(garlic::SeqEvent {
            message: garlic::NoteMessage {
                msg: garlic::SeqMsg::NoteOn,
                key: note_key,
                ..
            },
            ..
        }) => (note_key == key),
        _ => false
    }
}

const indented_linebreak: &str = "\n        ";

fn format_sequence(seq: &garlic::Sequence, number: &usize) -> String {
    let mut result = String::from(format!("let sequence{}: [SeqEvent; {}] = [", number, seq.len()));
    seq.iter().for_each(|event| result.push_str(&format!("{}{},", indented_linebreak, event)));
    result.push_str("\n    ];");
    return result;
}
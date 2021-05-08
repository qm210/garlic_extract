/*
* garlic_extract
* QM / Team210
*
* this extracts the garlic_crust Sequence from a midi file, given as command line argument
*
*/

mod garlic;

type GroupedMessageMap = std::collections::BTreeMap::<usize, Vec<garlic::NoteMessage>>;

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

    let mut time_grouped_noteons = GroupedMessageMap::new();
    let mut time_grouped_noteoffs = GroupedMessageMap::new();

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
                    midly::MidiMessage::NoteOn{..} => {
                        sort_into_map(&mut time_grouped_noteons, current_tick, garlic::NoteMessage::from(&message, &channel).unwrap());
                    },
                    midly::MidiMessage::NoteOff{..} => {
                        sort_into_map(&mut time_grouped_noteoffs, current_tick, garlic::NoteMessage::from(&message, &channel).unwrap());
                    }
                    _ => ()
                }
            }
        }
    }

    let mut sequences = Vec::<garlic::Seq>::new();

    let group_iterator = time_grouped_noteoffs.iter();
    for (tick, group) in group_iterator {
        let time = (*tick as f32) * secs_per_tick;
        println!("NoteOff group at {} -- {:?}", time, group);
    }

    let group_iterator = time_grouped_noteons.iter();
    for (tick, group) in group_iterator {
        let time = (*tick as f32) * secs_per_tick;
        println!("NoteOn group at {} -- {:?}", time, group);
    }

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

fn sort_into_map(map: &mut GroupedMessageMap, current_tick: usize, message: NoteMessage) {
    if let Some(current_events) = map.get_mut(&current_tick) {
        let position = current_events.iter().position(|msg| msg.key > message.key).unwrap_or(current_events.len());
        current_events.insert(position, message);
    } else {
        map.insert(current_tick, vec![message]);
    }
}
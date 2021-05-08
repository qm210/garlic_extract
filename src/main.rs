/*
* garlic_extract
* QM / Team210
*
* this extracts the garlic_crust Sequence from a midi file, given as command line argument
*
*/

mod garlic;

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

    let mut sequences = Vec::<garlic::Seq>::new();
    let mut open_notes = Vec::<midly::num::u7>::new();
    let mut time_grouped_events = std::collections::BTreeMap::<usize, Vec<midly::TrackEvent>>::new();

    let mut time = 0.;
    for (t, track) in track_iter.enumerate() {
        println!("------ track {} has {} events", t, track.len());

        //open_notes.clear();
        let mut current_track_iter = track.iter();
        let mut current_tick = 0;
        while let Some(&event) = current_track_iter.next() { // does this
            println!("-- event: {:?}", event);
            let delta = event.delta.as_int() as usize;
            if delta > 0 {
                current_tick += delta;
            }
            if let Some(current_events) = time_grouped_events.get_mut(&current_tick) {
                current_events.push(event);
            } else {
                time_grouped_events.insert(current_tick, vec![event]);
            }
        }
    }

    let group_iterator = time_grouped_events.iter();
    for (tick, group) in group_iterator {
        let time = (*tick as f32) * secs_per_tick;
        println!("group at {} -- {:?}", time, group);
    }
    /*
        for (ev, event) in .enumerate() {
            time += (event.delta.as_int() as f32) * secs_per_tick;

            match event.kind {
                midly::TrackEventKind::Midi { channel, message } => {
                    println!("-- {} -- MIDI event {:?} {:?}", time, channel, message);
                },
                _ => {
                    println!("-- event ignored: {:?}", event);
                }
            }
        }
    }
    */

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
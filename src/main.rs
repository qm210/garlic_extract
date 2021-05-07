/*
* garlic_extract
* QM / Team210
*
* this extracts the garlic_crust Sequence from a midi file, given as command line argument
*
*/

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

    let mut sequences = std::vec::Vec<std::vec::Vec<SeqEvent>>::new();

    for (t, track) in track_iter.enumerate() {
        println!("------ track {} has {} events", t, track.len());

        for (ev, event) in track.iter().enumerate() {
            println!("-- event {} printed as {:?}", ev, event);
        }
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
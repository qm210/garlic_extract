fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    println!("{}", args.len());

    if args.len() < 2 {
        println!("give midi file as argument!");
        return;
    }

    let midi_bytes = std::fs::read(&args[1]).unwrap();
    let smf = midly::Smf::parse(&midi_bytes).unwrap();

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("track {} has {} events", i, track.len());
    }
}

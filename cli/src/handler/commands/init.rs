use clap::ArgMatches;

pub fn handle(matches: &ArgMatches) -> () {
    // Safe, language is required
    let language = matches.get_one::<String>("language").unwrap();

    println!("Language: {}", language);
    println!("RUUN");
}
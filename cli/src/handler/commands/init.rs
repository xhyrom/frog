use clap::ArgMatches;

pub fn handle(matches: &ArgMatches) -> () {
    let name = matches.get_one::<String>("name").unwrap();
    let directory = matches.get_one::<String>("directory").unwrap();
    let language = matches.get_one::<String>("language").unwrap();

    println!("Name: {}", name);
    println!("Directory: {}", directory);
    println!("Language: {}", language);
}
use clap::{ App, load_yaml };

fn main() {
    println!("Hello, world!");
    let yaml = load_yaml!("chimp.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(config) = matches.value_of("config") {
        println!("Found custom config: {}", config);
    } 

    match matches.subcommand() {
        Some(("code", code_cmd)) => {
            println!("Running code stuff");
            code_handler(code_cmd);
        },
        _ => println!("Unknown command"),
    }
}

fn code_handler(args: &clap::ArgMatches) {
    // let checkoutVal = args.value_of("checkout");

    match args.value_of("checkout") {
        Some(b) => println!("Checking out: {}", b),
        None => println!("nothing to checkout"),
    }

    println!("Done handling the code command...");
}

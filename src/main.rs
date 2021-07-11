use clap::{ App, load_yaml };

use chimp_cli::CodeTraits;

fn main() {
    println!("Hello, world!");
    let yaml = load_yaml!("chimp.yml");
    let matches = App::from(yaml).get_matches();

    if matches.occurrences_of("config") != 0 {
        let config = matches.value_of("config").unwrap();
        println!("Found custom config: {}", config);
    } 

    match matches.subcommand() {
        Some(("code", code_cmd)) => {
            println!("Running code stuff");
            code_handler(code_cmd);
        },
        Some(("issue", issue_cmd)) => {
            println!("Running issue stuff");
            issue_handler(issue_cmd);
        },
        _ => println!("Unknown command"),
    }
}

fn code_handler(args: &clap::ArgMatches) {
    // let checkoutVal = args.value_of("checkout");
    let github = chimp_cli::code::GitHub{ owner: String::from("burtonr") };

    match args.value_of("checkout") {
        Some(b) => {
            println!("Checking out: {}", b);
            let msg = github.clone();
            println!("{}", msg);
        },
        None => println!("nothing to checkout"),
    }

    println!("Done handling the code command...");
}

fn issue_handler(_args: &clap::ArgMatches) {
    println!("Doing issue stuff");
}
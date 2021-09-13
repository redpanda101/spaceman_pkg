mod spaceman_package;
use spaceman_package::*;
use std::env;

#[derive(Copy, Clone)]
struct Options(bool, bool, bool);

fn calls(opts: Options, name: String) {
    let mut lang: Lang = Lang::C;
    if opts.2 { //help flag
        help(opts)
    }

    if opts.0 { //c++ flag
        lang = Lang::CPP;
    }
    
    if opts.1 { //create flag
        SpacemanPackage::new(name, lang).create();
    }
}

fn help(opts: Options){
    println!("help message goes here");
}

fn main() {
    let mut opts = Options(false, false, false); //sets all flags off
    let mut name: String = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        opts.2 = true;
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            "-n" | "new" | "--new" => opts.1 = true,
            "-cpp" | "--lang:cpp" => opts.0 = true,
            "-h" | "help" | "--help" => opts.2 = true,
            _ => name = args[i].clone(),
        }
    }

    calls(opts, name);
}

mod spaceman_package;
use spaceman_package::*;
use std::env;



struct SOptions([bool; 3]);

fn calls(opts: SOptions, name: String) {
    let flags = opts.0;
    let mut lang: Lang = Lang::C;
    if flags[2] {
        help(opts)
    }

    if flags[0] {
        lang = Lang::CPP;
    }
    
    if flags[1] {
        SpacemanPackage::new(name, lang).create();
    }
}

fn help(opts: SOptions){
    println!("help message goes here");
}

fn main() -> std::result::Result<(), i32> {
    let mut opts: SOptions = SOptions([false; 3]); //sets all flags off
    let mut name: String = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        opts.0[2] = true;
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            "-n" | "new" | "--new" => opts.0[1] = true,
            "-cpp" | "--lang:cpp" => opts.0[0] = true,
            "-h" | "help" | "--help" => opts.0[2] = true,
            _ => name = args[i].clone(),
        }
    }

    calls(opts, name);

    Ok(())
}

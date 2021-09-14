#[macro_use]
extern crate ini;

mod spaceman_package;
use spaceman_package::*;
use std::env;

#[derive(Copy, Clone)]
struct Options(bool, bool, bool, bool, bool);

fn calls(opts: Options, name: String) {
    let mut lang: Lang = Lang::C;
    if opts.4 {
        //help flag
        help(opts);
        return ();
    }

    if opts.0 {
        //c++ flag
        lang = Lang::CPP;
    }

    if opts.1 {
        //create flag
        SpacemanPackage::new(name, lang).create();
    }

    else if opts.2 {
        SpacemanPackage::new(name, lang).create_makefiles();
    }

    else if opts.3 {
        SpacemanPackage::new(name, lang).build();
    }
}

fn help(opts: Options) {
    if opts.0 {
        println!("Flag switches the default language");
    } else if opts.1 {
        println!("Flag creates a new project");
    } else if opts.4 {
        println!("Spaceman Package Manager:\n-n | new | --new : creates a new project \n-cpp | --lang:cpp : changes language to c++\n-h | help | --help : prints this message");
    }
}

fn main() {
    let mut opts = Options(false, false, false, false, false); //sets all flags off
    let mut name: String = String::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        opts.4 = true;
    }

    for i in 1..args.len() {
        match args[i].as_str() {
            "-n" | "new" | "--new" => opts.1 = true,
            "-cpp" | "--lang:cpp" => opts.0 = true,
            "-h" | "help" | "--help" => opts.4 = true,
            "-b" | "build" | "--build" => opts.3 = true,
            "-g" | "gen" | "--genetate" => opts.2 = true,
            _ => name = args[i].clone(),
        }
    }

    calls(opts, name);

    println!("done!")
}

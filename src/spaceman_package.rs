use std::fs;
use std::ops::Add; //for string add
//use simple_config_parser::config::Config; //for build, will implement later

///Language enum
pub enum Lang {
    C,
    CPP,
}

///SpacemanPackage struct
///used for containing all the info of a package
pub struct SpacemanPackage {
    name: String,
    language: Lang,
}

impl SpacemanPackage {
    ///Makes a new package
    ///Does not write to drive
    pub fn new(name: String, lang: Lang) -> SpacemanPackage {
        SpacemanPackage {
            name: name,
            language: lang,
        }
    }

    //File Structure of created package
    // (Package Name)
    //      - build
    //      - src
    //          - main.c / main.cc
    //      - spaceman.config


    /// ### `create(&self)`
    /// creates package on file system

    pub fn create(&self) {
        //file templates
        let hello_world_c = "#include <stdio.h>\nint main() {\n    printf(\"hello world\");\n}\0";
        let config_file = format!("name = \"{}\"\ncompiler = \"gnu\"\nmakefile_type = \"gnu\"", self.name);

        //create directories
        fs::create_dir(self.name.as_str()).expect("Could not create dir: "); //create project dir
        fs::create_dir(format!("{}/src", self.name).as_str()).expect("Could not create dir: "); //create source dir
        fs::create_dir(format!("{}/build", self.name).as_str()).expect("Could not create dir: "); //create build dir

        //if language is c, write main.c
        if let Lang::C = self.language {
            fs::write(format!("{}/src/main.c", self.name), hello_world_c).expect("Could not write to main: "); //write to main
        } 

        //else if language is c++, write main.cc
        else {
            fs::write(format!("{}/src/main.cc", self.name), hello_world_c).expect("Could not write to main: "); //write to main
        }
        fs::write(format!("{}/spaceman.config", self.name), config_file).expect("Could not write to config: ");
    }
    pub fn build(&self){
        //TODO: add build
    }
}

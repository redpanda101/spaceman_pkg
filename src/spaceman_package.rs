use std::fs;
use std::ops::Add; //for string add

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

    ///creates package on file system
    pub fn create(&self) {
        let hello_world_c = "#include <stdio.h>\nint main() {\n    printf(\"hello world\");\n}\0"; //hello world in c

        fs::create_dir(self.name.as_str()).expect("Could not create dir: "); //create project dir
        fs::create_dir(self.name.clone().add("/src").as_str()).expect("Could not create dir: "); //create source dir
        fs::create_dir(self.name.clone().add("/build").as_str()).expect("Could not create dir: "); //create build dir

        if let Lang::C = self.language {
            fs::write(self.name.clone().add("/src/main.c"), hello_world_c).expect("Could not write to main: "); //write to main
        } 
        else {
            fs::write(self.name.clone().add("/src/main.cc"), hello_world_c).expect("Could not write to main: "); //write to main
        }
    }
    pub fn build(&self){
        //TODO: add build
    }
}

use std::fs;
use std::process::Command;

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

    fn get_file_ext(&self) -> String {
        if let Lang::C = self.language {
            ".c".to_string()
        } else {
            ".cc".to_string()
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
        let hello_world_c = "#include <stdio.h>\nint main() {\n    printf(\"hello world\");\n}";
        let config_file = format!(
            "[spaceman_package]\nname={}\ncompiler=gnu\ndeps=\nlanguage={}",
            self.name,
            self.get_file_ext()
        );

        //create directories
        fs::create_dir(self.name.as_str()).expect("Could not create dir: "); //create project dir
        fs::create_dir(format!("{}/src", self.name).as_str()).expect("Could not create dir: "); //create source dir
        fs::create_dir(format!("{}/build", self.name).as_str()).expect("Could not create dir: "); //create build dir

        fs::write(
            format!("{}/src/main{}", self.name, self.get_file_ext()),
            hello_world_c,
        )
        .expect("Could not write to main: ");
        fs::write(format!("{}/spaceman.ini", self.name), config_file)
            .expect("Could not write to config: ");
    }

    pub fn create_makefiles(&self) {
        let config = ini!(format!("{}/spaceman.ini", self.name).as_str());
        let s_config = config.get("spaceman_package").unwrap();
        let compiler = s_config.get("compiler").unwrap().as_ref().unwrap();
        //let deps = config.get("deps").unwrap();
        let lang = s_config.get("language").unwrap().as_ref().unwrap();

        let compiler_cmd = match compiler.as_str() {
            "gcc" | "g++" | "gnu" => {
                if lang == ".c" {
                    "gcc"
                } else {
                    "g++"
                }
            }
            "clang" | "clang++" | "llvm" => {
                if lang == ".c" {
                    "clang"
                } else {
                    "clang++"
                }
            }
            _ => "gcc",
        };

        //write to makefile
        fs::write(
            format!("{}/build/Makefile", self.name),
            format!(//contents of makefile
                "debug:\n\t{} -o {} -c ../src/main{}\nrelease:\n\t{} -o {} -c ../src/main{}\ninstall:\n\tmake release\n\tcp build/{} /usr/bin/{}",
                compiler_cmd,
                self.name,
                self.get_file_ext(),
                compiler_cmd, //copied again for release
                self.name,
                self.get_file_ext(),
                self.name, self.name
            ),
        )
        .expect("Could not write Makefile: ");
    }

    pub fn build(&self) {
        self.create_makefiles();
        Command::new("make")
            .arg("debug")
            .arg(format!("--directory={}/build", self.name))
            .spawn()
            .expect("could not build program: ");
    }
}

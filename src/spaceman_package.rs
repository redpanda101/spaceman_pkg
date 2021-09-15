use std::fs;
use std::process::Command;

///Language enum
pub enum Lang {
    C,
    Cpp,
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
    pub fn new(pkg_name: String, lang: Lang) -> SpacemanPackage {
        SpacemanPackage {
            name: pkg_name,
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
        let hello_world_c = "#include <stdio.h>\nint main() {\n    printf(\"hello world\\n\");\n}";
        let config_file = format!(
            "[package]\nname={}\ncompiler=gnu\nlanguage={}\n; local dependancies for linking\ndeps=\n[files]\nmain=src/main{}\nbuild_files=\n",
            self.name,
            self.get_file_ext(),
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

    /// ### `create_makefiles(&self)`
    /// generates the makefiles in an already existing package
    pub fn create_makefiles(&self) {
        //load config gile
        let config = ini!(format!("{}/spaceman.ini", self.name).as_str());
        let s_config = config.get("package").unwrap();

        //get package info
        //note, the two unwraps are annoying but necessary
        let pkg_name = s_config.get("name").unwrap().as_ref().unwrap();
        let compiler = s_config.get("compiler").unwrap().as_ref().unwrap();
        let deps = s_config.get("deps").unwrap().as_ref().unwrap();
        let lang = s_config.get("language").unwrap().as_ref().unwrap();
        let files = config.get("files").unwrap();

        //more specific file stuff
        let main_file = files.get("main").unwrap().as_ref().unwrap();
        let mut build_files = String::new();

        //gets build files and splits at a space
        for file in files.get("build_files").unwrap().as_ref().unwrap().split(" ") {
            match file {
                "" | " " => (),
                _ => build_files.push_str(format!("../{} ", file).as_str()),
            }
        }
    
        //string to be used for linking
        let mut dependancy_link_string = String::new();

        //get dependencies
        for dep in deps.split(" ") {
            match dep {
                "" | " " => (),
                _ => dependancy_link_string.push_str(format!("-L{}", dep).as_str()),
            }
        }

        //Get compiler to use
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
                "debug:\n\t{0} -o {1} -g ../{2} {3} {4} \nrelease:\n\t{} -o {0} -O2 ../{2} {3} {4}\ninstall:\n\tmake release\n\tcp ./{0} /usr/bin/{0}\n",
                compiler_cmd,
                pkg_name,
                main_file,
                build_files,
                dependancy_link_string
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

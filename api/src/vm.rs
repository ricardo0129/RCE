use std::process::{Command, Stdio};
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub enum Language {
    Python,
    Cpp,
    Java,
}

fn language_extension(lang: &Language) -> &'static str {
    match lang {
        Language::Python => "py",
        Language::Cpp => "cpp",
        Language::Java => "java",
    }
}

fn ext_lang(lang: &str) -> Language {
    match lang {
        "python" => Language::Python,
        "cpp" => Language::Cpp,
        "java" => Language::Java,
        _ => Language::Java,
    }
}

fn language_compiler(lang: &Language) -> &'static str {
    match lang {
        Language::Python => "python3",
        Language::Cpp => "g++",
        Language::Java => "javac",
    }
}

fn language_runcommand(lang: &Language) -> &'static str {
    match lang {
        Language::Python => "",
        Language::Cpp => "./a.out",
        Language::Java => "java Main",
    }
}

pub enum Status {
    Timeout,
    Successful,
    CompileError,
    RuntimeError,
}

pub struct VM {
    id: u32,
    lang: Language,
    code: String,
    input: String,
}

impl VM {
    pub fn new(lang: &String, code: &String, input: &String) -> Self {
        let mut rng = rand::thread_rng();
        Self {id: rng.gen_range(0..10000), lang: ext_lang(lang.as_str()), code: code.clone(), input: input.clone()}
    }

    fn get_filename(&self) -> String {
        format!("main.{}", language_extension(&self.lang))
    }

    fn get_vm_dir(&self) -> String {
        format!("../dock/temp/{}", &self.id)
    }
    
    fn setup(&self) {
        println!("created vm {}", &self.id);
        let vm_dir = self.get_vm_dir();
        Command::new("mkdir")
                .arg(&vm_dir)
                .spawn()
                .expect("failed to create directory").wait().unwrap();
        Command::new("cp")
                .arg("../dock/scripts/script.sh")
                .arg(&vm_dir)
                .spawn()
                .expect("failed to copy script").wait().unwrap();

        let file_name = self.get_filename();
        let mut code_file = File::create(vm_dir.clone()+"/"+&file_name).unwrap();
        code_file.write_all(&self.code.as_bytes()).unwrap();

        let mut input_file = File::create(vm_dir.clone()+"/inputFile").unwrap();
        input_file.write_all(&self.input.as_bytes()).unwrap();
    }

    pub fn execute(&self) -> (Status, String) {
        //returns status of code and result if sucessful execution
        self.setup();

        Command::new("sh")
                .arg("../dock/setup.sh")
                .arg(self.get_vm_dir())
                .arg(language_compiler(&self.lang))
                .arg(self.get_filename())
                .arg(language_runcommand(&self.lang))
                .spawn()
                .expect("Could not start up VM").wait().unwrap();
        
        let mut output: String = String::new();

        if Path::new(&(self.get_vm_dir()+"/completed")).exists() {
            let res = Command::new("cat")
                              .arg(self.get_vm_dir()+"/completed") 
                              .output().unwrap();

            output = String::from_utf8(res.stdout).unwrap();
            //println!("result: {}", String::from_utf8(res.stdout).unwrap());
        }

        self.clean_up();

        (Status::Timeout, output)
    }

    fn clean_up(&self) {
        Command::new("rm")
                .arg("-r")
                .arg(self.get_vm_dir())
                .spawn().expect("Could not clean up folders");
    }
}

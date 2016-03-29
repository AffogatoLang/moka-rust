use common::util::ConfigurableProgram;
use common::util::ProgramFragment;

use std::option::Option;

pub struct ParseBuilder{
    is_verbose : bool,
    is_archive : bool,
    module_path : Option<String>,
    input_path : Option<String>,
    output_path : Option<String>
}

impl ParseBuilder {
    pub fn new() -> ParseBuilder {
        ParseBuilder {
            is_verbose : false,
            is_archive : false,
            module_path : Option::None,
            input_path : Option::None,
            output_path : Option::None
        }
    }
}

impl ConfigurableProgram<ParseRunner> for ParseBuilder {
    fn set_flag(&mut self, name:String, value:bool) -> &mut Self {
        let n:&str = &name;
        match n {
            "verbose" => self.is_verbose = value,
            "archive" => self.is_archive = value,
            _ => ()
        };
        self
    }
    fn set_arg(&mut self, name:String, value:String) -> &mut Self {
        let n:&str = &name;
        match n {
            "module" => self.module_path = Option::Some(value),
            "input" => self.input_path = Option::Some(value),
            "output" => self.output_path = Option::Some(value),
            _ => ()
        };
        self
    }
    fn config(&self) -> ParseRunner {
        let module = match self.module_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'module' def")
        };

        let input = match self.input_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'input' def")
        };

        let output = match self.output_path {
            Some(ref value) => value,
            None => panic!("Incorrectly configured ParseRunner, missing 'output' def")
        };

        ParseRunner {
            is_verbose: self.is_verbose,
            is_archive: self.is_archive,
            module_path: module.clone(),
            input_path: input.clone(),
            output_path: output.clone()
        }
    }
}

pub struct ParseRunner {
    is_verbose : bool,
    is_archive : bool,
    module_path : String,
    input_path : String,
    output_path : String
}

impl ProgramFragment for ParseRunner {
    fn run(&self) -> Result<(), String> {
        Result::Ok(())
    }
}
extern crate rustc_serialize;
extern crate docopt;
extern crate regex;
extern crate moka;
extern crate toml;

use docopt::Docopt;

use moka::compile::CompileBuilder;
use moka::parse::ParseBuilder;

use moka::common::python;

use moka::common::util::ConfigurableProgram;
use moka::common::util::ProgramFragment;

const MOKA_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE : &'static str = "
Compile or run a Moka module

Usage:
    moka [-va] use <module> <input> <output>
    moka [-va] compile <module> <output>
    moka -h
    moka --version

Options:
    -a, --archive   The specified module is an archive instead of a folder
    -h, --help      Show this text
    -v, --verbose   Enable verbose output
    --version       Show the installed Moka version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_archive: bool,
    flag_verbose: bool,
    flag_version: bool,
    flag_help: bool,

    cmd_use: bool,
    cmd_compile: bool,

    arg_module: String,
    arg_input: String,
    arg_output: String
}

fn main() {

    let mut path_to_pyth = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    path_to_pyth.push("resources");
    println!("{:?}", path_to_pyth);
    let pyth = python::run_file("./resources/py_env/interp_runner.py", vec![path_to_pyth.to_str().unwrap(), "{\"this\": []}"])
        .unwrap();
    println!("Out {}", String::from_utf8(pyth.stdout).unwrap());
    println!("Err {}", String::from_utf8(pyth.stderr).unwrap());

    
    let toml = r#"
    [meta]
    name= "Announcejs"
    version= "0.1.0"
    author= "Louis Capitanchik"
    license= "BSD 3-Clause"
    [options]
    core= "Announcejs"
    stripwhitespace= true
    "#;

    let value = toml::Parser::new(toml).parse().unwrap();
    println!("{:?}", value);

    let args: Args = Docopt::new(USAGE)
                            .and_then(|opts| opts.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("v{}", MOKA_VERSION);
        std::process::exit(0);
    }

    if args.flag_help {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    if args.flag_archive {
        println!("Archive format is not currently implemented");
        std::process::exit(1);
    }

    let result = if args.cmd_use {
        setup_and_use_parse(args)
    } else if args.cmd_compile {
        setup_and_use_compile(args)
    } else {
        Result::Err("No Such Command Currently Not Implemented".to_string())
    };

    match result {
        Err(e) => println!("{}", e),
        _ => ()
    }
}

fn setup_and_use_compile(args: Args) -> Result<(), String> {
    let compile_runner = CompileBuilder::new()
        .set_flag("verbose", args.flag_verbose)
        .set_flag("archive", args.flag_archive)
         // Unwrap is fine as cannot possibly be none
        .set_arg("module", args.arg_module)
        .set_arg("output", args.arg_output)
        .config();

    compile_runner.run()
}

fn setup_and_use_parse(args:Args) -> Result<(), String> {
    let parse_runner = ParseBuilder::new()
        .set_flag("verbose", args.flag_verbose)
        .set_flag("archive", args.flag_archive)
         // Unwrap is fine as cannot possibly be none
        .set_arg("module", args.arg_module)
        .set_arg("input", args.arg_input)
        .set_arg("output", args.arg_output)
        .config();

    parse_runner.run()
}

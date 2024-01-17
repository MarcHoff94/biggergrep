
use std::env;
use std::process;
use minigrep::Config;


fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    let curr_dir = &config.directory_path;
    if env::set_current_dir(curr_dir).is_err(){
        eprintln!("could not set current dir")
    };

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}


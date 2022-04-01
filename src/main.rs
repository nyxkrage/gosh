use goscript_engine as engine;
use std::env::args;
use std::path::Path;

fn run<P: AsRef<Path>>(path: P) -> usize {
    let mut std_path = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    std_path.push(std::env::var("GOSH_STD_PREFIX").unwrap_or("".to_string()));
    std_path.push("std");
    
    let path = path.as_ref().canonicalize().unwrap();
    
    let cfg = engine::Config {
        work_dir: Some("/".to_owned()),
        base_path: Some(std_path.canonicalize().unwrap().to_string_lossy().to_string()),
        trace_parser: false,
        trace_checker: false,
        trace_vm: false,
    };
    let mut engine = engine::Engine::new(cfg);

    engine.run(path.to_string_lossy().as_ref())
}

fn main() {
    let path = args().skip(1).collect::<String>();
    
    run(&path);
}

use super::*;
use std::{fs, process};

pub fn error_exit(msg: &str) -> ! {
    eprintln!("error: {}", msg);
    process::exit(1)
}

pub fn error_exit_with_src(msg: &str, src: &Option<Span>) -> ! {
    let mut str = String::default();
    str += msg;
    str += "\n";
    match src {
        None => {}
        Some(v) => {
            str += "\n";
            str += &v.to_string();
        }
    };
    error_exit(&str)
}

pub fn error_exit_with_srcs(msg: &str, srcs: &[&Option<Span>]) -> ! {
    let mut str = String::default();
    str += msg;
    str += "\n";
    for src in srcs {
        match src {
            None => {}
            Some(v) => {
                str += "\n";
                str += &v.to_string();
            }
        }
    }

    error_exit(&str)
}

pub fn temporary_source_name(file_name: &str, hash: &str) -> String {
    format!("{}.{}.fix", file_name, hash)
}

pub fn temporary_source_path(file_name: &str, hash: &str) -> PathBuf {
    let file_name = temporary_source_name(file_name, hash);
    PathBuf::from(INTERMEDIATE_PATH).join(file_name)
}

pub fn check_temporary_source(file_name: &str, hash: &str) -> bool {
    temporary_source_path(file_name, hash).exists()
}

pub fn save_temporary_source(source: &str, file_name: &str, hash: &str) {
    let path = temporary_source_path(file_name, hash);
    fs::create_dir_all(INTERMEDIATE_PATH).expect("Failed to create intermediate directory.");
    fs::write(path, source).expect(&format!("Failed to generate temporary file {}", file_name));
}

use std::io::{ self, Write, };
use std::path::{ Path, PathBuf, };

pub const OK: &str = "\x1b[32mok\x1b[0m";
pub const FAILED: &str = "\x1b[32mfailed\x1b[0m";


// Interpreting Test262 Tests
// https://github.com/tc39/test262/blob/master/INTERPRETING.md


fn read_dir<P: AsRef<Path>>(path: P, files: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    for dir_entry_res in std::fs::read_dir(path)? {
        let dir_entry = dir_entry_res?;

        let file_path = dir_entry.path();
        let file_type = dir_entry.file_type()?;
        
        if file_type.is_symlink() {
            // drop
            continue;
        }

        if file_type.is_dir() {
            read_dir(file_path.as_path(), files)?;
        }

        if file_type.is_file() {
            files.push(file_path);
        }
    }

    Ok(())
}

fn dir_files<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = vec![];
    
    if let Err(e) = read_dir(path, &mut files) {
        panic!("{:?}", e);
    }

    files
}

fn run<P: AsRef<Path>>(filepath: P) -> Result<(), ()> {
    let _content = match std::fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(e) => panic!("{:?}", e),
    };

    // TODO: exec source code.
    // "negative: SyntaxError" || "negative: ReferenceError" || "es6id" || "arrow-function"

    Ok(())
}

fn test_subject(name: &str, files: Vec<PathBuf>, ret: &mut bool) {
    // Red: \x1b[31mok\x1b[0m
    // Green: \x1b[32mok\x1b[0m
    let _ = io::stdout().write(format!("Subject: {}\n", name).as_bytes());
    
    for filepath in files {
        match run(filepath.clone()) {
            Ok(_) => {
                // let _ = io::stdout().write(format!("\t{} ... {}\n", filepath.display(), OK).as_bytes());
            },
            Err(e) => {
                let _ = io::stdout().write(format!("\t{} ... {}\n", filepath.display(), FAILED).as_bytes());
                let _ = io::stderr().write(format!("{:?}\n", e).as_bytes());

                if *ret {
                    *ret = false;
                }
            }
        }
    }
}

#[test]
fn test262() {
    let mut ok: bool = true;

    test_subject("test262::test::annexB", dir_files("./test262/test/annexB"), &mut ok);
    test_subject("test262::test::built-ins", dir_files("./test262/test/built-ins"), &mut ok);
    test_subject("test262::test::intl402", dir_files("./test262/test/intl402"), &mut ok);
    test_subject("test262::test::language", dir_files("./test262/test/language"), &mut ok);

    assert_eq!(ok, true);
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, stdin, stdout};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let result = if args.len() == 1 || (args.len() == 2 && args[1] == "-") {
        // Read from stdin
        run_nodecode(BufReader::new(stdin()))
    } else if args.len() == 2 {
        // Read from file
        let file_path = &args[1];
        match File::open(file_path) {
            Ok(file) => run_nodecode(BufReader::new(file)),
            Err(e) => {
                eprintln!("Error opening file '{}': {}", file_path, e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        eprintln!("Usage: {} [<file_path> | -]", args[0]);
        return ExitCode::FAILURE;
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run_nodecode<R: BufRead>(mut reader: R) -> Result<(), io::Error> {
    let mut last_line = Vec::new();
    let mut current_line = Vec::new();

    loop {
        current_line.clear();
        let bytes_read = reader.read_until(b'\n', &mut current_line)?;
        if bytes_read == 0 {
            break;
        }
        std::mem::swap(&mut last_line, &mut current_line);
    }

    let mut handle = stdout().lock();
    handle.write_all(&last_line)?;

    Ok(())
}

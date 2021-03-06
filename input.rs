use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::fmt::Debug;

pub fn read_input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

pub fn read_input_int() -> isize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap_or(-1);
}

pub fn read_input_until_empty() -> Vec<String> {
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let ln = line.unwrap_or_default();

        if ln.is_empty() {
            return lines;
        }

        lines.push(ln.trim().parse().unwrap());
    }

    return lines;
}

pub fn read_input_until_empty_as<T>() -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    let stdin = io::stdin();

    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let ln = line.unwrap_or_default();

        if ln.is_empty() {
            return lines;
        }

        let parsed= ln.trim().parse();
        if parsed.is_ok() {
            lines.push(parsed.unwrap());
        }
    }

    return lines;
}

pub fn read_input_indefinitely(function: fn(str: String)) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        if line.is_err() { println!("Line input resulted in error") }
        function(line.unwrap_or_default());
    }
}
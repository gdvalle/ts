#![feature(alloc_system)]
extern crate alloc_system;

extern crate chrono;

use std::env;
use std::io::{self, BufRead};

fn echo(time_fmt: &String, s: &String) {
    let now = chrono::Local::now();
    println!("{} {}", now.format(&time_fmt).to_string(), &s);
}

fn main() {
    let time_fmt = match env::var("TS_FORMAT") {
        Ok(v) => v,
        Err(_) => String::from("%Y-%m-%d %H:%M:%S%.3f"),
    };

    let stdin = io::stdin();
    loop {
        match stdin.lock().lines().next() {
            Some(l) => echo(&time_fmt, &l.unwrap()),
            None => break,
        }
    }
}

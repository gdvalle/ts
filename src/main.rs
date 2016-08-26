#![feature(alloc_system)]
extern crate alloc_system;

extern crate chrono;

use std::io::{self,BufRead};

fn echo(s: String) {
    let now = chrono::Local::now();
    println!("{} {}", now.format("%Y-%m-%d %H:%M:%S%.3f").to_string(), s);
}

fn main() {
    let stdin = io::stdin();
    loop {
        match stdin.lock().lines().next() {
            Some(l) => echo(l.unwrap()),
            None => break,
        }
    }
}


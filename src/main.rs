#![feature(test)]

extern crate chrono;
extern crate test;

use std::env;
use std::io::{self, BufRead, Write};

const DEFAULT_TIME_FMT: &str = "%Y-%m-%d %H:%M:%S%.3f";

fn echo(handle: &mut dyn Write, time_fmt: &String, s: &String) -> std::io::Result<()> {
    let now = chrono::Local::now();
    handle.write_all(now.format(&time_fmt).to_string().as_bytes())?;
    handle.write_all(b" ")?;
    handle.write_all(s.as_bytes())?;
    handle.write_all(b"\n")?;
    handle.flush()?;
    Ok(())
}

fn main() {
    let time_fmt = match env::var("TS_FORMAT") {
        Ok(v) => v,
        Err(_) => DEFAULT_TIME_FMT.to_string(),
    };

    let stdout = io::stdout();
    let mut writer = stdout.lock();

    for line in io::stdin().lock().lines() {
        echo(&mut writer, &time_fmt, &line.unwrap()).expect("failed writing to stdout");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::fs::File;

    #[bench]
    fn bench_echo(b: &mut Bencher) {
        let mut handle = File::open("/dev/null").unwrap();
        let line: String = std::iter::repeat("X").take(1000).collect();
        b.iter(|| echo(&mut handle, &DEFAULT_TIME_FMT.to_string(), &line));
    }
}
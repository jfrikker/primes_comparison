extern crate time;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use time::precise_time_ns;

fn report_execution(name: &str, cmdline: &Vec<&str>) {
    let count = count_primes(name, cmdline);
    print!("{:7} found {} primes", name, count);
    let elapsed = time_execution(name, cmdline);
    print!(" and took {:>5.2} seconds.\n", elapsed);
}

fn time_execution(name: &str, cmdline: &Vec<&str>) -> f64 {
    let start = precise_time_ns();
    let mut child = Command::new(cmdline[0])
        .args(&cmdline[1..])
        .arg("10000000")
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!("Failed to run {}", name));

    let exitcode = child.wait().expect("Failed to obtain child's exit code");
    if !exitcode.success() {
        panic!("Process {} exited with code {}.", name, exitcode);
    }
    let end = precise_time_ns();
    (end as f64 - start as f64) / 1000000000f64
}

fn count_primes(name: &str, cmdline: &Vec<&str>) -> usize {
    let mut child = Command::new(cmdline[0])
        .args(&cmdline[1..])
        .arg("10000000")
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to run {}", name));

    let stdout = child.stdout.as_mut().expect("Failed to obtain stdout handle");
    count_lines(&mut BufReader::new(stdout))
}

fn count_lines<T>(stream: &mut T) -> usize
    where T: BufRead {
    stream.lines().count()
}

fn main() {
    report_execution("C++", &vec!("./primes_cc"));
    report_execution("Rust", &vec!("./primes_rs"));
    report_execution("Java", &vec!("java", "Primes"));
    report_execution("Haskell", &vec!("./primes_hs"));
    report_execution("Python", &vec!("python", "primes.py"));
}

.PHONY: all

all : primes_hs primes_rs Primes.class primes_cc timing_report

primes_hs : primes.hs
	ghc -O3 -o primes_hs primes.hs

primes_rs : primes.rs
	rustc -O -o primes_rs primes.rs

Primes.class : Primes.java
	javac Primes.java

primes_cc : primes.cc
	g++ -O3 -o primes_cc primes.cc

timing_report_src/target/release/timing_report: timing_report_src/Cargo.toml timing_report_src/src/main.rs
	(cd timing_report_src && cargo build --release)

timing_report: timing_report_src/target/release/timing_report
	cp timing_report_src/target/release/timing_report timing_report

.PHONY: all

all : primes_hs primes_rs Primes.class primes_cc

primes_hs : primes.hs
	ghc -O3 -o primes_hs primes.hs

primes_rs : primes.rs
	rustc -O -o primes_rs primes.rs

Primes.class : Primes.java
	javac Primes.java

primes_cc : primes.cc
	g++ -O3 -o primes_cc primes.cc

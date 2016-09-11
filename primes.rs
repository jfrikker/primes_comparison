use std::cell::{Cell, RefCell};
use std::env::args;
use std::ops::{Add, Mul, Rem};

struct Primes<I> {
    primes : RefCell<Vec<I>>,
    max : Cell<I>
}

impl<I: Copy +
        From<u8> + 
        Add<I, Output = I> +
        Mul<I, Output = I> +
        Rem<I, Output = I> +
        Ord> Primes<I> {
    fn new() -> Primes<I> {
        Primes {
            primes: RefCell::new(vec!(I::from(2u8))),
            max: Cell::new(I::from(2u8))
        }
    }

    fn is_prime(&self, n: I) -> bool {
        while self.max.get() * self.max.get() < n {
            self.expand();
        }

        let primes = self.primes.borrow();
        if self.max.get() >= n {
            primes.binary_search(&n).is_ok()
        } else {
            match primes.iter()
                  .take_while(|&&i| i * i <= n)
                  .filter(|&&i| n % i == I::from(0))
                  .next() {
                Some(_) => false,
                None => true
            }
        }
    }

    fn expand(&self) {
        let max = self.max.get() + I::from(1);
        if self.is_prime(max) {
            let mut primes = self.primes.borrow_mut();
            primes.push(max);
        }
        self.max.set(max);
    }
}

fn main() {
    let mut args = args();
    args.next();
    let n: u32 = args.next().unwrap().parse().unwrap();
    let primes = Primes::new();
    for i in (1..n+1).filter(|i| primes.is_prime(*i)) {
        print!("{}\n", i);
    }
}

use std::cell::{Cell, RefCell};
use std::env::{args};

struct Primes {
    primes : RefCell<Vec<u32>>,
    max : Cell<u32>
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: RefCell::new(vec!(2)),
            max: Cell::new(2)
        }
    }

    fn is_prime(&self, n: u32) -> bool {
        while self.max.get() * self.max.get() < n {
            self.expand();
        }

        let primes = self.primes.borrow();
        if self.max.get() >= n {
            primes.binary_search(&n).is_ok()
        } else {
            for i in primes.iter().cloned() {
                if i * i > n {
                    return true;
                } else if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }

    fn expand(&self) {
        let max = self.max.get() + 1;
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

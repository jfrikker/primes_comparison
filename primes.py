from bisect import bisect_left
import sys

class Primes(object):
    def __init__(self):
        self._primes = [2]
        self._max = 2

    def is_prime(self, n):
        while self._max * self._max < n:
            self.expand()

        if self._max >= n:
            return self._primes[bisect_left(self._primes, n)] == n
        
        for i in self._primes:
            if i * i > n:
                return True
            if n % i == 0:
                return False
        return True

    def expand(self):
        if self.is_prime(self._max + 1):
            self._primes.append(self._max + 1)
        self._max += 1

primes = Primes()
for i in xrange(2, int(sys.argv[1])):
    if primes.is_prime(i):
        print i

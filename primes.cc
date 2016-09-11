#include <algorithm>
#include <iostream>
#include <stdint.h>
#include <vector>

template <class I>
class Primes {
  public:
    Primes();
    bool isPrime(I n) const;

  private:
    bool isPrimeInternal(I n);
    void expand();

    std::vector<I> primes;
    I max;
};

template <class I>
Primes<I>::Primes() {
  primes.push_back(2);
  max = 2;
}

template <class I>
void Primes<I>::expand() {
  if (isPrime(max + 1)) {
    primes.push_back(max + 1);
  }
  max++;
}

template <class I>
bool Primes<I>::isPrime(I n) const {
  return const_cast<Primes *>(this)->isPrimeInternal(n);
}

template <class I>
bool Primes<I>::isPrimeInternal(I n) {
  while (max * max < n) {
    expand();
  }

  if (max >= n) {
    return std::binary_search(primes.begin(), primes.end(), n);
  } else {
    for (std::vector<uint32_t>::iterator it = primes.begin(); it != primes.end(); it++) {
      uint32_t i = *it;
      if (i * i > n) {
        return true;
      } else if (n % i == 0) {
        return false;
      }
    }
    return true;
  }
}

int main(int argc, char** argv) {
  uint32_t n = atoi(argv[1]);
  Primes<uint32_t> primes;
  for (uint32_t i = 1; i <= n; i++) {
    if (primes.isPrime(i)) {
      std::cout << i << std::endl;
    }
  }
}

#include <algorithm>
#include <iostream>
#include <stdint.h>
#include <vector>

class Primes {
  public:
    Primes();
    bool isPrime(uint32_t n) const;

  private:
    bool isPrimeInternal(uint32_t n);
    void expand();

    std::vector<uint32_t> primes;
    uint32_t max;
};

Primes::Primes() {
  primes.push_back(2);
  max = 2;
}

void Primes::expand() {
  if (isPrime(max + 1)) {
    primes.push_back(max + 1);
  }
  max++;
}

bool Primes::isPrime(uint32_t n) const {
  return const_cast<Primes *>(this)->isPrimeInternal(n);
}

bool Primes::isPrimeInternal(uint32_t n) {
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
  Primes primes;
  for (uint32_t i = 1; i <= n; i++) {
    if (primes.isPrime(i)) {
      std::cout << i << std::endl;
    }
  }
}

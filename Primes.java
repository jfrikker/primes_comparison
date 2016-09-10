import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

class Primes {
  private List<Integer> primes = new ArrayList<>();
  private int max = 1;

  public Primes() {
    primes.add(2);
    max = 2;
  }

  public boolean isPrime(int n) {
    while (max * max < n) {
      expand();
    }

    if (max >= n) {
      return Collections.binarySearch(primes, n) >= 0;
    } else {
      for (int i : primes) {
        if (i * i > n) {
          return true;
        } else if (n % i == 0) {
          return false;
        }
      }

      return true;
    }
  }

  private void expand() {
    if (isPrime(max + 1)) {
      primes.add(max + 1);
    }
    max++;
  }

  public static void main(String[] argv) {
    int n = Integer.parseInt(argv[0]);
    Primes primes = new Primes();
    for (int i = 1; i <= n; i++) {
      if (primes.isPrime(i)) {
        System.out.println("" + i);
      }
    }
  }
}

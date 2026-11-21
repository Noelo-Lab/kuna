// Control-flow shapes for the K2 statement-decomposition smoke test.
// Regenerate with tests/data/k2_graph_oracle/gen_classes.sh.

import java.util.Iterator;
import java.util.List;

public class K2Shapes {
  private int field;
  private final Object lock = new Object();

  public int straight(int a, int b) {
    int c = a + b;
    return c * 2;
  }

  public int ifElse(int a) {
    if (a > 0) {
      a = a * 2;
    } else {
      a = -a;
    }
    return a;
  }

  public int whileLoop(int n) {
    int s = 0;
    while (n > 0) {
      s += n;
      n--;
    }
    return s;
  }

  public int doWhile(int n) {
    int s = 0;
    do {
      s += n;
    } while (--n > 0);
    return s;
  }

  public int forLabeled(int[][] grid) {
    int total = 0;
    outer:
    for (int i = 0; i < grid.length; i++) {
      for (int j = 0; j < grid[i].length; j++) {
        if (grid[i][j] < 0) {
          continue outer;
        }
        if (grid[i][j] == 99) {
          break outer;
        }
        total += grid[i][j];
      }
    }
    return total;
  }

  public String tableSwitch(int k) {
    switch (k) {
      case 0:
        return "zero";
      case 1:
      case 2:
        return "small";
      case 3:
        k++;
      default:
        return "big" + k;
    }
  }

  public int lookupSwitch(int k) {
    int r;
    switch (k) {
      case 10: r = 1; break;
      case 1000: r = 2; break;
      case 100000: r = 3; break;
      default: r = 0;
    }
    return r;
  }

  public int tryCatch(String s) {
    try {
      return Integer.parseInt(s);
    } catch (NumberFormatException e) {
      return -1;
    }
  }

  public int multiCatch(Object o) {
    try {
      return ((String) o).length();
    } catch (ClassCastException | NullPointerException e) {
      return 0;
    }
  }

  public int tryFinally(int a) {
    try {
      field = a;
      return a / field;
    } finally {
      field = 0;
    }
  }

  public void sync(int a) {
    synchronized (lock) {
      field += a;
    }
  }

  public synchronized void syncMethod() {
    field++;
  }

  public int iterate(List<Integer> xs) {
    int s = 0;
    for (Iterator<Integer> it = xs.iterator(); it.hasNext(); ) {
      Integer x = it.next();
      if (x == null) {
        break;
      }
      s += x;
    }
    return s;
  }

  public boolean shortCircuit(int a, int b) {
    return (a > 0 && b > 0) || (a < -10 && b < -10);
  }

  public int ternary(int a) {
    return a > 0 ? a : (a < -5 ? -a : 0);
  }

  public void throwing(int a) {
    if (a < 0) {
      throw new IllegalArgumentException("negative");
    }
    field = a;
  }

  public int nestedTry(String a, String b) {
    int r = 0;
    try {
      r = Integer.parseInt(a);
      try {
        r += Integer.parseInt(b);
      } catch (NumberFormatException e) {
        r = -2;
      }
    } catch (RuntimeException e) {
      r = -1;
    } finally {
      field = r;
    }
    return r;
  }

  public int infinite() {
    int i = 0;
    while (true) {
      if (++i > 100) {
        return i;
      }
    }
  }

  public int stringSwitch(String s) {
    switch (s) {
      case "a": return 1;
      case "b": return 2;
      case "Aa": return 3;
      case "BB": return 4;
      default: return 0;
    }
  }

  public int withResources(java.io.Reader r) throws java.io.IOException {
    try (java.io.BufferedReader br = new java.io.BufferedReader(r)) {
      return br.read();
    }
  }

  public static void main(String[] args) {
    System.out.println(new K2Shapes().ifElse(args.length));
  }
}

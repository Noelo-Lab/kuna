// Inputs for the CFG oracle: control-flow shapes the CFG passes must handle (Java 8 syntax,
// compiled for both --release 8 and --release 21).

import java.io.BufferedReader;
import java.io.IOException;
import java.io.StringReader;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.function.IntSupplier;

public class CfgCases {
  enum Color { RED, GREEN, BLUE }

  private final Object lock = new Object();
  private int counter;
  private static int sideEffect;

  static int straight(int a, int b) {
    int c = a + b;
    return c * 2;
  }

  static int ifElse(int x) {
    if (x > 0) {
      return 1;
    } else if (x < 0) {
      return -1;
    }
    return 0;
  }

  static boolean shortCircuit(int a, int b, Object o) {
    return (a > 0 && b > 0) || (o != null && a == b) || !(a < b);
  }

  static int ternaries(int a, int b) {
    int m = a > b ? a : b;
    return m > 10 ? (m > 20 ? 2 : 1) : 0;
  }

  static int forLoop(int n) {
    int s = 0;
    for (int i = 0; i < n; i++) {
      s += i;
    }
    return s;
  }

  static int whileLoop(int n) {
    int s = 0;
    while (n > 0) {
      s += n--;
    }
    return s;
  }

  static int doWhile(int n) {
    int s = 0;
    do {
      s += n;
      n -= 2;
    } while (n > 0);
    return s;
  }

  static int nestedLoops(int[][] grid) {
    int s = 0;
    for (int i = 0; i < grid.length; i++) {
      for (int j = 0; j < grid[i].length; j++) {
        if (grid[i][j] < 0) {
          continue;
        }
        if (grid[i][j] > 100) {
          break;
        }
        s += grid[i][j];
      }
    }
    return s;
  }

  static int labeled(int[][] grid) {
    int found = -1;
    outer:
    for (int i = 0; i < grid.length; i++) {
      inner:
      for (int j = 0; j < grid[i].length; j++) {
        if (grid[i][j] == 0) {
          continue outer;
        }
        if (grid[i][j] == 1) {
          found = i;
          break outer;
        }
        if (grid[i][j] == 2) {
          break inner;
        }
      }
    }
    return found;
  }

  static int foreach(List<Integer> xs) {
    int s = 0;
    for (int x : xs) {
      if (x == 7) {
        break;
      }
      s += x;
    }
    return s;
  }

  static int iteratorLoop(List<String> xs) {
    int n = 0;
    Iterator<String> it = xs.iterator();
    while (it.hasNext()) {
      String s = it.next();
      if (s.isEmpty()) {
        it.remove();
        continue;
      }
      n += s.length();
    }
    return n;
  }

  static void infinite() {
    while (true) {
      sideEffect++;
      if (sideEffect > 1000) {
        sideEffect = 0;
      }
    }
  }

  static void emptyInfinite() {
    for (;;) {
    }
  }

  static int infiniteWithReturn(int[] a) {
    int i = 0;
    while (true) {
      if (a[i] == 0) {
        return i;
      }
      i++;
    }
  }

  static int tableSwitch(int x) {
    switch (x) {
      case 0:
        return 10;
      case 1:
      case 2:
        return 20;
      case 3:
        x += 5;
      case 4:
        return x;
      default:
        return -1;
    }
  }

  static int lookupSwitch(int x) {
    int r = 0;
    switch (x) {
      case -100:
        r = 1;
        break;
      case 7:
        r = 2;
        break;
      case 1000:
        r = 3;
      case 99999:
        r += 4;
        break;
    }
    return r;
  }

  static int switchInLoop(int[] xs) {
    int s = 0;
    for (int x : xs) {
      switch (x) {
        case 1:
          continue;
        case 2:
          s += 2;
          break;
        case 3:
          return s;
        default:
          s++;
      }
    }
    return s;
  }

  static int stringSwitch(String s) {
    switch (s) {
      case "alpha":
        return 1;
      case "beta":
      case "gamma":
        return 2;
      case "Aa":
      case "BB":
        return 3;
      default:
        return 0;
    }
  }

  static int enumSwitch(Color c) {
    switch (c) {
      case RED:
        return 1;
      case GREEN:
        return 2;
      default:
        return 3;
    }
  }

  static int tryCatch(String s) {
    try {
      return Integer.parseInt(s);
    } catch (NumberFormatException e) {
      return -1;
    }
  }

  static int multiCatch(String s, int[] a) {
    try {
      return a[Integer.parseInt(s)];
    } catch (NumberFormatException | ArrayIndexOutOfBoundsException e) {
      return -1;
    } catch (RuntimeException e) {
      return -2;
    }
  }

  static int tryFinally(int x) {
    try {
      sideEffect = x;
      return x * 2;
    } finally {
      sideEffect = 0;
    }
  }

  static int tryCatchFinally(String s) {
    int r = 0;
    try {
      r = Integer.parseInt(s);
    } catch (NumberFormatException e) {
      r = -1;
    } finally {
      sideEffect += r;
    }
    return r;
  }

  @SuppressWarnings("finally")
  static int returnInFinally(int x) {
    try {
      if (x > 0) {
        throw new IllegalStateException();
      }
      return 1;
    } finally {
      return 2;
    }
  }

  static int nestedTry(String a, String b) {
    try {
      try {
        return Integer.parseInt(a) / Integer.parseInt(b);
      } catch (ArithmeticException e) {
        return 0;
      } finally {
        sideEffect++;
      }
    } catch (NumberFormatException e) {
      try {
        return Integer.parseInt(b);
      } catch (NumberFormatException e2) {
        return -1;
      }
    } finally {
      sideEffect--;
    }
  }

  static int tryInLoop(String[] xs) {
    int s = 0;
    for (String x : xs) {
      try {
        s += Integer.parseInt(x);
      } catch (NumberFormatException e) {
        continue;
      } finally {
        sideEffect++;
      }
      if (s > 100) {
        break;
      }
    }
    return s;
  }

  static String tryWithResources(String text) throws IOException {
    try (BufferedReader r = new BufferedReader(new StringReader(text))) {
      return r.readLine();
    }
  }

  static int tryWithTwoResources(String a, String b) {
    try (BufferedReader r1 = new BufferedReader(new StringReader(a));
         BufferedReader r2 = new BufferedReader(new StringReader(b))) {
      return r1.readLine().length() + r2.readLine().length();
    } catch (IOException e) {
      return -1;
    }
  }

  void synchronizedBlock() {
    synchronized (lock) {
      counter++;
    }
  }

  int synchronizedReturn(int x) {
    synchronized (lock) {
      if (x > 0) {
        return counter;
      }
      counter += x;
    }
    return -1;
  }

  void nestedSynchronized(Object other) {
    synchronized (lock) {
      synchronized (other) {
        counter++;
        if (counter > 10) {
          throw new IllegalStateException("too many");
        }
      }
    }
  }

  synchronized void synchronizedMethod() {
    counter--;
  }

  void synchronizedInTry() {
    try {
      synchronized (this) {
        counter = Integer.parseInt("12");
      }
    } catch (NumberFormatException e) {
      counter = 0;
    }
  }

  void throwInSync(boolean b) {
    synchronized (lock) {
      while (b) {
        if (counter++ > 5) {
          throw new RuntimeException();
        }
      }
    }
  }

  static int loopInFinally(int[] xs) {
    int s = 0;
    try {
      s = xs[0];
    } finally {
      for (int x : xs) {
        s += x;
      }
    }
    return s;
  }

  static void throwsOnly(int x) {
    if (x < 0) {
      throw new IllegalArgumentException("negative");
    }
    throw new UnsupportedOperationException();
  }

  static int assertions(int x) {
    assert x > 0 : "positive";
    return x;
  }

  static long longs(long a, double d, float f) {
    long r = a > 0L ? a : -a;
    if (d > 1.5) {
      r += (long) d;
    }
    if (f < 0.5f) {
      r -= 3;
    }
    return r;
  }

  static int lambdas(List<Integer> xs) {
    IntSupplier sup = () -> xs.size();
    xs.forEach(x -> sideEffect += x);
    return sup.getAsInt();
  }

  static List<String> collect(String[] xs) {
    List<String> out = new ArrayList<>();
    for (String x : xs) {
      if (x == null) {
        continue;
      }
      out.add(x.trim());
    }
    return out;
  }

  static int breakFromTry(int[] xs) {
    int i = 0;
    while (i < xs.length) {
      try {
        if (xs[i] == 0) {
          break;
        }
        i += 100 / xs[i];
      } catch (ArithmeticException e) {
        i++;
      }
    }
    return i;
  }

  static int catchRethrow(String s) {
    try {
      return Integer.parseInt(s);
    } catch (RuntimeException e) {
      if (s == null) {
        throw e;
      }
      throw new IllegalStateException(e);
    }
  }

  static int switchInTry(int x, String s) {
    try {
      switch (x) {
        case 1:
          return Integer.parseInt(s);
        case 2:
          return s.length();
        default:
          return 0;
      }
    } catch (NumberFormatException e) {
      return -1;
    } finally {
      sideEffect = x;
    }
  }

  static int complexConditions(int a, int b, int c) {
    int r = 0;
    while ((a > 0 || b > 0) && c != 0) {
      if (a > b ? a > c : b > c) {
        r++;
      }
      a--;
      b--;
      c = c > 0 ? c - 1 : c + 1;
    }
    return r;
  }
}

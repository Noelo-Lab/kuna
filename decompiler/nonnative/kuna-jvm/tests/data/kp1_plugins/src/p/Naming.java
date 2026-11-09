package p;

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;

public abstract class Naming {
  public static int st(long a, double b, int c, String d, File f, Class<?> k) {
    int x = c * 2;
    long y = a;
    boolean z = b > 0;
    String w = d + x;
    File g = new File(w);
    return x + (int) y + (z ? 1 : 0) + w.length() + f.getName().length() + k.hashCode() + g.hashCode();
  }

  public void inst(int[] arr, String[][] grid, Object... rest) {
    for (int i = 0; i < arr.length; i++) {
      for (int j = 0; j < grid.length; j++) {
        for (int k = 0; k < rest.length; k++) {
          for (int l = 0; l < 2; l++) {
            for (int m = 0; m < 2; m++) {
              System.out.println(arr[i] + grid[j][0] + rest[k] + l + m);
            }
          }
        }
      }
    }
    List<String> list = new ArrayList<>();
    Map.Entry<String, Integer> e = null;
    Map<String, List<Integer>> map = new HashMap<>();
    list.add(String.valueOf(e) + map);
    char c = 'c';
    byte bt = 1;
    short sh = 2;
    short sh2 = 3;
    float fl = 3f;
    double db = 4.0;
    Character boxed = c;
    Long ol = 4L;
    Byte ob = bt;
    Short os = sh;
    Boolean obool = true;
    Package pk = Naming.class.getPackage();
    Enum<?> en = null;
    Class<?> cl = Naming.class;
    Integer[] boxes = new Integer[2];
    int[][] grid2 = new int[2][2];
    StringBuilder sb = new StringBuilder();
    System.out.println(sb.append(c).append(bt).append(sh).append(sh2).append(fl).append(db).append(boxed).append(ol)
      .append(ob).append(os).append(obool).append(pk).append(en).append(cl).append(boxes.length).append(grid2.length));
  }

  public void lambdas(String s, int n) {
    Runnable r = () -> {
      int q = n + 1;
      String t = s + q;
      System.out.println(t);
    };
    Function<String, Integer> fn = str -> {
      int len = str.length();
      return len + n;
    };
    r.run();
    System.out.println(fn.apply(s));
    Object anon = new Object() {
      @Override
      public String toString() {
        int inner = n;
        String t = s;
        return t + inner;
      }
    };
    class Local {
      int f(int v) {
        int w = v + n;
        return w;
      }
    }
    System.out.println(anon + "" + new Local().f(3));
  }

  public abstract void abs(String name, int count);

  public native void nat(long handle, Object o);

  public static void keywords(Class<?> clazz, Package pack) {
    Object k = clazz;
    Class<?> other = clazz;
    Package p2 = pack;
    Enum<?> e = null;
    System.out.println(k + "" + other + p2 + e);
  }

  public static <T extends Comparable<T>> T generic(T a, T b, List<? extends T> rest) {
    T best = a.compareTo(b) > 0 ? a : b;
    for (T t : rest) {
      if (t.compareTo(best) > 0) {
        best = t;
      }
    }
    return best;
  }

  public class Inner {
    void m(Inner other, Naming outer) {
      Inner self = this;
      Map.Entry<Inner, Naming> pair = null;
      System.out.println(self + "" + other + outer + pair);
    }
  }
}

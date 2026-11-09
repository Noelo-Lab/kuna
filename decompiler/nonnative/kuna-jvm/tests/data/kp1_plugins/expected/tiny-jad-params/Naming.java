package p;

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.function.Function;

public abstract class Naming {
   public static int st(long a, double b, int c, String d, File f, Class<?> k) {
      int i = c * 2;
      long l = a;
      boolean bl = b > 0.0;
      String string = d + i;
      File file = new File(string);
      return i + (int)l + (bl ? 1 : 0) + string.length() + f.getName().length() + k.hashCode() + file.hashCode();
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

      List<String> list = new ArrayList();
      Entry<String, Integer> entry = null;
      Map<String, List<Integer>> map = new HashMap();
      list.add(String.valueOf(entry) + map);
      char c = 'c';
      byte b = 1;
      short s = 2;
      short t = 3;
      float f = 3.0F;
      double d = 4.0;
      Character character = c;
      Long long_ = 4L;
      Byte byte_ = b;
      Short short1 = s;
      Boolean boolean_ = true;
      Package package_ = Naming.class.getPackage();
      Enum<?> enum_ = null;
      Class<?> class_ = Naming.class;
      Integer[] integers = new Integer[2];
      int[][] n = new int[2][2];
      StringBuilder stringBuilder = new StringBuilder();
      System.out
         .println(
            stringBuilder.append(c)
               .append(b)
               .append(s)
               .append(t)
               .append(f)
               .append(d)
               .append(character)
               .append(long_)
               .append(byte_)
               .append(short1)
               .append(boolean_)
               .append(package_)
               .append(enum_)
               .append(class_)
               .append(integers.length)
               .append(n.length)
         );
   }

   public void lambdas(final String s, final int n) {
      class Local {
         int f(int v) {
            return v + n;
         }
      }

      Runnable runnable = () -> {
         int i = n + 1;
         String string = s + i;
         System.out.println(string);
      };
      Function<String, Integer> function = str -> {
         int i = str.length();
         return i + n;
      };
      runnable.run();
      System.out.println(function.apply(s));
      Object object = new Object() {
         public String toString() {
            int i = n;
            String string = s;
            return string + i;
         }
      };
      System.out.println(object + "" + new Local().f(3));
   }

   public abstract void abs(String var1, int var2);

   public native void nat(long var1, Object var3);

   public static void keywords(Class<?> clazz, Package pack) {
      Object object = clazz;
      Class<?> class_ = clazz;
      Package package_ = pack;
      Enum<?> enum_ = null;
      System.out.println(object + "" + class_ + package_ + enum_);
   }

   public static <T extends Comparable<T>> T generic(T a, T b, List<? extends T> rest) {
      T t = a.compareTo(b) > 0 ? a : b;

      for (T t1 : rest) {
         if (t1.compareTo(t) > 0) {
            t = t1;
         }
      }

      return t;
   }

   public class Inner {
      void m(Naming.Inner other, Naming outer) {
         Naming.Inner inner = this;
         Entry<Naming.Inner, Naming> entry = null;
         System.out.println(inner + "" + other + outer + entry);
      }
   }
}

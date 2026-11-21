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
      long j = a;
      boolean flag = b > 0.0;
      String s = d + i;
      File file1 = new File(s);
      return i + (int)j + (flag ? 1 : 0) + s.length() + f.getName().length() + k.hashCode() + file1.hashCode();
   }

   public void inst(int[] arr, String[][] grid, Object... rest) {
      for (int i = 0; i < arr.length; i++) {
         for (int j = 0; j < grid.length; j++) {
            for (int k = 0; k < rest.length; k++) {
               for (int l = 0; l < 2; l++) {
                  for (int i1 = 0; i1 < 2; i1++) {
                     System.out.println(arr[i] + grid[j][0] + rest[k] + l + i1);
                  }
               }
            }
         }
      }

      List<String> list = new ArrayList();
      Entry<String, Integer> entry = null;
      Map<String, List<Integer>> map = new HashMap();
      list.add(String.valueOf(entry) + map);
      char c0 = 'c';
      byte b0 = 1;
      short short1 = 2;
      short short2 = 3;
      float f = 3.0F;
      double d0 = 4.0;
      Character character = c0;
      Long olong = 4L;
      Byte obyte = b0;
      Short oshort = short1;
      Boolean obool = true;
      Package opackage = Naming.class.getPackage();
      Enum<?> oenum = null;
      Class<?> oclass = Naming.class;
      Integer[] ainteger = new Integer[2];
      int[][] aint = new int[2][2];
      StringBuilder stringbuilder = new StringBuilder();
      System.out
         .println(
            stringbuilder.append(c0)
               .append(b0)
               .append(short1)
               .append(short2)
               .append(f)
               .append(d0)
               .append(character)
               .append(olong)
               .append(obyte)
               .append(oshort)
               .append(obool)
               .append(opackage)
               .append(oenum)
               .append(oclass)
               .append(ainteger.length)
               .append(aint.length)
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
         String sx = s + i;
         System.out.println(sx);
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
            String sx = s;
            return sx + i;
         }
      };
      System.out.println(object + "" + new Local().f(3));
   }

   public abstract void abs(String var1, int var2);

   public native void nat(long var1, Object var3);

   public static void keywords(Class<?> clazz, Package pack) {
      Object object = clazz;
      Class<?> oclass = clazz;
      Package opackage = pack;
      Enum<?> oenum = null;
      System.out.println(object + "" + oclass + opackage + oenum);
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
         Naming.Inner naming$inner = this;
         Entry<Naming.Inner, Naming> entry = null;
         System.out.println(naming$inner + "" + other + outer + entry);
      }
   }
}

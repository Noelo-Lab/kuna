package p;

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Map.Entry;
import java.util.function.Function;

public abstract class Naming {
   public static int st(long i, double d0, int j, String s, File file1, Class<?> oclass) {
      int kx = j * 2;
      long l = i;
      boolean flag = d0 > 0.0;
      String s1 = s + kx;
      File file2 = new File(s1);
      return kx + (int)l + (flag ? 1 : 0) + s1.length() + file1.getName().length() + oclass.hashCode() + file2.hashCode();
   }

   public void inst(int[] aint, String[][] astring, Object... aobject) {
      for (int i = 0; i < aint.length; i++) {
         for (int j = 0; j < astring.length; j++) {
            for (int k = 0; k < aobject.length; k++) {
               for (int l = 0; l < 2; l++) {
                  for (int i1 = 0; i1 < 2; i1++) {
                     System.out.println(aint[i] + astring[j][0] + aobject[k] + l + i1);
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
      int[][] aint1 = new int[2][2];
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
               .append(aint1.length)
         );
   }

   public void lambdas(final String s, final int i) {
      class Local {
         int f(int j) {
            return j + i;
         }
      }

      Runnable runnable = () -> {
         int k = i + 1;
         String s2 = s + k;
         System.out.println(s2);
      };
      Function<String, Integer> function = s1 -> {
         int k = str.length();
         return k + i;
      };
      runnable.run();
      System.out.println(function.apply(s));
      Object object = new Object() {
         public String toString() {
            int j = i;
            String s1 = s;
            return s1 + j;
         }
      };
      System.out.println(object + "" + new Local().f(3));
   }

   public abstract void abs(String s, int i);

   public native void nat(long i, Object object);

   public static void keywords(Class<?> oclass, Package opackage) {
      Object object = oclass;
      Class<?> oclass1 = oclass;
      Package opackage1 = opackage;
      Enum<?> oenum = null;
      System.out.println(object + "" + oclass1 + opackage1 + oenum);
   }

   public static <T extends Comparable<T>> T generic(T t, T t1, List<? extends T> list) {
      T t2 = t.compareTo(t1) > 0 ? t : t1;

      for (T t3 : list) {
         if (t3.compareTo(t2) > 0) {
            t2 = t3;
         }
      }

      return t2;
   }

   public class Inner {
      void m(Naming.Inner naming$inner1, Naming naming) {
         Naming.Inner naming$inner2 = this;
         Entry<Naming.Inner, Naming> entry = null;
         System.out.println(naming$inner2 + "" + naming$inner1 + naming + entry);
      }
   }
}

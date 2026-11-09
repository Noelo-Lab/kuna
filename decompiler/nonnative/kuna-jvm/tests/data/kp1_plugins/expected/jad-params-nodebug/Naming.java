package p;

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Function;

public abstract class Naming {
   public static int st(long i, double d0, int j, String s, File file1, Class<?> oclass) {
      int k = j * 2;
      long l = i;
      boolean flag = d0 > 0.0;
      String s1 = s + k;
      File file2 = new File(s1);
      return k + (int)l + (flag ? 1 : 0) + s1.length() + file1.getName().length() + oclass.hashCode() + file2.hashCode();
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

      ArrayList arraylist = new ArrayList();
      Object object1 = null;
      HashMap hashmap = new HashMap();
      arraylist.add(String.valueOf(object1) + hashmap);
      char c0 = 'c';
      byte b2 = 1;
      byte b0 = 2;
      byte b1 = 3;
      float f = 3.0F;
      double d0 = 4.0;
      Character character = c0;
      Long olong = 4L;
      Byte obyte = b2;
      Short oshort = Short.valueOf(b0);
      Boolean obool = true;
      Package opackage = Naming.class.getPackage();
      Object object = null;
      Class<Naming> oclass = Naming.class;
      Integer[] ainteger = new Integer[2];
      int[][] aint1 = new int[2][2];
      StringBuilder stringbuilder = new StringBuilder();
      System.out
         .println(
            stringbuilder.append(c0)
               .append(b2)
               .append(b0)
               .append(b1)
               .append(f)
               .append(d0)
               .append(character)
               .append(olong)
               .append(obyte)
               .append(oshort)
               .append(obool)
               .append(opackage)
               .append(object)
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
      Function function = s1 -> {
         int k = s1.length();
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
      Class oclass1 = oclass;
      Class oclass2 = oclass;
      Package opackage1 = opackage;
      Object object = null;
      System.out.println(oclass1 + "" + oclass2 + opackage1 + object);
   }

   public static <T extends Comparable<T>> T generic(T comparable, T comparable1, List<? extends T> list) {
      Comparable comparable2 = comparable.compareTo(comparable1) > 0 ? comparable : comparable1;

      for (Comparable comparable3 : list) {
         if (comparable3.compareTo(comparable2) > 0) {
            comparable2 = comparable3;
         }
      }

      return (T)comparable2;
   }

   public class Inner {
      void m(Naming.Inner naming$inner1, Naming naming) {
         Naming.Inner naming$inner2 = this;
         Object object = null;
         System.out.println(naming$inner2 + "" + naming$inner1 + naming + object);
      }
   }
}

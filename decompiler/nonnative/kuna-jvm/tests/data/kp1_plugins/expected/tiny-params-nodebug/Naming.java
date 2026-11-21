package p;

import java.io.File;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.function.Function;

public abstract class Naming {
   public static int st(long l, double d, int i, String string, File file, Class<?> class_) {
      int j = i * 2;
      long m = l;
      boolean bl = d > 0.0;
      String string1 = string + j;
      File file1 = new File(string1);
      return j + (int)m + (bl ? 1 : 0) + string1.length() + file.getName().length() + class_.hashCode() + file1.hashCode();
   }

   public void inst(int[] i, String[][] strings, Object... objects) {
      for (int j = 0; j < i.length; j++) {
         for (int k = 0; k < strings.length; k++) {
            for (int l = 0; l < objects.length; l++) {
               for (int m = 0; m < 2; m++) {
                  for (int n = 0; n < 2; n++) {
                     System.out.println(i[j] + strings[k][0] + objects[l] + m + n);
                  }
               }
            }
         }
      }

      ArrayList arrayList = new ArrayList();
      Object object1 = null;
      HashMap hashMap = new HashMap();
      arrayList.add(String.valueOf(object1) + hashMap);
      char e = 'c';
      byte g = 1;
      byte b = 2;
      byte c = 3;
      float f = 3.0F;
      double d = 4.0;
      Character character = e;
      Long long_ = 4L;
      Byte byte1 = g;
      Short short_ = Short.valueOf(b);
      Boolean boolean_ = true;
      Package package_ = Naming.class.getPackage();
      Object object = null;
      Class<Naming> class_ = Naming.class;
      Integer[] integers = new Integer[2];
      int[][] o = new int[2][2];
      StringBuilder stringBuilder = new StringBuilder();
      System.out
         .println(
            stringBuilder.append(e)
               .append(g)
               .append(b)
               .append(c)
               .append(f)
               .append(d)
               .append(character)
               .append(long_)
               .append(byte1)
               .append(short_)
               .append(boolean_)
               .append(package_)
               .append(object)
               .append(class_)
               .append(integers.length)
               .append(o.length)
         );
   }

   public void lambdas(final String string, final int i) {
      class Local {
         int f(int j) {
            return j + i;
         }
      }

      Runnable runnable = () -> {
         int k = i + 1;
         String string2 = string + k;
         System.out.println(string2);
      };
      Function function = string1 -> {
         int k = string1.length();
         return k + i;
      };
      runnable.run();
      System.out.println(function.apply(string));
      Object object = new Object() {
         public String toString() {
            int j = i;
            String string1 = string;
            return string1 + j;
         }
      };
      System.out.println(object + "" + new Local().f(3));
   }

   public abstract void abs(String string, int i);

   public native void nat(long l, Object object);

   public static void keywords(Class<?> class_, Package package_) {
      Class class_x = class_;
      Class class_xx = class_;
      Package package_x = package_x;
      Object object = null;
      System.out.println(class_x + "" + class_xx + package_x + object);
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
      void m(Naming.Inner inner1, Naming naming) {
         Naming.Inner inner2 = this;
         Object object = null;
         System.out.println(inner2 + "" + inner1 + naming + object);
      }
   }
}

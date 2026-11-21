package p;

import org.jetbrains.annotations.NotNull;

public class NotNullChecks {
   public void one(@NotNull String s) {
      if (s == null) {
         $$$reportNull$$$0(0);
      }

      System.out.println(s);
   }

   public static void wide(long a, double b, @NotNull Object o, @NotNull String t) {
      if (o == null) {
         $$$reportNull$$$0(1);
      }

      if (t == null) {
         $$$reportNull$$$0(2);
      }

      System.out.println(a + b + o.hashCode() + t);
   }

   public void notAnnotated(String s, @NotNull String u) {
      if (s == null) {
         $$$reportNull$$$0(3);
      }

      System.out.println(s + u);
   }

   public void otherCall(@NotNull String s) {
      if (s == null) {
         System.out.println("null");
      }

      System.out.println(s);
   }

   public void notFirst(@NotNull String s) {
      System.out.println("first");
      if (s == null) {
         $$$reportNull$$$0(4);
      }

      System.out.println(s);
   }

   @NotNull
   public String ret(String s) {
      String r = s + "!";
      if (r == null) {
         $$$reportNull$$$0(5);
      }

      return r;
   }

   @NotNull
   public String retBranches(int i, @NotNull String s) {
      if (s == null) {
         $$$reportNull$$$0(6);
      }

      String r = s.trim();
      if (i == 0) {
         if (r == null) {
            $$$reportNull$$$0(7);
         }

         return r;
      } else {
         for (int j = 0; j < i; j++) {
            r = r + j;
         }

         if (r == null) {
            $$$reportNull$$$0(8);
         }

         return r;
      }
   }

   @NotNull
   public String retThrow(String s) {
      String r = s.trim();
      if (r == null) {
         throw new IllegalStateException("null");
      } else {
         return r;
      }
   }

   @NotNull
   public String retOther(String s, String t) {
      if (t == null) {
         $$$reportNull$$$0(9);
      }

      return s;
   }

   private static void $$$reportNull$$$0(int i) {
      throw new IllegalArgumentException(String.valueOf(i));
   }

   public class Inner {
      public Inner(@NotNull String s, @NotNull Object o) {
         if (s == null) {
            NotNullChecks.$$$reportNull$$$0(10);
         }

         if (o == null) {
            NotNullChecks.$$$reportNull$$$0(11);
         }

         System.out.println(s + o);
      }
   }
}

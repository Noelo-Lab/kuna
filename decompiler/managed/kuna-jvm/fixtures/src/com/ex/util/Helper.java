package com.ex.util;

public final class Helper {
   private Helper() {
   }

   public static int twice(int x) {
      return x * 2;
   }

   public static int loop(int n) {
      int total = 0;
      while (n > 0) {
         if (n % 3 == 0) {
            total += n;
         } else {
            total -= 1;
         }
         n--;
      }
      return total;
   }
}

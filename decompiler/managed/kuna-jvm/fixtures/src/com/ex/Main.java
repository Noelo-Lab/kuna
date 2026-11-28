package com.ex;

import com.ex.util.Helper;

public class Main {
   private final int base;

   public Main(int base) {
      this.base = base;
   }

   public static void main(String[] args) {
      Main m = new Main(args.length);
      System.out.println(check(args.length > 0 ? args[0] : "") ? "ok" : "no");
      Runnable r = new Runnable() {
         public void run() {
            System.out.println(Helper.twice(3));
         }
      };
      r.run();
      System.out.println(m.add(1, 2));
   }

   public static boolean check(String key) {
      return key.length() == 16 && Main.Checker.verify(key);
   }

   public int add(int a, int b) {
      return this.base + a + b;
   }

   public String add(String a, String b) {
      return a + b;
   }

   static class Checker {
      static boolean verify(String key) {
         int sum = 0;
         for (int i = 0; i < key.length(); i++) {
            sum += key.charAt(i);
         }
         return sum % 7 == 3;
      }
   }
}

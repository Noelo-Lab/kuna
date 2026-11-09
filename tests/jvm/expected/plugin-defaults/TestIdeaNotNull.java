package pkg;

import java.util.Random;
import org.jetbrains.annotations.NotNull;

public class TestIdeaNotNull {
   @NotNull
   public String test() {
      String s = "";
      int i = new Random().nextInt(12);
      if (i == 0) {
         return s;
      }

      for (int j = 0; j < i; j++) {
         s = s + i;
      }

      return s;
   }

   public void test1(@NotNull String s) {
      System.out.println(s);
   }

   public void test2(@NotNull String s, @NotNull Integer i) {
      System.out.println(s);
      System.out.println(i);
   }

   public void test3(@NotNull String s, @NotNull Integer i, @NotNull Float f) {
      System.out.println(s);
      System.out.println(i);
      if (i == 0) {
         System.out.println(f);
      }
   }

   @NotNull
   public String test4() {
      String s = "";
      int i = new Random().nextInt(12);

      for (int j = 0; j < i; j++) {
         s = s + i;
      }

      throw new RuntimeException(s);
   }

   @NotNull
   public String test5(@NotNull Integer i) {
      String s = "";
      if (i == 0) {
         return s;
      }

      for (int j = 0; j < i; j++) {
         s = s + i;
      }

      return s;
   }
}

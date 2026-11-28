package pkg;

public class TestNarrowLocalIncrement {
   static void take(byte var0) {
   }

   static void take(short var0) {
   }

   static void take(char var0) {
   }

   static void count(int var0) {
   }

   void bytes() {
      byte var1 = 0;
      take(var1++);
      take(++var1);
      take(var1--);
   }

   void shorts() {
      short var1 = 1000;
      take(var1++);
      take(--var1);
   }

   void chars() {
      char var1 = 'a';
      take(var1++);
      take(var1);
   }

   void ints() {
      int var1 = 5;
      count(var1++);
      count(var1);
   }
}

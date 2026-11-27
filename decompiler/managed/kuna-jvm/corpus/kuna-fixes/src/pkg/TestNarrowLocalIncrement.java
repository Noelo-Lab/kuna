package pkg;

public class TestNarrowLocalIncrement {
   static void take(byte b) {
   }

   static void take(short s) {
   }

   static void take(char c) {
   }

   static void count(int n) {
   }

   void bytes() {
      byte b = 0;
      take(b++);
      take(++b);
      take(b--);
   }

   void shorts() {
      short s = 1000;
      take(s++);
      take(--s);
   }

   void chars() {
      char c = 'a';
      take(c++);
      take(c);
   }

   void ints() {
      int n = 5;
      count(n++);
      count(n);
   }
}

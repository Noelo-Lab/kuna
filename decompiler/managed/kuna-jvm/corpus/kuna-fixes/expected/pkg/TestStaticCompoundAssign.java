package pkg;

public class TestStaticCompoundAssign {
   static int calls = bump();
   static int count;

   static int bump() {
      count = 10;
      return 1;
   }

   static {
      count += 2;
   }
}

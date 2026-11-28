package pkg;

public class TestStaticCompoundAssign {
   static int calls = bump();
   static int count;

   static {
      count += 2;
   }

   static int bump() {
      count = 10;
      return 1;
   }
}

package pkg;

public class TestInstanceCompoundAssign {
   int calls = this.bump();
   int count;

   {
      count += 2;
   }

   int bump() {
      count = 10;
      return 1;
   }
}

package pkg;

public class TestInstanceCompoundAssign {
   int calls = this.bump();
   int count;

   public TestInstanceCompoundAssign() {
      this.count += 2;
   }

   int bump() {
      this.count = 10;
      return 1;
   }
}

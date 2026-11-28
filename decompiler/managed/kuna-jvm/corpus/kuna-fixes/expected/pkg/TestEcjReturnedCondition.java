package pkg;

public class TestEcjReturnedCondition {
   int count;
   int[] bits;

   boolean empty() {
      return this.count == 0;
   }

   boolean within(int low, int high) {
      return this.count >= low && this.count <= high;
   }

   boolean either(boolean other) {
      return this.count > 2 || other;
   }

   boolean flagged(int index) {
      if (this.bits != null) {
         return (this.bits[index] & 4) != 0;
      }

      return false;
   }

   static boolean same(Object a, Object b) {
      if (a == null) {
         return b == null;
      }

      if (b == null) {
         return false;
      }

      return a.equals(b);
   }

   boolean explicit() {
      if (this.count > 3) {
         return true;
      }

      return false;
   }
}

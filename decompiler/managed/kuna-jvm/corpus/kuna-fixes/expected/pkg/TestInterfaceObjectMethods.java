package pkg;

public class TestInterfaceObjectMethods {
   static final class Pair implements TestInterfaceObjectMethods.Snapshot {
      private final int first;
      private final int second;

      Pair(int first, int second) {
         this.first = first;
         this.second = second;
      }

      @Override
      public Object clone() {
         return new TestInterfaceObjectMethods.Pair(this.first, this.second);
      }

      @Override
      public int size() {
         return 2;
      }
   }

   interface Snapshot extends Cloneable {
      Object clone();

      int size();
   }
}

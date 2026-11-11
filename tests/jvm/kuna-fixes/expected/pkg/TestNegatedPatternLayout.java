package pkg;

public class TestNegatedPatternLayout {
   int total;

   public int guardReturn(TestNegatedPatternLayout.Item item) {
      if (!(item instanceof TestNegatedPatternLayout.Box box)) {
         return -1;
      }

      box.grow();
      return box.size;
   }

   public void guardThrow(TestNegatedPatternLayout.Item item) {
      if (!(item instanceof TestNegatedPatternLayout.Box box)) {
         throw new IllegalStateException();
      }

      this.total += box.size;
   }

   public int guardOr(TestNegatedPatternLayout.Item item) {
      if (!(item instanceof TestNegatedPatternLayout.Box box && box.size != 0)) {
         return 0;
      }

      return box.size;
   }

   public int guardInLoop(TestNegatedPatternLayout.Item[] items) {
      int sum = 0;

      for (TestNegatedPatternLayout.Item item : items) {
         if (item instanceof TestNegatedPatternLayout.Box box) {
            sum += box.size;
         }
      }

      return sum;
   }

   public int guardBreak(TestNegatedPatternLayout.Item item) {
      int depth = 0;

      while (item != null) {
         if (!(item.next instanceof TestNegatedPatternLayout.Box box)) {
            break;
         }

         depth += box.size;
         item = box.next;
      }

      return depth;
   }

   public static class Box extends TestNegatedPatternLayout.Item {
      int size;

      void grow() {
         this.size++;
      }
   }

   public static class Item {
      TestNegatedPatternLayout.Item next;
   }
}

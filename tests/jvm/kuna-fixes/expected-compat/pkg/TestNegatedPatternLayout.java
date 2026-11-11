package pkg;

public class TestNegatedPatternLayout {
   int total;

   public int guardReturn(TestNegatedPatternLayout.Item item) {
      if (item instanceof TestNegatedPatternLayout.Box box) {
         box.grow();
         return box.size;
      } else {
         return -1;
      }
   }

   public void guardThrow(TestNegatedPatternLayout.Item item) {
      if (item instanceof TestNegatedPatternLayout.Box box) {
         this.total = this.total + box.size;
      } else {
         throw new IllegalStateException();
      }
   }

   public int guardOr(TestNegatedPatternLayout.Item item) {
      return item instanceof TestNegatedPatternLayout.Box box && box.size != 0 ? box.size : 0;
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

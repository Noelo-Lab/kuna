package pkg;

public class TestNegatedPatternLayout {
   public static class Item {
      Item next;
   }

   public static class Box extends Item {
      int size;

      void grow() {
         size++;
      }
   }

   int total;

   public int guardReturn(Item item) {
      if (!(item instanceof Box box)) {
         return -1;
      }
      box.grow();
      return box.size;
   }

   public void guardThrow(Item item) {
      if (!(item instanceof Box box)) {
         throw new IllegalStateException();
      }
      total += box.size;
   }

   public int guardOr(Item item) {
      if (!(item instanceof Box box) || box.size == 0) {
         return 0;
      }
      return box.size;
   }

   public int guardInLoop(Item[] items) {
      int sum = 0;
      for (Item item : items) {
         if (!(item instanceof Box box)) {
            continue;
         }
         sum += box.size;
      }
      return sum;
   }

   public int guardBreak(Item item) {
      int depth = 0;
      while (item != null) {
         if (!(item.next instanceof Box box)) {
            break;
         }
         depth += box.size;
         item = box.next;
      }
      return depth;
   }
}

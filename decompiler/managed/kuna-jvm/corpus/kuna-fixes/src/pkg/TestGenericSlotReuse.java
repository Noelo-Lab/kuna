package pkg;

public class TestGenericSlotReuse {
   static final Box<String> NAMES;
   static final Box<Integer> SIZES;

   static {
      Box<String> box = new Box<>();
      box.add("a");
      NAMES = box;
   }

   static {
      Box<Integer> box = new Box<>();
      box.add(1);
      SIZES = box;
   }

   static int blocks() {
      int n = 0;
      {
         Box<String> box = new Box<>();
         box.add("a");
         n += box.size();
      }
      {
         Box<Integer> box = new Box<>();
         box.add(1);
         n += box.size();
      }
      return n;
   }

   static class Box<T> {
      int size;

      void add(T t) {
         size++;
      }

      int size() {
         return size;
      }
   }
}

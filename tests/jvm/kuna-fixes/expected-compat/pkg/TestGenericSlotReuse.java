package pkg;

public class TestGenericSlotReuse {
   static final TestGenericSlotReuse.Box<String> NAMES;
   static final TestGenericSlotReuse.Box<Integer> SIZES;

   static int blocks() {
      int n = 0;
      TestGenericSlotReuse.Box<String> box = new TestGenericSlotReuse.Box<>();
      box.add("a");
      n += box.size();
      box = new TestGenericSlotReuse.Box<>();
      box.add(1);
      return n + box.size();
   }

   static {
      TestGenericSlotReuse.Box<String> box = new TestGenericSlotReuse.Box<>();
      box.add("a");
      NAMES = box;
      box = new TestGenericSlotReuse.Box<>();
      box.add(1);
      SIZES = box;
   }

   static class Box<T> {
      int size;

      void add(T t) {
         this.size++;
      }

      int size() {
         return this.size;
      }
   }
}

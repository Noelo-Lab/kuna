package pkg;

public class TestGenericSlotReuse {
   static final TestGenericSlotReuse.Box<String> NAMES;
   static final TestGenericSlotReuse.Box<Integer> SIZES;

   static int blocks() {
      int n = 0;
      TestGenericSlotReuse.Box<String> box = new TestGenericSlotReuse.Box<>();
      box.add("a");
      n += box.size();
      TestGenericSlotReuse.Box<Integer> boxx = new TestGenericSlotReuse.Box<>();
      boxx.add(1);
      n += boxx.size();
      return n;
   }

   static {
      TestGenericSlotReuse.Box<String> box = new TestGenericSlotReuse.Box<>();
      box.add("a");
      NAMES = box;
      TestGenericSlotReuse.Box<Integer> boxx = new TestGenericSlotReuse.Box<>();
      boxx.add(1);
      SIZES = boxx;
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

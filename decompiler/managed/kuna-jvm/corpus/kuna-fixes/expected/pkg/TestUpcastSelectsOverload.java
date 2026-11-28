package pkg;

public class TestUpcastSelectsOverload {
   static int arr(TestUpcastSelectsOverload.Named[] n) {
      return 1;
   }

   static int arr(TestUpcastSelectsOverload.Item[] i) {
      return 2;
   }

   static int take(TestUpcastSelectsOverload.Named n) {
      return 1;
   }

   static int take(TestUpcastSelectsOverload.Item i) {
      return 2;
   }

   static int calls(TestUpcastSelectsOverload.Item[] items, TestUpcastSelectsOverload.Box<TestUpcastSelectsOverload.Item> box) {
      return arr((TestUpcastSelectsOverload.Named[])items) + arr(items) + take((TestUpcastSelectsOverload.Named)box.get()) + take(box.get());
   }

   static class Box<T> {
      T value;

      T get() {
         return this.value;
      }
   }

   static class Item implements TestUpcastSelectsOverload.Named {
      @Override
      public String name() {
         return "item";
      }
   }

   interface Named {
      String name();
   }
}

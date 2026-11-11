package pkg;

public abstract class TestStaticTypeVariableScope<V> {
   static <V> V unwrap(TestStaticTypeVariableScope.Box<V> box) {
      return box.get();
   }

   static Object value(TestStaticTypeVariableScope.ListenableBox<?> box) {
      Object v = unwrap((TestStaticTypeVariableScope.Box<V>)box);
      if (v == null) {
         v = box;
      }

      return v;
   }

   V own(TestStaticTypeVariableScope.ListenableBox<V> box) {
      return unwrap(box);
   }

   interface Box<V> {
      V get();
   }

   interface ListenableBox<V> extends TestStaticTypeVariableScope.Box<V> {
   }
}

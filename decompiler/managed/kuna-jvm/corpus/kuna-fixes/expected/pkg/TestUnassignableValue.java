package pkg;

public class TestUnassignableValue {
   static TestUnassignableValue.Fn<String, String> field;

   static <E> TestUnassignableValue.Fn<E, E> identity() {
      TestUnassignableValue.Ident var0 = TestUnassignableValue.Ident.INSTANCE;
      return (TestUnassignableValue.Fn)var0;
   }

   static void assign() {
      TestUnassignableValue.Ident var0 = TestUnassignableValue.Ident.INSTANCE;
      field = (TestUnassignableValue.Fn)var0;
   }

   static int use(TestUnassignableValue.Fn<String, String> var0) {
      return 3;
   }

   static int use(Object var0) {
      return 4;
   }

   static int calls(TestUnassignableValue.Box<String> var0) {
      TestUnassignableValue.Ident var1 = TestUnassignableValue.Ident.INSTANCE;
      return use((TestUnassignableValue.Fn<String, String>)(TestUnassignableValue.Fn)var1)
         + use(var1)
         + var0.put((TestUnassignableValue.Fn<String, String>)(TestUnassignableValue.Fn)var1)
         + var0.put(var1);
   }

   static class Box<T> {
      int put(TestUnassignableValue.Fn<T, T> var1) {
         return 1;
      }

      int put(Object var1) {
         return 2;
      }
   }

   interface Fn<A, B> {
      B apply(A var1);
   }

   enum Ident implements TestUnassignableValue.Fn<Object, Object> {
      INSTANCE;

      @Override
      public Object apply(Object var1) {
         return var1;
      }
   }
}

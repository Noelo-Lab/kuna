package pkg;

public class TestUnassignableValue {
   interface Fn<A, B> {
      B apply(A a);
   }

   enum Ident implements Fn<Object, Object> {
      INSTANCE;

      public Object apply(Object o) {
         return o;
      }
   }

   static class Box<T> {
      int put(Fn<T, T> f) {
         return 1;
      }

      int put(Object o) {
         return 2;
      }
   }

   static Fn<String, String> field;

   @SuppressWarnings("unchecked")
   static <E> Fn<E, E> identity() {
      Fn<E, E> f = (Fn)Ident.INSTANCE;
      return f;
   }

   static void assign() {
      Fn raw = Ident.INSTANCE;
      field = raw;
   }

   static int use(Fn<String, String> f) {
      return 3;
   }

   static int use(Object o) {
      return 4;
   }

   static int calls(Box<String> box) {
      Ident i = Ident.INSTANCE;
      return use((Fn)i) + use(i) + box.put((Fn)i) + box.put(i);
   }
}

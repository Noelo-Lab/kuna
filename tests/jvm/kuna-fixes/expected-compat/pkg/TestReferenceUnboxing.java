package pkg;

public class TestReferenceUnboxing {
   static Integer field;

   static Object inner(Object v) {
      return v;
   }

   static Long nextDown(Object value) {
      return (Long)inner(value);
   }

   static Boolean flag(Object v) {
      return v instanceof Boolean ? (Boolean)v : null;
   }

   static Boolean readFlag(Object value, boolean defaultValue) {
      return value == null ? defaultValue : flag(value);
   }

   static Integer rebox(Integer x) {
      return x;
   }

   static void store(Integer x) {
      field = x;
   }

   static void add(TestReferenceUnboxing.Sink sink, Integer x) {
      sink.add(x);
   }

   static Object[] array(Integer x) {
      return new Object[]{x};
   }

   static boolean same(Integer x, Integer y) {
      return x == y;
   }

   static boolean differ(Double x, Double y) {
      return x != null && x != y;
   }

   static Integer choose(Integer x, boolean c) {
      return c ? x : 0;
   }

   static int unboxed(Integer x) {
      return x + 1;
   }

   interface Sink {
      void add(Object var1);
   }
}

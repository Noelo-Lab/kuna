package pkg;

public class TestReferenceUnboxing {
   static Integer field;

   static Object inner(Object v) {
      return v;
   }

   static Long nextDown(Object value) {
      return ((Long)inner(value)).longValue();
   }

   static Boolean flag(Object v) {
      return v instanceof Boolean ? (Boolean)v : null;
   }

   static Boolean readFlag(Object value, boolean defaultValue) {
      if (value == null) {
         return defaultValue;
      }

      return flag(value).booleanValue();
   }

   static Integer rebox(Integer x) {
      return x.intValue();
   }

   static void store(Integer x) {
      field = x.intValue();
   }

   static void add(TestReferenceUnboxing.Sink sink, Integer x) {
      sink.add(x.intValue());
   }

   static Object[] array(Integer x) {
      return new Object[]{x.intValue()};
   }

   static boolean same(Integer x, Integer y) {
      return x.intValue() == y;
   }

   static boolean differ(Double x, Double y) {
      return x != null && x.doubleValue() != y;
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

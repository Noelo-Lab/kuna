package pkg;

public class TestExternalSupertypeArgs {
   public static <E> ExtFunction<E, E> identity() {
      return TestExternalSupertypeArgs.IdentityFunction.INSTANCE;
   }

   static <K, V> ExtEntry<K, V> nullEntry() {
      return TestExternalSupertypeArgs.NullEntry.INSTANCE;
   }

   public static void main(String[] args) {
      ExtFunction<String, String> f = identity();
      System.out.println((String)f.apply("x") + " " + nullEntry().key() + " " + (String)new TestExternalSupertypeArgs.Holder().entry().key());
   }

   static class Holder<V> {
      ExtEntry<String, V> entry() {
         return TestExternalSupertypeArgs.NullEntry.INSTANCE;
      }
   }

   enum IdentityFunction implements ExtFunction<Object, Object> {
      INSTANCE;

      public Object apply(Object o) {
         return o;
      }
   }

   enum NullEntry implements ExtEntry<Object, Object> {
      INSTANCE;

      public Object key() {
         return null;
      }
   }
}

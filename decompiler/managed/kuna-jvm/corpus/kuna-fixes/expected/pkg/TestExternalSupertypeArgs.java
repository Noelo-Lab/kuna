package pkg;

public class TestExternalSupertypeArgs {
   public static <E> ExtFunction<E, E> identity() {
      return (ExtFunction<E, E>)TestExternalSupertypeArgs.IdentityFunction.INSTANCE;
   }

   static <K, V> ExtEntry<K, V> nullEntry() {
      return (ExtEntry<K, V>)TestExternalSupertypeArgs.NullEntry.INSTANCE;
   }

   public static void main(String[] args) {
      ExtFunction<String, String> f = identity();
      System.out.println((String)f.apply("x") + " " + nullEntry().key() + " " + (String)new TestExternalSupertypeArgs.Holder().entry().key());
   }

   static class Holder<V> {
      ExtEntry<String, V> entry() {
         ExtEntry<String, V> result = (ExtEntry)TestExternalSupertypeArgs.NullEntry.INSTANCE;
         return result;
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

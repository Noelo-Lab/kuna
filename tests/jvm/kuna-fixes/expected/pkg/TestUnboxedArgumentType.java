package pkg;

public class TestUnboxedArgumentType {
   static <K, V> Object[] of(K k1, V v1, K k2, V v2) {
      return new Object[]{k1, v1, k2, v2};
   }

   static <V> Object[] both(V a, V b) {
      return new Object[]{a, b};
   }

   static <V> V same(V v) {
      return v;
   }

   static Object[] pair(Object a, Object b) {
      return new Object[]{a, b};
   }

   public Object[] mixed() {
      return of("byte", (byte)0, "short", (short)0);
   }

   public Object[] shorts() {
      return both((short)2, (byte)2);
   }

   public Object[] chars() {
      return both('c', 99);
   }

   public Object[] withString() {
      return both((byte)1, "one");
   }

   public Object[] stringFirst() {
      return both("one", (byte)1);
   }

   public Object single() {
      return same((short)5);
   }

   public Object[] plain() {
      return pair((byte)0, (short)1);
   }
}

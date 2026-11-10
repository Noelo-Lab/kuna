package pkg;

public class TestJavacLayout {
   private int total;
   private final int[] table = new int[8];

   int hash(Object x) {
      return x == null ? 0 : x.hashCode();
   }

   int pick(boolean first, int a, int b) {
      return first ? a : b;
   }

   int sign(int v) {
      return v < 0 ? -1 : 1;
   }

   boolean positive(int v) {
      return v > 0;
   }

   boolean same(Object o) {
      if (this == o) {
         return true;
      } else if (o != null && this.getClass() == o.getClass()) {
         TestJavacLayout that = (TestJavacLayout)o;
         return this.total == that.total;
      } else {
         return false;
      }
   }

   int get(int index) {
      if (index >= 0 && index < this.table.length) {
         return this.table[index];
      } else {
         throw new IllegalArgumentException("index " + index);
      }
   }

   void add(int[] values) {
      if (values != null) {
         this.total += values.length;
      }
   }

   void sum(int[] values) {
      for (int v : values) {
         if (v >= 0) {
            this.total += v;
         }
      }
   }

   void update(int n) {
      if (n != 0) {
         this.total = n;
      } else {
         this.total++;
      }
   }
}

package pkg;

public class TestJavacLayout {
   private int total;
   private final int[] table = new int[8];

   int hash(Object x) {
      if (x == null) {
         return 0;
      }

      return x.hashCode();
   }

   int pick(boolean first, int a, int b) {
      if (first) {
         return a;
      }

      return b;
   }

   int sign(int v) {
      return v < 0 ? -1 : 1;
   }

   boolean positive(int v) {
      if (v > 0) {
         return true;
      }

      return false;
   }

   boolean same(Object o) {
      if (this == o) {
         return true;
      }

      if (o == null || this.getClass() != o.getClass()) {
         return false;
      }

      TestJavacLayout that = (TestJavacLayout)o;
      return this.total == that.total;
   }

   int get(int index) {
      if (index < 0 || index >= this.table.length) {
         throw new IllegalArgumentException("index " + index);
      }

      return this.table[index];
   }

   void add(int[] values) {
      if (values == null) {
         return;
      }

      this.total += values.length;
   }

   void sum(int[] values) {
      for (int v : values) {
         if (v < 0) {
            continue;
         }

         this.total += v;
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

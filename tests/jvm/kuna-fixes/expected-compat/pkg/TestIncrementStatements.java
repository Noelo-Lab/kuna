package pkg;

import java.util.List;

public class TestIncrementStatements {
   private int index;
   private int toRemove;
   private int docCount;
   private int pos;
   private int verified;

   static int[] keys(int n) {
      int[] keys = new int[n];
      int i = 0;

      while (i < n) {
         keys[i] = i++;
      }

      return keys;
   }

   int next() {
      this.toRemove = this.index++;
      return this.toRemove;
   }

   int commit(TestIncrementStatements partition) {
      return partition.docCount++;
   }

   static String unique(String base, List<String> names) {
      int counter = 0;
      String name = base + "_" + counter;

      while (names.contains(name)) {
         name = base + "_" + ++counter;
      }

      return name;
   }

   static int mask(long m, int bits) {
      return (int)(--m & bits);
   }

   boolean advance() {
      long pos = this.pos;
      if (++pos > this.verified) {
         return true;
      }

      this.pos = (int)pos;
      return false;
   }

   static int folded(int[] values, int i) {
      int x = values[i++];
      int y = ++i;
      int z = i++;
      return x + y + z + i;
   }

   static String foldedName(String base, int counter) {
      return base + "_" + ++counter;
   }

   int foldedField() {
      return this.index++ + this.toRemove;
   }
}

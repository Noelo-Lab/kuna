package pkg;

public class TestFieldIncrementShape {
   private long version;
   private int count;
   private String uuid;

   TestFieldIncrementShape incrementVersion() {
      this.version++;
      this.uuid = "_na_";
      return this;
   }

   void decrement(TestFieldIncrementShape other) {
      other.count--;
   }

   void bump(int[] counts, int i) {
      this.count++;
      counts[i]++;
      counts[i + 1]++;
      counts[i + 2]--;
   }

   int take() {
      return this.count++;
   }
}

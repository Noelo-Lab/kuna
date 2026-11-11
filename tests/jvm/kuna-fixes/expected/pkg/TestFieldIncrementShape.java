package pkg;

public class TestFieldIncrementShape {
   private long version;
   private int count;
   private String uuid;

   TestFieldIncrementShape incrementVersion() {
      this.version = this.version + 1L;
      this.uuid = "_na_";
      return this;
   }

   void decrement(TestFieldIncrementShape other) {
      other.count = other.count - 1;
   }

   void bump(int[] counts, int i) {
      this.count++;
      counts[i] = counts[i] + 1;
      counts[i + 1]++;
      counts[i + 2]--;
   }

   int take() {
      return this.count++;
   }
}

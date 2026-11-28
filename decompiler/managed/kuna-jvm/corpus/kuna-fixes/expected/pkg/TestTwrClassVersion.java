package pkg;

public class TestTwrClassVersion {
   private int total;

   TestTwrClassVersion.Res open(int n) {
      return null;
   }

   public int first() {
      try (TestTwrClassVersion.Res r = this.open(1)) {
         return r.read();
      }
   }

   public void both() {
      try (
         TestTwrClassVersion.Res a = this.open(1);
         TestTwrClassVersion.Res b = this.open(2);
      ) {
         this.total = a.read() + b.read();
      }
   }

   interface Res extends AutoCloseable {
      void close();

      int read();
   }
}

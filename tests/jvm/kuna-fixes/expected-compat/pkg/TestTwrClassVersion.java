package pkg;

public class TestTwrClassVersion {
   private int total;

   TestTwrClassVersion.Res open(int n) {
      return null;
   }

   public int first() {
      TestTwrClassVersion.Res r = this.open(1);

      int var2;
      try {
         var2 = r.read();
      } catch (Throwable var5) {
         if (r != null) {
            try {
               r.close();
            } catch (Throwable var4) {
               var5.addSuppressed(var4);
            }
         }

         throw var5;
      }

      if (r != null) {
         r.close();
      }

      return var2;
   }

   public void both() {
      TestTwrClassVersion.Res a = this.open(1);

      try {
         TestTwrClassVersion.Res b = this.open(2);

         try {
            this.total = a.read() + b.read();
         } catch (Throwable var7) {
            if (b != null) {
               try {
                  b.close();
               } catch (Throwable var6) {
                  var7.addSuppressed(var6);
               }
            }

            throw var7;
         }

         if (b != null) {
            b.close();
         }
      } catch (Throwable var8) {
         if (a != null) {
            try {
               a.close();
            } catch (Throwable var5) {
               var8.addSuppressed(var5);
            }
         }

         throw var8;
      }

      if (a != null) {
         a.close();
      }
   }

   interface Res extends AutoCloseable {
      void close();

      int read();
   }
}

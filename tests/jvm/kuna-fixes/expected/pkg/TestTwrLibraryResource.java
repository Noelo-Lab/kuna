package pkg;

import java.io.IOException;
import pkglib.TwrLock;
import pkglib.TwrStream;
import pkglib.TwrLock.Held;

public class TestTwrLibraryResource {
   private final TwrLock lock;
   private boolean flag;
   private int count;

   TestTwrLibraryResource(TwrLock lock) {
      this.lock = lock;
   }

   public void invalidate() {
      try (Held held = this.lock.write()) {
         this.clear();
         this.flag = false;
      }
   }

   public String name() {
      try (Held held = this.lock.write()) {
         this.clear();
         return "x";
      }
   }

   public int created() throws IOException {
      try (TwrStream s = new TwrStream()) {
         return s.read();
      }
   }

   public int two(String a, String b) throws IOException {
      try (
         TwrStream s = TwrStream.open(a);
         TwrStream t = TwrStream.open(b);
      ) {
         return s.read() + t.read();
      }
   }

   public int caught(String a) {
      try (TwrStream s = TwrStream.open(a)) {
         return s.read();
      } catch (IOException e) {
         return -1;
      }
   }

   public void early(boolean b) {
      try (Held held = this.lock.write()) {
         if (b) {
            return;
         }

         this.clear();
      }
   }

   public void existing(TwrStream s) throws IOException {
      try (s) {
         this.count += s.read();
      }
   }

   public void nested(String a) throws IOException {
      try (
         TwrStream s = TwrStream.open(a);
         Held held = this.lock.write();
      ) {
         this.count += s.read();
      }
   }

   public void logsCloseFailure() {
      Held held = this.lock.write();

      try {
         this.clear();
      } catch (Throwable t) {
         if (held != null) {
            try {
               held.close();
            } catch (Throwable t2) {
               this.log(t2);
            }
         }

         throw t;
      }

      if (held != null) {
         held.close();
      }
   }

   public void returnSkipsClose(boolean b) {
      Held held = this.lock.write();

      try {
         if (b) {
            return;
         }

         this.clear();
      } catch (Throwable t) {
         if (held != null) {
            try {
               held.close();
            } catch (Throwable t2) {
               t.addSuppressed(t2);
            }
         }

         throw t;
      }

      if (held != null) {
         held.close();
      }
   }

   public void workBeforeTry() {
      Held held = this.lock.write();
      this.clear();

      try {
         this.flag = true;
      } catch (Throwable t) {
         if (held != null) {
            try {
               held.close();
            } catch (Throwable t2) {
               t.addSuppressed(t2);
            }
         }

         throw t;
      }

      if (held != null) {
         held.close();
      }
   }

   void clear() {
   }

   void log(Throwable t) {
   }
}

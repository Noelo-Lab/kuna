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
      Held held = this.lock.write();

      try {
         this.clear();
         this.flag = false;
      } catch (Throwable var5) {
         if (held != null) {
            try {
               held.close();
            } catch (Throwable var4) {
               var5.addSuppressed(var4);
            }
         }

         throw var5;
      }

      if (held != null) {
         held.close();
      }
   }

   public String name() {
      Held held = this.lock.write();

      String var2;
      try {
         this.clear();
         var2 = "x";
      } catch (Throwable var5) {
         if (held != null) {
            try {
               held.close();
            } catch (Throwable var4) {
               var5.addSuppressed(var4);
            }
         }

         throw var5;
      }

      if (held != null) {
         held.close();
      }

      return var2;
   }

   public int created() throws IOException {
      TwrStream s = new TwrStream();

      int var2;
      try {
         var2 = s.read();
      } catch (Throwable var5) {
         try {
            s.close();
         } catch (Throwable var4) {
            var5.addSuppressed(var4);
         }

         throw var5;
      }

      s.close();
      return var2;
   }

   public int two(String a, String b) throws IOException {
      TwrStream s = TwrStream.open(a);

      int var5;
      try {
         TwrStream t = TwrStream.open(b);

         try {
            var5 = s.read() + t.read();
         } catch (Throwable var9) {
            if (t != null) {
               try {
                  t.close();
               } catch (Throwable var8) {
                  var9.addSuppressed(var8);
               }
            }

            throw var9;
         }

         if (t != null) {
            t.close();
         }
      } catch (Throwable var10) {
         if (s != null) {
            try {
               s.close();
            } catch (Throwable var7) {
               var10.addSuppressed(var7);
            }
         }

         throw var10;
      }

      if (s != null) {
         s.close();
      }

      return var5;
   }

   public int caught(String a) {
      try {
         TwrStream s = TwrStream.open(a);

         int var3;
         try {
            var3 = s.read();
         } catch (Throwable var6) {
            if (s != null) {
               try {
                  s.close();
               } catch (Throwable var5) {
                  var6.addSuppressed(var5);
               }
            }

            throw var6;
         }

         if (s != null) {
            s.close();
         }

         return var3;
      } catch (IOException e) {
         return -1;
      }
   }

   public void early(boolean b) {
      Held held = this.lock.write();

      label40: {
         try {
            if (b) {
               break label40;
            }

            this.clear();
         } catch (Throwable var6) {
            if (held != null) {
               try {
                  held.close();
               } catch (Throwable var5) {
                  var6.addSuppressed(var5);
               }
            }

            throw var6;
         }

         if (held != null) {
            held.close();
         }

         return;
      }

      if (held != null) {
         held.close();
      }
   }

   public void existing(TwrStream s) throws IOException {
      TwrStream var2 = s;

      try {
         this.count = this.count + s.read();
      } catch (Throwable var6) {
         if (var2 != null) {
            try {
               var2.close();
            } catch (Throwable var5) {
               var6.addSuppressed(var5);
            }
         }

         throw var6;
      }

      if (var2 != null) {
         var2.close();
      }
   }

   public void nested(String a) throws IOException {
      TwrStream s = TwrStream.open(a);

      try {
         Held held = this.lock.write();

         try {
            this.count = this.count + s.read();
         } catch (Throwable var8) {
            if (held != null) {
               try {
                  held.close();
               } catch (Throwable var7) {
                  var8.addSuppressed(var7);
               }
            }

            throw var8;
         }

         if (held != null) {
            held.close();
         }
      } catch (Throwable var9) {
         if (s != null) {
            try {
               s.close();
            } catch (Throwable var6) {
               var9.addSuppressed(var6);
            }
         }

         throw var9;
      }

      if (s != null) {
         s.close();
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

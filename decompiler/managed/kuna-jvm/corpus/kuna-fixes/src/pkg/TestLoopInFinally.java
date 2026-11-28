// A loop's exit inside a finally body is not the end of the finally: javac -g --release 17
package pkg;

import java.io.IOException;
import java.io.InputStream;

public class TestLoopInFinally {
   interface Work {
      void run() throws IOException;
   }

   interface Log {
      void put(String s);
   }

   static void closeAll(Work work, InputStream[] streams, int count) throws IOException {
      try {
         work.run();
      } finally {
         for (int i = 0; i < count; i++) {
            InputStream is = streams[i];
            if (is != null) {
               try {
                  is.close();
               } catch (Exception e) {
               }
            }
         }
      }
   }

   static void loopThenMore(Work work, Log log, int n) throws IOException {
      try {
         work.run();
      } finally {
         for (int i = 0; i < n; i++) {
            if (i == 3) {
               log.put("three");
            }
         }
         log.put("done");
      }
   }
}

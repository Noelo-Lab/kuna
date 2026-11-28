package pkg;

import java.io.IOException;
import java.io.InputStream;

public class TestLoopInFinally {
   static void closeAll(TestLoopInFinally.Work work, InputStream[] streams, int count) throws IOException {
      try {
         work.run();
      } finally {
         for (int i = 0; i < count; i++) {
            InputStream is = streams[i];
            if (is != null) {
               try {
                  is.close();
               } catch (Exception var13) {
               }
            }
         }
      }
   }

   static void loopThenMore(TestLoopInFinally.Work work, TestLoopInFinally.Log log, int n) throws IOException {
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

   interface Log {
      void put(String var1);
   }

   interface Work {
      void run() throws IOException;
   }
}

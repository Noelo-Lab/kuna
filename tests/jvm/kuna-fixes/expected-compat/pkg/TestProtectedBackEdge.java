package pkg;

public class TestProtectedBackEdge {
   boolean running;
   int runs;
   int count;
   int left;
   final Object lock = new Object();

   void returnInLoop() {
      while (true) {
         try {
            synchronized (this.lock) {
               if (this.count <= 0) {
                  return;
               }
            }

            this.count--;
         } finally {
            this.runs++;
         }
      }
   }

   void breakInLoop() {
      while (true) {
         try {
            label44: {
               synchronized (this.lock) {
                  if (this.count <= 0) {
                     break label44;
                  }
               }

               this.count--;
               continue;
            }

            this.left += 2;
         } finally {
            this.runs++;
         }

         return;
      }
   }

   void nestedLoop() {
      while (true) {
         try {
            synchronized (this.lock) {
               while (this.count > 2) {
                  if (this.left <= 0) {
                     return;
                  }

                  this.left--;
                  this.count--;
               }
            }

            this.count += 3;
         } finally {
            this.runs++;
         }
      }
   }

   void worker() {
      boolean flagged = false;
      boolean started = false;

      while (true) {
         try {
            synchronized (this.lock) {
               if (!started) {
                  if (this.running) {
                     return;
                  }

                  this.running = true;
                  started = true;
               }

               if (this.left == 0) {
                  this.running = false;
                  return;
               }

               this.left--;
            }

            flagged |= this.count > 3;

            try {
               this.count++;
            } finally {
               this.count += 2;
            }
         } finally {
            this.runs++;
            if (flagged) {
               this.count = -this.count;
            }
         }
      }
   }

   void tryInLoop() {
      while (true) {
         try {
            synchronized (this.lock) {
               if (this.count <= 0) {
                  return;
               }
            }

            this.count--;
         } finally {
            this.runs++;
         }
      }
   }
}

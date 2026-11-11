package pkg;

public class TestFieldReadReuse {
   TestFieldReadReuse.Lock lock = new TestFieldReadReuse.Lock();
   volatile Object value;
   int left;
   int right;

   int loop(TestFieldReadReuse.Lock l) {
      while (true) {
         int i = l.count;
         if (l.count > 0) {
            if (!l.tryInc(i)) {
               continue;
            }

            return i;
         }

         return -1;
      }
   }

   int plainField(TestFieldReadReuse.Lock l) {
      int p = l.plain;
      return l.plain > 2 ? p : 0;
   }

   Object cached() {
      Object local = this.value;
      return this.value == null ? (this.value = "made") : local;
   }

   TestFieldReadReuse.Lock last() {
      TestFieldReadReuse.Lock end = this.lock;

      while (true) {
         TestFieldReadReuse.Lock n = end.next;
         if (end.next == null) {
            return end;
         }

         end = n;
      }
   }

   void both(TestFieldReadReuse.Lock l) {
      this.left = this.right = l.plain;
   }

   void increment(TestFieldReadReuse.Lock l) {
      l.plain++;
      int old = l.plain++;
      this.right = old;
   }

   static class Lock {
      volatile int count = 1;
      int plain;
      TestFieldReadReuse.Lock next;

      boolean tryInc(int i) {
         this.count = i + 1;
         return i > 3;
      }
   }
}

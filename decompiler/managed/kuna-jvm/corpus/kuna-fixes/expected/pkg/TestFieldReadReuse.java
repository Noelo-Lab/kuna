package pkg;

public class TestFieldReadReuse {
   TestFieldReadReuse.Lock lock = new TestFieldReadReuse.Lock();
   volatile Object value;
   int left;
   int right;

   int loop(TestFieldReadReuse.Lock l) {
      int i;
      while ((i = l.count) > 0) {
         if (l.tryInc(i)) {
            return i;
         }
      }

      return -1;
   }

   int plainField(TestFieldReadReuse.Lock l) {
      int p;
      if ((p = l.plain) > 2) {
         return p;
      }

      return 0;
   }

   Object cached() {
      Object local;
      return (local = this.value) == null ? (this.value = "made") : local;
   }

   TestFieldReadReuse.Lock last() {
      TestFieldReadReuse.Lock n;
      TestFieldReadReuse.Lock end = this.lock;

      while ((n = end.next) != null) {
         end = n;
      }

      return end;
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

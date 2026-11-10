package pkg;

public class TestSynchronizedLockAssignment {
   private final TestSynchronizedLockAssignment.Probe probe;
   Object lock = new Object();
   Object[] slots = new Object[]{new Object(), new Object()};
   int count;
   int calls;

   public TestSynchronizedLockAssignment(TestSynchronizedLockAssignment.Probe probe) {
      this.probe = probe;
   }

   Object next() {
      this.calls++;
      return this.lock;
   }

   int swap() {
      Object l = this.lock;
      this.lock = new Object();
      synchronized (l) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int callBetween() {
      Object l = this.lock;
      this.probe.outside();
      synchronized (l) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int elementSwap(int i) {
      Object e = this.slots[i];
      this.slots[i] = new Object();
      synchronized (e) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int twoCalls() {
      Object l = this.next();
      synchronized (this.next()) {
         this.count++;
      }

      return this.calls;
   }

   int direct() {
      synchronized (this.lock) {
         this.lock = new Object();
         this.count++;
      }

      return this.count;
   }

   public interface Probe {
      void outside();

      void inside();
   }
}

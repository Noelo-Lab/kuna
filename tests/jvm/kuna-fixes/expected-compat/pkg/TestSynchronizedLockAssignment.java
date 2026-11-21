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
      this.lock = new Object();
      synchronized (this.lock) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int callBetween() {
      this.probe.outside();
      synchronized (this.lock) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int elementSwap(int i) {
      this.slots[i] = new Object();
      synchronized (this.slots[i]) {
         this.count++;
         this.probe.inside();
      }

      return this.count;
   }

   int twoCalls() {
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

package pkg

public class TestKtObjectMonitorCall {
   private final val lock: Any = Any()
   private final var ready: Boolean

   public fun await() {
      synchronized (this.lock) {
         while (!this.ready) {
            val var10000: Any = this.lock
            var10000.wait()
         }
      }
   }

   public fun signal() {
      synchronized (this.lock) {
         this.ready = true
         val var10000: Any = this.lock
         var10000.notifyAll()
      }
   }

   public fun awaitFor(ms: Long, ns: Int) {
      synchronized (this) {
         if (ns == 0) {
            this.wait(ms)
         } else {
            this.wait(ms, ns)
         }
      }
   }

   public fun signalOne() {
      synchronized (this) {
         this.notify()
      }
   }
}

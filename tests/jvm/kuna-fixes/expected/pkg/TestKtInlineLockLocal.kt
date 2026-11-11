package pkg

import org.jetbrains.annotations.NotNull

public class TestKtInlineLockLocal(probe: pkg.TestKtInlineLockLocal.Probe) {
   private final val probe: pkg.TestKtInlineLockLocal.Probe

   @JvmField
   @NotNull
   public final var lock: Any
      private set

   @JvmField
   @NotNull
   public final var slots: Array<Any>
      private set

   private final val fixed: Any
   public final var count: Int = 0

   init {
      this.probe = probe
      this.lock = Any()
      this.slots = arrayOf(Any(), Any())
      this.fixed = Any()
   }

   public fun field(): Boolean {
      val l: Any = this.lock
      synchronized (l) {
         val var4: Int = this.count++
         return this.probe.held(l)
      }
   }

   public fun element(i: Int): Boolean {
      val e: Any = this.slots[i]
      synchronized (e) {
         val var5: Int = this.count++
         return this.probe.held(e)
      }
   }

   public fun usedAfter(): Any {
      val l: Any = this.lock
      synchronized (l) {
         val var4: Int = this.count++
         return l
      }
   }

   public fun finalField(): Boolean {
      val l: Any = this.fixed
      synchronized (l) {
         val var4: Int = this.count++
         return this.probe.held(l)
      }
   }

   public fun single(): Int {
      synchronized (this.lock) {
         this.count++
         return this.count
      }
   }

   public interface Probe {
      public abstract fun held(o: Any): Boolean
   }
}

package pkg

public class TestKtFieldReadPaths {
   public fun inFinally(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src) {
      val v: Int = h.count

      try {
         src.work()
      } finally {
         src.use(v)
      }
   }

   public fun restoreOnFailure(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src) {
      val saved: Int = h.count

      try {
         src.work()
      } catch (var5: java.lang.Throwable) {
         h.count = saved
         throw var5
      }
   }

   public fun afterBranch(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, c: Boolean) {
      val v: Int = h.count
      if (c) {
         src.work()
      }

      src.use(v)
   }

   public fun afterFinally(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, k: Int) {
      val v: Int = h.count
      var x: Int = k

      try {
         x++
      } finally {
         src.work()
      }

      src.use(v + x)
   }

   public fun afterCondition(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, c: Boolean) {
      val v: Int = h.count
      src.use(if (c && src.flag()) v else 0)
   }

   public class Holder {
      @JvmField
      public final var count: Int
         private set
   }

   public interface Src {
      public abstract fun work() {
      }

      public abstract fun use(v: Int) {
      }

      public abstract fun flag(): Boolean {
      }
   }
}

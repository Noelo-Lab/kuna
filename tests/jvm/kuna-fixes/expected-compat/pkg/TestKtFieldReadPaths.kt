package pkg

public class TestKtFieldReadPaths {
   public fun inFinally(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src) {
      try {
         src.work()
      } finally {
         src.use(h.count)
      }
   }

   public fun restoreOnFailure(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src) {
      try {
         src.work()
      } catch (var5: java.lang.Throwable) {
         h.count = h.count
         throw var5
      }
   }

   public fun afterBranch(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, c: Boolean) {
      if (c) {
         src.work()
      }

      src.use(h.count)
   }

   public fun afterFinally(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, k: Int) {
      var x: Int = k

      try {
         x++
      } finally {
         src.work()
      }

      src.use(h.count + x)
   }

   public fun afterCondition(h: pkg.TestKtFieldReadPaths.Holder, src: pkg.TestKtFieldReadPaths.Src, c: Boolean) {
      src.use(if (c && src.flag()) h.count else 0)
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

package pkg

public class TestKtFinallyExitVersions {
   public fun constant(src: pkg.TestKtFinallyExitVersions.Src) {
      val x: Int = 1

      var var6: Byte
      try {
         src.work()
      } finally {
         var6 = 2
      }

      src.use(var6.toInt())
   }

   public fun computed(src: pkg.TestKtFinallyExitVersions.Src, k: Int) {
      var x: Int
      try {
         src.work()
      } finally {
         x = k + 7
      }

      src.use(x)
   }

   public fun conditional(src: pkg.TestKtFinallyExitVersions.Src, k: Int) {
      var x: Int = k

      try {
         src.work()
      } finally {
         if (src.g() > 0) {
            x = 3
         }
      }

      src.use(x)
   }

   public fun withCatch(src: pkg.TestKtFinallyExitVersions.Src) {
      var x: Int = 1

      try {
         src.work()
      } catch (var6: IllegalStateException) {
         x = 4
      } finally {
         x = x + src.g()
      }

      src.use(x)
   }

   public fun nested(src: pkg.TestKtFinallyExitVersions.Src) {
      val x: Int = 1

      var var10: Byte
      try {
         var var11: Byte
         try {
            src.work()
         } finally {
            var11 = 5
         }

         src.use(var11.toInt())
      } finally {
         var10 = 6
      }

      src.use(var10.toInt())
   }

   public fun loop(src: pkg.TestKtFinallyExitVersions.Src) {
      var x: Int = 0

      while (src.g() > 0) {
         try {
            if (src.g() > 5) {
               break
            }

            src.work()
         } finally {
            x = src.g()
         }
      }

      src.use(x)
   }

   public fun finallyLocal(src: pkg.TestKtFinallyExitVersions.Src): Any? {
      var flag: Boolean = false
      var var7: Boolean = false /* VF: Semaphore variable */

      var var4: Any?
      try {
         var7 = true
         val r: Any? = src.get()
         flag = true
         var4 = r
         var7 = false
      } finally {
         if (var7) {
            if (src.exit() && flag) {
               throw IllegalStateException()
            }
         }
      }

      if (src.exit()) {
         throw IllegalStateException()
      } else {
         return var4
      }
   }

   public interface Src {
      public abstract fun work()

      public abstract fun use(v: Int)

      public abstract fun useAny(v: Any?)

      public abstract fun g(): Int

      public abstract fun exit(): Boolean

      public abstract fun get(): Any?
   }
}

package pkg

public class TestKtFinallyExitVersions {
   public fun constant(src: pkg.TestKtFinallyExitVersions.Src) {
      try {
         src.work()
      } finally {
         val var6: Byte = 2
      }

      src.use(1)
   }

   public fun computed(src: pkg.TestKtFinallyExitVersions.Src, k: Int) {
      try {
         src.work()
      } finally {
         val var7: Int = k + 7
      }

      src.use(k)
   }

   public fun conditional(src: pkg.TestKtFinallyExitVersions.Src, k: Int) {
      try {
         src.work()
      } finally {
         if (src.g() > 0) {
            val var7: Byte = 3
         }
      }

      src.use(k)
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

      try {
         try {
            src.work()
         } finally {
            val var11: Byte = 5
         }

         src.use(x)
      } finally {
         val var10: Byte = 6
      }

      src.use(1)
   }

   public fun loop(src: pkg.TestKtFinallyExitVersions.Src) {
      while (src.g() > 0) {
         try {
            if (src.g() > 5) {
               break
            }

            src.work()
         } finally {
            val var6: Int = src.g()
         }
      }

      src.use(0)
   }

   public fun finallyLocal(src: pkg.TestKtFinallyExitVersions.Src): Any? {
      var flag: Boolean = false

      var var4: Any
      try {
         val var10: Boolean = true
         val r: Any = src.get()
         flag = true
         var4 = r
      } finally {
         if (var7) {
            if (src.exit() && flag) {
               throw IllegalStateException()
            }
         }
      }

      val var7: Boolean
      if (src.exit()) {
         throw IllegalStateException()
      } else {
         return var4
      }
   }

   public interface Src {
      public abstract fun work() {
      }

      public abstract fun use(v: Int) {
      }

      public abstract fun useAny(v: Any?) {
      }

      public abstract fun g(): Int {
      }

      public abstract fun exit(): Boolean {
      }

      public abstract fun get(): Any? {
      }
   }
}

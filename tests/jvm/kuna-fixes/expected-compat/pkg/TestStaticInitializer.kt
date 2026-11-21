package pkg

public object TestStaticInitializer {
   public final val table: IntArray
   public final val ready: Boolean

   @JvmStatic
   fun {
      val t: IntArray = IntArray(16)
      var i: Int = 0

      for (e in t.length..i) {
         t[i] = i * i
      }

      table = t

      try {
         var4 = t[15] > 100
      } catch (var3: IllegalStateException) {
         var4 = false
      }

      ready = var4
   }

   public class Platform {
      @JvmStatic
      fun {
         var var0: Boolean
         try {
            Class.forName("pkg.Missing")
            var0 = true
         } catch (var2: ClassNotFoundException) {
            var0 = false
         }

         isSupported = var0
         level = if (isSupported) 2 else 1
      }

      public companion object {
         public final val isSupported: Boolean
         public final val level: Int
      }
   }
}

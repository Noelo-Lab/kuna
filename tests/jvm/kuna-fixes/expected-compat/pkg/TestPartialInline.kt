package pkg

public class TestPartialInline {
   public fun withDefault(text: String?): Int {
      var var10000: java.lang.String = text
      if (text == null) {
         var10000 = "none"
      }

      return var10000.length()
   }

   public fun boxed(state: Any?): Int {
      var var10000: TestPartialInline.Box = state as? TestPartialInline.Box
      if ((state as? TestPartialInline.Box) == null) {
         var10000 = TestPartialInline.Box(0)
      }

      val box: TestPartialInline.Box = var10000
      synchronized (var10000) {
         return box.size
      }
   }

   public fun shifted(host: String?, suffix: String): Boolean {
      var h: java.lang.String = host
      if (host == null) {
         h = "localhost"
      }

      return h.length() > suffix.length()
   }

   public class Box(size: Int) {
      public final var size: Int
         internal set

      init {
         this.size = size
      }
   }
}

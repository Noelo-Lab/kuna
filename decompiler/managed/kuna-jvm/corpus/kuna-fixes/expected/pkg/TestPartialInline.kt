package pkg

public class TestPartialInline {
   public fun withDefault(text: String?): Int {
      var var10000: String? = text
      if (var10000 == null) {
         var10000 = "none"
      }

      return var10000.length
   }

   public fun boxed(state: Any?): Int {
      var var10000: TestPartialInline.Box? = state as? TestPartialInline.Box
      if (var10000 == null) {
         var10000 = TestPartialInline.Box(0)
      }

      val box: TestPartialInline.Box = var10000
      synchronized (box) {
         return box.size
      }
   }

   public fun shifted(host: String?, suffix: String): Boolean {
      var h: String? = host
      if (h == null) {
         h = "localhost"
      }

      return h.length > suffix.length
   }

   public class Box(size: Int) {
      public final var size: Int

      init {
         this.size = size
      }
   }
}

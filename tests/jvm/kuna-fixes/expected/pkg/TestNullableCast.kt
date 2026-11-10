package pkg

public class TestNullableCast(names: Map<String, String>) {
   private final val names: Map<String, String>

   init {
      this.names = names
   }

   public fun lengthOf(key: String): Int {
      val var10000: String? = this.names.get(key) as String?
      return if (var10000 == null) -1 else var10000.length
   }

   public fun describe(key: String): String {
      return this.describeName(this.names.get(key) as String?)
   }

   private fun describeName(name: String?): String {
      var var10000: String? = name
      if (var10000 == null) {
         var10000 = "none"
      }

      return var10000
   }
}

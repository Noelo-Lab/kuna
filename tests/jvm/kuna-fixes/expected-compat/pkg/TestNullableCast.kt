package pkg

public class TestNullableCast(names: Map<String, String>) {
   private final val names: Map<String, String>

   init {
      this.names = names
   }

   public fun lengthOf(key: String): Int {
      val var10000: java.lang.String = this.names.get(key) as java.lang.String
      return if (var10000 == null) -1 else var10000.length()
   }

   public fun describe(key: String): String {
      return this.describeName(this.names.get(key) as java.lang.String)
   }

   private fun describeName(name: String?): String {
      var var10000: java.lang.String = name
      if (name == null) {
         var10000 = "none"
      }

      return var10000
   }
}

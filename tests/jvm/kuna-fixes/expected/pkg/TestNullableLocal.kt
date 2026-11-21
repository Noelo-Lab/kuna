package pkg

public class TestNullableLocal(cache: Map<String, String>) {
   private final val cache: Map<String, String>
   public final var head: pkg.TestNullableLocal.Node? = null

   init {
      this.cache = cache
   }

   public fun sum(): Int {
      var total: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var node: TestNullableLocal.Node? = this.head
      while (node != null) {
         total += node.value

         node = node.next
      }

      return total
   }

   private fun suffixed(host: String?, suffix: String): Boolean {
      var h: String? = host
      if (h == null) {
         h = "x"
      }

      return h.length > suffix.length
   }

   public fun length(key: String): Int {
      val var10000: String? = this.cache.get(key) as String?
      return if (var10000 == null) -1 else var10000.length
   }

   public fun last(key: String): String {
      var found: String? = null

      for (k in this.cache.keys) {
         if (k.length == key.length) {
            found = k
         }
      }

      return if (found == null) "" else found
   }

   public fun check(key: String): Boolean {
      return this.suffixed(this.cache.get(key) as String?, key)
   }

   public class Node(next: pkg.TestNullableLocal.Node?, value: Int) {
      public final var next: pkg.TestNullableLocal.Node?
      public final var value: Int

      init {
         this.next = next
         this.value = value
      }
   }
}

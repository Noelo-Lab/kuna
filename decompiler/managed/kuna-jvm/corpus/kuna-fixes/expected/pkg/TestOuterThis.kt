package pkg

public class TestOuterThis(name: String) {
   private final val name: String
   private final var hits: Int = 0

   init {
      this.name = name
   }

   public open fun toString(): String {
      return "TestOuterThis(${this.name})"
   }

   public inner class Reader(limit: Int) {
      public final val limit: Int

      init {
         this.limit = limit
      }

      public fun read(): String {
         val var1: Int = this@TestOuterThis.hits
         this@TestOuterThis.hits = var1 + 1
         return "${this@TestOuterThis.name}${this.limit}${this@TestOuterThis.hits}"
      }

      public fun describe(): String {
         return "${this@TestOuterThis}/${this.limit}"
      }

      public inner class Cursor {
         public fun owner(): TestOuterThis {
            return this@TestOuterThis
         }

         public fun reader(): pkg.TestOuterThis.Reader {
            return this@Reader
         }
      }
   }
}

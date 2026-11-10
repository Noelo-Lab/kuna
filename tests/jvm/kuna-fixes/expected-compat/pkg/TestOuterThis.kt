package pkg

public class TestOuterThis(name: String) {
   private final val name: String
   private final var hits: Int

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
         TestOuterThis.this.hits = TestOuterThis.this.hits + 1
         return "${TestOuterThis.this.name}${this.limit}${TestOuterThis.this.hits}"
      }

      public fun describe(): String {
         return "$TestOuterThis.this/${this.limit}"
      }

      public inner class Cursor {
         public fun owner(): TestOuterThis {
            return TestOuterThis.this
         }

         public fun reader(): pkg.TestOuterThis.Reader {
            return Reader.this
         }
      }
   }
}

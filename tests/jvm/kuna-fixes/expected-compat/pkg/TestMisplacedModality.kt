package pkg

public open class TestMisplacedModality {
   public final var size: Long
      public final get() {
         return this.size + 1L
      }

      internal final set(value) {
         this.size = value * 2
      }


   public open val label: String
      public open get() {
         return "label${this.size}"
      }

}

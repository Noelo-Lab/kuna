package pkg

public open class TestMisplacedModality {
   public final var size: Long = 0L
      public get() {
         return field + 1L
      }

      internal set(value) {
         field = value * 2
      }


   public open val label: String
      public get() {
         return "label${this.size}"
      }

}

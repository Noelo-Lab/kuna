package pkg

public class TestDefaultPropertyInitializer(start: Int) {
   public final val start: Int
   public final var count: Int = 0
   private final var closed: Boolean = false
   public final var total: Long = 0L
   public final var ratio: Double = 0.0
   public final var scale: Float = 0.0F
   public final var initial: Char = '\u0000'
   public final var next: TestDefaultPropertyInitializer? = null
   public final val zero: Int = 0
   public final var assigned: Int
   public final var notDefault: Int
   public final lateinit var late: String

   init {
      this.start = start
      this.notDefault = 5
      this.assigned = this.start
   }

   public fun close() {
      this.closed = true
      val var1: Int = this.count++
   }

   public companion object {
      public final var instances: Int = 0
      public final var last: TestDefaultPropertyInitializer? = null
   }

   public object Registry {
      public final var size: Int = 0
      public final var enabled: Boolean = false
   }
}

package pkg

public class TestDefaultPropertyInitializer(start: Int) {
   public final val start: Int

   public final var count: Int = 0
      internal set

   private final var closed: Boolean = false

   public final var total: Long = 0L
      internal set

   public final var ratio: Double = 0.0
      internal set

   public final var scale: Float = 0.0F
      internal set

   public final var initial: Char = '\u0000'
      internal set

   public final var next: TestDefaultPropertyInitializer? = null
      internal set

   public final val zero: Int = 0

   public final var assigned: Int
      internal set

   public final var notDefault: Int
      internal set

   public final lateinit var late: String
      internal set

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
         internal set

      public final var last: TestDefaultPropertyInitializer? = null
         internal set
   }

   public object Registry {
      public final var size: Int = 0
         internal set

      public final var enabled: Boolean = false
         internal set
   }
}

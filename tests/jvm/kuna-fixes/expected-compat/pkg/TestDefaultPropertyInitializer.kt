package pkg

public class TestDefaultPropertyInitializer(start: Int) {
   public final val start: Int

   public final var count: Int
      internal set

   private final var closed: Boolean

   public final var total: Long
      internal set

   public final var ratio: Double
      internal set

   public final var scale: Float
      internal set

   public final var initial: Char
      internal set

   public final var next: TestDefaultPropertyInitializer?
      internal set

   public final val zero: Int

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
      public final var instances: Int
         internal set

      public final var last: TestDefaultPropertyInitializer?
         internal set
   }

   public object Registry {
      public final var size: Int
         internal set

      public final var enabled: Boolean
         internal set
   }
}

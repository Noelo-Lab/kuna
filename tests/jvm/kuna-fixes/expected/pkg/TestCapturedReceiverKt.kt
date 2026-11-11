package pkg

public operator fun Array<String>.iterator(): Iterator<String> {
   return object : Iterator<String> {
      public final var i: Int = 0

      public open operator fun hasNext(): Boolean {
         return this.i < `$this$iterator`.size
      }

      public override operator fun next(): String {
         return `$this$iterator`[this.i++]
      }

      fun remove() {
         throw UnsupportedOperationException("Operation is not supported for read-only collection")
      }
   }
}

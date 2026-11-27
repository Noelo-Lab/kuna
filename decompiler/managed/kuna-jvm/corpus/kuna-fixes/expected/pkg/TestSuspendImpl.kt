package pkg

public open class TestSuspendImpl(src: ImplSource) {
   public final val src: ImplSource

   init {
      this.src = src
   }

   public open suspend fun sum(n: Int): Int {
      var s: Int = 0

      repeat(n) { i ->
         s += this.src.next()
      }

      return s
   }

   public open suspend fun first(): Int {
      return this.src.next() + this.offset()
   }

   protected open fun offset(): Int {
      return 0
   }
}

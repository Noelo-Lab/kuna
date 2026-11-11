package pkg

public open class TestSuspendImpl(src: ImplSource) {
   public final val src: ImplSource

   init {
      this.src = src
   }

   public open suspend fun sum(n: Int): Int {
      return sum$suspendImpl(this, n, `$completion`)
   }

   public open suspend fun first(): Int {
      return first$suspendImpl(this, `$completion`)
   }

   protected open fun offset(): Int {
      return 0
   }
}

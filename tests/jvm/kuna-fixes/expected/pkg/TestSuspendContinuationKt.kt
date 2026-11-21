package pkg

public suspend fun sumOf(src: SuspendSource, n: Int): Int {
   var total: Int = 0

   repeat(n) { i ->
      total += src.next()
   }

   return total
}

public suspend fun firstPositive(src: SuspendSource, tries: Int): Int? {
   // $VF: Unable to resugar Kotlin loop from Java for loop
   var left: Int = tries
   while (left > 0) {
      val v: Int = src.next()
      if (v > 0) {
         return v
      }

      left--
   }

   return null
}

public fun summing(src: SuspendSource): SuspendSink {
   return object : SuspendSink {
      public final var sum: Long = 0L

      public override suspend fun put(v: Int) {
         this.sum = this.sum + (v + src.next())
         return Unit.INSTANCE
      }
   }
}

public suspend fun SequenceScope<Int>.yieldTwice(x: Int) {
   this.yield(x)
   this.yield(x + 1)
   return Unit.INSTANCE
}

package pkg

public fun twice(src: LambdaSource): suspend () -> Int {
   return {
      src.next() + src.next()
   }
}

public fun plain(x: Int): suspend () -> Int {
   return {
      x + 1
   }
}

public fun sumOf(src: LambdaSource, n: Int): suspend () -> Int {
   return {
      var s: Int = 0

      repeat(n) { i ->
         s += src.next()
      }

      s
   }
}

public fun lengthPlus(src: LambdaSource): suspend (String) -> Int {
   return { s: String ->
      s.length + src.next()
   }
}

public fun pump(src: LambdaSource): suspend (LambdaSink<Int>, Int) -> Unit {
   return pump_1@{ n: Int ->
      repeat(n) { i ->
         this@pump_1.emit(src.next())
      }
   }
}

public fun guarded(src: LambdaSource): suspend () -> String {
   return {
      var var2: String
      try {
         var2 = src.text(src.next())
      } catch (var6: IllegalStateException) {
         var2 = "caught ${var6.getMessage()}"
      }

      var2
   }
}

public fun scaled(mode: LambdaMode, src: LambdaSource): suspend () -> Int {
   return {
      var var10000: Int
      when (mode) {
         LambdaMode.ONE -> var10000 = src.next()
         LambdaMode.TEN -> var10000 = src.next() * 10
         else -> throw NoWhenBranchMatchedException()
      }

      var10000
   }
}

public fun firstPositive(src: LambdaSource): suspend () -> Int {
   return lambda_1@{
      repeat(5) { k ->
         val v: Int = src.next()
         if (v > 0) {
            return@lambda_1 v
         }
      }

      return@lambda_1 -1
   }
}

public suspend fun runTwice(block: suspend () -> Unit) {
   block()
   block()
   return Unit.INSTANCE
}

public fun nested(src: LambdaSource): suspend (LambdaSink<Int>) -> Unit {
   return nested_1@{
      runTwice({
         this@nested_1.emit(src.next())
      })
   }
}

private suspend fun helper(src: LambdaSource): Int {
   return src.next() + 1
}

public fun viaHelper(src: LambdaSource): suspend () -> Int {
   return {
      helper(src) * 2
   }
}

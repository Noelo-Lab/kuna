package pkg

public fun fail(message: String): Nothing {
   throw IllegalStateException(message)
}

private fun unsupported(): Nothing {
   throw UnsupportedOperationException("unsupported")
}

public fun pick(x: Int): Int {
   if (x < 0) {
      fail("negative")
   } else {
      return x
   }
}

public fun pickOrFail(x: Int): Int {
   if (x > 0) {
      return x
   } else {
      fail("not positive")
   }
}

public fun name(x: Int): String {
   var var10000: String
   when (x) {
      0 -> var10000 = "zero"
      1 -> var10000 = "one"
      else -> fail("other $x")
   }

   return var10000
}

public suspend fun nextOrNever(src: NothingSource): Int {
   val v: Int = src.next()
   if (v < 0) {
      src.never()
   } else {
      return v
   }
}

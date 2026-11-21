package pkg

public class TestThrowableCast {
   public fun fail(n: Int): Nothing {
      throw IllegalArgumentException("bad: $n")
   }

   public fun require(ok: Boolean, message: String) {
      if (!ok) {
         throw IllegalStateException(message)
      }
   }

   public fun <T : Throwable> rethrow(t: T): Nothing {
      throw t
   }

   public fun pick(a: RuntimeException, b: Error, first: Boolean): Nothing {
      throw if (first) a as Throwable else b as Throwable
   }

   public fun wrap(cause: Throwable): Nothing {
      throw RuntimeException("wrapped", cause)
   }
}

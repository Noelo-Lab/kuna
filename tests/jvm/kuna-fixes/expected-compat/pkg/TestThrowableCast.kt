package pkg

public class TestThrowableCast {
   public fun fail(n: Int): Nothing {
      throw IllegalArgumentException("bad: $n") as java.lang.Throwable
   }

   public fun require(ok: Boolean, message: String) {
      if (!ok) {
         throw IllegalStateException(message) as java.lang.Throwable
      }
   }

   public fun <T : Throwable> rethrow(t: T): Nothing {
      throw t as java.lang.Throwable
   }

   public fun pick(a: RuntimeException, b: Error, first: Boolean): Nothing {
      throw (if (first) a as java.lang.Throwable else b as java.lang.Throwable) as java.lang.Throwable
   }

   public fun wrap(cause: Throwable): Nothing {
      throw RuntimeException("wrapped", cause) as java.lang.Throwable
   }
}

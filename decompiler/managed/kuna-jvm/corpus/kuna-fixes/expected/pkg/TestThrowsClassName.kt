package pkg

public class TestThrowsClassName {
   @Throws(TestThrowsClassName.Failure::class)
   public fun load(path: String): Int {
      if ((path as CharSequence).length == 0) {
         throw TestThrowsClassName.Failure("empty")
      } else {
         return path.length
      }
   }

   @Throws(TestThrowsClassName.Failure::class, IllegalStateException::class, Throwable::class)
   public fun check(n: Int) {
      if (n < 0) {
         throw IllegalStateException("negative")
      }
   }

   public class Failure(message: String) : Exception(message)
}

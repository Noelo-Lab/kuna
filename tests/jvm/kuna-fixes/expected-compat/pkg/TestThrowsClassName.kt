package pkg

public class TestThrowsClassName {
   @Throws(pkg/TestThrowsClassName$Failure::class)
   public fun load(path: String): Int {
      if ((path as java.lang.CharSequence).length() == 0) {
         throw TestThrowsClassName.Failure("empty")
      } else {
         return path.length()
      }
   }

   @Throws(pkg/TestThrowsClassName$Failure::class, java/lang/IllegalStateException::class, java/lang/Throwable::class)
   public fun check(n: Int) {
      if (n < 0) {
         throw IllegalStateException("negative")
      }
   }

   public class Failure(message: String) : Exception(message)
}

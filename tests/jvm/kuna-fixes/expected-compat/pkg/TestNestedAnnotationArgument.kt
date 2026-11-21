package pkg

public class TestNestedAnnotationArgument {
   public final val size: Int = 3

   @Deprecated(message = "moved to val", replaceWith = @ReplaceWith(expression = "size", imports = []), level = DeprecationLevel.ERROR)
   public fun size(): Int {
      return this.size
   }

   @Deprecated(message = "moved to extension", replaceWith = @ReplaceWith(expression = "other.copy()", imports = ["pkg.copy"]))
   public fun copy(other: String): String {
      return "$other!"
   }
}

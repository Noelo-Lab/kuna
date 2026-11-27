package pkg

public class TestStringLiteralEscape {
   public fun receiverName(): String {
      return "\$this\$receiver"
   }

   public fun braces(): String {
      return "\${name}"
   }

   public fun price(n: Int): String {
      return "costs \$$n"
   }

   public fun template(x: Int): String {
      return "x=$x and \$x"
   }

   public fun lone(): String {
      return "a $ b $"
   }

   public fun feed(): String {
      return "page\u000cbreak"
   }

   public fun glued(key: String): String {
      return "${key}258EAF-E914"
   }

   public fun underscore(key: String): String {
      return "${key}_suffix"
   }

   public fun spaced(key: String): String {
      return "$key-E914"
   }

   public fun quote(a: String, b: Int): String {
      return "$a\"$b"
   }

   public fun dollarChar(a: String, b: Int): String {
      return "$a\$$b"
   }

   public fun newline(a: String, b: Int): String {
      return "$a\n$b"
   }
}

package pkg

class TestStringLiteralEscape {
    fun receiverName(): String = "\$this\$receiver"
    fun braces(): String = "\${name}"
    fun price(n: Int): String = "costs \$" + n
    fun template(x: Int): String = "x=$x and \$x"
    fun lone(): String = "a $ b \$"
    fun feed(): String = "page\u000cbreak"
    fun glued(key: String): String = "${key}258EAF-E914"
    fun underscore(key: String): String = "${key}_suffix"
    fun spaced(key: String): String = "$key-E914"
    fun quote(a: String, b: Int): String = a + '"' + b
    fun dollarChar(a: String, b: Int): String = a + '$' + b
    fun newline(a: String, b: Int): String = a + '\n' + b
}

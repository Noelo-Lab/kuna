package pkg

class TestJavaClassLiteral {
    val tag: String = TestJavaClassLiteral::class.java.name

    fun local(): String = TestJavaClassLiteral::class.java.simpleName

    companion object {
        val NAME: String = TestJavaClassLiteral::class.java.name
        val TYPE: Class<*> = Companion::class.java
    }
}

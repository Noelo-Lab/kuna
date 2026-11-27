package pkg

class TestExpressionValueCheck {
    fun describe(lib: JavaLib): String {
        val name: String = lib.name()
        return name + lib.name().length
    }

    companion object {
        val DEFAULT: String = JavaLib.version()
        val UPPER: String = JavaLib.version() + "!"
    }
}

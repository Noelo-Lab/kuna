package pkg

open class TestLocalVisibilityComment(val a: Int) {
    fun make(x: Int): Any = object {
        inner class Sub : TestLocalVisibilityComment(x)

        val sub = Sub()
    }
}

package pkg

class TestSuspendFunctionType(val action: suspend (String) -> Int) {
    var hook: (suspend () -> Unit)? = null

    fun pick(a: suspend () -> Unit, b: suspend (Int, String) -> Boolean, first: Boolean): Any = if (first) a else b

    fun plain(f: (Int) -> Int): Int = f(1)
}

package pkg

class TestDefaultArgAssignedParam {
    fun chain(s: Int, t: Int = compute { val u = measure(s); val v = u + u; v - 1 }): Int = s + t

    fun twice(x: Int, y: Int = compute { val q = measure(x); q * q }): Int = x * y

    fun plain(a: Int, b: Int = measure(a) + 1): Int = a * b

    fun measure(n: Int): Int = n * 3

    private inline fun compute(block: () -> Int): Int = block()
}

package pkg

class TestConstructorDiamond {
    class Bag<T>(val capacity: Int)

    class Pair<A, B>(val first: A, val second: B)

    private val bag: Bag<String> = Bag(4)

    fun empty(): Bag<Long> = Bag(0)

    fun <A, B> pair(a: A, b: B): Pair<A, B> = Pair(a, b)

    fun explicit(): Any = Bag<Int>(1)
}

package pkg

class User {
    fun pick(gen: Gen, range: IntRange): Int = gen.next(range)
}

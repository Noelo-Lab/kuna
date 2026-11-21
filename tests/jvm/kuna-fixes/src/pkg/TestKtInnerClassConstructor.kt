package pkg

class TestKtInnerClassConstructor(val base: Int) {
    inner class Part(val x: Int) {
        fun sum(): Int = base + x

        fun next(): Part = Part(x + 1)
    }

    inner class Empty {
        fun base(): Int = base
    }

    fun make(x: Int): Part = Part(x)

    fun empty(): Empty = Empty()

    fun other(o: TestKtInnerClassConstructor, x: Int): Part = o.Part(x)
}

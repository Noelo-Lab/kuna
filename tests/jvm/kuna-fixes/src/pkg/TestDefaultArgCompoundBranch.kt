package pkg

class TestDefaultArgCompoundBranch {
    fun make(names: List<String>): Any = object {
        fun total(lengths: List<Int> = names.map { it.length }): Int = lengths.sum()

        inner class Box(val sizes: List<Int> = names.map { it.length * 2 })

        fun simple(n: Int = 7): Int = n + 1
    }
}

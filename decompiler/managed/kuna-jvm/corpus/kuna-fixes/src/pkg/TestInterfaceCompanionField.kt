package pkg

interface TestInterfaceCompanionField {
    fun id(): Int

    companion object Key {
        fun twice(n: Int): Int = n * 2
    }
}

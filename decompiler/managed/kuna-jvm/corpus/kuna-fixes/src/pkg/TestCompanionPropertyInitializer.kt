package pkg

class TestCompanionPropertyInitializer(val id: Int) {
    companion object {
        val INSTANCES: MutableMap<String, TestCompanionPropertyInitializer> = LinkedHashMap()
        val FIRST = TestCompanionPropertyInitializer(1)
        @JvmField
        val SECOND = TestCompanionPropertyInitializer(2)
        var created: Int = INSTANCES.size + 7
        private val names = arrayOf("a", "b")
        const val LIMIT: Int = 64

        fun name(i: Int): String = names[i]
    }
}

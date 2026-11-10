package pkg

class TestKtPropertyFieldFlags {
    @Volatile
    var state: Any? = null

    @Volatile
    private var counter: Int = 0

    @Transient
    var cache: String? = null

    @Volatile
    @JvmField
    var shared: Any? = null

    @Transient
    private val memo: IntArray = IntArray(4)

    @delegate:Transient
    val label: String by lazy { "label" }

    @field:JvmSynthetic
    var synthetic: Int = 0

    var plain: Int = 0

    fun bump(): Int {
        counter = counter + 1
        memo[0] = counter
        return counter
    }

    object Registry {
        @Volatile
        var current: String? = null
    }

    companion object {
        @Volatile
        private var instance: TestKtPropertyFieldFlags? = null

        fun get(): TestKtPropertyFieldFlags? = instance
    }
}

package pkg

open class TestKtJvmFieldSetter {
    @JvmField
    var count: Int = 0

    @JvmField
    internal var name: String = ""

    @JvmField
    protected var tag: Any? = null

    @JvmField
    val fixed: Int = 1

    var hidden: Int = 0
        private set

    internal var total: Long = 0
        private set

    lateinit var label: String
        private set

    private var own: Int = 0

    fun touch(): Long {
        hidden += 1
        total += hidden
        own += 1
        label = "x"
        return total + own
    }

    companion object {
        @JvmField
        var shared: String? = null

        var limit: Int = 0
            private set
    }
}

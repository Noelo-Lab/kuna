package pkg

class TestDefaultPropertyInitializer(val start: Int) {
    var count: Int = 0
    private var closed = false
    var total: Long = 0L
    var ratio: Double = 0.0
    var scale: Float = 0.0f
    var initial: Char = '\u0000'
    var next: TestDefaultPropertyInitializer? = null
    val zero: Int = 0
    var assigned: Int = 0
    var notDefault: Int = 5
    lateinit var late: String

    init {
        assigned = start
    }

    fun close() {
        closed = true
        count++
    }

    companion object {
        var instances: Int = 0
        var last: TestDefaultPropertyInitializer? = null
    }

    object Registry {
        var size: Int = 0
        var enabled: Boolean = false
    }
}

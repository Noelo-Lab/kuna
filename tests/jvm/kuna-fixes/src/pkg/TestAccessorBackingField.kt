package pkg

class TestAccessorBackingField {
    @get:Synchronized
    var limit: Int = 64
        set(value) {
            require(value >= 1) { "limit < 1: $value" }
            synchronized(this) {
                field = value
            }
            changes++
        }

    var changes: Int = 0
        private set

    var name: String = "a"
        get() = field + "!"
        set(value) {
            field = value.trim()
        }

    val doubled: Int = 21
        get() = field * 2

    var counter: Long = 0L
        get() {
            field++
            return field
        }

    fun bump(): Int {
        limit = limit + 1
        return limit
    }

    companion object {
        var shared: String = "s"
            get() = field + "?"
            set(value) {
                field = value + "."
            }
    }
}

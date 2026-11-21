package pkg

class TestOuterThis(private val name: String) {
    private var hits = 0

    inner class Reader(val limit: Int) {
        fun read(): String {
            hits++
            return name + limit + this@TestOuterThis.hits
        }

        fun describe(): String = "${this@TestOuterThis}/" + this@Reader.limit

        inner class Cursor {
            fun owner(): TestOuterThis = this@TestOuterThis
            fun reader(): Reader = this@Reader
        }
    }

    override fun toString(): String = "TestOuterThis($name)"
}

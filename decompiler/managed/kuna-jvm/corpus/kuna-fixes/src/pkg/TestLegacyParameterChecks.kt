package pkg

class TestLegacyParameterChecks(val label: String) {
    open class Base(val size: Int) {
        constructor() : this(0)
    }

    class Holder(private val text: String, count: Int) : Base(text.length) {
        val total = count * 2
        val names = ArrayList<String>()

        fun describe(): String = text + total + names
    }

    class Secondary : Base {
        val text: String

        constructor(text: String) : super(text.length) {
            this.text = text
        }

        constructor(head: String, tail: String) : this(head + tail)
    }

    class Buffers {
        val data: ByteArray
        var owner: Boolean

        constructor() {
            data = ByteArray(8)
            owner = true
        }

        constructor(data: ByteArray, owner: Boolean) {
            this.data = data
            this.owner = owner
        }
    }

    inner class Tagged(val tag: String) : Base(tag.length + label.length)
}

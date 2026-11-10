package pkg

class TestUnrepresentableType {
    interface Source {
        fun read(): Int
    }

    private val counter = object : Source {
        var reads = 0
        override fun read(): Int = ++reads
    }

    fun twice(): Int {
        val source = object : Source {
            var last = 0
            override fun read(): Int {
                last += 2
                return last
            }
        }
        source.read()
        return source.read() + source.last + counter.read() + counter.reads
    }

    class Clause(val owner: Any, val register: (Any, Any, Any?) -> Unit)

    var calls = 0

    private fun register(target: Any, param: Any?) {
        calls += if (param == null) 1 else 2
    }

    @Suppress("UNCHECKED_CAST")
    val clause: Clause
        get() = Clause(this, TestUnrepresentableType::register as (Any, Any, Any?) -> Unit)
}

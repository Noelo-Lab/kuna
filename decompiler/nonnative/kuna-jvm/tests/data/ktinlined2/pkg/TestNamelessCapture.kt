package pkg

open class NamelessContext(val tag: String)

object NamelessEmpty : NamelessContext("empty")

interface NamelessSink {
    val context: NamelessContext
    fun put(value: Int): Int
}

inline fun namelessSink(context: NamelessContext, crossinline f: (Int) -> Int): NamelessSink = object : NamelessSink {
    override val context: NamelessContext
        get() = context

    override fun put(value: Int): Int = f(value) + context.tag.length
}

class TestNamelessCapture(private val base: Int) {
    fun sink(): NamelessSink = namelessSink(NamelessEmpty) { it * base }
}

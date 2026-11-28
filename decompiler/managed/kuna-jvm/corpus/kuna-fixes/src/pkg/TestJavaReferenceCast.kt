package pkg

class TestJavaReferenceCast<V> private constructor(private var values: Array<V>?, private var count: Long) {
    constructor() : this(null, 0L)

    fun first(): V? = values?.get(0)

    fun limited(): Long {
        val n = 32
        return Overloads.least(n.toLong(), count)
    }

    fun none(): Int = Overloads.name(null as String?)

    fun <T> arrayOrNull(size: Int): Array<T?> {
        @Suppress("UNCHECKED_CAST")
        return arrayOfNulls<Any>(size) as Array<T?>
    }

    fun <T> boxed(element: T): Box<T> = Overloads.one(element)

    class Box<T>(val value: T)

    object Overloads {
        @JvmStatic fun least(a: Long, b: Long): Long = if (a < b) a else b
        @JvmStatic fun name(s: String?): Int = 1
        @JvmStatic fun name(s: Box<String>?): Int = 2
        @JvmStatic fun <T> one(e: T): Box<T> = Box(e)
    }
}

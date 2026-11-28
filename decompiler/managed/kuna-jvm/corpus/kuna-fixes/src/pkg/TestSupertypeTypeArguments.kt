package pkg

class TestSupertypeTypeArguments<E>(private val seed: E) {
    open class Box<T>(val value: T, val tag: String)

    class IntBox(v: Int) : Box<Int>(v, "int")

    class Keyed<K, V>(key: K, val other: V) : Box<K>(key, "keyed")

    open inner class Holder<X>(val held: X)

    inner class Seeded : Holder<E>(seed)

    fun make(): Seeded = Seeded()
}

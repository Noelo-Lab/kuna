package pkg

class TestKtReusedInlineObject {
    fun interface Order<T> {
        fun compare(a: T, b: T): Int
    }

    interface Label<T> {
        fun label(value: T): String
    }

    inline fun <T> byKey(crossinline key: (T) -> Int): Order<T> = orderBy(key)

    inline fun <T> reversedByKey(crossinline key: (T) -> Int): Order<T> = orderByDescending(key)

    fun <T> plainByKey(key: (T) -> Int): Order<T> = orderBy(key)

    inline fun <T> labelled(crossinline name: (T) -> String): Label<T> = labelBy(name)

}

inline fun <T> orderBy(crossinline key: (T) -> Int): TestKtReusedInlineObject.Order<T> =
    TestKtReusedInlineObject.Order { a, b -> key(a) - key(b) }

inline fun <T> orderByDescending(crossinline key: (T) -> Int): TestKtReusedInlineObject.Order<T> =
    TestKtReusedInlineObject.Order { a, b -> key(b) - key(a) }

inline fun <T> labelBy(crossinline name: (T) -> String): TestKtReusedInlineObject.Label<T> =
    object : TestKtReusedInlineObject.Label<T> {
        override fun label(value: T): String = "<" + name(value) + ">"
    }

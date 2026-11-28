package pkg

inline fun <T> byLength(crossinline f: (T) -> String): Comparator<T> =
    Comparator { a, b -> f(a).length - f(b).length }

inline fun <T> byLengthOf(crossinline f: (T) -> String): Comparator<T> = byLength(f)

inline fun <T> nullsFirstBy(crossinline f: (T) -> String?): Comparator<T> = Comparator { a, b ->
    val x = f(a)
    val y = f(b)
    if (x == null) {
        return@Comparator if (y == null) 0 else -1
    }
    if (y == null) {
        return@Comparator 1
    }
    x.length - y.length
}

inline fun twice(crossinline g: (Int) -> Int): (Int) -> Int = { g(it) * 2 }

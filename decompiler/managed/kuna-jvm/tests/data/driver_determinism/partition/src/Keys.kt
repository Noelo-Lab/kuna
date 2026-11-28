package pkg

inline fun <T> byKey(crossinline key: (T) -> Int): Comparator<T> = Comparator { a, b -> key(a) - key(b) }

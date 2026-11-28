package pkg

fun <T> sortedByKey(list: List<T>, key: (T) -> Int): List<T> = list.sortedWith(byKey(key))

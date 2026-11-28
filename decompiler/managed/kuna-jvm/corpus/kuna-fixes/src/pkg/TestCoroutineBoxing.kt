package pkg

suspend fun count(items: List<String>): Int = items.size

suspend fun empty(items: List<String>): Boolean = items.isEmpty()

suspend fun scaled(items: List<String>, by: Long): Long? = if (items.isEmpty()) null else items.size * by

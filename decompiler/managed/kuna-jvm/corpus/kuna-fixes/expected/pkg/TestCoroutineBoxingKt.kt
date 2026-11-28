package pkg

public suspend fun count(items: List<String>): Int {
   return items.size
}

public suspend fun empty(items: List<String>): Boolean {
   return items.isEmpty()
}

public suspend fun scaled(items: List<String>, by: Long): Long? {
   return if (items.isEmpty()) null else items.size.toLong() * by
}

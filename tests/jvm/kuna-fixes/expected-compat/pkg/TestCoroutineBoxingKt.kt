package pkg

import kotlin.coroutines.jvm.internal.Boxing

public suspend fun count(items: List<String>): Int {
   return Boxing.boxInt(items.size())
}

public suspend fun empty(items: List<String>): Boolean {
   return Boxing.boxBoolean(items.isEmpty())
}

public suspend fun scaled(items: List<String>, by: Long): Long? {
   return if (items.isEmpty()) null else Boxing.boxLong((long)items.size() * by)
}

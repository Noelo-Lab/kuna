@file:JvmName(name = "TestBoxedReceiverTypeArgument")

package pkg

public fun <T> T.firstOr(bound: List<T>): T {
   return (if (bound.isEmpty()) this else bound.get(0)) as T
}

public fun Int.firstOr(bound: Collection<Int>): Int {
   if (bound is List<*>) {
      return (this.firstOr<Int>(bound as List<Int>) as Number).toInt()
   } else {
      return if (bound.isEmpty()) this else (bound.iterator().next() as Number).toInt()
   }
}

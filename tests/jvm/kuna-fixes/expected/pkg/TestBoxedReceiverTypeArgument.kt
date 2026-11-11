@file:JvmName(name = "TestBoxedReceiverTypeArgument")

package pkg

public fun <T> T.firstOr(bound: List<T>): T {
   return (if (bound.isEmpty()) this else bound.get(0)) as T
}

public fun Int.firstOr(bound: Collection<Int>): Int {
   if (bound is java.util.List) {
      return (this.firstOr<Int>(bound as MutableList<out Int>) as Number).toInt()
   } else {
      return if (bound.isEmpty()) this else (bound.iterator().next() as Number).toInt()
   }
}

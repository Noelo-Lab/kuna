@file:JvmName(name = "TestBoxedReceiverTypeArgument")

package pkg

public fun <T> T.firstOr(bound: List<T>): T {
   return (T)(if (bound.isEmpty()) `$this$firstOr` else bound.get(0))
}

public fun Int.firstOr(bound: Collection<Int>): Int {
   if (bound is java.util.List) {
      return (firstOr(Integer.valueOf(`$this$firstOr`), bound as MutableList<Int>) as java.lang.Number).intValue()
   } else {
      return if (bound.isEmpty()) `$this$firstOr` else (bound.iterator().next() as java.lang.Number).intValue()
   }
}

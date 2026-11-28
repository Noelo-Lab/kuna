@file:JvmName("TestBoxedReceiverTypeArgument")

package pkg

fun <T> T.firstOr(bound: List<T>): T = if (bound.isEmpty()) this else bound[0]

fun Int.firstOr(bound: Collection<Int>): Int {
    if (bound is List) {
        return this.firstOr<Int>(bound)
    }
    return if (bound.isEmpty()) this else bound.iterator().next()
}

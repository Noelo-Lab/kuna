package pkg

var hits = 0

@JvmOverloads
fun scale(x: Int, k: Int = 3): Int = x * k

fun maker(k: Int): () -> Any = {
    object {
        fun value(v: Int): Int {
            if (v > k) {
                hits++
                return v
            }
            return k
        }
    }
}

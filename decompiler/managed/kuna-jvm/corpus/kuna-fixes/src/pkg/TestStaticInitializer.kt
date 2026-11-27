package pkg

object TestStaticInitializer {
    val table: IntArray
    val ready: Boolean

    init {
        val t = IntArray(16)
        for (i in t.indices) t[i] = i * i
        table = t
        ready = try {
            t[15] > 100
        } catch (e: IllegalStateException) {
            false
        }
    }

    class Platform {
        companion object {
            val isSupported: Boolean = try {
                Class.forName("pkg.Missing")
                true
            } catch (e: ClassNotFoundException) {
                false
            }
            val level: Int = if (isSupported) 2 else 1
        }
    }
}

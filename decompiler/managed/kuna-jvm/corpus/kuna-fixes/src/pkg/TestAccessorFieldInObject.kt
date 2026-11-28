package pkg

class TestAccessorFieldInObject {
    var count: Int = 0
        set(v) {
            val r = object : Runnable {
                override fun run() {
                    field = v
                }
            }
            r.run()
        }

    var label: String = "a"
        get() {
            val r = object : Runnable {
                override fun run() {
                    println(field)
                }
            }
            r.run()
            return field
        }

    private var hits: Int = 0

    fun hit(): Runnable = object : Runnable {
        override fun run() {
            hits = hits + 1
        }
    }
}

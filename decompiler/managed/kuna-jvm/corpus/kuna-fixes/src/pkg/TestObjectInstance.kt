package pkg

object TestObjectInstance {
    private val pool = ArrayList<IntArray>()
    var taken = 0

    fun take(): IntArray {
        taken++
        return if (pool.isEmpty()) IntArray(8) else pool.removeAt(pool.size - 1)
    }

    fun recycle(segment: IntArray) {
        pool.add(segment)
    }

    class User {
        fun cycle(): Int {
            val s = TestObjectInstance.take()
            TestObjectInstance.recycle(s)
            return TestObjectInstance.taken + s.size
        }
    }
}

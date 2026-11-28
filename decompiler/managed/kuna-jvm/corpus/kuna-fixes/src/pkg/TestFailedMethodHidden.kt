package pkg

interface TestFailedMethodHidden {
    fun note(x: Int)

    suspend fun next(): Int

    suspend fun twice(): Int {
        val a = next()
        return a + next()
    }

    fun reset(x: Int) {
        if (x < 0) {
            note(0)
            return
        }
        note(x)
    }
}

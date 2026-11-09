package pkg

class TestSuspendTryFinally {
    interface Step {
        suspend fun run(): Int
        fun done()
    }

    suspend fun runThenDone(step: Step): Int {
        try {
            return step.run()
        } finally {
            step.done()
        }
    }
}

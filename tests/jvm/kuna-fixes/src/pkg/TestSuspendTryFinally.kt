package pkg

class TestSuspendTryFinally {
    interface Step {
        suspend fun run(): Int
        fun last(): Boolean
        fun done()
    }

    suspend fun runThenDone(step: Step): Int {
        try {
            return step.run()
        } finally {
            step.done()
        }
    }

    suspend fun runThenMaybeDone(step: Step) {
        try {
            step.run()
        } finally {
            if (step.last()) step.done()
        }
    }
}

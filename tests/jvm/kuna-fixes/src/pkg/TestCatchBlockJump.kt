package pkg

class TestCatchBlockJump {
    interface Task {
        fun run(): Int
    }

    class Failure : RuntimeException()

    private var failures: Int = 0

    fun runAll(tasks: Array<Task>): Int {
        var done = 0
        for (t in tasks) {
            val r = try {
                t.run()
            } catch (e: Failure) {
                failures++
                continue
            }
            done += r
        }
        return done
    }

    fun firstWorking(tasks: Array<Task>): Int {
        var result = -1
        for (t in tasks) {
            val r = try {
                t.run()
            } catch (e: Failure) {
                failures++
                break
            }
            result = r
        }
        return result + failures
    }
}

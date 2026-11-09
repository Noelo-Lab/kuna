package pkg

suspend fun step(): Int = 1

suspend fun once(done: () -> Unit) {
    try {
        step()
    } finally {
        done()
    }
}

suspend fun twice(done: () -> Unit): Int {
    try {
        val a = step()
        val b = step()
        return a + b
    } finally {
        done()
    }
}

suspend fun early(flag: Boolean, done: () -> Unit): Int {
    try {
        if (flag) {
            return 0
        }
        return step()
    } finally {
        done()
    }
}

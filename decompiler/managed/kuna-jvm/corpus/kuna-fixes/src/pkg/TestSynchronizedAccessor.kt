package pkg

class TestSynchronizedAccessor {
    @get:Synchronized var a: Int = 1
    @get:Synchronized @set:Synchronized var b: Int = 2
    var c: Int = 3
        @Synchronized get() = field + 1
        @Synchronized set(value) {
            field = value * 2
        }
    @get:Synchronized val d: Int get() = a + b
}

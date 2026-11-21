package pkg

import java.util.concurrent.atomic.AtomicReference

class TestSetterParameterName {
    var total: Long = 0L
        @JvmName("updateTotal") set

    var label: String = ""
        @JvmName("relabel") set

    var count: Int = 0
        set(amount) {
            field = if (amount < 0) 0 else amount
        }
}

var <T> AtomicReference<T>.current: T
    get() = get()
    set(value) {
        set(value)
    }

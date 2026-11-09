package kt4

open class CtorBase

class CtorOnly : CtorBase {
    constructor() : super() {
        println("secondary")
    }
}

class InitOnly {
    init {
        println("init")
    }
}

enum class E2 { A, B }

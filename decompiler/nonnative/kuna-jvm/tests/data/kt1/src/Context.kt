package pkg
interface Logger { fun log(s: String) }
context(Logger)
fun logged(x: Int): Int { log("x=$x"); return x }
class Scope { context(Logger) fun inScope() = log("in") }

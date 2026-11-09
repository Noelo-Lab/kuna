package pkg

import kotlin.jvm.functions.Function1
import kotlin.jvm.functions.Function2
import kotlin.jvm.functions.Function3
import kotlin.reflect.KFunction

public fun twice(x: Int): Int {
   return x * 2
}

public fun bound(adder: Adder): Int {
   return (((adder::add as KFunction) as Function2)(1, 2) as java.lang.Number).intValue()
}

public fun unbound(adder: Adder): Int {
   return (((Adder::add as KFunction) as Function3)(adder, 1, 2) as java.lang.Number).intValue()
}

public fun constructor(): Int {
   return (((::Adder as KFunction) as Function1)(3) as Adder).getK()
}

public fun topLevel(x: Int): Int {
   return (((::twice as KFunction) as Function1)(x) as java.lang.Number).intValue()
}

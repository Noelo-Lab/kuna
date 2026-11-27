package pkg

import kotlin.jvm.functions.Function2
import kotlin.jvm.functions.Function3
import kotlin.reflect.KFunction

public fun boundMangled(h: MangledHolder): Int {
   return (((h::mark as KFunction) as Function2)(MangledTag(1), 2) as Number).toInt()
}

public fun unboundMangled(h: MangledHolder): Int {
   return (((MangledHolder::mark as KFunction) as Function3)(h, MangledTag(3), 4) as Number).toInt()
}

package pkg

import kotlin.jvm.functions.Function2
import kotlin.jvm.functions.Function3
import kotlin.reflect.KFunction

public fun boundMangled(h: MangledHolder): Int {
   return (((h::mark as KFunction) as Function2)(MangledTag.box_impl/* $VF was: box-impl */(MangledTag.constructor_impl/* $VF was: constructor-impl */(1)), 2) as java.lang.Number)
      .intValue()
   }

public fun unboundMangled(h: MangledHolder): Int {
   return (((MangledHolder::mark as KFunction) as Function3)(
         h, MangledTag.box_impl/* $VF was: box-impl */(MangledTag.constructor_impl/* $VF was: constructor-impl */(3)), 4
      ) as java.lang.Number)
      .intValue()
   }

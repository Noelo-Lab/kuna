package pkg

import kotlin.reflect.KMutableProperty0
import kotlin.reflect.KMutableProperty1
import kotlin.reflect.KProperty0
import kotlin.reflect.KProperty1

public final val answer: Int = 42

public final val doubled: Int
   public final get() {
      return `$this$doubled`.getSize() * 2
   }


public fun bound(box: Box): String {
   return java.lang.String.valueOf(box::size)
}

public fun boundMutable(box: Box): Int {
   return ((box::count as KMutableProperty0).get() as java.lang.Number).intValue()
}

public fun member(box: Box): Int {
   return ((Box::size as KProperty1).get(box) as java.lang.Number).intValue()
}

public fun mutableMember(box: Box) {
   (Box::count as KMutableProperty1).set(box, 1)
}

public fun topLevel(): Int {
   return ((::answer as KProperty0).get() as java.lang.Number).intValue()
}

public fun extension(box: Box): Int {
   return ((Box::doubled as KProperty1).get(box) as java.lang.Number).intValue()
}

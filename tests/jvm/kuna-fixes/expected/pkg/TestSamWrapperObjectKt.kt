@file:SourceDebugExtension(["SMAP\nTestSamWrapperObject.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSamWrapperObject.kt\npkg/TestSamWrapperObjectKt\n*L\n1#1,21:1\n4#1:22\n*S KotlinDebug\n*F\n+ 1 TestSamWrapperObject.kt\npkg/TestSamWrapperObjectKt\n*L\n6#1:22\n*E\n"])

package pkg

import java.util.Comparator
import kotlin.jvm.functions.Function1
import kotlin.jvm.internal.SourceDebugExtension

public inline fun <T> byLength(crossinline f: (T) -> String): Comparator<T> {
   return object : Comparator {
      fun compare(a: T, b: T): Int {
         return (f(a) as java.lang.String).length() - (f(b) as java.lang.String).length()
      }
   }
}

public inline fun <T> byLengthOf(crossinline f: (T) -> String): Comparator<T> {
   return object : Comparator {
      fun compare(a: T, b: T): Int {
         return (f(a) as java.lang.String).length() - (f(b) as java.lang.String).length()
      }
   }
}

public inline fun <T> nullsFirstBy(crossinline f: (T) -> String?): Comparator<T> {
   return object : Comparator {
      fun compare(a: T, b: T): Int {
         val x: java.lang.String = f(a) as java.lang.String
         val y: java.lang.String = f(b) as java.lang.String
         if (x == null) {
            return if (y == null) 0 else -1
         } else {
            return if (y == null) 1 else x.length() - y.length()
         }
      }
   }
}

public inline fun twice(crossinline g: (Int) -> Int): (Int) -> Int {
   return object : Function1<Integer, Integer> {
      fun invoke(it: Int): Int {
         return (g(it) as java.lang.Number).intValue() * 2
      }
   }
}

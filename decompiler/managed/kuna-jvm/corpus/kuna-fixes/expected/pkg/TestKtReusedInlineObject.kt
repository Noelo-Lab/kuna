package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtReusedInlineObject.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtReusedInlineObject.kt\npkg/TestKtReusedInlineObject\n+ 2 TestKtReusedInlineObject.kt\npkg/TestKtReusedInlineObjectKt\n*L\n1#1,32:1\n23#2:33\n26#2:34\n23#2:35\n29#2,3:36\n*S KotlinDebug\n*F\n+ 1 TestKtReusedInlineObject.kt\npkg/TestKtReusedInlineObject\n*L\n12#1:33\n14#1:34\n16#1:35\n18#1:36,3\n*E\n"])
public class TestKtReusedInlineObject {
   public inline fun <T> byKey(crossinline key: (T) -> Int): pkg.TestKtReusedInlineObject.Order<T> {
      return orderBy(key)
   }

   public inline fun <T> reversedByKey(crossinline key: (T) -> Int): pkg.TestKtReusedInlineObject.Order<T> {
      return orderByDescending(key)
   }

   public fun <T> plainByKey(key: (T) -> Int): pkg.TestKtReusedInlineObject.Order<T> {
      return orderBy(key)
   }

   public inline fun <T> labelled(crossinline name: (T) -> String): pkg.TestKtReusedInlineObject.Label<T> {
      return labelBy(name)
   }

   public interface Label<T> {
      public abstract fun label(value: T): String
   }

   public fun interface Order<T> {
      public abstract fun compare(a: T, b: T): Int
   }
}

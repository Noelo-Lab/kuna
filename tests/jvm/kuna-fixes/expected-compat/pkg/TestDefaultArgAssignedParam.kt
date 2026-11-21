package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestDefaultArgAssignedParam.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestDefaultArgAssignedParam.kt\npkg/TestDefaultArgAssignedParam\n*L\n1#1,14:1\n12#1:15\n12#1:16\n*S KotlinDebug\n*F\n+ 1 TestDefaultArgAssignedParam.kt\npkg/TestDefaultArgAssignedParam\n*L\n4#1:15\n6#1:16\n*E\n"])
public class TestDefaultArgAssignedParam {
   public fun chain(s: Int, t: Int = this.measure(s)): Int {
      return s + t
   }

   public fun twice(x: Int, y: Int = this.measure(x)): Int {
      return x * y
   }

   public fun plain(a: Int, b: Int = this.measure(a) + 1): Int {
      return a * b
   }

   public fun measure(n: Int): Int {
      return n * 3
   }

   private inline fun compute(block: () -> Int): Int {
      return (block() as java.lang.Number).intValue()
   }
}

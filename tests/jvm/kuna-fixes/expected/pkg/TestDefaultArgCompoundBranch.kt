package pkg

import kotlin.jvm.internal.SourceDebugExtension

public class TestDefaultArgCompoundBranch {
   public fun make(names: List<String>): Any {
      return object {
         public fun total(lengths: List<Int> = ...): Int {
            return CollectionsKt.sumOfInt(lengths as Iterable<Int>)
         }

         public fun simple(n: Int = 7): Int {
            return n + 1
         }

         @SourceDebugExtension(["SMAP\nTestDefaultArgCompoundBranch.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestDefaultArgCompoundBranch.kt\npkg/TestDefaultArgCompoundBranch\$make$1\$Box\n+ 2 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,12:1\n1557#2:13\n1628#2,3:14\n*S KotlinDebug\n*F\n+ 1 TestDefaultArgCompoundBranch.kt\npkg/TestDefaultArgCompoundBranch\$make$1\$Box\n*L\n7#1:13\n7#1:14,3\n*E\n"])
         /* $VF: local visibility outside of method */ internal inner class Box(sizes: List<Int> = ...) {
            public final val sizes: List<Int>

            init {
               this.sizes = sizes
            }
         }
      }
   }
}

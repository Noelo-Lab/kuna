package pkg

import java.util.Comparator
import kotlin.jvm.internal.SourceDebugExtension

public interface TestJavaTypeSyntax<T : java.lang.Comparable<in T>> {
   @JvmStatic
   TestJavaTypeSyntax.Companion Companion = TestJavaTypeSyntax.Companion.`$$INSTANCE`;

   public val low: T

   public open fun below(value: T): Boolean {
   }

   public open fun <R> sink(items: MutableList<in R>, from: Comparator<in T>): Int {
   }

   public class Box<S> : TestJavaTypeSyntax<S> where S : java.lang.CharSequence, S : java.lang.Comparable<in S> {
      public open val low: S
         public open get() {
            throw IllegalStateException()
         }


      fun below(value: S): Boolean {
         return TestJavaTypeSyntax.DefaultImpls.below(this, value as S)
      }

      override fun <R> sink(items: MutableList<in R>, from: Comparator<in S>): Int {
         return TestJavaTypeSyntax.DefaultImpls.sink(this, items, from)
      }
   }

   public companion object

   // $VF: Class flags could not be determined
   internal class DefaultImpls {
      @JvmStatic
      fun <T : java.lang.Comparable<in T>> below(`$this`: TestJavaTypeSyntax<T>, value: T): Boolean {
         return value.compareTo(`$this`.low) < 0
      }

      @JvmStatic
      fun <T : java.lang.Comparable<in T>, R> sink(`$this`: TestJavaTypeSyntax<T>, items: MutableList<in R>, from: Comparator<in T>): Int {
         return items.size()
      }
   }

   @SourceDebugExtension(["SMAP\nTestJavaTypeSyntax.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Use\n+ 2 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Companion\n*L\n1#1,27:1\n24#2:28\n*S KotlinDebug\n*F\n+ 1 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Use\n*L\n16#1:28\n*E\n"])
   public class Use {
      public fun strings(n: Int): () -> Array<String?> {
         val `this_$iv`: TestJavaTypeSyntax.Companion = TestJavaTypeSyntax.Companion
         return TestJavaTypeSyntax$Use$strings$$inlined$arrayFactory$1(n)
      }

      public fun isIntArray(c: Class<*>): Boolean {
         return c == IntArray::class.java
      }

      public fun isStringArray(c: Class<*>): Boolean {
         return c == Array<java.lang.String>::class.java
      }
   }
}

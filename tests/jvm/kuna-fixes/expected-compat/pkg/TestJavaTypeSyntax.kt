package pkg

import java.util.Comparator
import kotlin.jvm.internal.SourceDebugExtension

public interface TestJavaTypeSyntax<T extends java.lang.Comparable<? super T>> {
   @JvmStatic
   TestJavaTypeSyntax.Companion Companion = TestJavaTypeSyntax.Companion.$$INSTANCE;

   public val low: Any

   public open fun below(value: Any): Boolean {
   }

   public open fun <R> sink(items: MutableList<in R>, from: Comparator<in Any>): Int {
   }

   public class Box<S extends java.lang.CharSequence & java.lang.Comparable<? super S>> : TestJavaTypeSyntax<S> {
      public open val low: Any
         public open get() {
            throw IllegalStateException()
         }


      fun below(value: S): Boolean {
         TestJavaTypeSyntax.DefaultImpls.below(this, (S)value)
      }

      override fun <R> sink(items: MutableList<R>, from: Comparator<in S>): Int {
         TestJavaTypeSyntax.DefaultImpls.sink(this, items, from)
      }
   }

   public companion object

   // $VF: Class flags could not be determined
   internal class DefaultImpls {
      @JvmStatic
      fun <T extends java.lang.Comparable<? super T>> below(`$this`: TestJavaTypeSyntax<T>, value: T): Boolean {
         value.compareTo(`$this`.low) < 0
      }

      @JvmStatic
      fun <T extends java.lang.Comparable<? super T>, R> sink(`$this`: TestJavaTypeSyntax<T>, items: MutableList<R>, from: Comparator<in T>): Int {
         items.size()
      }
   }

   @SourceDebugExtension(["SMAP\nTestJavaTypeSyntax.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Use\n+ 2 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Companion\n*L\n1#1,27:1\n24#2:28\n*S KotlinDebug\n*F\n+ 1 TestJavaTypeSyntax.kt\npkg/TestJavaTypeSyntax$Use\n*L\n16#1:28\n*E\n"])
   public class Use {
      public fun strings(n: Int): () -> Array<String?> {
         val `this_$iv`: TestJavaTypeSyntax.Companion = TestJavaTypeSyntax.Companion
         return TestJavaTypeSyntax$Use$strings$$inlined$arrayFactory$1(n)
      }

      public fun isIntArray(c: Class<*>): Boolean {
         return c == int[]::class.java
      }

      public fun isStringArray(c: Class<*>): Boolean {
         return c == java.lang.String[]::class.java
      }
   }
}

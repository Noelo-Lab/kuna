package pkg

interface TestJavaTypeSyntax<T : Comparable<T>> {
   val low: T

   fun below(value: T): Boolean = value < low

   fun <R> sink(items: MutableList<in R>, from: Comparator<in T>): Int = items.size

   class Box<S> : TestJavaTypeSyntax<S> where S : CharSequence, S : Comparable<S> {
      override val low: S
         get() = throw IllegalStateException()
   }

   class Use {
      fun strings(n: Int): () -> Array<String?> = arrayFactory<String> { n }

      fun isIntArray(c: Class<*>): Boolean = c == IntArray::class.java

      fun isStringArray(c: Class<*>): Boolean = c == Array<String>::class.java
   }

   companion object {
      inline fun <reified V> arrayFactory(crossinline size: () -> Int): () -> Array<V?> = { arrayOfNulls<V>(size()) }
   }
}

package pkg

import java.util.NoSuchElementException
import kotlin.jvm.internal.SourceDebugExtension

public class TestKtClassHeaderMetadata {
   public fun first(source: pkg.TestKtClassHeaderMetadata.Source<CharSequence>): CharSequence {
      return source.next() as CharSequence
   }

   public fun widen(source: pkg.TestKtClassHeaderMetadata.Source<String>): CharSequence {
      return this.first(source)
   }

   public fun feed(sink: pkg.TestKtClassHeaderMetadata.Sink<String>, value: String) {
      sink.put(value)
   }

   public fun record(recorder: pkg.TestKtClassHeaderMetadata.Recorder, value: String): Any? {
      this.feed(recorder, value)
      return recorder.last
   }

   public fun total(countdown: pkg.TestKtClassHeaderMetadata.Countdown): Int {
      var sum: Int = 0

      while (countdown.hasNext()) {
         sum += countdown.next()
      }

      return sum
   }

   public class Countdown(left: Int) : Iterator<Int> {
      private final var left: Int

      init {
         this.left = left
      }

      public open operator fun hasNext(): Boolean {
         return this.left > 0
      }

      public override operator fun next(): Int {
         val var1: Int = this.left
         this.left = var1 + -1
         return var1
      }
   }

   public object Empty : TestKtClassHeaderMetadata.Source<Nothing> {
      public override fun next(): Nothing {
         throw NoSuchElementException("empty")
      }
   }

   @SourceDebugExtension(["SMAP\nTestKtClassHeaderMetadata.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtClassHeaderMetadata.kt\npkg/TestKtClassHeaderMetadata\$Ranked\n+ 2 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,59:1\n1971#2,14:60\n*S KotlinDebug\n*F\n+ 1 TestKtClassHeaderMetadata.kt\npkg/TestKtClassHeaderMetadata\$Ranked\n*L\n35#1:60,14\n*E\n"])
   public class Ranked<T>(items: List<T>) where T : CharSequence, T : Comparable<T> {
      private final val items: List<T>

      init {
         this.items = items
      }

      public fun longest(): T? {
         val `iterator$iv`: Iterator<*> = (this.items as Iterable<*>).iterator()
         val var10000: Any?
         if (!`iterator$iv`.hasNext()) {
            var10000 = null
         } else {
            var `maxElem$iv`: Any? = `iterator$iv`.next()
            if (!`iterator$iv`.hasNext()) {
               var10000 = `maxElem$iv`
            } else {
               var var9: Int = (`maxElem$iv` as CharSequence).length

               do {
                  val var10: Any? = `iterator$iv`.next()
                  val var11: Int = (var10 as CharSequence).length
                  if (var9 < var11) {
                     `maxElem$iv` = var10
                     var9 = var11
                  }
               } while (`iterator$iv`.hasNext())

               var10000 = `maxElem$iv`
            }
         }

         return var10000 as T
      }
   }

   public class Recorder : TestKtClassHeaderMetadata.Sink<Any?> {
      public final var last: Any? = null

      public override fun put(value: Any?) {
         this.last = value
      }
   }

   public interface Sink<in T> {
      public abstract fun put(value: T)
   }

   public class Sorted<T : Comparable<T>>(items: List<T>) {
      private final val items: List<T>

      init {
         this.items = items
      }

      public fun smallest(): T? {
         return CollectionsKt.minOrNull(this.items as Iterable<*>) as T
      }
   }

   public interface Source<out T> {
      public abstract fun next(): T
   }
}

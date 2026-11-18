package pkg

import java.util.NoSuchElementException
import kotlin.jvm.internal.SourceDebugExtension
import kotlin.jvm.internal.markers.KMappedMarker

public class TestKtClassHeaderMetadata {
   public fun first(source: pkg.TestKtClassHeaderMetadata.Source<CharSequence>): CharSequence {
      return source.next() as java.lang.CharSequence
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

   public class Countdown(left: Int) : java.util.Iterator<Integer>, KMappedMarker {
      private final var left: Int

      init {
         this.left = left
      }

      public open operator fun hasNext(): Boolean {
         return this.left > 0
      }

      public open operator fun next(): Int {
         val var1: Int = this.left
         this.left += -1
         return var1
      }

      fun remove() {
         throw UnsupportedOperationException("Operation is not supported for read-only collection")
      }
   }

   public object Empty : TestKtClassHeaderMetadata.Source {
      public open fun next(): Nothing {
         throw NoSuchElementException("empty")
      }
   }

   @SourceDebugExtension(["SMAP\nTestKtClassHeaderMetadata.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtClassHeaderMetadata.kt\npkg/TestKtClassHeaderMetadata$Ranked\n+ 2 _Collections.kt\nkotlin/collections/CollectionsKt___CollectionsKt\n*L\n1#1,59:1\n1971#2,14:60\n*S KotlinDebug\n*F\n+ 1 TestKtClassHeaderMetadata.kt\npkg/TestKtClassHeaderMetadata$Ranked\n*L\n35#1:60,14\n*E\n"])
   public class Ranked<T extends java.lang.CharSequence & java.lang.Comparable<? super T>>(items: List<Any>) {
      private final val items: List<Any>

      init {
         this.items = items
      }

      public fun longest(): Any? {
         val `iterator$iv`: java.util.Iterator = (this.items as java.lang.Iterable).iterator()
         val var10000: Any
         if (!`iterator$iv`.hasNext()) {
            var10000 = null
         } else {
            var `maxElem$iv`: Any = `iterator$iv`.next()
            if (!`iterator$iv`.hasNext()) {
               var10000 = `maxElem$iv`
            } else {
               var var9: Int = (`maxElem$iv` as java.lang.CharSequence).length()

               do {
                  val var10: Any = `iterator$iv`.next()
                  val var11: Int = (var10 as java.lang.CharSequence).length()
                  if (var9 < var11) {
                     `maxElem$iv` = var10
                     var9 = var11
                  }
               } while (`iterator$iv`.hasNext())

               var10000 = `maxElem$iv`
            }
         }

         return (T)var10000
      }
   }

   public class Recorder : TestKtClassHeaderMetadata.Sink<Object> {
      public final var last: Any?
         internal set

      public override fun put(value: Any?) {
         this.last = value
      }
   }

   public interface Sink<T> {
      public abstract fun put(value: Any) {
      }
   }

   public class Sorted<T extends java.lang.Comparable<? super T>>(items: List<Any>) {
      private final val items: List<Any>

      init {
         this.items = items
      }

      public fun smallest(): Any? {
         return (T)CollectionsKt.minOrNull(this.items as java.lang.Iterable)
      }
   }

   public interface Source<T> {
      public abstract fun next(): Any {
      }
   }
}

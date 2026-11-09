package pkg

import java.util.ArrayList
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestTypeParameterId.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestTypeParameterId.kt\npkg/TestTypeParameterId\n+ 2 _Arrays.kt\nkotlin/collections/ArraysKt___ArraysKt\n*L\n1#1,23:1\n11102#2:24\n11437#2,3:25\n*S KotlinDebug\n*F\n+ 1 TestTypeParameterId.kt\npkg/TestTypeParameterId\n*L\n13#1:24\n13#1:25,3\n*E\n"])
public class TestTypeParameterId<K, V extends java.lang.Comparable<? super V>>(keys: Array<K>, best: V) {
   private final val keys: Array<K>
   private final var best: V

   init {
      this.keys = (K[])keys
      this.best = best as V
   }

   public fun key(i: Int): K {
      return this.keys[i]
   }

   public fun offer(value: V): V {
      if (value.compareTo(this.best) > 0) {
         this.best = value as V
      }

      return this.best
   }

   public fun pairs(values: List<V>): Map<K, V> {
      return MapsKt.toMap(ArraysKt.zip(this.keys, values as java.lang.Iterable) as java.lang.Iterable)
   }

   public fun <R> mapKeys(transform: (K) -> R): List<R> {
      val `$this$mapTo$iv$iv`: Array<Any> = this.keys
      val `destination$iv$iv`: java.util.Collection = ArrayList(this.keys.length) as java.util.Collection

      for (`item$iv$iv` in `$this$mapTo$iv$iv`) {
         `destination$iv$iv`.add(transform(`item$iv$iv`))
      }

      return `destination$iv$iv` as MutableList<R>
   }

   public inner class Cursor(index: Int) {
      private final var index: Int

      init {
         this.index = index
      }

      public fun current(): K {
         return TestTypeParameterId.this.keys[this.index]
      }

      public fun advance(step: V): Pair<K, V> {
         val var2: Int = this.index++
         return TuplesKt.to(TestTypeParameterId.this.keys[this.index], step)
      }
   }
}

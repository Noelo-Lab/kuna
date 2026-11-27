package pkg

import java.util.ArrayList
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtEnclosingTypeParameter.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtEnclosingTypeParameter.kt\npkg/TestKtEnclosingTypeParameter\n*L\n1#1,53:1\n45#1,3:54\n45#1,3:57\n*S KotlinDebug\n*F\n+ 1 TestKtEnclosingTypeParameter.kt\npkg/TestKtEnclosingTypeParameter\n*L\n49#1:54,3\n51#1:57,3\n*E\n"])
public class TestKtEnclosingTypeParameter<K>(key: K) {
   private final val key: K

   init {
      this.key = key as K
   }

   public fun <T> constant(value: T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<T> {
         public override fun next(): T {
            return value as T
         }
      }
   }

   public fun keys(): pkg.TestKtEnclosingTypeParameter.Source<K> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<K> {
         public override fun next(): K {
            return this@TestKtEnclosingTypeParameter.key
         }
      }
   }

   public fun <T> pairs(value: T): Iterator<Pair<K, T>> {
      return object : Iterator<Pair<K, T>> {
         public final var left: Int = 2

         public open operator fun hasNext(): Boolean {
            return this.left > 0
         }

         public override operator fun next(): Pair<K, T> {
            this.left += -1
            return TuplesKt.to(this@TestKtEnclosingTypeParameter.key, value)
         }
      } as Iterator<Pair<K, T>>
   }

   public fun <T> pumpOf(value: T): pkg.TestKtEnclosingTypeParameter.Pump<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Pump<T> {
         public override fun drainTo(sink: MutableCollection<in T>, count: Int) {
            var target: MutableCollection<in T> = sink

            repeat(count) { n ->
               target.add(value)
               if (target.size > 8) {
                  target = ArrayList()
               }
            }
         }
      }
   }

   public inline fun <T> computed(crossinline make: () -> T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<T> {
         public override fun next(): T {
            return make() as T
         }
      }
   }

   public fun <T> later(value: T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<T> {
         public override fun next(): T {
            return value as T
         }
      }
   }

   public fun name(): pkg.TestKtEnclosingTypeParameter.Source<String> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {
         public override fun next(): Any {
            return "name"
         }
      }
   }

   public interface Pump<out T> {
      public abstract fun drainTo(sink: MutableCollection<in T>, count: Int)
   }

   public interface Source<out T> {
      public abstract fun next(): T
   }
}

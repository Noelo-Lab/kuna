package pkg

import java.util.ArrayList
import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestKtEnclosingTypeParameter.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestKtEnclosingTypeParameter.kt\npkg/TestKtEnclosingTypeParameter\n*L\n1#1,53:1\n45#1,3:54\n45#1,3:57\n*S KotlinDebug\n*F\n+ 1 TestKtEnclosingTypeParameter.kt\npkg/TestKtEnclosingTypeParameter\n*L\n49#1:54,3\n51#1:57,3\n*E\n"])
public class TestKtEnclosingTypeParameter<K>(key: Any) {
   private final val key: Any

   init {
      this.key = (K)key
   }

   public fun <T> constant(value: T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {
         public override fun next(): Any {
            return (T)value
         }
      }
   }

   public fun keys(): pkg.TestKtEnclosingTypeParameter.Source<Any> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {
         public override fun next(): Any {
            return TestKtEnclosingTypeParameter.this.key
         }
      }
   }

   public fun <T> pairs(value: T): Iterator<Pair<Any, T>> {
      return (java.util.Iterator<Pair<K, T>>)object : Iterator<Pair<Any, Any>> {
         public final var left: Int = 2
            internal set

         public open operator fun hasNext(): Boolean {
            return this.left > 0
         }

         public open operator fun next(): Pair<Any, Any> {
            this.left += -1
            return TuplesKt.to(TestKtEnclosingTypeParameter.this.key, value)
         }

         fun remove() {
            throw UnsupportedOperationException("Operation is not supported for read-only collection")
         }
      }
   }

   public fun <T> pumpOf(value: T): pkg.TestKtEnclosingTypeParameter.Pump<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Pump<Any> {
         public override fun drainTo(sink: MutableCollection<in Any>, count: Int) {
            var target: java.util.Collection = sink

            repeat(count) { n ->
               target.add(value)
               if (target.size() > 8) {
                  target = ArrayList() as java.util.Collection
               }
            }
         }
      }
   }

   public inline fun <T> computed(crossinline make: () -> T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {
         public override fun next(): Any {
            return (T)make()
         }
      }
   }

   public fun <T> later(value: T): pkg.TestKtEnclosingTypeParameter.Source<T> {
      return TestKtEnclosingTypeParameter$later$$inlined$computed$1(value)
   }

   public fun name(): pkg.TestKtEnclosingTypeParameter.Source<String> {
      return TestKtEnclosingTypeParameter$name$$inlined$computed$1()
   }

   public interface Pump<T> {
      public abstract fun drainTo(sink: MutableCollection<in Any>, count: Int) {
      }
   }

   public interface Source<T> {
      public abstract fun next(): Any {
      }
   }
}

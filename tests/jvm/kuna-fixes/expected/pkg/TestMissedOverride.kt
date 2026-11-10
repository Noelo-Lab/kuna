package pkg

import kotlin.jvm.internal.Intrinsics

public class TestMissedOverride {
   public abstract class Base<T> {
      public abstract fun accept(item: T): Boolean

      public open fun name(): CharSequence {
         return "base" as CharSequence
      }
   }

   public class Buffer : TestMissedOverride.Sink, java.lang.Comparable<TestMissedOverride.Buffer> {
      public final var total: Long = 0L
         internal set

      public override fun buffer(): pkg.TestMissedOverride.Buffer {
         return this
      }

      public override fun write(value: Int): pkg.TestMissedOverride.Buffer {
         this.total += value
         return this
      }

      public override val size: Long
         public get() {
            return this.total
         }


      public override val owner: pkg.TestMissedOverride.Buffer
         public get() {
            return this
         }


      public override operator fun compareTo(other: pkg.TestMissedOverride.Buffer): Int {
         return Intrinsics.compare(this.total, other.total)
      }
   }

   public interface Sink {
      public abstract fun buffer(): pkg.TestMissedOverride.Sink

      public abstract fun write(value: Int): pkg.TestMissedOverride.Sink

      public val size: Long

      public val owner: Any
   }

   public class Strings : TestMissedOverride.Base<String>() {
      public override fun accept(item: String): Boolean {
         return (item as CharSequence).length == 0
      }

      public override fun name(): String {
         return "strings"
      }
   }
}

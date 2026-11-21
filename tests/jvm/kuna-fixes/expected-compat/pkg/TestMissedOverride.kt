package pkg

import kotlin.jvm.internal.Intrinsics

public class TestMissedOverride {
   public abstract class Base<T> {
      public abstract fun accept(item: Any): Boolean {
      }

      public open fun name(): CharSequence {
         return "base" as java.lang.CharSequence
      }
   }

   public class Buffer : TestMissedOverride.Sink, java.lang.Comparable<TestMissedOverride.Buffer> {
      public final var total: Long
         internal set

      public open fun buffer(): pkg.TestMissedOverride.Buffer {
         return this
      }

      public open fun write(value: Int): pkg.TestMissedOverride.Buffer {
         this.total += value
         return this
      }

      public open val size: Long
         public open get() {
            return this.total
         }


      public open val owner: pkg.TestMissedOverride.Buffer
         public open get() {
            return this
         }


      public open operator fun compareTo(other: pkg.TestMissedOverride.Buffer): Int {
         return Intrinsics.compare(this.total, other.total)
      }
   }

   public interface Sink {
      public abstract fun buffer(): pkg.TestMissedOverride.Sink {
      }

      public abstract fun write(value: Int): pkg.TestMissedOverride.Sink {
      }

      public val size: Long

      public val owner: Any
   }

   public class Strings : TestMissedOverride.Base<java.lang.String> {
      public open fun accept(item: String): Boolean {
         return (item as java.lang.CharSequence).length() == 0
      }

      public open fun name(): String {
         return "strings"
      }
   }
}

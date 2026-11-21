package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestAccessorBackingField.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestAccessorBackingField.kt\npkg/TestAccessorBackingField\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,45:1\n1#2:46\n*E\n"])
public class TestAccessorBackingField {
   public final var limit: Int = 64
      public final set(value) {
         if (value < 1) {
            throw IllegalArgumentException(("limit < 1: $value").toString())
         } else {
            synchronized (this) {
               this.limit = value
            }

            val var5: Int = this.changes++
         }
      }


   public final var changes: Int
      private set

   public final var name: String = "a"
      public final get() {
         return "${this.name}!"
      }

      public final set(value) {
         this.name = StringsKt.trim(value as java.lang.CharSequence).toString()
      }


   public final val doubled: Int = 21
      public final get() {
         return this.doubled * 2
      }


   public final var counter: Long
      public final get() {
         val var1: Int = this.counter++
         return this.counter
      }

      internal set

   public fun bump(): Int {
      this.limit = this.limit + 1
      return this.limit
   }

   public companion object {
      public final var shared: String
         public final get() {
            return "${TestAccessorBackingField.shared}?"
         }

         public final set(value) {
            TestAccessorBackingField.shared = "$value."
         }

   }
}

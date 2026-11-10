package pkg

import kotlin.jvm.internal.SourceDebugExtension

@SourceDebugExtension(["SMAP\nTestAccessorBackingField.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestAccessorBackingField.kt\npkg/TestAccessorBackingField\n+ 2 fake.kt\nkotlin/jvm/internal/FakeKt\n*L\n1#1,45:1\n1#2:46\n*E\n"])
public class TestAccessorBackingField {
   public final var limit: Int = 64
      public set(value) {
         if (value < 1) {
            throw IllegalArgumentException(("limit < 1: $value").toString())
         } else {
            synchronized (this) {
               field = value
            }

            val var5: Int = this.changes++
         }
      }


   public final var changes: Int = 0
      private set

   public final var name: String = "a"
      public get() {
         return "${field}!"
      }

      public set(value) {
         field = StringsKt.trim(value as CharSequence).toString()
      }


   public final val doubled: Int = 21
      public get() {
         return field * 2
      }


   public final var counter: Long = 0L
      public get() {
         val var1: Int = field++
         return field
      }


   public fun bump(): Int {
      this.limit = this.limit + 1
      return this.limit
   }

   public companion object {
      public final var shared: String = "s"
         public get() {
            return "${field}?"
         }

         public set(value) {
            field = "$value."
         }

   }
}

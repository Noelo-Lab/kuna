package pkg

public class TestKtDataClassPropertyParameters {
   public fun use(h: pkg.TestKtDataClassPropertyParameters.Header): Int {
      return h.size + h.value
   }

   public data class Defaults(a: Int = 1, b: String? = null) {
      public final val a: Int
      public final val b: String?

      init {
         this.a = a
         this.b = b
      }

      public operator fun component1(): Int {
         return this.a
      }

      public operator fun component2(): String? {
         return this.b
      }

      public fun copy(a: Int = this.a, b: String? = this.b): pkg.TestKtDataClassPropertyParameters.Defaults {
         return TestKtDataClassPropertyParameters.Defaults(a, b)
      }

      public open fun toString(): String {
         return "Defaults(a=${this.a}, b=${this.b})"
      }

      public open fun hashCode(): Int {
         return Integer.hashCode(this.a) * 31 + (if (this.b == null) 0 else this.b.hashCode())
      }

      public open operator fun equals(other: Any?): Boolean {
         label28@
         if (this === other) {
            return true
         } else {
            return other is TestKtDataClassPropertyParameters.Defaults
               && this.a == (other as TestKtDataClassPropertyParameters.Defaults).a
               && this.b == (other as TestKtDataClassPropertyParameters.Defaults).b
            }
      }

      fun Defaults() {
         this(0, null, 3, null)
      }
   }

   public data class Entry<A>(first: Any, rest: List<Any>) {
      private final val first: Any
      internal final val rest: List<Any>

      init {
         this.first = (A)first
         this.rest = rest
      }

      public fun head(): Any {
         return this.first
      }

      private operator fun component1(): Any {
         return this.first
      }

      internal operator fun component2(): List<Any> {
         return this.rest
      }

      public fun copy(first: Any = this.first, rest: List<Any> = this.rest): pkg.TestKtDataClassPropertyParameters.Entry<Any> {
         return TestKtDataClassPropertyParameters.Entry<>((A)first, rest)
      }

      public open fun toString(): String {
         return "Entry(first=${this.first}, rest=${this.rest})"
      }

      public open fun hashCode(): Int {
         return (if (this.first == null) 0 else this.first.hashCode()) * 31 + this.rest.hashCode()
      }

      public open operator fun equals(other: Any?): Boolean {
         label28@
         if (this === other) {
            return true
         } else {
            return other is TestKtDataClassPropertyParameters.Entry
               && this.first == (other as TestKtDataClassPropertyParameters.Entry).first
               && this.rest == (other as TestKtDataClassPropertyParameters.Entry).rest
            }
      }
   }

   public data class Header(name: String, value: Int) {
      public final val name: String

      public final var value: Int
         internal set

      public final val size: Int

      init {
         this.name = name
         this.value = value
         this.size = this.name.length() + this.value
      }

      public operator fun component1(): String {
         return this.name
      }

      public operator fun component2(): Int {
         return this.value
      }

      public fun copy(name: String = this.name, value: Int = this.value): pkg.TestKtDataClassPropertyParameters.Header {
         return TestKtDataClassPropertyParameters.Header(name, value)
      }

      public open fun toString(): String {
         return "Header(name=${this.name}, value=${this.value})"
      }

      public open fun hashCode(): Int {
         return this.name.hashCode() * 31 + Integer.hashCode(this.value)
      }

      public open operator fun equals(other: Any?): Boolean {
         label28@
         if (this === other) {
            return true
         } else {
            return other is TestKtDataClassPropertyParameters.Header
               && this.name == (other as TestKtDataClassPropertyParameters.Header).name
               && this.value == (other as TestKtDataClassPropertyParameters.Header).value
            }
      }
   }
}

package pkg

import java.util.ArrayList

public class TestLegacyParameterChecks(label: String) {
   public final val label: String

   init {
      this.label = label
   }

   public open class Base(size: Int) {
      public final val size: Int

      init {
         this.size = size
      }

      public constructor() : this(0)
   }

   public class Buffers {
      public final val data: ByteArray
      public final var owner: Boolean

      public constructor() {
         this.data = ByteArray(8)
         this.owner = true
      }

      public constructor(data: ByteArray, owner: Boolean) {
         this.data = data
         this.owner = owner
      }
   }

   public class Holder(text: String, count: Int) : TestLegacyParameterChecks.Base(text.length) {
      private final val text: String
      public final val total: Int
      public final val names: ArrayList<String>

      init {
         this.text = text
         this.total = count * 2
         this.names = ArrayList()
      }

      public fun describe(): String {
         return "${this.text}${this.total}${this.names}"
      }
   }

   public class Secondary : TestLegacyParameterChecks.Base {
      public final val text: String

      public constructor(text: String) : super(text.length) {
         this.text = text
      }

      public constructor(head: String, tail: String) : this("$head$tail")
   }

   public inner class Tagged(tag: String) : TestLegacyParameterChecks.Base(tag.length + this@TestLegacyParameterChecks.label.length) {
      public final val tag: String

      init {
         this.tag = tag
      }
   }
}

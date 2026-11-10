package pkg

import java.util.ArrayList
import kotlin.jvm.internal.Intrinsics

public class TestLegacyParameterChecks(label: String) {
   public final val label: String

   init {
      Intrinsics.checkParameterIsNotNull(label, "label")
      super()
      this.label = label
   }

   public open class Base(size: Int) {
      public final val size: Int

      init {
         this.size = size
      }

      public constructor() : this(0)   }

   public class Buffers {
      public final val data: ByteArray = ByteArray(8)

      public final var owner: Boolean = true
         internal set


      public constructor(data: ByteArray, owner: Boolean) : Intrinsics.checkParameterIsNotNull(data, "data") {
         super()
         this.data = data
         this.owner = owner
      }
   }

   public class Holder(text: String, count: Int) : TestLegacyParameterChecks.Base {
      private final val text: String
      public final val total: Int
      public final val names: ArrayList<String>

      init {
         Intrinsics.checkParameterIsNotNull(text, "text")
         super(text.length())
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

      public constructor(text: String) : Intrinsics.checkParameterIsNotNull(text, "text") {
         super(text.length())
         this.text = text
      }

      public constructor(head: String, tail: String) : Intrinsics.checkParameterIsNotNull(head, "head") {
         Intrinsics.checkParameterIsNotNull(tail, "tail")
         this("$head$tail")
      }
   }

   public inner class Tagged(tag: String) : TestLegacyParameterChecks.Base {
      public final val tag: String

      init {
         Intrinsics.checkParameterIsNotNull(tag, "tag")
         super(tag.length() + TestLegacyParameterChecks.this.label.length())
         this.tag = tag
      }
   }
}

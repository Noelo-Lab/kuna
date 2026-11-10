package pkg

import java.util.ArrayList

public class TestHiddenEmptySuperCall {
   public open class Base {
      public open fun size(): Int {
         return 0
      }
   }

   public open class Gen<T> {
      public final val items: ArrayList<T> = ArrayList()
   }

   public class Holder {
      @JvmStatic
      public TestHiddenEmptySuperCall.Holder.Companion Companion = TestHiddenEmptySuperCall.Holder.Companion(null);

      public companion object : TestHiddenEmptySuperCall.Base()
   }

   public open inner class InnerBase

   public inner class InnerSub(n: Int) : TestHiddenEmptySuperCall.InnerBase() {
      public final val n: Int

      init {
         this.n = n
      }
   }

   public class OnlySecondary : TestHiddenEmptySuperCall.Base {
      public constructor(s: String) {
         this.remember(s)
      }

      public constructor(n: Int)

      public fun remember(s: String): Int {
         return s.length() + this.size()
      }
   }

   public class Plain(label: String) : TestHiddenEmptySuperCall.Base() {
      public final val label: String

      init {
         this.label = label
      }

      public override fun size(): Int {
         return this.label.length()
      }
   }

   public object Single : TestHiddenEmptySuperCall.Base() {
      @JvmStatic
      public TestHiddenEmptySuperCall.Single INSTANCE = TestHiddenEmptySuperCall.Single();
   }

   public class Typed(n: Int) : TestHiddenEmptySuperCall.Gen<String>() {
      public final val n: Int

      init {
         this.n = n
      }
   }
}

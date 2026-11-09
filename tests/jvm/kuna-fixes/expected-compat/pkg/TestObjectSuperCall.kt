package pkg

public class TestObjectSuperCall(q: Int) {
   public final val q: Int

   init {
      this.q = q
   }

   public fun make(k: Int, s: String): pkg.TestObjectSuperCall.Base {
      return object : pkg.TestObjectSuperCall.Base, pkg.TestObjectSuperCall.Marker {
         public override fun get(): Int {
            return this.getN() * 2
         }
      } as TestObjectSuperCall.Base
   }

   public fun plain(): pkg.TestObjectSuperCall.Plain {
      return object : pkg.TestObjectSuperCall.Plain {
         public override fun get(): Int {
            return 5
         }
      }
   }

   public fun captured(r: Int): pkg.TestObjectSuperCall.Base {
      return object : pkg.TestObjectSuperCall.Base {
         public override fun get(): Int {
            return r + TestObjectSuperCall.this.q
         }
      }
   }

   public abstract class Base {
      public final val n: Int
      public final val tag: String

      open fun Base(n: Int, tag: java.lang.String) {
         this.n = n
         this.tag = tag
      }

      public abstract fun get(): Int {
      }
   }

   public interface Marker

   public open class Plain {
      public open fun get(): Int {
         return 0
      }
   }
}

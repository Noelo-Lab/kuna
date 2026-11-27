package pkg

import kotlin.jvm.internal.TypeIntrinsics

public class TestUnrepresentableType {
   private final val counter = object : pkg.TestUnrepresentableType.Source {
      public final var reads: Int = 0

      public override fun read(): Int {
         this.reads++
         return this.reads
      }
   }

   public final var calls: Int = 0

   public fun twice(): Int {
      val source = object : pkg.TestUnrepresentableType.Source {
         public final var last: Int = 0

         public override fun read(): Int {
            this.last += 2
            return this.last
         }
      }
      source.read()
      return source.read() + source.last + this.counter.read() + this.counter.reads
   }

   private fun register(target: Any, param: Any?) {
      this.calls += if (param == null) 1 else 2
   }

   public final val clause: pkg.TestUnrepresentableType.Clause
      public get() {
         val var10003 = TestUnrepresentableType::register
         return TestUnrepresentableType.Clause(this, TypeIntrinsics.beforeCheckcastToFunctionOfArity(var10003, 3) as (Any?, Any?, Any?) -> Unit)
      }


   public class Clause(owner: Any, register: (Any, Any, Any?) -> Unit) {
      public final val owner: Any
      public final val register: (Any, Any, Any?) -> Unit

      init {
         this.owner = owner
         this.register = register
      }
   }

   public interface Source {
      public abstract fun read(): Int
   }
}

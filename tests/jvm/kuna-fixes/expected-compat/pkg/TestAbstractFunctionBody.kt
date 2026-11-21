package pkg

public abstract class TestAbstractFunctionBody {
   public abstract fun dispose() {
   }

   public abstract fun size(): Int {
   }

   protected abstract fun name(prefix: String): String {
   }

   public open fun describe(): String {
      return "${this.name("x")}${this.size()}"
   }

   public external fun nativeHash(seed: Long): Int {
   }

   public abstract class Partial : TestAbstractFunctionBody.Source {
      public abstract override fun close() {
      }

      public override fun read(into: IntArray, count: Long): Long {
         return count
      }
   }

   public interface Source {
      public abstract fun read(into: IntArray, count: Long): Long {
      }

      public abstract fun close() {
      }
   }
}

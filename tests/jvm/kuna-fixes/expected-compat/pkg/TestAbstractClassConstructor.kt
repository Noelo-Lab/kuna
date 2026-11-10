package pkg

public class TestAbstractClassConstructor {
   public abstract class Checked : TestAbstractClassConstructor.Shape {
      public final val limit: Int

      open fun Checked(limit: Int) {
         super("checked")
         this.limit = limit
         if (this.limit < 0) {
            throw IllegalArgumentException("negative")
         }
      }
   }

   public abstract class Counter {
      public final var count: Int
         internal set

      open fun Counter() {
      }

      public abstract fun step(): Int {
      }
   }

   public abstract class Shape {
      public final val name: String

      open fun Shape(name: java.lang.String) {
         this.name = name
      }

      public abstract fun area(): Int {
      }
   }

   public class Square(side: Int) : TestAbstractClassConstructor.Shape("square") {
      public final val side: Int

      init {
         this.side = side
      }

      public override fun area(): Int {
         return this.side * this.side
      }
   }

   public abstract class Tagged {
      public final val arity: Int
      public final val tag: String?

      open fun Tagged(arity: Int, tag: java.lang.String?) {
         this.arity = arity
         this.tag = tag
      }

      open fun Tagged(arity: Int) {
         this(arity, null)
      }
   }
}

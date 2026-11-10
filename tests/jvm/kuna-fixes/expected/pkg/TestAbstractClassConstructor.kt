package pkg

public class TestAbstractClassConstructor {
   public abstract class Checked(limit: Int) : TestAbstractClassConstructor.Shape("checked") {
      public final val limit: Int

      init {
         this.limit = limit
         if (this.limit < 0) {
            throw IllegalArgumentException("negative")
         }
      }
   }

   public abstract class Counter protected constructor() {
      public final var count: Int
         internal set

      public abstract fun step(): Int {
      }
   }

   public abstract class Shape(name: String) {
      public final val name: String

      init {
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

   public abstract class Tagged(arity: Int, tag: String?) {
      public final val arity: Int
      public final val tag: String?

      init {
         this.arity = arity
         this.tag = tag
      }

      public constructor(arity: Int) : this(arity, null)
   }
}

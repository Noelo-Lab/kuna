package pkg

public open class TestDefaultArgsSuperNamed(a: Int, b: String = "b", c: Long = 3L) {
   public final val a: Int
   public final val b: String
   public final val c: Long

   init {
      this.a = a
      this.b = b
      this.c = c
   }

   public fun skipsMiddle(x: Int): Any {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: new pkg/TestDefaultArgsSuperNamed$skipsMiddle$1
      // 3: dup
      // 4: iload 1
      // 5: invokespecial pkg/TestDefaultArgsSuperNamed$skipsMiddle$1.<init> (I)V
      // 8: areturn
   }

   public fun innerSuper(z: Int): Any {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: new pkg/TestDefaultArgsSuperNamed$innerSuper$1
      // 3: dup
      // 4: aload 0
      // 5: iload 1
      // 6: invokespecial pkg/TestDefaultArgsSuperNamed$innerSuper$1.<init> (Lpkg/TestDefaultArgsSuperNamed;I)V
      // 9: areturn
   }

   public fun picksPrimary(x: Int): Any {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: new pkg/TestDefaultArgsSuperNamed$picksPrimary$1
      // 3: dup
      // 4: iload 1
      // 5: invokespecial pkg/TestDefaultArgsSuperNamed$picksPrimary$1.<init> (I)V
      // 8: areturn
   }

   public fun picksSecondary(x: Int): Any {
      return object {
         public final val sub: pkg.TestDefaultArgsSuperNamed.picksSecondary.1.Sub = Sub()

         // $VF: local visibility outside of methodSupplier
internal inner class Sub : TestDefaultArgsSuperNamed.Overloaded(x)
      }
   }

   public open inner class Inner(p: Int, q: String = ...) {
      public final val p: Int
      public final val q: String

      init {
         this.p = p
         this.q = q
      }
   }

   public open class Overloaded(a: Int, b: String = "b") {
      public final val a: Int
      public final val b: String

      init {
         this.a = a
         this.b = b
      }

      public constructor(n: Int) : this(n, "n")   }
}

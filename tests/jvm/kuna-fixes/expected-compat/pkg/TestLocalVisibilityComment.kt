package pkg

public open class TestLocalVisibilityComment(a: Int) {
   public final val a: Int

   init {
      this.a = a
   }

   public fun make(x: Int): Any {
      return object {
         public final val sub: pkg.TestLocalVisibilityComment.make.1.Sub = Sub()

         // $VF: local visibility outside of methodSupplier
internal inner class Sub : TestLocalVisibilityComment(x)
      }
   }
}

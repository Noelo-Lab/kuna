package pkg

public class TestReferenceInequality {
   public fun same(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Boolean {
      return a === b
   }

   public fun notSame(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Boolean {
      return a != b
   }

   public fun notEqual(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Boolean {
      return !(a == b)
   }

   public fun equal(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Boolean {
      return a == b
   }

   public fun branchNotSame(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Int {
      return if (a != b) 1 else 2
   }

   public fun branchSame(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Int {
      return if (a === b) 1 else 2
   }

   public fun branchNotEqual(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Int {
      return if (!(a == b)) 1 else 2
   }

   public fun loopNotSame(a: pkg.TestReferenceInequality.Key, b: pkg.TestReferenceInequality.Key): Int {
      var n: Int = 0

      // $VF: Unable to resugar Kotlin loop from Java for loop
      var x: TestReferenceInequality.Key = a
      while (true) {
         if (x != b) break
         n++

         x = if (n > 3) b else a
      }

      return n
   }

   public class Key(id: Int) {
      public final val id: Int

      init {
         this.id = id
      }

      public open operator fun equals(other: Any?): Boolean {
         return other is TestReferenceInequality.Key && (other as TestReferenceInequality.Key).id == this.id
      }

      public open fun hashCode(): Int {
         return this.id
      }
   }
}

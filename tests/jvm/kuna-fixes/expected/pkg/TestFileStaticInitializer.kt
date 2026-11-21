@file:JvmName(name = "TestFileStaticInitializer")

package pkg

public val loaded: Boolean = run {
   var var0: Boolean
   try {
      Class.forName("pkg.Missing")
      var0 = true
   } catch (var5: Throwable) {
      var0 = false
   }

   var0
}

public val squares: IntArray = run {
   val var6: IntArray = IntArray(8)
   var a: IntArray = var6
   var i: Int = 0

   // $VF: Unable to resugar Kotlin loop from Java for loop
   val var4: Int = a.size
   while (i < var4) {
      a[i] = i * i

      i++
   }

   var6
}

public val cubes: IntArray = run {
   val var7: IntArray = IntArray(8)
   var a: IntArray = var7
   var i: Int = 0

   // $VF: Unable to resugar Kotlin loop from Java for loop
   val var11: Int = a.size
   while (i < var11) {
      a[i] = i * i * i

      i++
   }

   var7
}

public val nameLength: Int = Object::class.java.getName().length * 2

public fun describe(): String {
   return "${loaded} ${squares[3]} ${cubes[2]} ${nameLength}"
}

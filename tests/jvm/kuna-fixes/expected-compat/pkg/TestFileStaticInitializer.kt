@file:JvmName(name = "TestFileStaticInitializer")

package pkg

public final val loaded: Boolean
public final val squares: IntArray
public final val cubes: IntArray
public final val nameLength: Int

public fun describe(): String {
   return "${loaded} ${squares[3]} ${cubes[2]} ${nameLength}"
}

fun {
   var var0: Boolean
   try {
      Class.forName("pkg.Missing")
      var0 = true
   } catch (var5: java.lang.Throwable) {
      var0 = false
   }

   loaded = var0
   val var6: IntArray = IntArray(8)
   var a: IntArray = var6
   var i: Int = 0

   for (var4 in var6.length..i) {
      a[i] = i * i
   }

   squares = var6
   val var7: IntArray = IntArray(8)
   a = var7
   i = 0

   for (var11 in var7.length..i) {
      a[i] = i * i * i
   }

   cubes = var7
   nameLength = Object.class.getName().length() * 2
}

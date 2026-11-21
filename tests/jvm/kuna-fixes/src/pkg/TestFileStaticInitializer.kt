@file:JvmName("TestFileStaticInitializer")

package pkg

val loaded: Boolean = try {
    Class.forName("pkg.Missing")
    true
} catch (e: Throwable) {
    false
}

val squares: IntArray = IntArray(8).also { a ->
    for (i in a.indices) a[i] = i * i
}

val cubes: IntArray = IntArray(8).also { a ->
    for (i in a.indices) a[i] = i * i * i
}

val nameLength: Int = Any::class.java.name.length * 2

fun describe(): String = "$loaded ${squares[3]} ${cubes[2]} $nameLength"

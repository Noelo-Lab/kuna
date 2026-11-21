@file:JvmMultifileClass
@file:JvmName("TestFileClassConstructor")

package pkg

public fun twice(size: Int): Int {
   return TestFileClassConstructor.area(size, 2)
}

public fun describe(size: Int): String {
   return if (size > 10) "large" else "small"
}

open fun TestFileClassConstructor__FileClassSizesKt() {
}

@file:JvmName(name = "TestExtensionProperty")

package pkg

public val String.firstChar: Char
   public get() {
      return this[0]
   }


public val <T> List<T>.secondOrNull: T?
   public get() {
      return (if (this.size > 1) this.get(1) else null) as T
   }


public var StringBuilder.lastChar: Char
   public get() {
      return this[this.length - 1]
   }

   public set(value) {
      this.setCharAt(this.length - 1, value)
   }


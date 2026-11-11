@file:JvmName(name = "TestExtensionProperty")

package pkg

public final val firstChar: Char
   public final get() {
      return `$this$firstChar`.charAt(0)
   }


public final val secondOrNull: T?
   public final get() {
      return (T)(if (`$this$secondOrNull`.size() > 1) `$this$secondOrNull`.get(1) else null)
   }


public final var lastChar: Char
   public final get() {
      return `$this$lastChar`.charAt(`$this$lastChar`.length() - 1)
   }

   public final set(value) {
      `$this$lastChar`.setCharAt(`$this$lastChar`.length() - 1, value)
   }


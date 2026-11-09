package pkg

public class TestArrayOfArrays {
   public fun rows(n: Int): Array<ByteArray?> {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: iload 1
      // 1: anewarray 15
      // 4: areturn
   }

   public fun grid(n: Int): Array<Array<pkg.TestArrayOfArrays.Cell>?> {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 0: iload 1
      // 1: anewarray 21
      // 4: areturn
   }

   public fun table(n: Int): Array<IntArray> {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: bipush 0
      // 01: istore 2
      // 02: iload 1
      // 03: anewarray 25
      // 06: astore 3
      // 07: iload 2
      // 08: iload 1
      // 09: if_icmpge 1d
      // 0c: iload 2
      // 0d: istore 4
      // 0f: aload 3
      // 10: iload 4
      // 12: iload 4
      // 14: newarray 10
      // 16: aastore
      // 17: iinc 2 1
      // 1a: goto 07
      // 1d: aload 3
      // 1e: areturn
   }

   public class Cell(v: Int) {
      public final val v: Int

      init {
         this.v = v
      }
   }
}

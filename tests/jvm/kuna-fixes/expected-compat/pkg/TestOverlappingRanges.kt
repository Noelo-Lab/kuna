package pkg

import java.io.Closeable

public class TestOverlappingRanges(store: pkg.TestOverlappingRanges.Store) {
   private final val store: pkg.TestOverlappingRanges.Store

   init {
      this.store = store
   }

   public fun civilized(res: Closeable, name: String): Boolean {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: ldc "res"
      // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
      // 06: aload 2
      // 07: ldc "name"
      // 09: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
      // 0c: aload 1
      // 0d: astore 3
      // 0e: aconst_null
      // 0f: astore 4
      // 11: nop
      // 12: aload 3
      // 13: astore 5
      // 15: bipush 0
      // 16: istore 6
      // 18: nop
      // 19: aload 0
      // 1a: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 1d: aload 2
      // 1e: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 23: bipush 1
      // 24: istore 8
      // 26: aload 3
      // 27: aload 4
      // 29: invokestatic kotlin/io/CloseableKt.closeFinally (Ljava/io/Closeable;Ljava/lang/Throwable;)V
      // 2c: iload 8
      // 2e: ireturn
      // 2f: astore 7
      // 31: nop
      // 32: getstatic kotlin/Unit.INSTANCE Lkotlin/Unit;
      // 35: astore 5
      // 37: aload 3
      // 38: aload 4
      // 3a: invokestatic kotlin/io/CloseableKt.closeFinally (Ljava/io/Closeable;Ljava/lang/Throwable;)V
      // 3d: goto 54
      // 40: astore 5
      // 42: aload 5
      // 44: astore 4
      // 46: aload 5
      // 48: athrow
      // 49: astore 5
      // 4b: aload 3
      // 4c: aload 4
      // 4e: invokestatic kotlin/io/CloseableKt.closeFinally (Ljava/io/Closeable;Ljava/lang/Throwable;)V
      // 51: aload 5
      // 53: athrow
      // 54: aload 0
      // 55: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 58: aload 2
      // 59: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 5e: bipush 0
      // 5f: ireturn
   }

   public fun readUninterruptibly() {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: bipush 0
      // 01: istore 1
      // 02: nop
      // 03: nop
      // 04: nop
      // 05: aload 0
      // 06: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 09: invokeinterface pkg/TestOverlappingRanges$Store.read ()V 1
      // 0e: nop
      // 0f: iload 1
      // 10: ifeq 1c
      // 13: aload 0
      // 14: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 17: invokeinterface pkg/TestOverlappingRanges$Store.interrupt ()V 1
      // 1c: return
      // 1d: astore 2
      // 1e: aload 0
      // 1f: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 22: invokeinterface pkg/TestOverlappingRanges$Store.clearInterrupt ()V 1
      // 27: bipush 1
      // 28: istore 1
      // 29: goto 03
      // 2c: astore 2
      // 2d: aload 0
      // 2e: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 31: aload 2
      // 32: invokeinterface pkg/TestOverlappingRanges$Store.log (Lpkg/TestOverlappingRanges$IoFailure;)V 2
      // 37: nop
      // 38: iload 1
      // 39: ifeq 45
      // 3c: aload 0
      // 3d: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 40: invokeinterface pkg/TestOverlappingRanges$Store.interrupt ()V 1
      // 45: return
      // 46: astore 2
      // 47: iload 1
      // 48: ifeq 54
      // 4b: aload 0
      // 4c: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 4f: invokeinterface pkg/TestOverlappingRanges$Store.interrupt ()V 1
      // 54: aload 2
      // 55: athrow
   }

   public fun logged(name: String): Boolean {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: ldc "name"
      // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
      // 06: aload 0
      // 07: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 0a: aload 1
      // 0b: invokeinterface pkg/TestOverlappingRanges$Store.open (Ljava/lang/String;)Ljava/lang/Object; 2
      // 10: astore 2
      // 11: aconst_null
      // 12: astore 3
      // 13: nop
      // 14: nop
      // 15: aload 0
      // 16: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 19: aload 1
      // 1a: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 1f: bipush 1
      // 20: istore 4
      // 22: aload 0
      // 23: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 26: aload 2
      // 27: aload 3
      // 28: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 2d: iload 4
      // 2f: ireturn
      // 30: astore 4
      // 32: aload 0
      // 33: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 36: aload 4
      // 38: invokeinterface pkg/TestOverlappingRanges$Store.log (Lpkg/TestOverlappingRanges$IoFailure;)V 2
      // 3d: aload 0
      // 3e: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 41: aload 2
      // 42: aload 3
      // 43: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 48: goto 63
      // 4b: astore 4
      // 4d: aload 4
      // 4f: astore 3
      // 50: aload 4
      // 52: athrow
      // 53: astore 4
      // 55: aload 0
      // 56: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 59: aload 2
      // 5a: aload 3
      // 5b: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 60: aload 4
      // 62: athrow
      // 63: aload 0
      // 64: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 67: aload 1
      // 68: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 6d: bipush 0
      // 6e: ireturn
   }

   public fun sharedTail(name: String, flag: Boolean): Boolean {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: ldc "name"
      // 03: invokestatic kotlin/jvm/internal/Intrinsics.checkNotNullParameter (Ljava/lang/Object;Ljava/lang/String;)V
      // 06: aload 0
      // 07: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 0a: aload 1
      // 0b: invokeinterface pkg/TestOverlappingRanges$Store.open (Ljava/lang/String;)Ljava/lang/Object; 2
      // 10: astore 3
      // 11: aconst_null
      // 12: astore 4
      // 14: nop
      // 15: iload 2
      // 16: ifeq 43
      // 19: nop
      // 1a: aload 0
      // 1b: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 1e: aload 1
      // 1f: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 24: bipush 1
      // 25: istore 5
      // 27: aload 0
      // 28: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 2b: aload 3
      // 2c: aload 4
      // 2e: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 33: iload 5
      // 35: ireturn
      // 36: astore 5
      // 38: aload 0
      // 39: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 3c: aload 5
      // 3e: invokeinterface pkg/TestOverlappingRanges$Store.log (Lpkg/TestOverlappingRanges$IoFailure;)V 2
      // 43: aload 0
      // 44: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 47: aload 3
      // 48: aload 4
      // 4a: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 4f: goto 6c
      // 52: astore 5
      // 54: aload 5
      // 56: astore 4
      // 58: aload 5
      // 5a: athrow
      // 5b: astore 5
      // 5d: aload 0
      // 5e: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 61: aload 3
      // 62: aload 4
      // 64: invokeinterface pkg/TestOverlappingRanges$Store.close (Ljava/lang/Object;Ljava/lang/Throwable;)V 3
      // 69: aload 5
      // 6b: athrow
      // 6c: aload 0
      // 6d: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 70: aload 1
      // 71: invokeinterface pkg/TestOverlappingRanges$Store.delete (Ljava/lang/String;)V 2
      // 76: bipush 0
      // 77: ireturn
   }

   public fun countedRead(): Int {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: bipush 0
      // 01: istore 1
      // 02: bipush 0
      // 03: istore 2
      // 04: nop
      // 05: nop
      // 06: iinc 2 1
      // 09: nop
      // 0a: aload 0
      // 0b: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 0e: invokeinterface pkg/TestOverlappingRanges$Store.read ()V 1
      // 13: iload 2
      // 14: istore 3
      // 15: iload 1
      // 16: ifeq 22
      // 19: aload 0
      // 1a: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 1d: invokeinterface pkg/TestOverlappingRanges$Store.interrupt ()V 1
      // 22: iload 3
      // 23: ireturn
      // 24: astore 3
      // 25: bipush 1
      // 26: istore 1
      // 27: goto 05
      // 2a: astore 3
      // 2b: iload 1
      // 2c: ifeq 38
      // 2f: aload 0
      // 30: getfield pkg/TestOverlappingRanges.store Lpkg/TestOverlappingRanges$Store;
      // 33: invokeinterface pkg/TestOverlappingRanges$Store.interrupt ()V 1
      // 38: aload 3
      // 39: athrow
   }

   public class InterruptedFailure : TestOverlappingRanges.IoFailure

   public open class IoFailure : RuntimeException

   public interface Store {
      public abstract fun open(name: String): Any {
      }

      public abstract fun delete(name: String) {
      }

      public abstract fun close(handle: Any, cause: Throwable?) {
      }

      public abstract fun read() {
      }

      public abstract fun clearInterrupt() {
      }

      public abstract fun interrupt() {
      }

      public abstract fun log(e: pkg.TestOverlappingRanges.IoFailure) {
      }
   }
}

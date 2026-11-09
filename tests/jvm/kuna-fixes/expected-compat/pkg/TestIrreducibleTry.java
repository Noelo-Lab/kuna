package pkg;

public class TestIrreducibleTry {
   public static int run(int param0, TestIrreducibleTry.Src param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 0
      // 01: ifne 22
      // 04: aload 1
      // 05: invokeinterface pkg/TestIrreducibleTry$Src.open ()V 1
      // 0a: aload 1
      // 0b: invokeinterface pkg/TestIrreducibleTry$Src.next ()I 1
      // 10: istore 2
      // 11: iload 2
      // 12: ifeq 31
      // 15: aload 1
      // 16: invokeinterface pkg/TestIrreducibleTry$Src.step ()I 1
      // 1b: istore 3
      // 1c: iload 3
      // 1d: ifne 28
      // 20: iload 2
      // 21: ireturn
      // 22: aload 1
      // 23: invokeinterface pkg/TestIrreducibleTry$Src.resume ()V 1
      // 28: aload 1
      // 29: invokeinterface pkg/TestIrreducibleTry$Src.tick ()V 1
      // 2e: goto 0a
      // 31: aload 1
      // 32: invokeinterface pkg/TestIrreducibleTry$Src.close ()V 1
      // 37: bipush -1
      // 38: ireturn
      // 39: astore 4
      // 3b: aload 1
      // 3c: invokeinterface pkg/TestIrreducibleTry$Src.close ()V 1
      // 41: aload 4
      // 43: athrow
      // try (2 -> 12): 25 null
      // try (16 -> 21): 25 null
   }

   public static int poll(int param0, TestIrreducibleTry.Src param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 0
      // 01: tableswitch 151 0 3 31 51 84 119
      // 20: aload 1
      // 21: invokeinterface pkg/TestIrreducibleTry$Src.open ()V 1
      // 26: aload 1
      // 27: invokeinterface pkg/TestIrreducibleTry$Src.next ()I 1
      // 2c: istore 2
      // 2d: iload 2
      // 2e: ifne 40
      // 31: bipush 10
      // 33: ireturn
      // 34: aload 1
      // 35: invokeinterface pkg/TestIrreducibleTry$Src.resume ()V 1
      // 3a: aload 1
      // 3b: invokeinterface pkg/TestIrreducibleTry$Src.tick ()V 1
      // 40: aload 1
      // 41: invokeinterface pkg/TestIrreducibleTry$Src.step ()I 1
      // 46: istore 3
      // 47: aload 1
      // 48: invokeinterface pkg/TestIrreducibleTry$Src.next ()I 1
      // 4d: istore 2
      // 4e: iload 2
      // 4f: ifne 61
      // 52: bipush 20
      // 54: ireturn
      // 55: aload 1
      // 56: invokeinterface pkg/TestIrreducibleTry$Src.resume ()V 1
      // 5b: aload 1
      // 5c: invokeinterface pkg/TestIrreducibleTry$Src.tick ()V 1
      // 61: aload 1
      // 62: invokeinterface pkg/TestIrreducibleTry$Src.step ()I 1
      // 67: ifne 26
      // 6a: aload 1
      // 6b: invokeinterface pkg/TestIrreducibleTry$Src.next ()I 1
      // 70: istore 2
      // 71: iload 2
      // 72: ifne 84
      // 75: bipush 30
      // 77: ireturn
      // 78: aload 1
      // 79: invokeinterface pkg/TestIrreducibleTry$Src.resume ()V 1
      // 7e: aload 1
      // 7f: invokeinterface pkg/TestIrreducibleTry$Src.tick ()V 1
      // 84: aload 1
      // 85: invokeinterface pkg/TestIrreducibleTry$Src.tick ()V 1
      // 8a: goto 26
      // 8d: astore 4
      // 8f: aload 1
      // 90: invokeinterface pkg/TestIrreducibleTry$Src.close ()V 1
      // 95: aload 4
      // 97: athrow
      // 98: bipush -1
      // 99: ireturn
      // try (2 -> 7): 46 null
      // try (13 -> 21): 46 null
      // try (27 -> 35): 46 null
      // try (41 -> 46): 46 null
   }

   public interface Src {
      void open();

      int next();

      int step();

      void resume();

      void close();

      void tick();
   }
}

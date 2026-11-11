package pkg;

public class TestTypeSwitchPatternType {
   static Integer next() {
      return 3;
   }

   static void sink(String var0) {
   }

   static void boxed(Integer var0) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 1
      // 07: bipush 0
      // 08: istore 2
      // 09: aload 1
      // 0a: iload 2
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ 42, java/lang/Integer ]
      // 10: lookupswitch 60 2 0 28 1 36
      // 2c: ldc "42"
      // 2e: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 31: goto 54
      // 34: aload 1
      // 35: astore 3
      // 36: aload 3
      // 37: invokevirtual java/lang/Integer.intValue ()I
      // 3a: bipush 50
      // 3c: if_icmplt 44
      // 3f: bipush 2
      // 40: istore 2
      // 41: goto 09
      // 44: ldc "small"
      // 46: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 49: goto 54
      // 4c: ldc "other"
      // 4e: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 51: goto 54
      // 54: return
   }

   static void local() {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: invokestatic pkg/TestTypeSwitchPatternType.next ()Ljava/lang/Integer;
      // 03: astore 0
      // 04: aload 0
      // 05: dup
      // 06: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 09: pop
      // 0a: astore 1
      // 0b: bipush 0
      // 0c: istore 2
      // 0d: aload 1
      // 0e: iload 2
      // 0f: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ 42, java/lang/Integer ]
      // 14: lookupswitch 60 2 0 28 1 36
      // 30: ldc "42"
      // 32: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 35: goto 58
      // 38: aload 1
      // 39: astore 3
      // 3a: aload 3
      // 3b: invokevirtual java/lang/Integer.intValue ()I
      // 3e: bipush 50
      // 40: if_icmplt 48
      // 43: bipush 2
      // 44: istore 2
      // 45: goto 0d
      // 48: ldc "small"
      // 4a: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 4d: goto 58
      // 50: ldc "other"
      // 52: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 55: goto 58
      // 58: return
   }

   static void string(String var0) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 1
      // 07: bipush 0
      // 08: istore 2
      // 09: aload 1
      // 0a: iload 2
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ "a", java/lang/String ]
      // 10: lookupswitch 59 2 0 28 1 36
      // 2c: ldc "a"
      // 2e: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 31: goto 53
      // 34: aload 1
      // 35: astore 3
      // 36: aload 3
      // 37: invokevirtual java/lang/String.length ()I
      // 3a: bipush 3
      // 3b: if_icmpgt 43
      // 3e: bipush 2
      // 3f: istore 2
      // 40: goto 09
      // 43: ldc "long"
      // 45: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 48: goto 53
      // 4b: ldc "other"
      // 4d: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 50: goto 53
      // 53: return
   }

   static void character(Character var0) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 0
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 1
      // 07: bipush 0
      // 08: istore 2
      // 09: aload 1
      // 0a: iload 2
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ 97, java/lang/Character ]
      // 10: lookupswitch 60 2 0 28 1 36
      // 2c: ldc "a"
      // 2e: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 31: goto 54
      // 34: aload 1
      // 35: astore 3
      // 36: aload 3
      // 37: invokevirtual java/lang/Character.charValue ()C
      // 3a: bipush 120
      // 3c: if_icmpgt 44
      // 3f: bipush 2
      // 40: istore 2
      // 41: goto 09
      // 44: ldc "late"
      // 46: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 49: goto 54
      // 4c: ldc "other"
      // 4e: invokestatic pkg/TestTypeSwitchPatternType.sink (Ljava/lang/String;)V
      // 51: goto 54
      // 54: return
   }
}

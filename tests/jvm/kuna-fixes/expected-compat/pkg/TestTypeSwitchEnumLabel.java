package pkg;

public class TestTypeSwitchEnumLabel {
   static TestTypeSwitchEnumLabel.Shape next() {
      return TestTypeSwitchEnumLabel.Kind.CIRCLE;
   }

   static void sink(String s) {
   }

   void statement(TestTypeSwitchEnumLabel.Shape param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 2
      // 07: bipush 0
      // 08: istore 3
      // 09: aload 2
      // 0a: iload 3
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ null.invoke Ljava/lang/Enum$EnumDesc;, pkg/TestTypeSwitchEnumLabel$Kind, pkg/TestTypeSwitchEnumLabel$Polygon ]
      // 10: tableswitch 64 0 2 28 36 50
      // 2c: ldc "circle"
      // 2e: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 31: goto 55
      // 34: aload 2
      // 35: checkcast pkg/TestTypeSwitchEnumLabel$Kind
      // 38: astore 4
      // 3a: ldc "kind"
      // 3c: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 3f: goto 55
      // 42: aload 2
      // 43: checkcast pkg/TestTypeSwitchEnumLabel$Polygon
      // 46: astore 5
      // 48: ldc "polygon"
      // 4a: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 4d: goto 55
      // 50: ldc "other"
      // 52: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 55: return
   }

   int expression(TestTypeSwitchEnumLabel.Shape param1) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 2
      // 07: bipush 0
      // 08: istore 3
      // 09: aload 2
      // 0a: iload 3
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ null.invoke Ljava/lang/Enum$EnumDesc;, null.invoke Ljava/lang/Enum$EnumDesc;, pkg/TestTypeSwitchEnumLabel$Polygon ]
      // 10: tableswitch 46 0 2 28 32 36
      // 2c: bipush 1
      // 2d: goto 3f
      // 30: bipush 2
      // 31: goto 3f
      // 34: aload 2
      // 35: checkcast pkg/TestTypeSwitchEnumLabel$Polygon
      // 38: astore 4
      // 3a: bipush 3
      // 3b: goto 3f
      // 3e: bipush 4
      // 3f: ireturn
   }

   void guardedLocal() {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: invokestatic pkg/TestTypeSwitchEnumLabel.next ()Lpkg/TestTypeSwitchEnumLabel$Shape;
      // 03: astore 1
      // 04: aload 1
      // 05: astore 2
      // 06: bipush 0
      // 07: istore 3
      // 08: aload 2
      // 09: iload 3
      // 0a: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ null.invoke Ljava/lang/Enum$EnumDesc;, pkg/TestTypeSwitchEnumLabel$Kind ]
      // 0f: tableswitch 68 -1 1 25 33 41
      // 28: ldc "null"
      // 2a: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 2d: goto 5b
      // 30: ldc "square"
      // 32: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 35: goto 5b
      // 38: aload 2
      // 39: checkcast pkg/TestTypeSwitchEnumLabel$Kind
      // 3c: astore 4
      // 3e: aload 4
      // 40: getstatic pkg/TestTypeSwitchEnumLabel$Kind.CIRCLE Lpkg/TestTypeSwitchEnumLabel$Kind;
      // 43: if_acmpeq 4b
      // 46: bipush 2
      // 47: istore 3
      // 48: goto 08
      // 4b: ldc "circle"
      // 4d: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 50: goto 5b
      // 53: ldc "other"
      // 55: invokestatic pkg/TestTypeSwitchEnumLabel.sink (Ljava/lang/String;)V
      // 58: goto 5b
      // 5b: return
   }

   enum Kind implements TestTypeSwitchEnumLabel.Shape {
      CIRCLE,
      SQUARE;
   }

   static final class Polygon implements TestTypeSwitchEnumLabel.Shape {
   }

   interface Shape {
   }
}

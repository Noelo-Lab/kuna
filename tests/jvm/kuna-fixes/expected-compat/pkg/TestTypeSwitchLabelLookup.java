package pkg;

public class TestTypeSwitchLabelLookup {
   static void sink(String s) {
   }

   void arrow(Object o) {
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
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ java/lang/Integer, java/lang/String ]
      // 10: lookupswitch 61 2 0 28 1 28
      // 2c: aload 2
      // 2d: instanceof java/lang/Integer
      // 30: ifeq 36
      // 33: goto 45
      // 36: aload 2
      // 37: instanceof java/lang/String
      // 3a: ifeq 40
      // 3d: goto 45
      // 40: bipush 2
      // 41: istore 3
      // 42: goto 09
      // 45: ldc "number or text"
      // 47: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 4a: goto 55
      // 4d: ldc "other"
      // 4f: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 52: goto 55
      // 55: return
   }

   void guarded(Object o, boolean b) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: dup
      // 02: invokestatic java/util/Objects.requireNonNull (Ljava/lang/Object;)Ljava/lang/Object;
      // 05: pop
      // 06: astore 3
      // 07: bipush 0
      // 08: istore 4
      // 0a: aload 3
      // 0b: iload 4
      // 0d: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ java/lang/Integer, java/lang/String ]
      // 12: lookupswitch 61 2 0 26 1 26
      // 2c: aload 3
      // 2d: instanceof java/lang/Integer
      // 30: ifeq 36
      // 33: goto 3d
      // 36: aload 3
      // 37: instanceof java/lang/String
      // 3a: ifeq 41
      // 3d: iload 2
      // 3e: ifne 47
      // 41: bipush 2
      // 42: istore 4
      // 44: goto 0a
      // 47: ldc "number or text"
      // 49: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 4c: goto 57
      // 4f: ldc "other"
      // 51: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 54: goto 57
      // 57: return
   }

   int expression(Object o) {
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
      // 0b: invokedynamic typeSwitch (Ljava/lang/Object;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ java/lang/Integer, java/lang/String, java/lang/Long ]
      // 10: tableswitch 71 0 2 28 28 57
      // 2c: aload 2
      // 2d: instanceof java/lang/Integer
      // 30: ifeq 36
      // 33: goto 45
      // 36: aload 2
      // 37: instanceof java/lang/String
      // 3a: ifeq 40
      // 3d: goto 45
      // 40: bipush 2
      // 41: istore 3
      // 42: goto 09
      // 45: bipush 1
      // 46: goto 5b
      // 49: aload 2
      // 4a: checkcast java/lang/Long
      // 4d: astore 4
      // 4f: aload 4
      // 51: invokevirtual java/lang/Long.intValue ()I
      // 54: goto 5b
      // 57: bipush 3
      // 58: goto 5b
      // 5b: ireturn
   }

   void shapes(TestTypeSwitchLabelLookup.Shape param1) {
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
      // 0b: invokedynamic typeSwitch (Lpkg/TestTypeSwitchLabelLookup$Shape;I)I bsm=java/lang/runtime/SwitchBootstraps.typeSwitch (Ljava/lang/invoke/MethodHandles$Lookup;Ljava/lang/String;Ljava/lang/invoke/MethodType;[Ljava/lang/Object;)Ljava/lang/invoke/CallSite; args=[ null.invoke Ljava/lang/Enum$EnumDesc;, null.invoke Ljava/lang/Enum$EnumDesc;, pkg/TestTypeSwitchLabelLookup$Polygon, pkg/TestTypeSwitchLabelLookup$Line, pkg/TestTypeSwitchLabelLookup$Point ]
      // 10: tableswitch 87 0 4 36 36 44 44 44
      // 34: ldc "kind"
      // 36: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 39: goto 6f
      // 3c: aload 2
      // 3d: instanceof pkg/TestTypeSwitchLabelLookup$Polygon
      // 40: ifeq 46
      // 43: goto 5f
      // 46: aload 2
      // 47: instanceof pkg/TestTypeSwitchLabelLookup$Line
      // 4a: ifeq 50
      // 4d: goto 5f
      // 50: aload 2
      // 51: instanceof pkg/TestTypeSwitchLabelLookup$Point
      // 54: ifeq 5a
      // 57: goto 5f
      // 5a: bipush 5
      // 5b: istore 3
      // 5c: goto 09
      // 5f: ldc "figure"
      // 61: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 64: goto 6f
      // 67: ldc "other"
      // 69: invokestatic pkg/TestTypeSwitchLabelLookup.sink (Ljava/lang/String;)V
      // 6c: goto 6f
      // 6f: return
   }

   void constants(Integer o) {
      switch (o) {
         case 5:
         case 1:
            sink("5 or 7");
            break;
         case 1:
            sink("1");
            break;
         case Integer i when i > 100:
            sink("big");
            break;
         default:
            sink("other");
      }
   }

   String substituted(Integer o) {
      return switch (o) {
         case Integer var5 -> "one";
         case Integer i when i > 10 -> "big";
         default -> "other";
      };
   }

   enum Kind implements TestTypeSwitchLabelLookup.Shape {
      CIRCLE,
      SQUARE;
   }

   static final class Line implements TestTypeSwitchLabelLookup.Shape {
   }

   record Point(int x) implements TestTypeSwitchLabelLookup.Shape {
   }

   static final class Polygon implements TestTypeSwitchLabelLookup.Shape {
   }

   interface Shape {
   }
}

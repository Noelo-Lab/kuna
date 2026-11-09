package pkg;

public class TestSwitchExprConstLub {
   static void use(Object o) {
   }

   static <T> T id(T t) {
      return t;
   }

   static void each(TestSwitchExprConstLub.IntSink sink) {
      sink.accept(34);
   }

   public void stringOrChar(int c) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 37 2 34 27 92 32
      // 1c: ldc "\\\""
      // 1e: goto 2b
      // 21: ldc "\\\\"
      // 23: goto 2b
      // 26: iload 1
      // 27: i2c
      // 28: invokestatic java/lang/Character.valueOf (C)Ljava/lang/Character;
      // 2b: invokestatic pkg/TestSwitchExprConstLub.use (Ljava/lang/Object;)V
      // 2e: return
   }

   public void inLambda() {
      each(c -> {
         // $VF: Couldn't be decompiled
         // Bytecode:
         // 00: iload 0
         // 01: lookupswitch 24 1 34 19
         // 14: ldc "\\\""
         // 16: goto 1e
         // 19: iload 0
         // 1a: i2c
         // 1b: invokestatic java/lang/Character.valueOf (C)Ljava/lang/Character;
         // 1e: invokestatic pkg/TestSwitchExprConstLub.use (Ljava/lang/Object;)V
         // 21: return
      });
   }

   public Object stringOrInt(int x) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 39 2 1 27 2 32
      // 1c: ldc "one"
      // 1e: goto 2c
      // 21: bipush 2
      // 22: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
      // 25: goto 2c
      // 28: iload 1
      // 29: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
      // 2c: areturn
   }

   public Object boolOrString(int x) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 26 1 1 19
      // 14: bipush 1
      // 15: invokestatic java/lang/Boolean.valueOf (Z)Ljava/lang/Boolean;
      // 18: goto 1d
      // 1b: ldc "no"
      // 1d: astore 2
      // 1e: aload 2
      // 1f: invokestatic pkg/TestSwitchExprConstLub.use (Ljava/lang/Object;)V
      // 22: aload 2
      // 23: areturn
   }

   public Object classOrString(int x) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 24 1 1 19
      // 14: ldc java/lang/String
      // 16: goto 1b
      // 19: ldc "s"
      // 1b: areturn
   }

   public Object stringOrBuilder(int x) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 24 1 1 19
      // 14: ldc "s"
      // 16: goto 22
      // 19: new java/lang/StringBuilder
      // 1c: dup
      // 1d: ldc "b"
      // 1f: invokespecial java/lang/StringBuilder.<init> (Ljava/lang/String;)V
      // 22: areturn
   }

   public Object stringSelector(String s) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: aload 1
      // 01: astore 2
      // 02: bipush -1
      // 03: istore 3
      // 04: aload 2
      // 05: invokevirtual java/lang/String.hashCode ()I
      // 08: lookupswitch 31 1 97 20
      // 1c: aload 2
      // 1d: ldc "a"
      // 1f: invokevirtual java/lang/String.equals (Ljava/lang/Object;)Z
      // 22: ifeq 27
      // 25: bipush 0
      // 26: istore 3
      // 27: iload 3
      // 28: lookupswitch 25 1 0 20
      // 3c: ldc "A"
      // 3e: goto 48
      // 41: aload 1
      // 42: invokevirtual java/lang/String.length ()I
      // 45: invokestatic java/lang/Integer.valueOf (I)Ljava/lang/Integer;
      // 48: invokestatic pkg/TestSwitchExprConstLub.id (Ljava/lang/Object;)Ljava/lang/Object;
      // 4b: areturn
   }

   public void blockArm(int c) {
      // $VF: Couldn't be decompiled
      // Bytecode:
      // 00: iload 1
      // 01: lookupswitch 24 1 10 19
      // 14: ldc "\\n"
      // 16: goto 29
      // 19: iload 1
      // 1a: bipush 32
      // 1c: if_icmpge 24
      // 1f: ldc "ctl"
      // 21: goto 29
      // 24: iload 1
      // 25: i2c
      // 26: invokestatic java/lang/Character.valueOf (C)Ljava/lang/Character;
      // 29: invokestatic pkg/TestSwitchExprConstLub.use (Ljava/lang/Object;)V
      // 2c: return
   }

   public Object charOrInt(int x) {
      return switch (x) {
         case 1 -> 'a';
         default -> 7;
      };
   }

   public Object stringOrNull(int x) {
      return switch (x) {
         case 1 -> "s";
         default -> null;
      };
   }

   public char charSwitch(int x) {
      return switch (x) {
         case 1 -> 'a';
         default -> 'b';
      };
   }

   public Object ternary(boolean b, int c) {
      return b ? "s" : (char)c;
   }

   interface IntSink {
      void accept(int var1);
   }
}

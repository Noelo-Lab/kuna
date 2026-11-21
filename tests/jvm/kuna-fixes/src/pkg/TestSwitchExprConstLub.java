package pkg;

public class TestSwitchExprConstLub {
   static void use(Object o) {
   }

   static <T> T id(T t) {
      return t;
   }

   static void each(IntSink sink) {
      sink.accept(34);
   }

   public void stringOrChar(int c) {
      use(switch (c) {
         case 34 -> "\\\"";
         case 92 -> "\\\\";
         default -> (char)c;
      });
   }

   public void inLambda() {
      each(c -> use(switch (c) {
         case 34 -> "\\\"";
         default -> (char)c;
      }));
   }

   public Object stringOrInt(int x) {
      return switch (x) {
         case 1 -> "one";
         case 2 -> 2;
         default -> x;
      };
   }

   public Object boolOrString(int x) {
      Object o = switch (x) {
         case 1 -> true;
         default -> "no";
      };
      use(o);
      return o;
   }

   public Object classOrString(int x) {
      return switch (x) {
         case 1 -> String.class;
         default -> "s";
      };
   }

   public Object stringOrBuilder(int x) {
      return switch (x) {
         case 1 -> "s";
         default -> new StringBuilder("b");
      };
   }

   public Object stringSelector(String s) {
      return id(switch (s) {
         case "a" -> "A";
         default -> s.length();
      });
   }

   public void blockArm(int c) {
      use(switch (c) {
         case 10 -> "\\n";
         default -> {
            if (c < 32) {
               yield "ctl";
            }

            yield (char)c;
         }
      });
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
      void accept(int c);
   }
}

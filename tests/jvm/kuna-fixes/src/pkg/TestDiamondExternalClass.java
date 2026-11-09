package pkg;

// DiamondLib, DiamondBox and DiamondPlain stand for a library that is not given to the decompiler:
// their class files are not part of the case.
public class TestDiamondExternalClass {
   static final DiamondLib<String, Void> PARSER = new DiamondLib<>("p", true, objs -> (String)objs[0]);
   private final DiamondLib<Integer, String> parser = new DiamondLib<>("q", (objs, c) -> objs.length + c.length());
   static final DiamondLib<?, Void> WILD = new DiamondLib<>("w", true, objs -> objs.length);
   static final DiamondBox<String> BOX = new DiamondBox<>();
   static final Object OBJECT = new DiamondBox<String>();
   static final DiamondPlain PLAIN = new DiamondPlain();
   static DiamondLib<String, Void> assigned;

   static DiamondLib<String, Void> local() {
      DiamondLib<String, Void> p = new DiamondLib<>("p", true, objs -> (String)objs[0]);
      p.use();
      return p;
   }

   static DiamondLib<String, Void> ret() {
      return new DiamondLib<>("p", true, objs -> (String)objs[0]);
   }

   static void assign() {
      assigned = new DiamondLib<>("p", true, objs -> (String)objs[0]);
   }

   static void take(DiamondLib<String, Void> p) {
   }

   static <T> void takeAny(DiamondLib<T, Void> p) {
   }

   static void arg() {
      take(new DiamondLib<>("p", true, objs -> (String)objs[0]));
      takeAny(new DiamondLib<>("p", true, objs -> (String)objs[0]));
   }

   static DiamondBox<String> pick(boolean b, DiamondBox<String> other) {
      return b ? new DiamondBox<>() : other;
   }

   interface Fn<A, R> {
      R apply(A a);
   }

   interface Fn2<A, B, R> {
      R apply(A a, B b);
   }

   static class Holder<T> {
      final DiamondBox<T> box = new DiamondBox<>();

      DiamondLib<T, Void> parser(T value) {
         return new DiamondLib<>("h", true, objs -> objs.length > 0 ? value : null);
      }
   }
}

class DiamondLib<V, C> {
   DiamondLib(String name, boolean lenient, TestDiamondExternalClass.Fn<Object[], V> builder) {
   }

   DiamondLib(String name, TestDiamondExternalClass.Fn2<Object[], C, V> builder) {
   }

   void use() {
   }
}

class DiamondBox<T> {
}

class DiamondPlain {
}

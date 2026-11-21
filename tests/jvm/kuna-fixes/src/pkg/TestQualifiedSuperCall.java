package pkg;

public class TestQualifiedSuperCall {
   int base;

   public TestQualifiedSuperCall(int base) {
      this.base = base;
   }

   public class Inner {
      int v;

      public Inner(int v) {
         this.v = v + base;
      }
   }

   public class Implicit extends Inner {
      public Implicit() {
         super(1);
      }
   }

   public class Qualified extends Inner {
      public Qualified(TestQualifiedSuperCall o) {
         o.super(2);
      }

      public Qualified(TestQualifiedSuperCall o, int y) {
         o.super(y * 2);
      }

      public Qualified(int y) {
         new TestQualifiedSuperCall(y).super(y + 1);
      }

      public Qualified(TestQualifiedSuperCall[] os, int i) {
         os[i].super(i);
      }
   }

   public class Deep {
      public class Deeper extends Inner {
         public Deeper() {
            super(8);
         }

         public Deeper(TestQualifiedSuperCall o) {
            o.super(9);
         }
      }
   }

   public static class Nested extends Inner {
      public Nested(TestQualifiedSuperCall o) {
         o.super(3);
      }
   }

   public Inner local(TestQualifiedSuperCall o, int n) {
      class Local extends Inner {
         Local() {
            o.super(n);
         }
      }
      return new Local();
   }

   public Inner anon(TestQualifiedSuperCall o, int n) {
      return o.new Inner(n) {
         int get() {
            return this.v + 1;
         }
      };
   }

   public Inner anonImplicit(int n) {
      return new Inner(n) {
         int get() {
            return this.v + 2;
         }
      };
   }
}

package pkg;

import java.util.Objects;

public class TestQualifiedSuperCall {
   int base;

   public TestQualifiedSuperCall(int base) {
      this.base = base;
   }

   public TestQualifiedSuperCall.Inner local(final TestQualifiedSuperCall o, final int n) {
      class Local extends TestQualifiedSuperCall.Inner {
         Local() {
            super(n);
         }
      }

      return new Local();
   }

   public TestQualifiedSuperCall.Inner anon(TestQualifiedSuperCall o, int n) {
      Objects.requireNonNull(o);
      return new TestQualifiedSuperCall.Inner(o, n) {
         {
            Objects.requireNonNull(x0);
         }

         int get() {
            return this.v + 1;
         }
      };
   }

   public TestQualifiedSuperCall.Inner anonImplicit(int n) {
      return new TestQualifiedSuperCall.Inner(n) {
         int get() {
            return this.v + 2;
         }
      };
   }

   public class Deep {
      public class Deeper extends TestQualifiedSuperCall.Inner {
         public Deeper() {
            super(8);
         }

         public Deeper(TestQualifiedSuperCall o) {
            super(9);
         }
      }
   }

   public class Implicit extends TestQualifiedSuperCall.Inner {
      public Implicit() {
         super(1);
      }
   }

   public class Inner {
      int v;

      public Inner(int v) {
         this.v = v + TestQualifiedSuperCall.this.base;
      }
   }

   public static class Nested extends TestQualifiedSuperCall.Inner {
      public Nested(TestQualifiedSuperCall o) {
         super(3);
      }
   }

   public class Qualified extends TestQualifiedSuperCall.Inner {
      public Qualified(TestQualifiedSuperCall o) {
         super(2);
      }

      public Qualified(TestQualifiedSuperCall o, int y) {
         Objects.requireNonNull(o);
         super(y * 2);
      }

      public Qualified(int y) {
         super(y + 1);
      }

      public Qualified(TestQualifiedSuperCall[] os, int i) {
         TestQualifiedSuperCall var10001 = os[i];
         Objects.requireNonNull(os[i]);
         super(i);
      }
   }
}

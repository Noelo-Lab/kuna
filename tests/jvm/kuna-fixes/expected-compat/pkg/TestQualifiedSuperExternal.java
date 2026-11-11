package pkg;

import java.util.Objects;
import pkglib.OuterLib;
import pkglib.OuterLib.Inner;

public class TestQualifiedSuperExternal extends OuterLib {
   public class Implicit extends Inner {
      public Implicit(int v) {
         super(TestQualifiedSuperExternal.this, v);
      }
   }

   public static class Nested extends Inner {
      public Nested(OuterLib o) {
         super(o, 3);
      }
   }

   public class Qualified extends Inner {
      public Qualified(OuterLib o, int v) {
         Objects.requireNonNull(o);
         super(o, v * 2);
      }
   }
}

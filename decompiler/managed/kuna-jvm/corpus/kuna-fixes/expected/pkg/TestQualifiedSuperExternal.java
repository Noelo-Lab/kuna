package pkg;

import pkglib.OuterLib;
import pkglib.OuterLib.Inner;

public class TestQualifiedSuperExternal extends OuterLib {
   public class Implicit extends Inner {
      public Implicit(int v) {
         super(v);
      }
   }

   public static class Nested extends Inner {
      public Nested(OuterLib o) {
         o.super(3);
      }
   }

   public class Qualified extends Inner {
      public Qualified(OuterLib o, int v) {
         o.super(v * 2);
      }
   }
}

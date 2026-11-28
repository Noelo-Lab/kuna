package pkg;

import pkglib.OuterLib;

public class TestQualifiedSuperExternal extends OuterLib {
   public class Implicit extends Inner {
      public Implicit(int v) {
         super(v);
      }
   }

   public class Qualified extends Inner {
      public Qualified(OuterLib o, int v) {
         o.super(v * 2);
      }
   }

   public static class Nested extends OuterLib.Inner {
      public Nested(OuterLib o) {
         o.super(3);
      }
   }
}

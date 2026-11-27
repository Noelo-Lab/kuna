package pkg;

public class TestSelectingCast {
   void f(TestSelectingCast.Decl d) {
   }

   void f(TestSelectingCast.Named n) {
   }

   void f(TestSelectingCast.Leaf l) {
   }

   void put(TestSelectingCast.Decl d, Object o) {
   }

   void put(TestSelectingCast.Decl d, String s) {
   }

   void put(TestSelectingCast.Decl d, Integer i) {
   }

   void one(Object o) {
   }

   void one(String s) {
   }

   void run(final TestSelectingCast.Member m, TestSelectingCast.Leaf l) {
      this.f((TestSelectingCast.Named)m);
      this.f((TestSelectingCast.Decl)m);
      this.f(l);
      this.f((TestSelectingCast.Named)l);
      this.put(m, (Object)null);
      this.put(l, (String)null);
      this.put(m, "x");
      this.one((String)null);
      this.one((Object)null);
      (new Runnable() {
         public void run() {
            TestSelectingCast.this.f((TestSelectingCast.Named)m);
         }
      }).run();
   }

   abstract static class Decl {
   }

   static class Leaf extends TestSelectingCast.Member {
   }

   static class Member extends TestSelectingCast.Decl implements TestSelectingCast.Named {
   }

   interface Named {
   }
}

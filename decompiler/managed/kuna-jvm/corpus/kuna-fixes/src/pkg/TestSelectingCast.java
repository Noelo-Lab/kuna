package pkg;

public class TestSelectingCast {
   interface Named {
   }

   abstract static class Decl {
   }

   static class Member extends Decl implements Named {
   }

   static class Leaf extends Member {
   }

   void f(Decl d) {
   }

   void f(Named n) {
   }

   void f(Leaf l) {
   }

   void put(Decl d, Object o) {
   }

   void put(Decl d, String s) {
   }

   void put(Decl d, Integer i) {
   }

   void one(Object o) {
   }

   void one(String s) {
   }

   void run(Member m, Leaf l) {
      this.f((Named)m);
      this.f((Decl)m);
      this.f(l);
      this.f((Named)l);
      this.put(m, (Object)null);
      this.put(l, (String)null);
      this.put(m, "x");
      this.one(null);
      this.one((Object)null);
      new Runnable() {
         public void run() {
            TestSelectingCast.this.f((Named)m);
         }
      }.run();
   }
}

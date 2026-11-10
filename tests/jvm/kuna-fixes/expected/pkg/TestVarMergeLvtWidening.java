package pkg;

public class TestVarMergeLvtWidening {
   static final Object MARK = new Object();

   static void use(TestVarMergeLvtWidening.Mods mods) {
   }

   public static TestVarMergeLvtWidening.Mods overriding(TestVarMergeLvtWidening.Element element) {
      TestVarMergeLvtWidening.Mods mods = element.getMods();
      if (!mods.has(1) && !mods.has(2)) {
         mods = new TestVarMergeLvtWidening.LinkedMods(mods);
         mods.drop(4);
         use(mods);
         return mods;
      } else {
         return null;
      }
   }

   public static TestVarMergeLvtWidening.Mods copied(TestVarMergeLvtWidening.Element element, boolean copy) {
      TestVarMergeLvtWidening.Mods mods = element.getMods();
      if (copy) {
         TestVarMergeLvtWidening.LinkedMods linked = new TestVarMergeLvtWidening.LinkedMods(mods);
         linked.trim();
         mods = linked;
      }

      mods.drop(4);
      return mods;
   }

   public int firstLength(TestVarMergeLvtWidening.Source src) {
      Object o = src.next();
      if (o == MARK) {
         return -1;
      }

      String s = (String)src.next();
      return s.length();
   }

   public interface Element {
      TestVarMergeLvtWidening.Mods getMods();
   }

   public static class LinkedMods implements TestVarMergeLvtWidening.Mods {
      public LinkedMods(TestVarMergeLvtWidening.Mods from) {
      }

      @Override
      public boolean has(int flag) {
         return false;
      }

      @Override
      public void drop(int flag) {
      }

      public void trim() {
      }
   }

   public interface Mods {
      boolean has(int var1);

      void drop(int var1);
   }

   public interface Source {
      Object next();
   }
}

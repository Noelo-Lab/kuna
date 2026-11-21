package pkg;

public class TestVarMergeLvtWidening {
   static final Object MARK = new Object();

   public interface Mods {
      boolean has(int flag);

      void drop(int flag);
   }

   public static class LinkedMods implements Mods {
      public LinkedMods(Mods from) {
      }

      public boolean has(int flag) {
         return false;
      }

      public void drop(int flag) {
      }

      public void trim() {
      }
   }

   public interface Element {
      Mods getMods();
   }

   public interface Source {
      Object next();
   }

   static void use(Mods mods) {
   }

   public static Mods overriding(Element element) {
      Mods mods = element.getMods();
      if (mods.has(1) || mods.has(2)) {
         return null;
      }
      mods = new LinkedMods(mods);
      mods.drop(4);
      use(mods);
      return mods;
   }

   public static Mods copied(Element element, boolean copy) {
      Mods mods = element.getMods();
      if (copy) {
         LinkedMods linked = new LinkedMods(mods);
         linked.trim();
         mods = linked;
      }
      mods.drop(4);
      return mods;
   }

   public int firstLength(Source src) {
      {
         Object o = src.next();
         if (o == MARK) {
            return -1;
         }
      }
      {
         String s = (String)src.next();
         return s.length();
      }
   }
}

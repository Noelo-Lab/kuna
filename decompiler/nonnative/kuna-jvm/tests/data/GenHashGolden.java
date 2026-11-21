import java.util.*;

public class GenHashGolden {
  static Random rnd = new Random(20261109L);
  static String[] WORDS;

  static int intKey(int flavor) {
    switch (flavor) {
      case 0: return rnd.nextInt(40);
      case 1: return rnd.nextInt(12) * 16;
      case 2: return rnd.nextInt(30) * 64 + rnd.nextInt(2);
      case 3: return rnd.nextInt(2000) - 1000;
      case 4: return rnd.nextInt(20) * 65536 + rnd.nextInt(3);
      default: return rnd.nextInt();
    }
  }

  static String strKey(int flavor) {
    switch (flavor) {
      case 0: return WORDS[rnd.nextInt(WORDS.length)];
      case 1: return "var" + rnd.nextInt(60);
      case 2: return "java/lang/" + WORDS[rnd.nextInt(WORDS.length)];
      case 3: return "" + (char) ('a' + rnd.nextInt(26)) + (char) ('A' + rnd.nextInt(26));
      default: {
        String[] odd = {"été", "中文", "naïve", "😀x", "λ", "", "Aa", "BB"};
        return odd[rnd.nextInt(odd.length)];
      }
    }
  }

  static String enc(Object k) {
    if (k instanceof Integer) return k.toString();
    String s = (String) k;
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < s.length(); i++) sb.append(String.format("%04x", (int) s.charAt(i)));
    return "x" + sb;
  }

  static boolean treeified(Object coll) {
    try {
      Object map = coll;
      if (coll instanceof HashSet) {
        java.lang.reflect.Field mf = HashSet.class.getDeclaredField("map");
        mf.setAccessible(true);
        map = mf.get(coll);
      }
      java.lang.reflect.Field tf = HashMap.class.getDeclaredField("table");
      tf.setAccessible(true);
      Object[] table = (Object[]) tf.get(map);
      if (table == null) return false;
      for (Object n : table) if (n != null && n.getClass().getName().endsWith("TreeNode")) return true;
      return false;
    } catch (Exception e) { throw new RuntimeException(e); }
  }

  static <K> String join(Collection<K> c) {
    StringBuilder sb = new StringBuilder();
    for (K k : c) { if (sb.length() > 0) sb.append(','); sb.append(enc(k)); }
    return sb.toString();
  }

  @SuppressWarnings("unchecked")
  static <K> void scenario(boolean map, boolean str, boolean linked) {
    int flavor = rnd.nextInt(str ? 5 : 6);
    int capChoice = rnd.nextInt(6);
    int cap = capChoice == 0 ? 0 : capChoice == 1 ? 1 : capChoice == 2 ? rnd.nextInt(40) : -1;
    StringBuilder ops = new StringBuilder();
    Set<Object> set = null; Map<Object, Integer> m = null;
    if (map) m = cap < 0 ? (linked ? new LinkedHashMap<>() : new HashMap<>()) : (linked ? new LinkedHashMap<>(cap) : new HashMap<>(cap));
    else set = cap < 0 ? (linked ? new LinkedHashSet<>() : new HashSet<>()) : (linked ? new LinkedHashSet<>(cap) : new HashSet<>(cap));
    int n = 1 + rnd.nextInt(rnd.nextInt(4) == 0 ? 300 : 40);
    List<Object> seen = new ArrayList<>();
    boolean tree = false;
    for (int i = 0; i < n; i++) {
      tree |= treeified(map ? (Object) m : (Object) set);
      int op = rnd.nextInt(100);
      if (ops.length() > 0) ops.append(',');
      if (op < 62) {
        Object k = str ? (Object) strKey(flavor) : (Object) intKey(flavor);
        seen.add(k);
        ops.append('a').append(enc(k));
        if (map) m.put(k, i); else set.add(k);
      } else if (op < 80 && !seen.isEmpty()) {
        Object k = seen.get(rnd.nextInt(seen.size()));
        ops.append('r').append(enc(k));
        if (map) m.remove(k); else set.remove(k);
      } else if (op < 83) {
        ops.append('c');
        if (map) m.clear(); else set.clear();
      } else if (op < 89) {
        ops.append('C');
        if (map) m = linked ? new LinkedHashMap<>(m) : new HashMap<>(m);
        else set = linked ? new LinkedHashSet<>(set) : new HashSet<>(set);
      } else if (op < 92 && !map && !linked) {
        ops.append('K');
        set = (Set<Object>) ((HashSet<Object>) set).clone();
      } else {
        int cnt = rnd.nextInt(30);
        List<Object> ks = new ArrayList<>();
        for (int j = 0; j < cnt; j++) ks.add(str ? (Object) strKey(flavor) : (Object) intKey(flavor));
        seen.addAll(ks);
        ops.append('A');
        for (int j = 0; j < ks.size(); j++) { if (j > 0) ops.append('/'); ops.append(enc(ks.get(j))); }
        if (map) {
          LinkedHashMap<Object, Integer> src = new LinkedHashMap<>();
          for (Object k : ks) src.put(k, i);
          m.putAll(src);
        } else set.addAll(ks);
      }
    }
    tree |= treeified(map ? (Object) m : (Object) set);
    System.out.println((tree ? "t" : "") + (map ? "M" : "S") + ";" + (str ? "T" : "I") + ";" + (linked ? "L" : "H") + ";" + cap + ";" + ops + ";" + join(map ? m.keySet() : set));
  }

  public static void main(String[] a) {
    WORDS = new String[]{"Object", "String", "Integer", "List", "Map", "Set", "Iterator", "Exception", "Runnable", "Thread",
      "value", "index", "result", "this", "i", "j", "k", "var1", "var2", "x", "y", "z", "name", "type", "size", "count", "buffer",
      "main", "run", "get", "set", "add", "remove", "toString", "hashCode", "equals", "clone", "length", "append", "foo", "bar"};
    for (int i = 0; i < 400; i++) scenario(false, false, false);
    for (int i = 0; i < 300; i++) scenario(false, true, false);
    for (int i = 0; i < 200; i++) scenario(true, false, false);
    for (int i = 0; i < 200; i++) scenario(true, true, false);
    for (int i = 0; i < 60; i++) scenario(false, rnd.nextBoolean(), true);
    for (int i = 0; i < 60; i++) scenario(true, rnd.nextBoolean(), true);
    // String.hashCode vectors
    for (int i = 0; i < 40; i++) { String s = strKey(i % 5); System.out.println("H;" + enc(s) + ";" + s.hashCode()); }
  }
}

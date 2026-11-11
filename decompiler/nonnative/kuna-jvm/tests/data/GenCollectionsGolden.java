// Differential traces for the util.collections ports, recorded on Vineflower 1.12.0's own classes:
//
//   javac -cp vineflower-1.12.0.jar -d GEN_DIR GenCollectionsGolden.java
//   java -cp vineflower-1.12.0.jar:GEN_DIR GenCollectionsGolden > collections_golden.txt
//
// Each scenario starts with a header line ("SPARSE n", "SFORMS", "VB", "FIXED n") and is a
// sequence of operations, one per line; "op args = result" lines carry the observed result.
import org.jetbrains.java.decompiler.util.collections.FastSparseSetFactory;
import org.jetbrains.java.decompiler.util.collections.FastSparseSetFactory.FastSparseSet;
import org.jetbrains.java.decompiler.util.collections.SFormsFastMapDirect;
import org.jetbrains.java.decompiler.util.collections.VBStyleCollection;
import org.jetbrains.java.decompiler.util.collections.fixed.FastFixedSet;
import org.jetbrains.java.decompiler.util.collections.fixed.FastFixedSetFactory;

import java.util.*;
import java.util.stream.Collectors;

public class GenCollectionsGolden {
  static final Random R = new Random(0xC011EC7L);
  static final StringBuilder OUT = new StringBuilder();

  static void line(String s) { OUT.append(s).append('\n'); }

  public static void main(String[] args) {
    int[] sparseSizes = {0, 1, 5, 31, 32, 33, 64, 64, 64, 100};
    for (int n : sparseSizes) for (int rep = 0; rep < 3; rep++) sparse(n);
    for (int rep = 0; rep < 30; rep++) sforms();
    for (int rep = 0; rep < 30; rep++) vb();
    int[] fixedSizes = {0, 1, 10, 63, 64, 65, 128, 130};
    for (int n : fixedSizes) for (int rep = 0; rep < 3; rep++) fixed(n);
    System.out.print(OUT);
  }

  static String list(Iterable<?> it) {
    StringBuilder b = new StringBuilder();
    for (Object o : it) { if (b.length() > 0) b.append(','); b.append(o); }
    return b.length() == 0 ? "-" : b.toString();
  }

  static int b(boolean v) { return v ? 1 : 0; }

  static void sparse(int n) {
    line("SPARSE " + n);
    List<Integer> init = new ArrayList<>();
    for (int i = 0; i < n; i++) init.add(i);
    FastSparseSetFactory<Integer> factory = new FastSparseSetFactory<>(init);
    FastSparseSet<Integer>[] sets = new FastSparseSet[5];
    for (int i = 0; i < sets.length; i++) { sets[i] = factory.createEmptySet(); line("n " + i); }
    int range = n + 40;
    for (int step = 0; step < 220; step++) {
      int s = R.nextInt(sets.length), t = (s + 1 + R.nextInt(sets.length - 1)) % sets.length;
      int v = R.nextInt(range);
      switch (R.nextInt(16)) {
        case 0: case 1: case 2: case 3: sets[s].add(v); line("a " + s + " " + v); break;
        case 4: sets[s].remove(v); line("r " + s + " " + v); break;
        case 5: line("c " + s + " " + v + " = " + b(sets[s].contains(v))); break;
        case 6: sets[t] = sets[s].getCopy(); line("cp " + s + " " + t); break;
        case 7: sets[s].union(sets[t]); line("u " + s + " " + t); break;
        case 8: sets[s].intersection(sets[t]); line("i " + s + " " + t); break;
        case 9: sets[s].complement(sets[t]); line("x " + s + " " + t); break;
        case 10: line("k " + s + " = " + sets[s].getCardinality()); break;
        case 11: line("e " + s + " = " + b(sets[s].isEmpty())); break;
        case 12: line("it " + s + " = " + list(sets[s])); break;
        case 13: line("p " + s + " = " + sets[s].toPlainSet()); break;
        case 14: line("q " + s + " " + t + " = " + b(sets[s].equals(sets[t])) + " " + sets[s].hashCode()); break;
        case 15: sets[s] = factory.createEmptySet(); line("n " + s); break;
      }
    }
    for (int i = 0; i < sets.length; i++) line("it " + i + " = " + list(sets[i]));
  }

  static int sformsKey() {
    switch (R.nextInt(3)) {
      case 0: return R.nextInt(24);
      case 1: return 10000 + R.nextInt(24);
      default: return -1 - R.nextInt(24);
    }
  }

  static void sforms() {
    line("SFORMS");
    List<Integer> init = new ArrayList<>();
    for (int i = 0; i < 64; i++) init.add(i);
    FastSparseSetFactory<Integer> factory = new FastSparseSetFactory<>(init);
    SFormsFastMapDirect[] maps = new SFormsFastMapDirect[4];
    for (int i = 0; i < maps.length; i++) maps[i] = new SFormsFastMapDirect(factory);
    for (int step = 0; step < 220; step++) {
      int m = R.nextInt(maps.length), o = (m + 1 + R.nextInt(maps.length - 1)) % maps.length;
      int key = sformsKey(), ver = 1 + R.nextInt(90);
      switch (R.nextInt(14)) {
        case 0: case 1: case 2: maps[m].setCurrentVar(key, ver); line("set " + m + " " + key + " " + ver); break;
        case 3: case 4: {
          FastSparseSet<Integer> set = maps[m].get(key);
          if (set != null) set.add(ver);
          else { set = factory.createEmptySet(); set.add(ver); maps[m].put(key, set); }
          line("addv " + m + " " + key + " " + ver);
          break;
        }
        case 5: maps[m].remove(key); line("rm " + m + " " + key); break;
        case 6: if (R.nextInt(4) == 0) { maps[m].removeAllFields(); line("rmf " + m); } else { maps[m].removeAllStacks(); line("rms " + m); } break;
        case 7: maps[o] = maps[m].getCopy(); line("cp " + m + " " + o); break;
        case 8: maps[m].union(maps[o]); line("u " + m + " " + o); break;
        case 9: maps[m].intersection(maps[o]); line("i " + m + " " + o); break;
        case 10: maps[m].complement(maps[o]); line("x " + m + " " + o); break;
        case 11: line("s " + m + " = " + maps[m].size() + " " + b(maps[m].isEmpty()) + " " + b(maps[m].containsKey(key)) + " " + key); break;
        case 12: line("str " + m + " = " + maps[m]); break;
        case 13: line("ent " + m + " = " + maps[m].entryList().stream().map(e -> e.getKey() + ":" + list(e.getValue())).collect(Collectors.joining(";"))); break;
      }
    }
    for (int i = 0; i < maps.length; i++) line("str " + i + " = " + maps[i]);
  }

  static void vb() {
    line("VB");
    VBStyleCollection<String, Integer> c = new VBStyleCollection<>();
    int counter = 0;
    for (int step = 0; step < 160; step++) {
      int size = c.size();
      int key = R.nextInt(30);
      String e = "e" + (counter++);
      switch (R.nextInt(15)) {
        case 0: c.add(e); line("add " + e); break;
        case 1: case 2: c.addWithKey(e, key); line("addk " + e + " " + key); break;
        case 3: line("put " + e + " " + key + " = " + c.putWithKey(e, key)); break;
        case 4: { int i = R.nextInt(size + 1); c.add(i, e); line("addat " + i + " " + e); break; }
        case 5: { int i = R.nextInt(size + 1); c.addWithKeyAndIndex(i, e, key); line("addki " + i + " " + e + " " + key); break; }
        case 6: c.removeWithKey(key); line("rmk " + key); break;
        case 7: if (size > 0) { int i = R.nextInt(size); line("rmat " + i + " = " + c.remove(i)); } break;
        case 8: line("getk " + key + " = " + c.getWithKey(key) + " " + b(c.containsKey(key)) + " " + (c.containsKey(key) ? c.getIndexByKey(key) : -1)); break;
        case 9: if (size > 0) { int i = R.nextInt(size); line("getat " + i + " = " + c.get(i) + " " + c.getKey(i)); } break;
        case 10: if (R.nextInt(6) == 0) { c.clear(); line("clear"); } else { c = c.clone(); line("clone"); } break;
        case 11: if (size > 0) { int i = R.nextInt(size); line("setat " + i + " " + e + " = " + c.set(i, e)); } break;
        case 12: { List<String> l = new ArrayList<>(); int k = R.nextInt(3); for (int j = 0; j < k; j++) l.add(e + "_" + j); c.addAll(l); line("addall " + (l.isEmpty() ? "-" : String.join(",", l))); break; }
        case 13: { List<String> l = new ArrayList<>(); List<Integer> ks = new ArrayList<>(); int k = R.nextInt(3); for (int j = 0; j < k; j++) { l.add(e + "_" + j); ks.add(R.nextInt(30)); } c.addAllWithKey(l, ks); line("addallk " + (l.isEmpty() ? "-" : String.join(",", l)) + " " + (ks.isEmpty() ? "-" : list(ks))); break; }
        case 14: { String probe = "e" + R.nextInt(Math.max(counter, 1)); line("vb " + probe + " = " + c.toStringVb() + " " + list(c.getLstKeys()) + " " + c.indexOf(probe)); break; }
      }
    }
    line("vb none = " + c.toStringVb() + " " + list(c.getLstKeys()) + " " + c.indexOf("none"));
    StringBuilder idx = new StringBuilder();
    for (int k = 0; k < 30; k++) idx.append(c.containsKey(k) ? c.getIndexByKey(k) : -1).append(',');
    line("idx = " + idx);
  }

  static void fixed(int n) {
    line("FIXED " + n);
    List<Integer> entries = new ArrayList<>();
    for (int i = 0; i < n; i++) entries.add(i * 3);
    FastFixedSetFactory<Integer> factory = FastFixedSetFactory.create(entries);
    FastFixedSet<Integer>[] sets = new FastFixedSet[4];
    for (int i = 0; i < sets.length; i++) { sets[i] = factory.createEmptySet(); line("n " + i); }
    for (int step = 0; step < (n == 0 ? 20 : 200); step++) {
      int s = R.nextInt(sets.length), t = (s + 1 + R.nextInt(sets.length - 1)) % sets.length;
      int v = n == 0 ? 0 : 3 * R.nextInt(n);
      switch (R.nextInt(17)) {
        case 0: case 1: case 2: if (n > 0) line("a " + s + " " + v + " = " + b(sets[s].add(v))); break;
        case 3: if (n > 0) line("r " + s + " " + v + " = " + b(sets[s].remove(v))); break;
        case 4: if (n > 0) line("c " + s + " " + v + " = " + b(sets[s].contains(v))); break;
        case 5: line("aa " + s + " " + t + " = " + b(sets[s].addAll(sets[t]))); break;
        case 6: line("ra " + s + " " + t + " = " + b(sets[s].retainAll(sets[t]))); break;
        case 7: line("xa " + s + " " + t + " = " + b(sets[s].removeAll(sets[t]))); break;
        case 8: line("ca " + s + " " + t + " = " + b(sets[s].containsAll(sets[t])) + " " + b(sets[s].equals(sets[t]))); break;
        case 9: line("sz " + s + " = " + sets[s].getRealSize() + " " + b(sets[s].isEmpty())); break;
        case 10: line("it " + s + " = " + list(sets[s]) + " " + sets[s] + " " + sets[s].toPlainSet()); break;
        case 11: sets[t] = sets[s].clone(); line("cl " + s + " " + t); break;
        case 12: { int k = 2 + R.nextInt(4); for (Iterator<Integer> it = sets[s].iterator(); it.hasNext(); ) if (it.next() % k == 0) it.remove(); line("ret " + s + " " + k); break; }
        case 13: { List<Integer> l = pick(entries); line("aac " + s + " " + list(l) + " = " + b(sets[s].addAll(l))); break; }
        case 14: { List<Integer> l = pick(entries); line("rac " + s + " " + list(l) + " = " + b(sets[s].retainAll(l))); break; }
        case 15: { List<Integer> l = pick(entries); line("xac " + s + " " + list(l) + " = " + b(sets[s].removeAll(l))); break; }
        case 16: if (R.nextBoolean()) { sets[s] = factory.createCopiedSet(); line("all " + s); } else { sets[s].clear(); line("clr " + s); } break;
      }
    }
    for (int i = 0; i < sets.length; i++) line("it " + i + " = " + list(sets[i]) + " " + sets[i] + " " + sets[i].toPlainSet());
  }

  static List<Integer> pick(List<Integer> entries) {
    List<Integer> l = new ArrayList<>();
    if (entries.isEmpty()) return l;
    int k = R.nextInt(6);
    for (int j = 0; j < k; j++) l.add(entries.get(R.nextInt(entries.size())));
    return l;
  }
}

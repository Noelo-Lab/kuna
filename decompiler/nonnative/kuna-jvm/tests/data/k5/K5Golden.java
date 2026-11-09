// Golden vectors for the K5 slice, computed by the real Vineflower 1.12.0 classes.
//
//   javac -cp vineflower-1.12.0.jar -d out K5Golden.java
//   java -cp vineflower-1.12.0.jar:out K5Golden > k5_golden.txt
//
// Enum hashCode() is identity in Java, so hashes that fold in an enum are
// printed with the enum's identity contribution subtracted; the Rust side
// subtracts the ordinal (its JavaHash) the same way. The two unreachable
// DIRECT nodes of the graph sit in a HashSet<DirectNode> whose order therefore
// depends on DirectNodeType's identity hash in Java; across repeated runs it
// came out as below, which is also the order under the ordinal convention.

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;

import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.collectors.VarNamesCollector;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.VarExprent;
import org.jetbrains.java.decompiler.modules.decompiler.flow.*;
import org.jetbrains.java.decompiler.modules.decompiler.sforms.SFormsConstructor;
import org.jetbrains.java.decompiler.modules.decompiler.stats.DummyExitStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.Statement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionNode;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionsGraph;
import org.jetbrains.java.decompiler.struct.gen.VarType;

public class K5Golden {
  static sun.misc.Unsafe unsafe() throws Exception {
    Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    f.setAccessible(true);
    return (sun.misc.Unsafe) f.get(null);
  }

  static void setField(Object o, Class<?> cls, String name, Object value) throws Exception {
    Field f = cls.getDeclaredField(name);
    f.setAccessible(true);
    f.set(o, value);
  }

  static Statement stat(int id) {
    return new Statement(Statement.StatementType.GENERAL, id) {};
  }

  static Statement dummyExit(int id) throws Exception {
    Statement s = (Statement) unsafe().allocateInstance(DummyExitStatement.class);
    setField(s, Statement.class, "id", id);
    return s;
  }

  static String ids(Collection<DirectNode> nodes) {
    StringBuilder sb = new StringBuilder();
    for (DirectNode n : nodes) {
      if (sb.length() > 0) sb.append(' ');
      sb.append(n.id);
    }
    return sb.toString();
  }

  public static void main(String[] args) throws Exception {
    DecompilerContext ctx = (DecompilerContext) unsafe().allocateInstance(DecompilerContext.class);
    setField(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    DecompilerContext.setCurrentContext(ctx);

    // VarVersionPair hashCode / toString / equals.
    int[][] pairs = {{0, 0}, {1, 1}, {-1, 1}, {10000, 3}, {Integer.MAX_VALUE, 7},
      {Integer.MIN_VALUE, -1}, {123456789, 987654321}, {-77, -2147483647}};
    for (int[] p : pairs) {
      VarVersionPair v = new VarVersionPair(p[0], p[1]);
      System.out.println("vvp " + p[0] + " " + p[1] + " " + v.hashCode() + " " + v
        + " " + v.equals(new VarVersionPair(p[0], p[1])) + " " + v.equals(new VarVersionPair(p[0], p[1] + 1)));
    }

    // HashSet<VarVersionPair> iteration order (the JavaHash feeds the JDK table).
    Set<VarVersionPair> set = new HashSet<>();
    StringBuilder ins = new StringBuilder();
    for (int i = 0; i < 40; i++) {
      int var = (i * 7919) % 53 - 9;
      int ver = (i * 31) % 5;
      set.add(new VarVersionPair(var, ver));
      ins.append(i == 0 ? "" : " ").append(var).append(',').append(ver);
    }
    set.remove(new VarVersionPair(-9, 0));
    System.out.println("vvpset_insert " + ins);
    StringBuilder order = new StringBuilder();
    for (VarVersionPair v : set) order.append(order.length() == 0 ? "" : " ").append(v.var).append(',').append(v.version);
    System.out.println("vvpset_order " + order);

    // VarNamesCollector.
    VarNamesCollector vc = new VarNamesCollector(Arrays.asList("var1", "this", "var1x"));
    StringBuilder names = new StringBuilder();
    names.append(vc.getFreeName(1)).append(' ');
    names.append(vc.getFreeName("this")).append(' ');
    names.append(vc.getFreeName(2)).append(' ');
    names.append(vc.getFreeName("var1")).append(' ');
    names.append(vc.getFreeName("var2"));
    vc.addName("a");
    vc.addNames(Arrays.asList("b", "c", "a"));
    for (int i = 0; i < 30; i++) {
      vc.getFreeName(i % 7);
    }
    System.out.println("vnc_free " + names);
    System.out.println("vnc_used " + String.join(" ", vc.getUsedNames()));

    // DirectNodeType ids and DirectNode hash/equals.
    for (DirectNodeType t : DirectNodeType.values()) {
      for (int sid : new int[] {0, 5, 123, 99999}) {
        DirectNode n = DirectNode.forStat(t, stat(sid), null);
        int h = n.hashCode() - 31 * t.hashCode();
        System.out.println("dnode " + t.ordinal() + " " + sid + " " + n.id + " " + h);
      }
    }
    DirectNode na = DirectNode.forStat(DirectNodeType.TRY, stat(5), null);
    DirectNode nb = DirectNode.forStat(DirectNodeType.TRY, stat(5), null);
    DirectNode nc = DirectNode.forStat(DirectNodeType.CATCH, stat(5), null);
    DirectNode nd = DirectNode.forStat(DirectNodeType.TRY, stat(6), null);
    System.out.println("dnode_eq " + na.equals(nb) + " " + na.equals(nc) + " " + na.equals(nd) + " " + na.equals(na));

    // DirectEdge hash/equals/toString.
    int[][] edges = {{0, 1, 0, 2, 0}, {5, 17, 8, 4, 1}, {11, 99999, 0, 0, 0}, {3, 42, 3, 42, 1}};
    for (int[] e : edges) {
      DirectNodeType st = DirectNodeType.values()[e[0]];
      DirectNodeType dt = DirectNodeType.values()[e[2]];
      DirectEdgeType et = DirectEdgeType.values()[e[4]];
      DirectEdge edge = new DirectEdge(DirectNode.forStat(st, stat(e[1]), null), DirectNode.forStat(dt, stat(e[3]), null), et);
      int h = edge.hashCode() - (29791 * st.hashCode() + 961 * dt.hashCode() + et.hashCode());
      DirectEdge same = new DirectEdge(DirectNode.forStat(st, stat(e[1]), null), DirectNode.forStat(dt, stat(e[3]), null), et);
      DirectEdge other = new DirectEdge(DirectNode.forStat(st, stat(e[1]), null), DirectNode.forStat(dt, stat(e[3]), null),
        DirectEdgeType.values()[1 - e[4]]);
      System.out.println("dedge " + e[0] + " " + e[1] + " " + e[2] + " " + e[3] + " " + e[4] + " " + h
        + " " + edge.equals(same) + " " + edge.equals(other) + " " + edge);
    }

    // DirectGraph: build, sortReversePostOrder, iterateExprents.
    DirectGraph g = new DirectGraph();
    DirectNode[] n = new DirectNode[10];
    for (int i = 1; i <= 8; i++) {
      n[i] = DirectNode.forStat(DirectNodeType.DIRECT, stat(i), null);
    }
    n[9] = DirectNode.forStat(DirectNodeType.DIRECT, dummyExit(9), null);
    for (int i : new int[] {3, 1, 9, 6, 2, 5, 4, 8, 7}) {
      g.nodes.addWithKey(n[i], n[i].id);
    }
    g.first = n[1];
    int[][] reg = {{1, 2}, {1, 3}, {2, 4}, {3, 4}, {4, 2}, {4, 5}, {1, 3}, {5, 7}, {3, 7}};
    for (int[] r : reg) n[r[0]].addSuccessor(DirectEdge.of(n[r[0]], n[r[1]]));
    n[2].addSuccessor(DirectEdge.exception(n[2], n[6]));
    n[8].addSuccessor(DirectEdge.of(n[8], n[7]));
    for (int i = 1; i <= 8; i++) {
      List<DirectEdge> s = n[i].getSuccessors(DirectEdgeType.REGULAR);
      List<DirectEdge> p = n[i].getPredecessors(DirectEdgeType.REGULAR);
      List<DirectNode> ss = new ArrayList<>();
      for (DirectEdge e : s) ss.add(e.getDestination());
      List<DirectNode> pp = new ArrayList<>();
      for (DirectEdge e : p) pp.add(e.getSource());
      System.out.println("dg_adj " + i + " [" + ids(ss) + "] [" + ids(pp) + "] "
        + n[i].hasSuccessors(DirectEdgeType.EXCEPTION) + " " + n[i].hasPredecessors(DirectEdgeType.EXCEPTION));
    }
    g.sortReversePostOrder();
    System.out.println("dg_rpo " + ids(g.nodes));
    System.out.println("dg_extra " + ids(g.extraNodes));

    // exprents: node i holds vars with indices i*10 .. i*10+2
    for (int i = 1; i <= 8; i++) {
      List<Exprent> lst = new ArrayList<>();
      for (int k = 0; k < 3; k++) lst.add(new VarExprent(i * 10 + k, VarType.VARTYPE_INT, null));
      n[i].exprents = lst;
    }
    List<String> seen = new ArrayList<>();
    boolean full = g.iterateExprents(e -> {
      int idx = ((VarExprent) e).getIndex();
      seen.add(String.valueOf(idx));
      return idx % 10 == 1 ? 2 : 0;
    });
    System.out.println("dg_iter " + full + " " + String.join(" ", seen));
    StringBuilder left = new StringBuilder();
    for (int i = 1; i <= 8; i++) {
      left.append(i == 1 ? "" : " ").append(n[i].exprents.size());
    }
    System.out.println("dg_left " + left);
    List<String> seen2 = new ArrayList<>();
    boolean full2 = g.iterateExprents(e -> {
      int idx = ((VarExprent) e).getIndex();
      seen2.add(String.valueOf(idx));
      return idx == 40 ? 1 : 0;
    });
    System.out.println("dg_cancel " + full2 + " " + String.join(" ", seen2));

    // VarVersionsGraph helpers (single-root orders are deterministic in Java too).
    VarVersionsGraph vg = new VarVersionsGraph();
    Map<String, VarVersionNode> vn = new LinkedHashMap<>();
    int[][] vpairs = {{1, 1}, {1, 2}, {1, 3}, {1, 4}, {1, 5}, {2, 1}, {2, 2}, {2, 3}, {2, 4}, {2, 5}};
    for (int[] p : vpairs) vn.put(p[0] + "_" + p[1], vg.createNode(new VarVersionPair(p[0], p[1])));
    // var 1: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 4, 4 -> 5 ; var 2 mirrors it
    int[][] vedges = {{1, 2}, {1, 3}, {2, 4}, {3, 4}, {4, 5}};
    for (int var = 1; var <= 2; var++) {
      for (int[] e : vedges) {
        SFormsConstructor.makeReadEdge(vn.get(var + "_" + e[1]), vn.get(var + "_" + e[0]));
      }
    }
    Method rpo = VarVersionsGraph.class.getDeclaredMethod("getReversedPostOrder", Collection.class);
    rpo.setAccessible(true);
    @SuppressWarnings("unchecked")
    List<VarVersionNode> order1 = (List<VarVersionNode>) rpo.invoke(null, List.of(vn.get("1_1")));
    StringBuilder o1 = new StringBuilder();
    for (VarVersionNode x : order1) o1.append(o1.length() == 0 ? "" : " ").append(x);
    System.out.println("vg_rpo " + o1);
    System.out.println("vg_analog " + vg.areVarsAnalogous(1, 2));
    vg.createNode(new VarVersionPair(3, 1));
    vg.createNode(new VarVersionPair(3, 2));
    SFormsConstructor.makeReadEdge(vn.get("1_2"), vg.nodes.getWithKey(new VarVersionPair(3, 1)));
    System.out.println("vg_analog2 " + vg.areVarsAnalogous(1, 3) + " " + vg.areVarsAnalogous(1, 2));
    Set<VarVersionNode> dom = new HashSet<>(List.of(vn.get("1_2"), vn.get("1_3")));
    Set<VarVersionNode> dom2 = new HashSet<>(List.of(vn.get("1_2"), vn.get("2_3")));
    System.out.println("vg_domset " + vg.isDominatorSet(vn.get("1_5"), dom) + " " + vg.isDominatorSet(vn.get("1_5"), dom2)
      + " " + vg.isDominatorSet(vn.get("1_2"), dom));
    System.out.println("vg_node " + vn.get("1_4") + " " + vn.get("1_4").hasSinglePredecessor() + " "
      + vn.get("1_5").hasSinglePredecessor() + " " + vn.get("1_5").getSinglePredecessor() + " " + vn.get("1_5").hasAnySuccessors());
  }
}

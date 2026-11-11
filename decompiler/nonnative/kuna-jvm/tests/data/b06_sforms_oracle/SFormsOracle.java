// Oracle for tests/b06_sforms.rs: runs the real Vineflower 1.12.0
// FlattenStatementsHelper, SSAConstructorSparseEx and SSAUConstructorSparseEx on
// hand-built statement graphs decomposed by DomHelper.processStatement.
//
//   javac -cp vineflower-1.12.0.jar -d out SFormsOracle.java
//   java -cp vineflower-1.12.0.jar:out \
//     org.jetbrains.java.decompiler.modules.decompiler.decompose.SFormsOracle > expected.txt
package org.jetbrains.java.decompiler.modules.decompiler.decompose;

import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.flow.*;
import org.jetbrains.java.decompiler.modules.decompiler.sforms.SSAConstructorSparseEx;
import org.jetbrains.java.decompiler.modules.decompiler.sforms.SSAUConstructorSparseEx;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionNode;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionsGraph;
import org.jetbrains.java.decompiler.struct.StructMember;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.collections.FastSparseSetFactory.FastSparseSet;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;

public class SFormsOracle {
  interface Scenario {
    void build() throws Exception;
  }

  interface Customize {
    void apply(RootStatement root) throws Exception;
  }

  static Statement first;
  static GeneralStatement general;
  static DummyExitStatement exit;
  static List<Statement> nodes;
  static List<VarExprent> vars;
  static StructMethod mt;

  static void reset() throws Exception {
    Field unsafeField = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    unsafeField.setAccessible(true);
    sun.misc.Unsafe unsafe = (sun.misc.Unsafe) unsafeField.get(null);
    DecompilerContext ctx = (DecompilerContext) unsafe.allocateInstance(DecompilerContext.class);
    Field cc = DecompilerContext.class.getDeclaredField("counterContainer");
    cc.setAccessible(true);
    CounterContainer counters = new CounterContainer();
    counters.setCounter(CounterContainer.VAR_COUNTER, 3);
    cc.set(ctx, counters);
    Field props = DecompilerContext.class.getDeclaredField("properties");
    props.setAccessible(true);
    props.set(ctx, new HashMap<String, Object>());
    DecompilerContext.setCurrentContext(ctx);

    // public int straight(int a, int b)
    mt = (StructMethod) unsafe.allocateInstance(StructMethod.class);
    Field flags = StructMember.class.getDeclaredField("accessFlags");
    flags.setAccessible(true);
    flags.set(mt, 1);
    Field desc = StructMethod.class.getDeclaredField("descriptor");
    desc.setAccessible(true);
    desc.set(mt, "(II)I");
    vars = new ArrayList<>();
  }

  static void graph(int n) {
    nodes = new ArrayList<>();
    for (int i = 0; i < n; i++) {
      nodes.add(BasicBlockStatement.create());
    }
    first = nodes.get(0);
    exit = new DummyExitStatement();
    general = new GeneralStatement(first, nodes, null);
  }

  static void link(int from, int to) {
    Statement stat = nodes.get(from);
    Statement dest = to < 0 ? exit : nodes.get(to);
    int type;
    if (dest == first) {
      type = StatEdge.TYPE_CONTINUE;
      dest = general;
    } else if (to < 0) {
      type = StatEdge.TYPE_BREAK;
    } else {
      type = StatEdge.TYPE_REGULAR;
    }
    stat.addSuccessor(new StatEdge(type, stat, dest, type == StatEdge.TYPE_REGULAR ? null : general));
  }

  static void linkException(int from, int to, String type) {
    Statement stat = nodes.get(from);
    stat.addSuccessor(new StatEdge(stat, nodes.get(to), type == null ? null : List.of(type)));
  }

  static void setIf(int node) throws Exception {
    Field f = Statement.class.getDeclaredField("lastBasicType");
    f.setAccessible(true);
    f.set(nodes.get(node), Statement.LastBasicType.IF);
  }

  static VarExprent var(int index) {
    VarExprent v = new VarExprent(index, VarType.VARTYPE_INT, null);
    vars.add(v);
    return v;
  }

  static void add(int block, Exprent e) {
    Statement st = nodes.get(block);
    if (st.getExprents() == null) {
      st.setExprents(new ArrayList<>());
    }
    st.getExprents().add(e);
  }

  // block: dst = value
  static void write(int block, int dst, int value) {
    VarExprent left = var(dst);
    ConstExprent right = new ConstExprent(value, false, null);
    add(block, new AssignmentExprent(left, right, null));
  }

  // block: dst = src
  static void copy(int block, int dst, int src) {
    VarExprent left = var(dst);
    VarExprent right = var(src);
    add(block, new AssignmentExprent(left, right, null));
  }

  static Exprent assign(int dst, int src) {
    VarExprent left = var(dst);
    VarExprent right = var(src);
    return new AssignmentExprent(left, right, null);
  }

  static void p(String key, Object value) {
    System.out.println(key + " = " + value);
  }

  static String err(Throwable e) {
    if (e instanceof java.lang.reflect.InvocationTargetException) {
      e = e.getCause();
    }
    return e.getClass().getSimpleName() + ": " + e.getMessage();
  }

  static RootStatement process(String key) throws Exception {
    general.buildContinueSet();
    general.buildMonitorFlags();
    RootStatement root = new RootStatement(general, exit, mt);
    Object tracer = new DomTracer("domhelper_0", null);
    Method m = DomHelper.class.getDeclaredMethod("processStatement", Statement.class, RootStatement.class, HashMap.class, DomTracer.class);
    m.setAccessible(true);
    boolean ok = (Boolean) m.invoke(null, root, root, new LinkedHashMap<>(), tracer);
    p(key + ".ok", ok);
    return root;
  }

  static DoStatement findDo(Statement stat) {
    if (stat instanceof DoStatement) {
      return (DoStatement) stat;
    }
    for (Statement st : stat.getStats()) {
      DoStatement d = findDo(st);
      if (d != null) {
        return d;
      }
    }
    return null;
  }

  static CatchAllStatement findCatchAll(Statement stat) {
    if (stat instanceof CatchAllStatement) {
      return (CatchAllStatement) stat;
    }
    for (Statement st : stat.getStats()) {
      CatchAllStatement d = findCatchAll(st);
      if (d != null) {
        return d;
      }
    }
    return null;
  }

  static List<IfStatement> findIfs(Statement stat, List<IfStatement> out) {
    if (stat instanceof IfStatement) {
      out.add((IfStatement) stat);
    }
    for (Statement st : stat.getStats()) {
      findIfs(st, out);
    }
    return out;
  }

  // every if statement gets a head exprent reading v5
  static final Customize TAIL = root -> {
    for (IfStatement ifs : findIfs(root, new ArrayList<>())) {
      ifs.getHeadexprentList().set(0, var(5));
    }
  };

  static String ids(List<DirectEdge> edges, boolean dest) {
    List<String> out = new ArrayList<>();
    for (DirectEdge e : edges) {
      out.add((dest ? e.getDestination() : e.getSource()).id);
    }
    return out.toString();
  }

  static String nodeId(DirectNode node) {
    return node == null ? "null" : node.id;
  }

  static void dumpGraph(String key, DirectGraph g) {
    p(key + ".flat.first", g.first.id);
    for (DirectNode node : g.nodes) {
      p(key + ".flat.node", node.id + " " + node.type.name() + " stat=" + node.statement.id
        + " block=" + (node.block == null ? "null" : node.block.id)
        + " tf=" + nodeId(node.tryFinally)
        + " ex=" + node.exprents.stream().filter(Objects::nonNull).count()
        + " succ=" + ids(node.getSuccessors(DirectEdgeType.REGULAR), true)
        + " esucc=" + ids(node.getSuccessors(DirectEdgeType.EXCEPTION), true)
        + " pred=" + ids(node.getPredecessors(DirectEdgeType.REGULAR), false)
        + " epred=" + ids(node.getPredecessors(DirectEdgeType.EXCEPTION), false));
    }
    List<String> extra = new ArrayList<>();
    for (DirectNode node : g.extraNodes) {
      extra.add(node.id);
    }
    p(key + ".flat.extra", extra);
    p(key + ".flat.neg", g.mapNegIfBranch);
  }

  static String pair(VarVersionPair p) {
    return p.var + "_" + p.version;
  }

  static String set(FastSparseSet<Integer> s) {
    List<Integer> out = new ArrayList<>();
    for (Integer i : s) {
      out.add(i);
    }
    return out.toString();
  }

  static String vars() {
    List<String> out = new ArrayList<>();
    for (VarExprent v : vars) {
      out.add(v.getIndex() + "_" + v.getVersion());
    }
    return out.toString();
  }

  static void run(String key, Scenario scenario, Customize customize) throws Exception {
    run(key, scenario, customize, true);
  }

  // ssa = false where Java's null head slot (an empty list in kuna) would reach processSforms.
  static void run(String key, Scenario scenario, Customize customize, boolean withSsa) throws Exception {
    reset();
    scenario.build();
    RootStatement root = process(key);
    customize.apply(root);
    try {
      dumpGraph(key, new FlattenStatementsHelper().buildDirectGraph(root));
    } catch (Throwable e) {
      p(key + ".flat.error", err(e));
    }
    if (!withSsa) {
      return;
    }

    try {
      SSAConstructorSparseEx ssa = new SSAConstructorSparseEx();
      ssa.splitVariables(root, mt);
      p(key + ".ssa.vars", vars());
      List<String> phi = new ArrayList<>();
      for (Map.Entry<VarVersionPair, FastSparseSet<Integer>> e : ssa.getPhi().entrySet()) {
        phi.add(pair(e.getKey()) + "=" + set(e.getValue()));
      }
      p(key + ".ssa.phi", phi);
      List<String> lookup = new ArrayList<>();
      for (Map.Entry<VarVersionPair, Integer> e : ssa.getSimpleReversePhiLookup().entrySet()) {
        lookup.add(pair(e.getKey()) + "=" + e.getValue());
      }
      p(key + ".ssa.lookup", lookup);
      p(key + ".ssa.reads", ssa.getReadCount(new VarVersionPair(5, 1)) + "," + ssa.getReadCount(new VarVersionPair(1, 1)));
    } catch (Throwable e) {
      p(key + ".ssa.error", err(e));
    }

    reset();
    scenario.build();
    root = process(key + ".u");
    customize.apply(root);
    try {
      SSAUConstructorSparseEx ssau = new SSAUConstructorSparseEx();
      ssau.splitVariables(root, mt);
      p(key + ".ssau.vars", vars());
      VarVersionsGraph g = ssau.getSsuVersions();
      List<VarVersionNode> all = new ArrayList<>(g.nodes);
      for (VarVersionNode node : all) {
        List<String> preds = new ArrayList<>();
        for (VarVersionNode n : node.predecessors) {
          preds.add(pair(n.asPair()));
        }
        List<String> succs = new ArrayList<>();
        for (VarVersionNode n : node.successors) {
          succs.add(pair(n.asPair()));
        }
        StringBuilder doms = new StringBuilder();
        for (VarVersionNode other : all) {
          try {
            doms.append(g.isDominatorSet(node, Set.of(other)) ? '1' : '0');
          } catch (RuntimeException e) {
            doms.append('E');
          }
        }
        p(key + ".ssau.node", pair(node.asPair()) + " " + node.state + " preds=" + preds + " succs=" + succs
          + " live=" + node.live + " doms=" + doms);
      }
      List<String> ranges = new ArrayList<>();
      for (Map.Entry<VarVersionPair, Integer> e : ssau.getMapVersionFirstRange().entrySet()) {
        ranges.add(pair(e.getKey()) + "=" + e.getValue());
      }
      p(key + ".ssau.ranges", ranges);
      List<String> assigns = new ArrayList<>();
      for (Map.Entry<VarVersionPair, VarVersionPair> e : ssau.getVarAssignmentMap().entrySet()) {
        assigns.add(pair(e.getKey()) + "=" + pair(e.getValue()));
      }
      p(key + ".ssau.assign", assigns);
    } catch (Throwable e) {
      p(key + ".ssau.error", err(e));
    }
  }

  static final Customize NONE = root -> { };

  static Customize loop(DoStatement.Type type, boolean init, boolean cond, boolean inc) {
    return root -> {
      DoStatement d = findDo(root);
      d.setLooptype(type);
      if (type == DoStatement.Type.FOR_EACH) {
        d.setInitExprent(var(9));
        d.setIncExprent(var(5));
        return;
      }
      if (init) {
        d.setInitExprent(assign(9, 5));
      }
      if (cond) {
        d.setConditionExprent(var(5));
      }
      if (inc) {
        d.setIncExprent(assign(9, 5));
      }
    };
  }

  public static void main(String[] args) throws Exception {
    Scenario ifelse = () -> {
      graph(4);
      setIf(0);
      link(0, 1); link(0, 2); link(1, 3); link(2, 3); link(3, -1);
      write(0, 5, 1); write(1, 5, 2); copy(2, 6, 5); copy(3, 7, 5); copy(3, 8, 1);
    };
    run("ifelse", ifelse, NONE);
    run("ifelse_tail", ifelse, TAIL);

    Scenario seq3 = () -> {
      graph(3);
      link(0, 1); link(1, 2); link(2, -1);
      write(0, 5, 1); copy(1, 5, 5); copy(2, 6, 5); copy(2, 2, 6);
    };
    run("seq3", seq3, NONE);

    Scenario loop2 = () -> {
      graph(2);
      setIf(1);
      link(0, 1); link(1, 0); link(1, -1);
      copy(0, 5, 1); copy(1, 1, 5);
    };
    run("loop2", loop2, NONE);

    Scenario ifbreak = () -> {
      graph(3);
      setIf(0);
      link(0, 1); link(0, -1); link(1, 2); link(2, -1);
      write(0, 5, 1); write(1, 5, 2); copy(2, 6, 5);
    };
    run("ifbreak", ifbreak, NONE);

    Scenario inner = () -> {
      graph(4);
      setIf(2);
      link(0, 1); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
      write(0, 5, 0); copy(1, 6, 5); copy(2, 5, 6); copy(3, 7, 5);
    };
    run("inner", inner, NONE);

    Scenario exitloop = () -> {
      graph(4);
      setIf(1);
      link(0, 1); link(1, 2); link(1, 3); link(2, 1); link(3, -1);
      write(0, 5, 0); copy(1, 6, 5); copy(2, 5, 6); copy(3, 7, 5);
    };
    run("exitloop", exitloop, NONE);

    Scenario loopif = () -> {
      graph(4);
      setIf(0);
      setIf(2);
      link(0, 1); link(0, 3); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
      write(0, 5, 0); copy(1, 6, 5); copy(2, 5, 6); copy(3, 7, 5);
    };
    run("loopif", loopif, NONE);
    run("loopif_while", loopif, loop(DoStatement.Type.WHILE, false, true, false));
    run("loopif_while_empty", loopif, loop(DoStatement.Type.WHILE, false, false, false), false);
    run("loopif_dowhile", loopif, loop(DoStatement.Type.DO_WHILE, false, true, false));
    run("loopif_for", loopif, loop(DoStatement.Type.FOR, true, true, true));
    run("loopif_for_noinit", loopif, loop(DoStatement.Type.FOR, false, true, true));
    run("loopif_foreach", loopif, loop(DoStatement.Type.FOR_EACH, false, false, false));
    run("loopif_tail", loopif, TAIL);

    Scenario trycatch = () -> {
      graph(3);
      link(0, 2); linkException(0, 1, "java/lang/Exception"); link(1, 2); link(2, -1);
      write(0, 5, 1); write(0, 5, 2); copy(1, 6, 3); copy(2, 7, 5);
    };
    run("trycatch", trycatch, NONE);

    Scenario trycatch2 = () -> {
      graph(4);
      link(0, 1); link(1, 3); linkException(1, 2, "java/lang/Exception"); link(2, 3); link(3, -1);
      write(0, 5, 1); write(1, 5, 2); write(1, 5, 3); copy(2, 6, 5); copy(2, 7, 3); copy(3, 8, 5);
    };
    run("trycatch2", trycatch2, NONE);

    Scenario catchall = () -> {
      graph(4);
      link(0, 1); link(1, 3); linkException(1, 2, null); link(2, -1); link(3, -1);
      write(0, 5, 1); write(1, 5, 2); copy(2, 6, 5); copy(3, 7, 5);
    };
    run("catchall", catchall, NONE);
    run("finally", catchall, root -> findCatchAll(root).setFinally(true));

    Scenario loop3 = () -> {
      graph(5);
      setIf(2);
      link(0, 1); link(1, 2); link(2, 3); link(3, 1); link(2, 4); link(4, -1);
      write(0, 5, 0); copy(1, 6, 5); copy(2, 7, 6); copy(3, 5, 7); copy(4, 8, 5);
    };
    run("loop3", loop3, NONE);

    Scenario tryloop = () -> {
      graph(5);
      setIf(0);
      setIf(3);
      link(0, 1); link(0, 4); link(1, 3); linkException(1, 2, "java/lang/Exception"); link(2, 3);
      link(3, 1); link(3, 4); link(4, -1);
      write(0, 5, 0); copy(1, 5, 5); copy(2, 6, 5); copy(2, 7, 3); copy(3, 8, 5); copy(4, 9, 5);
    };
    run("tryloop", tryloop, NONE);
  }
}

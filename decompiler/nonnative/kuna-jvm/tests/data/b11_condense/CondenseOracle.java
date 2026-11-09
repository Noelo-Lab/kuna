// Oracle for tests/b11_condense.rs: runs the real Vineflower 1.12.0
// DomHelper.processStatement on hand-built statement graphs, then
// SequenceHelper.condenseSequences, EliminateLoopsHelper's loop elimination
// (eliminateLoopsRec + condenseSequences, the part of eliminateLoops before
// SimplifyExprentsHelper) and SequenceHelper.destroyAndFlattenStatement.
//
//   ./gen.sh   (JDK 21 javac/java on PATH, VF_JAR = the Vineflower 1.12.0 jar)
package org.jetbrains.java.decompiler.modules.decompiler.decompose;

import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.decompiler.EliminateLoopsHelper;
import org.jetbrains.java.decompiler.modules.decompiler.SequenceHelper;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;

public class CondenseOracle {
  static final int NULL = 0, EMPTY = 1, CONST = 2;

  static Statement first;
  static GeneralStatement general;
  static DummyExitStatement exit;
  static List<Statement> nodes;
  static RootStatement root;

  static void reset() throws Exception {
    Field unsafeField = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    unsafeField.setAccessible(true);
    sun.misc.Unsafe unsafe = (sun.misc.Unsafe) unsafeField.get(null);
    DecompilerContext ctx = (DecompilerContext) unsafe.allocateInstance(DecompilerContext.class);
    Field cc = DecompilerContext.class.getDeclaredField("counterContainer");
    cc.setAccessible(true);
    cc.set(ctx, new CounterContainer());
    Field props = DecompilerContext.class.getDeclaredField("properties");
    props.setAccessible(true);
    props.set(ctx, new HashMap<String, Object>());
    DecompilerContext.setCurrentContext(ctx);
  }

  // n basic blocks (exprents null, empty, or one int constant each), a dummy exit
  // and the general statement over them, in that order.
  static void graph(int n, int mode) throws Exception {
    reset();
    nodes = new ArrayList<>();
    for (int i = 0; i < n; i++) {
      if (mode == NULL) {
        int id = DecompilerContext.getCounterContainer().getCounterAndIncrement(CounterContainer.STATEMENT_COUNTER);
        nodes.add(new BasicBlockStatement(new BasicBlock(id)));
      } else {
        BasicBlockStatement bb = BasicBlockStatement.create();
        if (mode == CONST) {
          bb.getExprents().add(new ConstExprent(i, false, null));
        }
        nodes.add(bb);
      }
    }
    first = nodes.get(0);
    exit = new DummyExitStatement();
    general = new GeneralStatement(first, nodes, null);
  }

  // graphToStatement's edge rules; to == -1 is the method exit.
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
    stat.addSuccessor(new StatEdge(stat, nodes.get(to), List.of(type)));
  }

  static void setIf(int node) throws Exception {
    Field f = Statement.class.getDeclaredField("lastBasicType");
    f.setAccessible(true);
    f.set(nodes.get(node), Statement.LastBasicType.IF);
  }

  static void p(String key, Object value) {
    System.out.println(key + " = " + value);
  }

  static String sorted(Collection<?> items) {
    List<String> lst = new ArrayList<>();
    for (Object o : items) {
      lst.add(String.valueOf(o));
    }
    Collections.sort(lst);
    return lst.toString();
  }

  static String exprents(Statement s) {
    if (s.getExprents() == null) {
      return "null";
    }
    List<String> ids = new ArrayList<>();
    for (Exprent e : s.getExprents()) {
      ids.add(Integer.toString(e.id));
    }
    return ids.toString();
  }

  static void dump(String key, Statement s, String indent) {
    StringBuilder sb = new StringBuilder();
    sb.append(indent).append(s);
    sb.append(" parent=").append(s.getParent());
    sb.append(" first=").append(s.getFirst());
    sb.append(" succs=").append(s.getAllSuccessorEdges());
    sb.append(" preds=").append(s.getAllPredecessorEdges());
    sb.append(" labels=").append(sorted(s.getLabelEdges()));
    sb.append(" exprents=").append(exprents(s));
    if (s instanceof IfStatement ifs) {
      sb.append(" iftype=").append(ifs.iftype).append(" negated=").append(ifs.isNegated())
        .append(" ifstat=").append(ifs.getIfstat()).append(" elsestat=").append(ifs.getElsestat());
    } else if (s instanceof DoStatement dos) {
      sb.append(" looptype=").append(dos.getLooptype().ordinal());
    } else if (s instanceof CatchStatement cs) {
      sb.append(" exct=").append(cs.getExctStrings());
    } else if (s instanceof GeneralStatement gs) {
      sb.append(" placeholder=").append(gs.isPlaceholder());
    }
    System.out.println(key + " = " + sb);
    for (Statement st : s.getStats()) {
      dump(key, st, indent + "  ");
    }
  }

  static boolean process(String key) throws Exception {
    general.buildContinueSet();
    general.buildMonitorFlags();
    root = new RootStatement(general, exit, null);
    Method m = DomHelper.class.getDeclaredMethod("processStatement", Statement.class, RootStatement.class, HashMap.class, DomTracer.class);
    m.setAccessible(true);
    boolean ok = (Boolean) m.invoke(null, root, root, new LinkedHashMap<>(), new DomTracer("domhelper_0", null));
    p(key + ".ok", ok);
    return ok;
  }

  static void condense(String key) {
    try {
      SequenceHelper.condenseSequences(root);
      p(key + ".condensed", true);
    } catch (Throwable t) {
      p(key + ".error", true);
    }
    dump(key, root, "");
  }

  static void eliminate(String key) throws Exception {
    Method m = EliminateLoopsHelper.class.getDeclaredMethod("eliminateLoopsRec", Statement.class);
    m.setAccessible(true);
    boolean ret = (Boolean) m.invoke(null, root);
    p(key + ".eliminated", ret);
    if (ret) {
      SequenceHelper.condenseSequences(root);
      dump(key, root, "");
    }
  }

  static Statement find(Statement s, Class<?> type) {
    if (type.isInstance(s)) {
      return s;
    }
    for (Statement st : s.getStats()) {
      Statement found = find(st, type);
      if (found != null) {
        return found;
      }
    }
    return null;
  }

  static void flatten(String key, Class<?> type) {
    Statement target = find(root, type);
    p(key + ".target", target);
    BasicBlockStatement bstat = SequenceHelper.destroyAndFlattenStatement(target);
    p(key + ".bstat", bstat);
    dump(key, root, "");
  }

  static void shapes(String prefix, int mode) throws Exception {
    graph(3, mode);
    link(0, 1); link(1, 2); link(2, -1);
    if (process(prefix + "seq3")) condense(prefix + "seq3");

    graph(5, mode);
    link(0, 1); link(1, 2); link(2, 3); link(3, 4); link(4, -1);
    if (process(prefix + "seq5")) condense(prefix + "seq5");

    graph(4, mode);
    setIf(0);
    link(0, 1); link(0, 2); link(1, 3); link(2, 3); link(3, -1);
    if (process(prefix + "ifelse")) condense(prefix + "ifelse");

    graph(2, mode);
    setIf(1);
    link(0, 1); link(1, 0); link(1, -1);
    if (process(prefix + "loop2")) condense(prefix + "loop2");

    graph(3, mode);
    setIf(0);
    link(0, 1); link(0, -1); link(1, 2); link(2, -1);
    if (process(prefix + "ifbreak")) condense(prefix + "ifbreak");

    graph(4, mode);
    setIf(2);
    link(0, 1); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
    if (process(prefix + "inner")) condense(prefix + "inner");

    graph(3, mode);
    link(0, 2); linkException(0, 1, "java/lang/Exception"); link(1, 2); link(2, -1);
    if (process(prefix + "trycatch")) condense(prefix + "trycatch");

    graph(5, mode);
    setIf(2);
    link(0, 1); link(1, 2); link(2, 3); link(3, 1); link(2, 4); link(4, -1);
    if (process(prefix + "loop3")) condense(prefix + "loop3");

    graph(6, mode);
    setIf(1); setIf(3);
    link(0, 1); link(1, 2); link(1, 4); link(2, 3); link(3, 1); link(3, 5); link(4, 5); link(5, -1);
    if (process(prefix + "loopexits")) condense(prefix + "loopexits");
  }

  public static void main(String[] args) throws Exception {
    shapes("null.", NULL);
    shapes("empty.", EMPTY);
    shapes("const.", CONST);

    graph(4, NULL);
    setIf(2);
    link(0, 1); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
    if (process("elim.inner")) {
      condense("elim.inner");
      eliminate("elim.inner");
    }

    graph(5, NULL);
    setIf(1); setIf(3);
    link(0, 1); link(1, 2); link(1, 4); link(2, 3); link(3, 1); link(3, 0); link(4, -1);
    if (process("elim.nested")) {
      condense("elim.nested");
      eliminate("elim.nested");
    }

    graph(4, NULL);
    setIf(1); setIf(2);
    link(0, 1); link(1, 2); link(1, 3); link(2, 1); link(2, 0); link(3, -1);
    if (process("elim.twoback")) {
      condense("elim.twoback");
      eliminate("elim.twoback");
    }

    graph(4, NULL);
    setIf(0); setIf(1);
    link(0, 1); link(0, 3); link(1, 3); link(1, 2); link(2, 1); link(3, 1);
    if (process("elim.reject")) {
      condense("elim.reject");
      eliminate("elim.reject");
    }

    graph(6, NULL);
    setIf(0); setIf(1); setIf(2); setIf(3); setIf(4);
    link(0, 1); link(0, 2); link(1, 4); link(1, 3); link(2, 5); link(2, 4); link(3, 0); link(3, 1);
    link(4, 5); link(4, 0); link(5, -1);
    if (process("elim.found6")) {
      condense("elim.found6");
      eliminate("elim.found6");
    }

    graph(8, NULL);
    setIf(0); setIf(1); setIf(2); setIf(4); setIf(6);
    link(0, 1); link(0, 5); link(1, 4); link(1, 3); link(2, 5); link(2, 0); link(3, 1); link(4, 7);
    link(4, 2); link(5, 6); link(6, 7); link(6, 0);
    if (process("elim.found8")) {
      condense("elim.found8");
      eliminate("elim.found8");
    }

    graph(8, NULL);
    setIf(0); setIf(1); setIf(3); setIf(4); setIf(5); setIf(6);
    link(0, 1); link(0, 3); link(1, 6); link(1, 5); link(2, 7); link(3, 0); link(3, 4); link(4, 6); link(4, 0); link(5, 7); link(5, 1); link(6, 2); link(6, 3); link(7, 2);
    if (process("elim.breaks8a")) {
      condense("elim.breaks8a");
      eliminate("elim.breaks8a");
    }

    graph(8, NULL);
    setIf(0); setIf(1); setIf(2); setIf(3); setIf(4); setIf(5); setIf(6);
    link(0, 2); link(0, 5); link(1, 4); link(1, 6); link(2, 7); link(2, 0); link(3, -1); link(3, 1); link(4, 0); link(4, 5); link(5, 3); link(5, -1); link(6, 2); link(6, 7); link(7, -1);
    if (process("elim.breaks8b")) {
      condense("elim.breaks8b");
      eliminate("elim.breaks8b");
    }

    graph(5, NULL);
    setIf(0); setIf(1); setIf(3);
    link(0, 1); link(0, 4); link(1, 4); link(1, 3); link(2, 0); link(3, 1); link(3, 0); link(4, 2);
    if (process("elim.breakreject5")) {
      condense("elim.breakreject5");
      eliminate("elim.breakreject5");
    }

    graph(7, NULL);
    setIf(3); setIf(4); setIf(5);
    link(0, 2); link(1, 2); link(2, 3); link(3, 4); link(3, -1); link(4, 2); link(4, 6); link(5, 6); link(5, 1); link(6, 5);
    if (process("elim.breakreject7")) {
      condense("elim.breakreject7");
      eliminate("elim.breakreject7");
    }

    graph(4, CONST);
    setIf(0);
    link(0, 1); link(0, 2); link(1, 3); link(2, 3); link(3, -1);
    if (process("flat.if")) {
      flatten("flat.if", IfStatement.class);
    }

    graph(3, CONST);
    link(0, 1); link(1, 2); link(2, -1);
    if (process("flat.block")) {
      condense("flat.block");
      flatten("flat.block", BasicBlockStatement.class);
    }
  }
}

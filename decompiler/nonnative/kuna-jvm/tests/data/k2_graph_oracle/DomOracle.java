// Oracle for tests/k2_dom_helper.rs: runs the real Vineflower 1.12.0
// FastExtendedPostdominanceHelper, DomHelper.calcPostDominators and
// DomHelper.processStatement on hand-built statement graphs.
//
//   javac -cp vineflower-1.12.0.jar -d out GraphOracle.java DomOracle.java
//   java -cp vineflower-1.12.0.jar:out \
//     org.jetbrains.java.decompiler.modules.decompiler.decompose.DomOracle > dom_expected.txt
package org.jetbrains.java.decompiler.modules.decompiler.decompose;

import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.stats.*;
import org.jetbrains.java.decompiler.util.collections.VBStyleCollection;

import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.*;

public class DomOracle {
  static Statement first;
  static GeneralStatement general;
  static DummyExitStatement exit;
  static List<Statement> nodes;

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

  // n basic blocks, a dummy exit and the general statement over them, in that order.
  static void graph(int n) {
    nodes = new ArrayList<>();
    for (int i = 0; i < n; i++) {
      nodes.add(BasicBlockStatement.create());
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

  static void dump(String key, Statement s, String indent) {
    StringBuilder sb = new StringBuilder();
    sb.append(indent).append(s);
    sb.append(" first=").append(s.getFirst());
    sb.append(" succs=").append(s.getAllSuccessorEdges());
    sb.append(" labels=").append(sorted(s.getLabelEdges()));
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

  static void process(String key) throws Exception {
    general.buildContinueSet();
    general.buildMonitorFlags();
    RootStatement root = new RootStatement(general, exit, null);
    Class<?> tracerClass = DomTracer.class;
    Object tracer = new DomTracer("domhelper_0", null);
    Method m = DomHelper.class.getDeclaredMethod("processStatement", Statement.class, RootStatement.class, HashMap.class, tracerClass);
    m.setAccessible(true);
    try {
      boolean ok = (Boolean) m.invoke(null, root, root, new LinkedHashMap<>(), tracer);
      p(key + ".ok", ok);
    } catch (java.lang.reflect.InvocationTargetException e) {
      p(key + ".error", e.getCause());
    }
    p(key + ".comments", root.commentLines);
    dump(key, root, "");
  }

  static void postdoms(String key) {
    p(key + ".ext", new FastExtendedPostdominanceHelper().getExtendedPostdominators(general));
    p(key + ".post", DomHelper.calcPostDominators(general).toStringVb());
  }

  public static void main(String[] args) throws Exception {
    reset();
    graph(4);
    setIf(0);
    link(0, 1); link(0, 2); link(1, 3); link(2, 3); link(3, -1);
    postdoms("diamond");
    process("ifelse");

    reset();
    graph(4);
    link(0, 1); link(1, 2); link(2, 1); link(1, 3); link(3, -1);
    postdoms("loop");

    reset();
    graph(3);
    link(0, 2); linkException(0, 1, "java/lang/Exception"); link(1, 2); link(2, -1);
    postdoms("handler");

    reset();
    graph(3);
    link(0, 1); link(1, 2); link(2, -1);
    process("seq3");

    reset();
    graph(2);
    setIf(1);
    link(0, 1); link(1, 0); link(1, -1);
    process("loop2");

    reset();
    graph(3);
    setIf(0);
    link(0, 1); link(0, -1); link(1, 2); link(2, -1);
    process("ifbreak");

    reset();
    graph(4);
    link(0, 1); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
    process("inner_general_tail");

    reset();
    graph(4);
    setIf(2);
    link(0, 1); link(1, 2); link(2, 1); link(2, 3); link(3, -1);
    process("inner");

    reset();
    graph(3);
    link(0, 2); linkException(0, 1, "java/lang/Exception"); link(1, 2); link(2, -1);
    process("trycatch");

    reset();
    graph(5);
    setIf(2);
    link(0, 1); link(1, 2); link(2, 3); link(3, 1); link(2, 4); link(4, -1);
    process("loop3");
  }
}

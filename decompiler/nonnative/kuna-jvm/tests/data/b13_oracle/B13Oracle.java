// Oracle for tests/b13_structure_helpers.rs: builds small statement graphs with
// the real Vineflower 1.12.0 classes, runs IfHelper / LabelHelper on them and
// prints what the Rust port must reproduce.
//
//   javac -cp vineflower-1.12.0.jar -d out B13Oracle.java
//   java -cp vineflower-1.12.0.jar:out \
//     org.jetbrains.java.decompiler.modules.decompiler.stats.B13Oracle > expected.txt
package org.jetbrains.java.decompiler.modules.decompiler.stats;

import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.decompiler.IfHelper;
import org.jetbrains.java.decompiler.modules.decompiler.LabelHelper;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ExitExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.IfExprent;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.collections.ListStack;

import java.lang.reflect.Field;
import java.util.*;

public class B13Oracle {
  static final int REG = StatEdge.TYPE_REGULAR;
  static final int BREAK = StatEdge.TYPE_BREAK;
  static final int FINALLYEXIT = StatEdge.TYPE_FINALLYEXIT;

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
    // kuna's test context starts from the option defaults; the only one these paths read is on.
    props.set(ctx, new HashMap<String, Object>(Map.of("ternary-in-if", "1")));
    DecompilerContext.setCurrentContext(ctx);
  }

  static Statement bb() {
    return BasicBlockStatement.create();
  }

  static StatEdge edge(int type, Statement source, Statement destination) {
    StatEdge e = new StatEdge(type, source, destination);
    source.addSuccessor(e);
    return e;
  }

  static StatEdge edge(int type, Statement source, Statement destination, Statement closure) {
    StatEdge e = new StatEdge(type, source, destination, closure);
    source.addSuccessor(e);
    return e;
  }

  static IfExprent cond() {
    ListStack<Exprent> stack = new ListStack<>();
    stack.push(new ConstExprent(1, true, null));
    return new IfExprent(IfExprent.Type.VALUE, stack, null);
  }

  static Exprent ret() {
    return new ExitExprent(ExitExprent.Type.RETURN, null, VarType.VARTYPE_VOID, null, null);
  }

  // head's direct successors are [fallthrough, if edge]; the fallthrough moves to the if.
  static IfStatement ifThen(Statement head, Statement body) {
    IfStatement ifs = new IfStatement(head, 0, null);
    if (body != null) {
      ifs.setIfstat(body);
      ifs.getStats().addWithKey(body, body.id);
    }
    ifs.setAllParent();
    ifs.getHeadexprentList().set(0, cond());
    return ifs;
  }

  static IfStatement ifElse(Statement head, Statement ifBody, Statement elseBody) {
    edge(REG, head, elseBody);
    edge(REG, head, ifBody);
    Statement join = bb();
    StatEdge a = edge(REG, ifBody, join);
    StatEdge b = edge(REG, elseBody, join);
    IfStatement ifs = new IfStatement(head, 2, null);
    ifBody.removeSuccessor(a);
    elseBody.removeSuccessor(b);
    ifs.setAllParent();
    ifs.getHeadexprentList().set(0, cond());
    return ifs;
  }

  static String expr(Exprent e) {
    if (e instanceof FunctionExprent f) {
      StringBuilder sb = new StringBuilder(f.getFuncType().name()).append("#").append(f.id).append("(");
      for (int i = 0; i < f.getLstOperands().size(); i++) {
        if (i > 0) {
          sb.append(", ");
        }
        sb.append(expr(f.getLstOperands().get(i)));
      }
      return sb.append(")").toString();
    }
    return "#" + e.id;
  }

  static String flags(List<StatEdge> edges) {
    List<String> out = new ArrayList<>();
    for (StatEdge e : edges) {
      out.add(e + " explicit=" + e.explicit + " labeled=" + e.labeled);
    }
    return out.toString();
  }

  static String labels(Statement stat) {
    List<String> out = new ArrayList<>();
    for (StatEdge e : stat.getLabelEdges()) {
      out.add(e.toString());
    }
    Collections.sort(out);
    return out.toString();
  }

  static void p(String key, Object value) {
    System.out.println(key + " = " + value);
  }

  public static void main(String[] args) throws Exception {
    // IfHelper.prettifyIfs: an if/else whose if branch only returns.
    reset();
    {
      Statement h = bb(), i = bb(), e = bb();
      DummyExitStatement exit = new DummyExitStatement();
      edge(BREAK, i, exit);
      i.getExprents().add(ret());
      e.getExprents().add(ret());
      e.getExprents().add(ret());
      IfStatement ifs = ifElse(h, i, e);
      RootStatement root = new RootStatement(ifs, exit, null);
      p("pretty.iftype", ifs.iftype);
      p("pretty.changed", IfHelper.prettifyIfs(root));
      p("pretty.iftype2", ifs.iftype);
      p("pretty.root", root.getStats());
      Statement seq = root.getFirst();
      p("pretty.seq", seq.getStats());
      p("pretty.seq.parent", seq.getParent());
      p("pretty.ifs", ifs.getStats());
      p("pretty.ifs.parent", ifs.getParent());
      p("pretty.e.parent", e.getParent());
      p("pretty.h.succs", h.getAllSuccessorEdges());
      p("pretty.ifs.succs", ifs.getAllSuccessorEdges());
      p("pretty.e.preds", e.getAllPredecessorEdges());
      p("pretty.elsestat", ifs.getElsestat());
      p("pretty.elseedge", ifs.getElseEdge());
    }

    // LabelHelper.identifyLabels: if { if/else breaking to c } c; return
    reset();
    {
      Statement c = bb();
      c.getExprents().add(ret());
      Statement h0 = bb(), h1 = bb(), t = bb(), e = bb();
      IfStatement inner = ifElse(h1, t, e);
      edge(REG, h0, c);
      edge(REG, h0, inner);
      IfStatement outer = ifThen(h0, inner);
      SequenceStatement seq = new SequenceStatement(Arrays.asList(outer, c));
      seq.setAllParent();
      edge(BREAK, t, c, seq);
      edge(BREAK, e, c, seq);
      DummyExitStatement exit = new DummyExitStatement();
      RootStatement root = new RootStatement(seq, exit, null);
      edge(BREAK, c, exit, root);
      LabelHelper.identifyLabels(root);
      p("labels.h0", flags(h0.getAllSuccessorEdges()));
      p("labels.outer", flags(outer.getAllSuccessorEdges()));
      p("labels.h1", flags(h1.getAllSuccessorEdges()));
      p("labels.t", flags(t.getAllSuccessorEdges()));
      p("labels.e", flags(e.getAllSuccessorEdges()));
      p("labels.inner", flags(inner.getAllSuccessorEdges()));
      p("labels.c", flags(c.getAllSuccessorEdges()));
      p("labels.c.preds", c.getAllPredecessorEdges());
      p("labels.seq.labels", labels(seq));
      p("labels.root.labels", labels(root));
      p("labels.outer.labels", labels(outer));
    }

    // IfHelper.mergeIfs: if (a) { if (b) goto A; goto B } A  ->  if (a && !b) goto B
    reset();
    {
      Statement h1 = bb(), h2 = bb(), a = bb(), b = bb();
      edge(FINALLYEXIT, h2, b);
      edge(BREAK, h2, a);
      IfStatement child = ifThen(h2, null);
      edge(FINALLYEXIT, h1, a);
      edge(REG, h1, child);
      IfStatement parent = ifThen(h1, child);
      DummyExitStatement exit = new DummyExitStatement();
      RootStatement root = new RootStatement(parent, exit, null);
      p("merge.before", expr(parent.getHeadexprent()) + " " + expr(child.getHeadexprent()));
      p("merge.changed", IfHelper.mergeIfs(parent, new HashSet<>()));
      p("merge.stats", parent.getStats());
      p("merge.ifstat", parent.getIfstat());
      p("merge.ifedge", parent.getIfEdge());
      p("merge.iftype", parent.iftype);
      p("merge.h1.succs", h1.getAllSuccessorEdges());
      p("merge.parent.succs", parent.getAllSuccessorEdges());
      p("merge.h2.succs", h2.getAllSuccessorEdges());
      p("merge.child.succs", child.getAllSuccessorEdges());
      p("merge.a.preds", a.getAllPredecessorEdges());
      p("merge.b.preds", b.getAllPredecessorEdges());
      p("merge.cond", expr(parent.getHeadexprent().getCondition()));
      p("merge.root", root.getStats());
    }

    // IfHelper.mergeIfs: if (a) goto A; if (b) goto A; B  ->  if (a || b) goto A; B
    reset();
    {
      Statement h1 = bb(), h2 = bb(), a = bb(), b = bb();
      h1.getExprents().add(new ConstExprent(7, true, null));
      edge(FINALLYEXIT, h2, b);
      edge(FINALLYEXIT, h2, a);
      IfStatement second = ifThen(h2, null);
      edge(REG, h1, second);
      edge(FINALLYEXIT, h1, a);
      IfStatement first = ifThen(h1, null);
      SequenceStatement seq = new SequenceStatement(Arrays.asList(first, second));
      seq.setAllParent();
      DummyExitStatement exit = new DummyExitStatement();
      RootStatement root = new RootStatement(seq, exit, null);
      p("or.changed", IfHelper.mergeIfs(seq, new HashSet<>()));
      p("or.seq", seq.getStats());
      p("or.seq.first", seq.getFirst());
      p("or.second", second.getStats());
      p("or.second.first", second.getFirst());
      p("or.ifedge", second.getIfEdge());
      p("or.elseedge", second.getElseEdge());
      p("or.h1.parent", h1.getParent());
      p("or.h1.succs", h1.getAllSuccessorEdges());
      p("or.h2.succs", h2.getAllSuccessorEdges());
      p("or.first.succs", first.getAllSuccessorEdges());
      p("or.second.succs", second.getAllSuccessorEdges());
      p("or.a.preds", a.getAllPredecessorEdges());
      p("or.b.preds", b.getAllPredecessorEdges());
      p("or.cond", expr(second.getHeadexprent().getCondition()));
      p("or.root", root.getStats());
    }

    // IfHelper.mergeIfs: if (a) { if (b) goto A; goto B } else { if (c) goto A; goto B }
    //   ->  if (a ? b : c) goto A; goto B
    reset();
    {
      Statement h0 = bb(), h1 = bb(), h2 = bb(), a = bb(), b = bb();
      edge(FINALLYEXIT, h1, b);
      edge(FINALLYEXIT, h1, a);
      IfStatement first = ifThen(h1, null);
      edge(FINALLYEXIT, h2, b);
      edge(FINALLYEXIT, h2, a);
      IfStatement second = ifThen(h2, null);
      IfStatement main = ifElse(h0, first, second);
      DummyExitStatement exit = new DummyExitStatement();
      RootStatement root = new RootStatement(main, exit, null);
      p("ternary.iftype", main.iftype);
      p("ternary.changed", IfHelper.mergeIfs(main, new HashSet<>()));
      p("ternary.iftype2", main.iftype);
      p("ternary.stats", main.getStats());
      p("ternary.ifstat", main.getIfstat());
      p("ternary.elsestat", main.getElsestat());
      p("ternary.ifedge", main.getIfEdge());
      p("ternary.elseedge", main.getElseEdge());
      p("ternary.h0.succs", h0.getAllSuccessorEdges());
      p("ternary.main.succs", main.getAllSuccessorEdges());
      p("ternary.h1.succs", h1.getAllSuccessorEdges());
      p("ternary.h2.succs", h2.getAllSuccessorEdges());
      p("ternary.first.succs", first.getAllSuccessorEdges());
      p("ternary.second.succs", second.getAllSuccessorEdges());
      p("ternary.a.preds", a.getAllPredecessorEdges());
      p("ternary.b.preds", b.getAllPredecessorEdges());
      p("ternary.cond", expr(main.getHeadexprent().getCondition()));
      p("ternary.root", root.getStats());
    }
  }
}

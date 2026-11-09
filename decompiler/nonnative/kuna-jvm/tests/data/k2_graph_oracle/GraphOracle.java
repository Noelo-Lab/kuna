// Oracle for tests/k2_statement_graph.rs: builds small statement graphs with the
// real Vineflower 1.12.0 classes and prints what the Rust port must reproduce.
//
//   javac -cp vineflower-1.12.0.jar -d out GraphOracle.java
//   java -cp vineflower-1.12.0.jar:out \
//     org.jetbrains.java.decompiler.modules.decompiler.stats.GraphOracle > expected.txt
package org.jetbrains.java.decompiler.modules.decompiler.stats;

import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.DominatorEngine;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.DominatorTreeExceptionFilter;
import org.jetbrains.java.decompiler.modules.decompiler.decompose.StrongConnectivityHelper;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.stats.Statement.EdgeDirection;

import java.lang.reflect.Field;
import java.util.*;

public class GraphOracle {
  static final int REG = StatEdge.TYPE_REGULAR;
  static final int EXC = StatEdge.TYPE_EXCEPTION;
  static final int BREAK = StatEdge.TYPE_BREAK;

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

  static Statement bb() {
    return BasicBlockStatement.create();
  }

  static StatEdge edge(int type, Statement source, Statement destination) {
    StatEdge e = new StatEdge(type, source, destination);
    source.addSuccessor(e);
    return e;
  }

  static void p(String key, Object value) {
    System.out.println(key + " = " + value);
  }

  public static void main(String[] args) throws Exception {
    reset();
    {
      Statement a = bb(), b = bb(), c = bb();
      StatEdge reg = edge(REG, a, b);
      StatEdge exc = new StatEdge(a, c, List.of("java/lang/Exception"));
      a.addSuccessor(exc);
      p("edges.all", a.getAllSuccessorEdges());
      p("edges.direct", a.getAllDirectSuccessorEdges());
      p("edges.regexc", a.getSuccessorEdges(REG | EXC));
      p("edges.neigh", c.getNeighbours(EXC, EdgeDirection.BACKWARD));
      a.removeSuccessor(reg);
      p("edges.after", a.getAllSuccessorEdges());
      p("edges.first", a.getFirstSuccessor());
    }

    reset();
    {
      Statement a = bb(), b = bb(), c = bb(), exit = bb();
      GeneralStatement general = new GeneralStatement(a, List.of(a, b, c), null);
      edge(REG, a, b);
      edge(REG, b, c);
      c.addSuccessor(new StatEdge(BREAK, c, exit, general));
      general.setAllParent();
      p("seq.rpo", general.getReversePostOrderList());
      p("seq.prpo", general.getPostReversePostOrderList());
      Statement seq = SequenceStatement.isHead2Block(a);
      p("seq.stats", seq.getStats());
      p("seq.post", seq.getPost());
      general.collapseNodesToStatement(seq);
      p("seq.general.stats", general.getStats());
      p("seq.general.first", general.getFirst());
      p("seq.c.preds", c.getAllPredecessorEdges());
      p("seq.labels", seq.getLabelEdges());
      p("seq.succs", seq.getAllSuccessorEdges());
    }

    reset();
    {
      Statement a = bb(), b = bb(), y = bb(), x = bb();
      SequenceStatement seq = new SequenceStatement(Arrays.asList(a, b));
      seq.setAllParent();
      edge(REG, a, b);
      StatEdge labeled = new StatEdge(BREAK, b, y, a);
      b.addSuccessor(labeled);
      seq.replaceStatement(a, x);
      p("replace.stats", seq.getStats());
      p("replace.first", seq.getFirst());
      p("replace.x.succs", x.getAllSuccessorEdges());
      p("replace.labeled", labeled);
      Statement stranger = bb();
      try {
        seq.replaceStatement(stranger, a);
      } catch (Throwable t) {
        p("replace.err", t.toString());
      }
      b.replaceWithEmpty();
      p("replace.empty.stats", seq.getStats());
    }

    reset();
    {
      Statement h = bb(), t = bb(), out = bb();
      GeneralStatement general = new GeneralStatement(h, List.of(h, t), null);
      edge(REG, h, t);
      h.addSuccessor(new StatEdge(BREAK, h, out, general));
      IfStatement ifs = new IfStatement(h, 1, null);
      p("if1.iftype", ifs.iftype);
      p("if1.ifedge", ifs.getIfEdge());
      p("if1.post", ifs.getPost());
      p("if1.h.succs", h.getAllSuccessorEdges());
      general.collapseNodesToStatement(ifs);
      p("if1.general.stats", general.getStats());
      p("if1.succs", ifs.getAllSuccessorEdges());
    }

    reset();
    {
      Statement h = bb(), e = bb(), i = bb(), q = bb();
      edge(REG, h, e);
      edge(REG, h, i);
      edge(REG, i, q);
      edge(REG, e, q);
      IfStatement ifs = new IfStatement(h, 2, q);
      p("if2.iftype", ifs.iftype);
      p("if2.ifstat", ifs.getIfstat());
      p("if2.elsestat", ifs.getElsestat());
      p("if2.ifedge", ifs.getIfEdge());
      p("if2.elseedge", ifs.getElseEdge());
      p("if2.post", ifs.getPost());
      p("if2.stats", ifs.getStats());
      ifs.fixIfInvariantEmptyElseBranch();
      p("if2.fix.iftype", ifs.iftype);
      p("if2.fix.stats", ifs.getStats());
      p("if2.fix.h.succs", h.getAllSuccessorEdges());
      p("if2.fix.succs", ifs.getAllSuccessorEdges());
    }

    reset();
    {
      Statement a = bb();
      GeneralStatement general = new GeneralStatement(a, List.of(a), null);
      StatEdge back = edge(REG, a, a);
      Statement dos = DoStatement.isHead(a);
      general.collapseNodesToStatement(dos);
      p("do.back", back);
      p("do.preds", dos.getAllPredecessorEdges());
      p("do.a.cont", a.getContinueSet());
      p("do.cont", dos.getContinueSet());
      p("do.labeled", dos.isLabeled());
      p("do.general.first", general.getFirst());
    }

    reset();
    {
      Statement a = bb(), b = bb(), c = bb();
      GeneralStatement general = new GeneralStatement(a, List.of(a, b, c), null);
      edge(REG, a, b);
      edge(REG, b, a);
      edge(REG, b, c);
      StrongConnectivityHelper helper = new StrongConnectivityHelper(general);
      p("scc", helper.getComponents());
      p("scc.exits", StrongConnectivityHelper.getExitReps(helper.getComponents()));
    }

    reset();
    {
      Statement a = bb(), b = bb(), c = bb(), d = bb();
      GeneralStatement general = new GeneralStatement(a, List.of(a, b, c, d), null);
      edge(REG, a, b);
      edge(REG, a, c);
      edge(REG, b, d);
      edge(REG, c, d);
      p("dom.rpo", general.getReversePostOrderList());
      DominatorEngine engine = new DominatorEngine(general);
      engine.initialize();
      p("dom.idoms", engine.getOrderedIDoms().toStringVb());
      p("dom.all.a", engine.allDomsFor(a.id));
      p("dom.all.b", engine.allDomsFor(b.id));
      p("dom.d.by.b", engine.isDominator(d.id, b.id));
      DominatorTreeExceptionFilter filter = new DominatorTreeExceptionFilter(general);
      filter.initialize();
      p("dom.accept", filter.acceptStatementPair(a.id, d.id));
    }

    reset();
    {
      Statement h = bb(), s1 = bb(), s2 = bb();
      SwitchStatement sw = new SwitchStatement();
      sw.setFirst(h);
      sw.getStats().addWithKey(h, h.id);
      sw.setDefaultCase(s1);
      p("switch.default", sw.getDefaultEdge());
      p("switch.stats", sw.getStats());
      sw.addCase(Arrays.asList((Exprent) null, null), s2);
      p("switch.cases", sw.getCaseStatements());
      p("switch.stats2", sw.getStats());
      p("switch.h.succs", h.getAllSuccessorEdges());
      sw.removeCase(s2);
      p("switch.cases3", sw.getCaseStatements());
      p("switch.stats3", sw.getStats());
      p("switch.h.succs3", h.getAllSuccessorEdges());
    }

    reset();
    {
      Statement head = bb(), handler = bb(), next = bb();
      GeneralStatement general = new GeneralStatement(head, List.of(head, handler, next), null);
      edge(REG, head, next);
      head.addSuccessor(new StatEdge(head, handler, List.of("java/lang/RuntimeException")));
      edge(REG, handler, next);
      Statement cs = CatchStatement.isHead(head);
      p("catch.stats", cs.getStats());
      p("catch.post", cs.getPost());
      p("catch.exct", ((CatchStatement) cs).getExctStrings());
      p("catch.vars", ((CatchStatement) cs).getVars().size());
      general.collapseNodesToStatement(cs);
      p("catch.general.stats", general.getStats());
      p("catch.succs", cs.getAllSuccessorEdges());
      p("catch.next.preds", next.getAllPredecessorEdges());
    }
  }
}

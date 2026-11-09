// Oracle for tests/b04_render.rs: renders small statement trees and
// expressions with the real Vineflower 1.12.0 classes and runs a few match
// patterns over them. Leaves are `assert false` and `monitorexit`
// expressions, whose rendering needs nothing beyond the classes under test.
// Regenerate with `sh gen.sh <vineflower-1.12.0.jar>`.
package org.jetbrains.java.decompiler.modules.decompiler.stats;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.match.IMatchable;
import org.jetbrains.java.decompiler.struct.match.MatchEngine;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.collections.ListStack;

import java.util.*;

public class B04Oracle {
  static final int REG = StatEdge.TYPE_REGULAR;
  static final int BREAK = StatEdge.TYPE_BREAK;
  static final int CONTINUE = StatEdge.TYPE_CONTINUE;

  static final StringBuilder OUT = new StringBuilder();

  static void install() {
    Map<String, Object> props = new HashMap<>(IFernflowerPreferences.DEFAULTS);
    StructContext sc = new StructContext(null, null);
    ClassesProcessor cp = new ClassesProcessor(sc);
    DecompilerContext ctx = new DecompilerContext(props, new PrintStreamLogger(System.err), sc, cp, null);
    DecompilerContext.setCurrentContext(ctx);
  }

  static void reset() {
    DecompilerContext.startMethod(null);
  }

  static Exprent af() {
    List<Exprent> params = new ArrayList<>();
    params.add(null);
    return new AssertExprent(params);
  }

  static Exprent mexit() {
    return new MonitorExprent(MonitorExprent.Type.EXIT, af(), null);
  }

  static Exprent ifExpr(Exprent condition) {
    ListStack<Exprent> stack = new ListStack<>();
    stack.push(condition);
    return new IfExprent(IfExprent.Type.VALUE, stack, null);
  }

  static Exprent field(String name) {
    return new FieldExprent(name, "pkg/E", true, null, FieldDescriptor.parseDescriptor("Lpkg/E;"), null);
  }

  static BasicBlockStatement bb() {
    return BasicBlockStatement.create();
  }

  static StatEdge edge(int type, Statement source, Statement destination) {
    StatEdge e = new StatEdge(type, source, destination);
    source.addSuccessor(e);
    return e;
  }

  static StatEdge edgeC(int type, Statement source, Statement destination, Statement closure) {
    StatEdge e = new StatEdge(type, source, destination, closure);
    source.addSuccessor(e);
    return e;
  }

  static List<Exprent> values(Exprent... values) {
    return new ArrayList<>(Arrays.asList(values));
  }

  static void out(String name, TextBuffer buf) {
    OUT.append("== ").append(name).append('\n');
    OUT.append(buf.convertToStringAndAllowDataDiscard());
    OUT.append("\n<<\n");
  }

  static void match(String name, IMatchable target, String[] vars, String... description) {
    MatchEngine engine = new MatchEngine(description);
    boolean res = engine.match(target);
    OUT.append("match ").append(name).append(' ').append(res);
    for (String v : vars) {
      Object value = engine.getVariableValue(v);
      OUT.append(' ').append(v).append('=').append(value == null ? "null" : Integer.toString(((Statement) value).id));
    }
    OUT.append('\n');
  }

  public static void main(String[] args) {
    install();

    reset();
    {
      BasicBlockStatement h = bb(), t = bb(), out = bb();
      h.getExprents().add(af());
      GeneralStatement general = new GeneralStatement(h, Arrays.asList(h, t), null);
      edge(REG, h, t);
      edgeC(BREAK, h, out, general);
      IfStatement ifs = new IfStatement(h, 1, null);
      ifs.getHeadexprentList().set(0, ifExpr(af()));
      general.collapseNodesToStatement(ifs);
      out("if_break", general.toJava(0));
    }

    reset();
    {
      BasicBlockStatement h = bb(), i = bb(), h2 = bb(), x = bb(), out2 = bb(), p = bb();
      h.getExprents().add(af());
      i.getExprents().add(mexit());
      x.getExprents().add(af());
      edge(REG, h2, x);
      StatEdge brk = edgeC(BREAK, h2, out2, null);
      brk.labeled = false;
      IfStatement inner = new IfStatement(h2, 1, null);
      inner.getHeadexprentList().set(0, ifExpr(af()));
      edge(REG, h, inner);
      edge(REG, h, i);
      edge(REG, i, p);
      StatEdge ip = edge(REG, inner, p);
      ip.explicit = false;
      IfStatement outer = new IfStatement(h, 2, p);
      outer.getHeadexprentList().set(0, ifExpr(af()));
      out("if_else_if", outer.toJava(0));
    }

    reset();
    {
      BasicBlockStatement h = bb(), i = bb(), e = bb(), p = bb();
      h.getExprents().add(af());
      i.getExprents().add(af());
      e.getExprents().add(mexit());
      edge(REG, h, e);
      edge(REG, h, i);
      edge(REG, i, p);
      edge(REG, e, p);
      IfStatement ifs = new IfStatement(h, 2, p);
      ifs.getHeadexprentList().set(0, ifExpr(af()));
      out("if_else", ifs.toJava(1));
      match("if_ifelse", ifs, new String[]{"$i$"}, "statement type:if iftype:ifelse", " statement position:1 ret:$i$");
      match("if_if", ifs, new String[0], "statement type:if iftype:if");
      match("if_neg", ifs, new String[]{"$e$"}, "statement type:if", " statement position:-1 exprsize:1 ret:$e$");
    }

    reset();
    {
      IfStatement ifs = new IfStatement();
      ifs.setFirst(bb());
      ifs.getStats().addWithKey(ifs.getFirst(), ifs.getFirst().id);
      ifs.setIfEdge(edge(CONTINUE, ifs.getFirst(), bb()));
      out("if_null_condition", ifs.toJava(0));
    }

    for (DoStatement.Type type : DoStatement.Type.values()) {
      reset();
      BasicBlockStatement head = bb();
      head.getExprents().add(af());
      DoStatement d = new DoStatement(head);
      d.setLooptype(type);
      switch (type) {
        case INFINITE:
          edgeC(CONTINUE, head, d, d);
          break;
        case DO_WHILE:
        case WHILE:
          d.setConditionExprent(af());
          break;
        case FOR:
          d.setInitExprent(mexit());
          d.setConditionExprent(af());
          d.setIncExprent(mexit());
          break;
        case FOR_EACH:
          d.setInitExprent(af());
          d.setIncExprent(mexit());
          break;
      }
      out("do_" + type.name().toLowerCase(Locale.ROOT), d.toJava(1));
    }

    reset();
    {
      BasicBlockStatement head = bb();
      DoStatement d = new DoStatement(head);
      d.setLooptype(DoStatement.Type.FOR);
      d.setConditionExprent(af());
      d.setIncExprent(mexit());
      out("do_for_no_init", d.toJava(0));
    }

    reset();
    {
      BasicBlockStatement h = bb(), c1 = bb(), c2 = bb(), d = bb();
      h.getExprents().add(af());
      c1.getExprents().add(af());
      c2.getExprents().add(mexit());
      d.getExprents().add(af());
      StatEdge e1 = edge(REG, h, c1);
      StatEdge e2 = edge(REG, h, c2);
      StatEdge e3 = edge(REG, h, c2);
      StatEdge ed = edge(REG, h, d);
      SwitchStatement sw = new SwitchStatement(h, null);
      sw.setDefaultEdge(ed);
      sw.getCaseStatements().addAll(Arrays.asList(c1, c2, d));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e1)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e2, e3)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(ed)));
      sw.getCaseValues().add(values(field("A")));
      sw.getCaseValues().add(values(field("B"), field("C")));
      sw.getCaseValues().add(values((Exprent) null));
      sw.getCaseGuards().add(null);
      sw.getCaseGuards().add(af());
      sw.scopeCaseStatement(c2);
      sw.getHeadexprentList().set(0, new SwitchHeadExprent(af(), null));
      out("switch", sw.toJava(0));
      sw.setPhantom(true);
      out("switch_phantom", sw.toJava(0));
    }

    reset();
    {
      BasicBlockStatement h = bb(), c1 = bb(), c2 = bb(), c3 = bb(), c4 = bb(), d = bb(), after = bb();
      c1.getExprents().add(new YieldExprent(af(), VarType.VARTYPE_INT));
      c2.getExprents().add(af());
      c2.getExprents().add(mexit());
      c3.getExprents().add(af());
      c4.getExprents().add(new ExitExprent(ExitExprent.Type.THROW, af(), null, null, null));
      StatEdge e1 = edge(REG, h, c1);
      StatEdge e2 = edge(REG, h, c2);
      StatEdge e3 = edge(REG, h, c3);
      StatEdge e4 = edge(REG, h, c4);
      StatEdge ed = edge(REG, h, d);
      edgeC(BREAK, d, after, null);
      SwitchStatement sw = new SwitchStatement(h, null);
      sw.setDefaultEdge(ed);
      sw.getCaseStatements().addAll(Arrays.asList(c1, c2, c3, c4, d));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e1)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e2)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e3)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e4)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(ed)));
      sw.getCaseValues().add(values(field("A")));
      sw.getCaseValues().add(values(field("B")));
      sw.getCaseValues().add(values(field("C")));
      sw.getCaseValues().add(values(field("D"), field("E")));
      sw.getCaseValues().add(values((Exprent) null));
      sw.getCaseGuards().add(null);
      sw.getCaseGuards().add(null);
      sw.getCaseGuards().add(af());
      sw.getHeadexprentList().set(0, new SwitchHeadExprent(af(), null));
      sw.setPhantom(true);
      out("switch_expr", new SwitchExprent(sw, VarType.VARTYPE_INT, false, false).toJava(1));
    }

    reset();
    {
      BasicBlockStatement h = bb(), c1 = bb(), c2 = bb();
      c1.getExprents().add(af());
      c2.getExprents().add(mexit());
      StatEdge e1 = edge(REG, h, c1);
      StatEdge e2 = edge(REG, h, c2);
      StatEdge ed = edge(REG, h, c2);
      SwitchStatement sw = new SwitchStatement(h, null);
      sw.setDefaultEdge(ed);
      sw.getCaseStatements().addAll(Arrays.asList(c1, c2));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e1)));
      sw.getCaseEdges().add(new ArrayList<>(Arrays.asList(e2, ed)));
      sw.getCaseValues().add(values(field("A")));
      sw.getCaseValues().add(values(field("B"), null));
      sw.getHeadexprentList().set(0, new SwitchHeadExprent(af(), null));
      sw.setPhantom(true);
      out("switch_expr_default", new SwitchExprent(sw, VarType.VARTYPE_INT, false, true).toJava(0));
    }

    reset();
    {
      BasicBlockStatement head = bb(), body = bb(), exc = bb();
      head.getExprents().add(af());
      body.getExprents().add(af());
      SynchronizedStatement s = new SynchronizedStatement(head, body, exc);
      s.getHeadexprentList().set(0, new MonitorExprent(MonitorExprent.Type.ENTER, af(), null));
      out("sync", s.toJava(1));
      SynchronizedStatement bare = new SynchronizedStatement(bb(), bb(), bb());
      out("sync_null_head", bare.toJava(0));
    }

    reset();
    {
      BasicBlockStatement head = bb(), handler = bb();
      head.getExprents().add(af());
      handler.getExprents().add(mexit());
      CatchAllStatement ca = new CatchAllStatement(head, handler);
      ca.setFinally(true);
      out("finally", ca.toJava(1));
    }

    reset();
    {
      BasicBlockStatement a = bb(), b = bb(), c = bb(), out = bb();
      a.getExprents().add(af());
      b.getExprents().add(mexit());
      SequenceStatement seq = new SequenceStatement(Arrays.asList(a, c, b));
      edgeC(BREAK, b, out, seq);
      out("sequence", seq.toJava(1));
      match("seq_size", seq, new String[0], "statement type:sequence statsize:3");
      match("seq_size_wrong", seq, new String[0], "statement type:sequence statsize:2");
      match("seq_type_wrong", seq, new String[0], "statement type:if");
      match("seq_children", seq, new String[]{"$s$", "$a$", "$c$"},
            "statement type:sequence ret:$s$", " statement type:basicblock exprsize:1 ret:$a$", " statement type:basicblock exprsize:0 ret:$c$");
      match("seq_last", seq, new String[]{"$x$"}, "statement type:sequence", " statement position:-1 exprsize:1 ret:$x$");
      match("seq_first_wrong", seq, new String[0], "statement type:sequence", " statement position:0 exprsize:0");
      match("seq_null_exprents", seq, new String[0], "statement type:sequence", " statement exprsize:-1");
      match("seq_too_many", seq, new String[0], "statement", " statement", " statement", " statement", " statement");
      match("seq_same_var", seq, new String[]{"$v$"}, "statement", " statement ret:$v$", " statement ret:$v$");
      match("bb_on_seq", seq, new String[0], "statement type:basicblock exprsize:1");
    }

    reset();
    {
      BasicBlockStatement a = bb(), b = bb(), c = bb();
      a.getExprents().add(af());
      b.getExprents().add(mexit());
      c.getExprents().add(af());
      out("sequence_plain", new SequenceStatement(Arrays.asList(a, b, c)).toJava(0));
    }

    reset();
    {
      BasicBlockStatement a = bb(), out = bb();
      a.getExprents().add(af());
      edgeC(BREAK, a, out, a);
      out("basic_block_labeled", a.toJava(1));
    }

    System.out.print(OUT);
  }
}

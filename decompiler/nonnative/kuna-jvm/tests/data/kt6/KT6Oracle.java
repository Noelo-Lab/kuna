// Oracle for tests/kt6_render.rs: wraps small Java statement trees in the Kotlin plugin's
// statement classes (directly or through ReplaceStatsPass) and renders them with the real
// Vineflower 1.12.0 classes and its bundled Kotlin plugin. Leaves are `assert false` and
// `monitorexit` expressions (no Kotlin counterpart), int constants and variables.
// Regenerate with `sh gen-render.sh <vineflower-1.12.0.jar> <vineflower-kotlin-0.1.0.jar>`.
package org.jetbrains.java.decompiler.modules.decompiler.stats;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.decompiler.PrintStreamLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.struct.StructContext;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.TextBuffer;
import org.jetbrains.java.decompiler.util.collections.ListStack;
import org.vineflower.kotlin.pass.ReplaceStatsPass;
import org.vineflower.kotlin.stat.*;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.util.*;

public class KT6Oracle {
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

  static Exprent intConst(int value) {
    return new ConstExprent(value, false, null);
  }

  static VarExprent var(int index, VarType type) {
    return new VarExprent(index, type, null);
  }

  static Exprent func(FunctionExprent.FunctionType type, Exprent... operands) {
    return new FunctionExprent(type, new ArrayList<>(Arrays.asList(operands)), null);
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

  static void parents(Statement stat) {
    for (Statement st : stat.getStats()) {
      st.setParent(stat);
      parents(st);
    }
  }

  static void replaceStats(Statement stat) throws Exception {
    Method replace = ReplaceStatsPass.class.getDeclaredMethod("replace", Statement.class);
    replace.setAccessible(true);
    replace.invoke(null, stat);
  }

  interface Render {
    TextBuffer render() throws Exception;
  }

  static void out(String name, Render render) {
    OUT.append("== ").append(name).append('\n');
    try {
      OUT.append(render.render().convertToStringAndAllowDataDiscard());
    } catch (Throwable t) {
      Throwable cause = t instanceof InvocationTargetException ? t.getCause() : t;
      OUT.append("!! ").append(cause.getClass().getSimpleName());
    }
    OUT.append("\n<<\n");
  }

  static DoStatement loop(DoStatement.Type type) {
    BasicBlockStatement head = bb();
    head.getExprents().add(af());
    DoStatement d = new DoStatement(head);
    d.setLooptype(type);
    return d;
  }

  static void forLoop(String name, VarType type, Exprent init, Exprent condition, FunctionExprent.FunctionType inc) {
    reset();
    DoStatement d = loop(DoStatement.Type.FOR);
    d.setInitExprent(new AssignmentExprent(var(1, type), init, null));
    d.setConditionExprent(condition);
    d.setIncExprent(func(inc, var(1, type)));
    out(name, () -> new KDoStatement(d).toJava(1));
  }

  public static void main(String[] args) throws Exception {
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
      out("kif_break", () -> new KIfStatement(ifs).toJava(0));
    }

    reset();
    {
      BasicBlockStatement h = bb(), t = bb(), dest = bb();
      h.getExprents().add(mexit());
      edge(REG, h, t);
      StatEdge c = edgeC(CONTINUE, h, dest, null);
      c.labeled = false;
      IfStatement ifs = new IfStatement(h, 1, null);
      ifs.getHeadexprentList().set(0, ifExpr(af()));
      KIfStatement kif = new KIfStatement(ifs);
      out("kif_continue", () -> kif.toJava(1));
      c.explicit = false;
      out("kif_implicit", () -> kif.toJava(1));
    }

    reset();
    {
      BasicBlockStatement h = bb(), t = bb(), out = bb(), a = bb();
      a.getExprents().add(af());
      KSequenceStatement kseq = new KSequenceStatement(new SequenceStatement(Arrays.asList(a)));
      edge(REG, h, t);
      StatEdge b = edgeC(BREAK, h, out, kseq);
      b.labeled = false;
      IfStatement ifs = new IfStatement(h, 1, null);
      ifs.getHeadexprentList().set(0, ifExpr(mexit()));
      ifs.addLabeledEdge(edgeC(BREAK, t, out, null));
      KIfStatement kif = new KIfStatement(ifs);
      out("kif_return_run", () -> kif.toJava(0));
      b.labeled = true;
      out("kif_return_label", () -> kif.toJava(0));
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
      out("kif_else", () -> new KIfStatement(ifs).toJava(1));
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
      SequenceStatement holder = new SequenceStatement(Arrays.asList(outer));
      parents(holder);
      replaceStats(holder);
      out("kif_else_if", () -> holder.getFirst().toJava(0));
    }

    reset();
    {
      BasicBlockStatement a = bb(), b = bb(), c = bb(), out = bb();
      a.getExprents().add(af());
      b.getExprents().add(mexit());
      SequenceStatement seq = new SequenceStatement(Arrays.asList(a, c, b));
      edgeC(BREAK, b, out, seq);
      KSequenceStatement plain = new KSequenceStatement(seq);
      out("kseq", () -> plain.toJava(1));
      seq.addLabeledEdge(edgeC(BREAK, c, out, null));
      KSequenceStatement labeled = new KSequenceStatement(seq);
      out("kseq_labeled", () -> labeled.toJava(1));
    }

    for (DoStatement.Type type : DoStatement.Type.values()) {
      reset();
      DoStatement d = loop(type);
      BasicBlockStatement head = (BasicBlockStatement) d.getFirst();
      switch (type) {
        case INFINITE -> edgeC(CONTINUE, head, d, d);
        case DO_WHILE, WHILE -> d.setConditionExprent(af());
        case FOR -> {
          d.setInitExprent(mexit());
          d.setConditionExprent(af());
          d.setIncExprent(mexit());
        }
        case FOR_EACH -> {
          d.setInitExprent(var(3, VarType.VARTYPE_INT));
          d.setIncExprent(af());
        }
      }
      String name = type.name().toLowerCase(Locale.ROOT);
      out("kdo_" + name, () -> new KDoStatement(d).toJava(1));
      d.addLabeledEdge(edgeC(BREAK, head, bb(), null));
      out("kdo_" + name + "_labeled", () -> new KDoStatement(d).toJava(1));
    }

    reset();
    {
      DoStatement d = loop(DoStatement.Type.FOR);
      d.setConditionExprent(af());
      d.setIncExprent(mexit());
      out("kdo_for_no_init", () -> new KDoStatement(d).toJava(0));
    }

    forLoop("kdo_for_repeat", VarType.VARTYPE_INT, intConst(0),
      func(FunctionExprent.FunctionType.LT, var(1, VarType.VARTYPE_INT), intConst(10)),
      FunctionExprent.FunctionType.PPI);
    forLoop("kdo_for_range", VarType.VARTYPE_INT, intConst(2),
      func(FunctionExprent.FunctionType.LT, intConst(7), var(1, VarType.VARTYPE_INT)),
      FunctionExprent.FunctionType.IPP);
    forLoop("kdo_for_down_to", VarType.VARTYPE_INT, intConst(9),
      func(FunctionExprent.FunctionType.LT, var(1, VarType.VARTYPE_INT), intConst(3)),
      FunctionExprent.FunctionType.MMI);
    forLoop("kdo_for_var_bound", VarType.VARTYPE_INT, intConst(0),
      func(FunctionExprent.FunctionType.LT, var(1, VarType.VARTYPE_INT), var(2, VarType.VARTYPE_INT)),
      FunctionExprent.FunctionType.PPI);
    forLoop("kdo_for_long", VarType.VARTYPE_LONG, new ConstExprent(VarType.VARTYPE_LONG, 0L, null),
      func(FunctionExprent.FunctionType.LT, var(1, VarType.VARTYPE_LONG), new ConstExprent(VarType.VARTYPE_LONG, 5L, null)),
      FunctionExprent.FunctionType.PPI);
    forLoop("kdo_for_abandoned", VarType.VARTYPE_INT, intConst(0),
      func(FunctionExprent.FunctionType.LT, af(), af()),
      FunctionExprent.FunctionType.PPI);
    forLoop("kdo_for_not_less", VarType.VARTYPE_INT, intConst(0),
      func(FunctionExprent.FunctionType.LE, var(1, VarType.VARTYPE_INT), intConst(3)),
      FunctionExprent.FunctionType.PPI);

    reset();
    {
      BasicBlockStatement h = bb(), c1 = bb(), c2 = bb(), c3 = bb(), c4 = bb(), d = bb();
      h.getExprents().add(af());
      c1.getExprents().add(af());
      c2.getExprents().add(mexit());
      c3.getExprents().add(af());
      c3.getExprents().add(mexit());
      d.getExprents().add(af());
      StatEdge ed = edge(REG, h, d);
      StatEdge e1 = edge(REG, h, c1);
      StatEdge e2 = edge(REG, h, c2);
      StatEdge e3 = edge(REG, h, c3);
      StatEdge e4 = edge(REG, h, c4);
      StatEdge e5 = edge(REG, h, c4);
      SwitchStatement sw = new SwitchStatement(h, null);
      sw.getCaseStatements().addAll(Arrays.asList(c1, c2, c3, c4, d));
      sw.getCaseEdges().add(new ArrayList<>(List.of(e1)));
      sw.getCaseEdges().add(new ArrayList<>(List.of(e2)));
      sw.getCaseEdges().add(new ArrayList<>(List.of(e3)));
      sw.getCaseEdges().add(new ArrayList<>(List.of(e4, e5)));
      sw.getCaseEdges().add(new ArrayList<>(List.of(ed)));
      sw.getCaseValues().add(new ArrayList<>(List.of(intConst(1))));
      sw.getCaseValues().add(new ArrayList<>(List.of(intConst(2))));
      sw.getCaseValues().add(new ArrayList<>(List.of(intConst(3))));
      sw.getCaseValues().add(new ArrayList<>(List.of(intConst(4), intConst(5))));
      List<Exprent> defaultValues = new ArrayList<>();
      defaultValues.add(null);
      sw.getCaseValues().add(defaultValues);
      sw.getHeadexprentList().set(0, new SwitchHeadExprent(af(), null));
      KSwitchStatement ksw = new KSwitchStatement(sw);
      out("kswitch", () -> ksw.toJava(1));
      sw.addLabeledEdge(edgeC(BREAK, c1, bb(), null));
      KSwitchStatement labeled = new KSwitchStatement(sw);
      out("kswitch_labeled", () -> labeled.toJava(1));
      sw.setPhantom(true);
      KSwitchStatement phantom = new KSwitchStatement(sw);
      out("kswitch_phantom", () -> phantom.toJava(1));
    }

    reset();
    {
      BasicBlockStatement body = bb(), h1 = bb(), h2 = bb(), out = bb();
      body.getExprents().add(af());
      h1.getExprents().add(mexit());
      h2.getExprents().add(af());
      CatchStatement cs = new CatchStatement();
      cs.setFirst(body);
      cs.getStats().addWithKey(body, body.id);
      cs.getStats().addWithKey(h1, h1.id);
      cs.getStats().addWithKey(h2, h2.id);
      cs.getExctStrings().add(new ArrayList<>(List.of("java/lang/Exception")));
      cs.getExctStrings().add(new ArrayList<>(List.of("java/lang/Error")));
      cs.getVars().add(var(7, VarType.VARTYPE_OBJECT));
      cs.getVars().add(var(8, VarType.VARTYPE_OBJECT));
      KCatchStatement kcs = new KCatchStatement(cs);
      out("kcatch", () -> kcs.toJava(1));
      kcs.addLabeledEdge(edgeC(BREAK, body, out, null));
      out("kcatch_labeled", () -> kcs.toJava(1));
    }

    reset();
    {
      BasicBlockStatement body = bb(), h1 = bb();
      body.getExprents().add(af());
      h1.getExprents().add(mexit());
      CatchStatement cs = new CatchStatement();
      cs.setFirst(body);
      cs.getStats().addWithKey(body, body.id);
      cs.getStats().addWithKey(h1, h1.id);
      cs.getExctStrings().add(new ArrayList<>(List.of("java/lang/Exception")));
      cs.getVars().add(var(4, VarType.VARTYPE_OBJECT));
      cs.getResources().add(af());
      cs.getResources().add(mexit());
      out("kcatch_resources", () -> new KCatchStatement(cs).toJava(1));
    }

    System.out.print(OUT);
  }
}

// Golden vectors for the B10 slice (SimplifyExprentsHelper, SecondaryFunctionsHelper,
// ConcatenationHelper), computed by the real Vineflower 1.12.0 classes. Regenerate with
// `sh gen.sh <vineflower-1.12.0.jar>`. Every scenario resets the expression and variable
// counters and builds its exprents in the order tests/b10_simplify.rs mirrors.

import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.function.Supplier;

import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.ConcatenationHelper;
import org.jetbrains.java.decompiler.modules.decompiler.SecondaryFunctionsHelper;
import org.jetbrains.java.decompiler.modules.decompiler.SimplifyExprentsHelper;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.BasicBlockStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.DummyExitStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.RootStatement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.consts.PooledConstant;
import org.jetbrains.java.decompiler.struct.consts.PrimitiveConstant;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;

public class GenB10Oracle {
  static final StringBuilder OUT = new StringBuilder();
  static StructClass CL;
  static StructMethod MT;

  static void p(String tag, Object... fields) {
    OUT.append(tag);
    for (Object f : fields) {
      String s = String.valueOf(f);
      if (s.isEmpty() || s.contains(" ")) throw new IllegalStateException("bad token in " + tag + ": '" + s + "'");
      OUT.append(' ').append(s);
    }
    OUT.append('\n');
  }

  static sun.misc.Unsafe unsafe() throws Exception {
    Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    f.setAccessible(true);
    return (sun.misc.Unsafe) f.get(null);
  }

  static void setField(Object o, Class<?> cls, String name, Object value) {
    try {
      Field f = cls.getDeclaredField(name);
      f.setAccessible(true);
      f.set(o, value);
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static void install() throws Exception {
    sun.misc.Unsafe u = unsafe();
    DecompilerContext ctx = (DecompilerContext) u.allocateInstance(DecompilerContext.class);
    setField(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    setField(ctx, DecompilerContext.class, "properties", new HashMap<String, Object>(IFernflowerPreferences.getDefaults()));
    ClassesProcessor cp = (ClassesProcessor) u.allocateInstance(ClassesProcessor.class);
    setField(cp, ClassesProcessor.class, "mapRootClasses", new HashMap<>());
    setField(ctx, DecompilerContext.class, "classProcessor", cp);
    DecompilerContext.setCurrentContext(ctx);
  }

  static void reset() {
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.EXPRESSION_COUNTER, 1);
    DecompilerContext.getCounterContainer().setCounter(CounterContainer.VAR_COUNTER, 100);
  }

  // ---- formatting ----

  static String b(boolean v) {
    return v ? "1" : "0";
  }

  static String bits(BitSet s) {
    if (s.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (int i = s.nextSetBit(0); i >= 0; i = s.nextSetBit(i + 1)) {
      if (sb.length() > 0) sb.append('.');
      sb.append(i);
    }
    return sb.toString();
  }

  static String desc(Exprent e) {
    StringBuilder sb = new StringBuilder(e.type.name());
    if (e instanceof FunctionExprent f) {
      sb.append(':').append(f.getFuncType().name());
      if (!f.doesCast()) sb.append('~');
    } else if (e instanceof VarExprent v) {
      sb.append(':').append(v.getIndex()).append('_').append(v.getVersion());
      if (v.isStack()) sb.append('s');
    } else if (e instanceof ConstExprent c) {
      sb.append(':').append(c.getValue() == null ? "N" : String.valueOf(c.getValue()));
      sb.append('/').append(c.getConstType().type.name());
    } else if (e instanceof FieldExprent f) {
      sb.append(':').append(f.getName());
    } else if (e instanceof InvocationExprent i) {
      sb.append(':').append(i.getName());
    } else if (e instanceof AssignmentExprent a) {
      if (a.getCondType() != null) sb.append(':').append(a.getCondType().name());
    } else if (e instanceof NewExprent n) {
      sb.append(':').append(n.getNewType().value);
      if (n.getConstructor() != null) sb.append("+c");
    } else if (e instanceof ExitExprent x) {
      sb.append(':').append(x.getExitType().name());
    }
    sb.append('#').append(e.id);
    if (e.bytecode != null) sb.append('@').append(bits(e.bytecode));
    List<Exprent> children = e.getAllExprents();
    if (!children.isEmpty()) {
      sb.append('(');
      for (int i = 0; i < children.size(); i++) {
        if (i > 0) sb.append(',');
        sb.append(desc(children.get(i)));
      }
      sb.append(')');
    }
    return sb.toString();
  }

  static String descs(List<Exprent> lst) {
    if (lst.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (Exprent e : lst) {
      if (sb.length() > 0) sb.append('|');
      sb.append(desc(e));
    }
    return sb.toString();
  }

  static String guard(Supplier<String> body) {
    try {
      return body.get();
    } catch (RuntimeException e) {
      return "throw:" + e.getClass().getSimpleName();
    }
  }

  // ---- builders (mirrored one to one in b10_simplify.rs) ----

  static final VarType INT = VarType.VARTYPE_INT;
  static final VarType BOOL = VarType.VARTYPE_BOOLEAN;
  static final VarType LONG = VarType.VARTYPE_LONG;
  static final VarType FLOAT = VarType.VARTYPE_FLOAT;
  static final VarType STR = VarType.VARTYPE_STRING;
  static final String SB = "java/lang/StringBuilder";

  static VarType obj(String cls) {
    return new VarType(CodeType.OBJECT, 0, cls);
  }

  static List<Exprent> list(Exprent... es) {
    return new ArrayList<>(Arrays.asList(es));
  }

  static ConstExprent ci(int v) {
    return new ConstExprent(v, false, null);
  }

  static ConstExprent cbool(int v) {
    return new ConstExprent(BOOL, v, null);
  }

  static ConstExprent cs(String s) {
    return new ConstExprent(STR, s, null);
  }

  static VarExprent var(int index, VarType t) {
    return new VarExprent(index, t, null);
  }

  static VarExprent var(int index) {
    return var(index, INT);
  }

  static VarExprent svar(int index) {
    VarExprent v = var(index);
    v.setStack(true);
    return v;
  }

  static FunctionExprent fn(FunctionType t, Exprent... ops) {
    return new FunctionExprent(t, list(ops), null);
  }

  static FunctionExprent ppmm(FunctionType t, Exprent operand) {
    FunctionExprent f = fn(t, operand);
    f.setImplicitType(INT);
    return f;
  }

  static AssignmentExprent asg(Exprent l, Exprent r) {
    return new AssignmentExprent(l, r, null);
  }

  static FieldExprent ifield(String name, Exprent inst) {
    return new FieldExprent(name, "a/B", false, inst, FieldDescriptor.parseDescriptor("I"), null);
  }

  static InvocationExprent inv(String cls, String name, String desc, Exprent inst, Exprent... params) {
    InvocationExprent e = new InvocationExprent();
    e.setName(name);
    e.setClassname(cls);
    e.setStringDescriptor(desc);
    e.setDescriptor(MethodDescriptor.parseDescriptor(desc));
    e.setStatic(inst == null);
    e.setInstance(inst);
    e.setLstParameters(list(params));
    return e;
  }

  static InvocationExprent init(String cls, String desc, Exprent inst, Exprent... params) {
    InvocationExprent e = inv(cls, "<init>", desc, inst, params);
    e.setStatic(false);
    e.setFunctype(InvocationExprent.Type.INIT);
    return e;
  }

  static NewExprent newObj(String cls) {
    return new NewExprent(obj(cls), new ArrayList<>(), null);
  }

  static NewExprent sb() {
    NewExprent n = newObj(SB);
    n.setConstructor(init(SB, "()V", null));
    return n;
  }

  static InvocationExprent append(Exprent inst, String paramDesc, Exprent param) {
    return inv(SB, "append", "(" + paramDesc + ")Ljava/lang/StringBuilder;", inst, param);
  }

  static InvocationExprent sbToString(Exprent inst) {
    return inv(SB, "toString", "()Ljava/lang/String;", inst);
  }

  static InvocationExprent concat(String name, String desc, List<PooledConstant> bootstrap, Exprent... params) {
    InvocationExprent e = inv("java/lang/invoke/StringConcatFactory", name, desc, null, params);
    setField(e, InvocationExprent.class, "bootstrapArguments", bootstrap);
    return e;
  }

  static BasicBlockStatement block(List<Exprent> exprents) {
    BasicBlockStatement bb = new BasicBlockStatement(new BasicBlock(1));
    bb.setExprents(exprents);
    new RootStatement(bb, new DummyExitStatement(), MT);
    return bb;
  }

  // ---- scenarios ----

  static void simp(String label, boolean firstInvocation, Supplier<List<Exprent>> build) {
    reset();
    String r = guard(() -> {
      BasicBlockStatement bb = block(build.get());
      boolean res = SimplifyExprentsHelper.simplifyStackVarsStatement(bb, new HashSet<>(), null, CL, firstInvocation);
      return b(res) + " " + descs(bb.getExprents());
    });
    OUT.append("S ").append(label).append(' ').append(r).append('\n');
  }

  static void sec(String label, boolean force, Supplier<List<Exprent>> build) {
    reset();
    String r = guard(() -> {
      BasicBlockStatement bb = block(build.get());
      VarProcessor vp = new VarProcessor(MT, MethodDescriptor.parseDescriptor(MT.getDescriptor()));
      boolean res = force
        ? SecondaryFunctionsHelper.identifySecondaryFunctions(bb, vp, new SecondaryFunctionsHelper.IdentifySecondaryOptions().forceTernarySimplification())
        : SecondaryFunctionsHelper.identifySecondaryFunctions(bb, vp);
      return b(res) + " " + descs(bb.getExprents()) + " " + DecompilerContext.getCounterContainer().getCounter(CounterContainer.VAR_COUNTER);
    });
    OUT.append("F ").append(label).append(' ').append(r).append('\n');
  }

  static void upd(String label, Supplier<List<Exprent>> build) {
    reset();
    String r = guard(() -> {
      BasicBlockStatement bb = block(build.get());
      boolean res = SecondaryFunctionsHelper.updateAssignments(bb);
      return b(res) + " " + descs(bb.getExprents());
    });
    OUT.append("U ").append(label).append(' ').append(r).append('\n');
  }

  static void pbn(String label, Supplier<Exprent> build) {
    reset();
    String r = guard(() -> {
      Exprent e = build.get();
      Exprent ret = SecondaryFunctionsHelper.propagateBoolNot(e);
      return (ret == null ? "N" : desc(ret)) + " " + desc(e);
    });
    OUT.append("P ").append(label).append(' ').append(r).append('\n');
  }

  static void cat(String label, Supplier<List<Exprent>> build) {
    reset();
    String r = guard(() -> {
      BasicBlockStatement bb = block(build.get());
      RootStatement root = bb.getTopParent();
      ConcatenationHelper.simplifyStringConcat(root);
      return descs(bb.getExprents()) + " " + b(root.commentLines != null);
    });
    OUT.append("C ").append(label).append(' ').append(r).append('\n');
  }

  static void simplify() {
    simp("trivial_stack", true, () -> list(asg(svar(10000), svar(10000)), asg(var(1), ci(5))));
    simp("ppi_field", true, () -> list(
      asg(ifield("f", var(1, obj("a/B"))), fn(FunctionType.ADD, ifield("f", var(1, obj("a/B"))), ci(1))),
      asg(var(2), ci(0))));
    simp("mmi_field_const_first", true, () -> list(
      asg(ifield("f", var(1, obj("a/B"))), fn(FunctionType.ADD, ci(1), ifield("f", var(1, obj("a/B"))))),
      asg(var(2), ci(0))));
    simp("ppi_var_kept", true, () -> list(asg(var(1), fn(FunctionType.ADD, var(1), ci(1))), asg(var(2), ci(0))));
    simp("ipp", true, () -> list(asg(var(1), var(2)), ppmm(FunctionType.PPI, var(2))));
    simp("imm", true, () -> list(asg(var(1), var(2)), ppmm(FunctionType.MMI, var(2))));
    simp("ipp2", true, () -> list(asg(var(1), var(2)), asg(var(2), fn(FunctionType.ADD, var(1), ci(1)))));
    simp("imm2", true, () -> list(asg(var(1), var(2)), asg(var(2), fn(FunctionType.SUB, var(1), ci(1)))));
    simp("stack_assign", true, () -> list(asg(svar(10000), ci(7)), asg(var(3), ci(7))));
    simp("stack_assign_chain", true, () -> list(asg(var(4), asg(svar(10000), ci(7))), asg(var(3), ci(7))));
    simp("stack_assign2", false, () -> list(asg(svar(10000), ci(7)), asg(var(3), svar(10000))));
    simp("stack_assign2_first", true, () -> list(asg(svar(10000), ci(7)), asg(var(3), svar(10000))));
    simp("ctor_remote", true, () -> list(
      asg(var(1, obj("a/X")), newObj("a/X")),
      inv("a/B", "foo", "()V", null),
      init("a/X", "(I)V", var(1, obj("a/X")), ci(5))));
    simp("ctor_remote_used", true, () -> list(
      asg(var(1, obj("a/X")), newObj("a/X")),
      inv("a/B", "foo", "(La/X;)V", null, var(1, obj("a/X"))),
      init("a/X", "(I)V", var(1, obj("a/X")), ci(5))));
    simp("simple_ctor", true, () -> list(init("a/X", "(I)V", newObj("a/X"), ci(3))));
    simp("swap_ctor", true, () -> list(
      asg(var(1, obj("a/X")), newObj("a/X")),
      asg(var(2, obj("a/X")), var(1, obj("a/X"))),
      init("a/X", "()V", var(1, obj("a/X")))));
    simp("monitor_exit", true, () -> {
      MonitorExprent m = new MonitorExprent(MonitorExprent.Type.EXIT, var(1, obj("a/B")), null);
      m.setRemove(true);
      return list(m, asg(var(2), ci(1)));
    });
    simp("monitor_exit_kept", true, () -> {
      MonitorExprent m = new MonitorExprent(MonitorExprent.Type.EXIT, var(1, obj("a/B")), null);
      return list(m, asg(var(2), ci(1)));
    });
    simp("inline_ppi", true, () -> list(
      ppmm(FunctionType.PPI, var(1)),
      asg(new ArrayExprent(var(2, new VarType(CodeType.INT, 1)), var(1), INT, null), ci(2))));
    simp("inline_ppi_second", false, () -> list(
      ppmm(FunctionType.PPI, var(1)),
      asg(new ArrayExprent(var(2, new VarType(CodeType.INT, 1)), var(1), INT, null), ci(2))));
    simp("inline_ppi_blocked_sub", true, () -> list(
      ppmm(FunctionType.MMI, var(1)),
      asg(var(3), fn(FunctionType.SUB, var(1), ci(2)))));
    simp("null_check", true, () -> {
      InvocationExprent rnn = inv("java/util/Objects", "requireNonNull", "(Ljava/lang/Object;)Ljava/lang/Object;", null, var(1, obj("a/B")));
      rnn.setSyntheticNullCheck();
      return list(rnn, asg(var(2), ci(0)));
    });
    simp("null_check_kept", true, () -> list(
      inv("java/util/Objects", "requireNonNull", "(Ljava/lang/Object;)Ljava/lang/Object;", null, var(1, obj("a/B"))),
      asg(var(2), ci(0))));
    simp("assign_return", true, () -> list(
      asg(var(1), inv("a/B", "foo", "()I", null)),
      new ExitExprent(ExitExprent.Type.RETURN, var(1), INT, null, null)));
    simp("assign_return_stack", true, () -> list(
      asg(svar(10000), inv("a/B", "foo", "()I", null)),
      new ExitExprent(ExitExprent.Type.RETURN, svar(10000), INT, null, null)));
  }

  static void secondary() {
    sec("ppi_stmt", false, () -> list(ppmm(FunctionType.PPI, var(1))));
    sec("mmi_stmt", false, () -> list(ppmm(FunctionType.MMI, var(1))));
    sec("xor_m1", false, () -> list(asg(var(1), fn(FunctionType.XOR, var(2), ci(-1)))));
    sec("xor_m1_long", false, () -> list(asg(var(1, LONG), fn(FunctionType.XOR, new ConstExprent(LONG, -1L, null), var(2, LONG)))));
    sec("compound_add", false, () -> list(asg(var(1), fn(FunctionType.ADD, var(1), ci(5)))));
    sec("compound_add_rev", false, () -> list(asg(var(1), fn(FunctionType.ADD, ci(5), var(1)))));
    sec("compound_sub_rev_kept", false, () -> list(asg(var(1), fn(FunctionType.SUB, ci(5), var(1)))));
    sec("compound_cast", false, () -> list(asg(var(1, VarType.VARTYPE_BYTE), fn(FunctionType.I2B, fn(FunctionType.ADD, var(1, VarType.VARTYPE_BYTE), var(2))))));
    sec("not_lt", false, () -> list(asg(var(1, BOOL), fn(FunctionType.BOOL_NOT, fn(FunctionType.LT, var(2), var(3))))));
    sec("lcmp_lt", false, () -> list(asg(var(1, BOOL), fn(FunctionType.LT, fn(FunctionType.LCMP, var(2, LONG), var(3, LONG)), ci(0)))));
    sec("lcmp_eq_rev", false, () -> list(asg(var(1, BOOL), fn(FunctionType.EQ, ci(1), fn(FunctionType.LCMP, var(2, LONG), var(3, LONG))))));
    sec("fcmpl_lt", false, () -> list(asg(var(1, BOOL), fn(FunctionType.LT, fn(FunctionType.FCMPL, var(2, FLOAT), var(3, FLOAT)), ci(0)))));
    sec("fcmpg_gt", false, () -> list(asg(var(1, BOOL), fn(FunctionType.GT, fn(FunctionType.FCMPG, var(2, FLOAT), var(3, FLOAT)), ci(0)))));
    sec("fcmpl_ge_m1", false, () -> list(asg(var(1, BOOL), fn(FunctionType.GE, fn(FunctionType.FCMPL, var(2, FLOAT), var(3, FLOAT)), ci(-1)))));
    sec("ternary_bool", false, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), cbool(1), cbool(0)))));
    sec("ternary_bool_not", false, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), cbool(0), cbool(1)))));
    sec("ternary_or_off", false, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), cbool(1), var(3, BOOL)))));
    sec("ternary_or_forced", true, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), cbool(1), var(3, BOOL)))));
    sec("ternary_and_forced", true, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), var(3, BOOL), cbool(0)))));
    sec("ternary_self_forced", true, () -> list(asg(var(1, BOOL), fn(FunctionType.TERNARY, var(2, BOOL), var(2, BOOL), var(3, BOOL)))));
    sec("eq_bool_false", false, () -> list(asg(var(1, BOOL), fn(FunctionType.EQ, var(2, BOOL), cbool(0)))));
    sec("ne_bool_false", false, () -> list(asg(var(1, BOOL), fn(FunctionType.NE, cbool(0), var(2, BOOL)))));
    sec("not_const", false, () -> list(asg(var(1, BOOL), fn(FunctionType.BOOL_NOT, cbool(0)))));
    sec("lcmp_stmt", false, () -> list(fn(FunctionType.LCMP, var(2, LONG), var(3, LONG))));
    sec("dcmpg_stmt", false, () -> list(fn(FunctionType.DCMPG, var(2, VarType.VARTYPE_DOUBLE), var(3, VarType.VARTYPE_DOUBLE))));
    sec("i2l_widen", false, () -> list(asg(var(1, LONG), fn(FunctionType.I2L, var(2)))));
    sec("i2b_of_l2i", false, () -> list(asg(var(1, VarType.VARTYPE_BYTE), fn(FunctionType.I2B, fn(FunctionType.L2I, var(2, LONG))))));
    sec("add_casts", false, () -> list(asg(var(1, LONG), fn(FunctionType.ADD, fn(FunctionType.I2L, var(2)), fn(FunctionType.I2L, var(3))))));
    sec("shl_cast", false, () -> list(asg(var(1, LONG), fn(FunctionType.SHL, fn(FunctionType.I2L, var(2)), var(3)))));
    sec("not_and", false, () -> list(asg(var(1, BOOL), fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOLEAN_AND, var(2, BOOL), var(3, BOOL))))));
    sec("boxing_stmt", false, () -> list(inv("java/lang/Integer", "valueOf", "(I)Ljava/lang/Integer;", null, var(2))));
    sec("concat_nested", false, () -> list(asg(var(1, STR), sbToString(append(append(sb(), "Ljava/lang/String;", cs("a")), "I", var(2))))));
  }

  static void updates() {
    upd("mul", () -> list(asg(var(1), fn(FunctionType.MUL, var(1), var(2)))));
    upd("sub_rev_kept", () -> list(asg(var(1), fn(FunctionType.SUB, var(2), var(1)))));
    upd("cond_kept", () -> list(new AssignmentExprent(var(1), fn(FunctionType.ADD, var(1), var(2)), FunctionType.ADD, null)));
    upd("ushr", () -> list(asg(var(1), fn(FunctionType.USHR, var(1), ci(3))), asg(var(5), fn(FunctionType.OR, var(5), var(6)))));
  }

  static void propagate() {
    pbn("not_and", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOLEAN_AND, var(1, BOOL), var(2, BOOL))));
    pbn("not_or_nested", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOLEAN_OR, fn(FunctionType.EQ, var(1), var(2)), fn(FunctionType.BOOL_NOT, var(3, BOOL)))));
    pbn("not_not", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.BOOL_NOT, var(1, BOOL))));
    pbn("not_ternary", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.TERNARY, var(1, BOOL), var(2, BOOL), fn(FunctionType.LT, var(3), var(4)))));
    pbn("not_lt_float", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.LT, var(1, FLOAT), var(2, FLOAT))));
    pbn("not_le_int", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.LE, var(1), var(2))));
    pbn("not_eq_float", () -> fn(FunctionType.BOOL_NOT, fn(FunctionType.EQ, var(1, FLOAT), var(2, FLOAT))));
    pbn("not_var", () -> fn(FunctionType.BOOL_NOT, var(1, BOOL)));
    pbn("plain_eq", () -> fn(FunctionType.EQ, var(1), var(2)));
  }

  static void concats() {
    cat("sb_str_int", () -> list(asg(var(1, STR), sbToString(append(append(sb(), "Ljava/lang/String;", cs("a")), "I", var(2))))));
    cat("sb_int_int", () -> list(asg(var(1, STR), sbToString(append(append(sb(), "I", var(2)), "I", var(3))))));
    cat("sb_valueof", () -> list(asg(var(1, STR), sbToString(append(append(sb(), "Ljava/lang/String;",
      inv("java/lang/String", "valueOf", "(Ljava/lang/Object;)Ljava/lang/String;", null, var(2, obj("java/lang/Object")))),
      "Ljava/lang/String;", cs("x"))))));
    cat("sb_ctor_arg", () -> list(asg(var(1, STR), sbToString(append(((Supplier<Exprent>) () -> {
      NewExprent n = newObj(SB);
      n.setConstructor(init(SB, "(Ljava/lang/String;)V", null, var(3, STR)));
      return n;
    }).get(), "C", var(2, VarType.VARTYPE_CHAR))))));
    cat("sb_bad_append", () -> list(asg(var(1, STR), sbToString(append(sb(), "Ljava/lang/CharSequence;", var(2, obj("java/lang/CharSequence")))))));
    cat("indy_recipe", () -> list(asg(var(1, STR), concat("makeConcatWithConstants", "(II)Ljava/lang/String;",
      list0(new PrimitiveConstant(CodeConstants.CONSTANT_String, "x=\u0001,y=\u0001")), var(2), var(3)))));
    cat("indy_single", () -> list(asg(var(1, STR), concat("makeConcat", "(I)Ljava/lang/String;", new ArrayList<>(), var(2)))));
    cat("indy_string_first", () -> list(asg(var(1, STR), concat("makeConcat", "(Ljava/lang/String;I)Ljava/lang/String;", new ArrayList<>(), var(2, STR), var(3)))));
    cat("indy_short", () -> list(asg(var(1, STR), concat("makeConcatWithConstants", "(I)Ljava/lang/String;",
      list0(new PrimitiveConstant(CodeConstants.CONSTANT_String, "\u0001-\u0001")), var(2)))));
    cat("indy_valueof", () -> list(asg(var(1, STR), concat("makeConcatWithConstants", "(Ljava/lang/String;)Ljava/lang/String;",
      list0(new PrimitiveConstant(CodeConstants.CONSTANT_String, "v\u0001")),
      inv("java/lang/String", "valueOf", "(I)Ljava/lang/String;", null, var(2))))));
  }

  static List<PooledConstant> list0(PooledConstant... cs) {
    return new ArrayList<>(Arrays.asList(cs));
  }

  public static void main(String[] args) throws Exception {
    install();
    CL = StructClass.create(new DataInputFullStream(Files.readAllBytes(Paths.get(args[0]))), true);
    MT = CL.getMethods().get(0);
    simplify();
    secondary();
    updates();
    propagate();
    concats();
    System.out.print(OUT);
  }
}

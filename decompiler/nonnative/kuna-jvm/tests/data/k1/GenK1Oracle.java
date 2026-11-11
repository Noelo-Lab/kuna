// Golden vectors for the K1 slice (the expression IR), computed by the real
// Vineflower 1.12.0 classes. Regenerate with `sh gen.sh <vineflower-1.12.0.jar>`.
// Every scenario resets EXPRESSION_COUNTER to 1 and builds its exprents in the
// order tests/k1_exprents.rs mirrors.

import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.util.*;
import java.util.function.Supplier;

import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.SwitchStatement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericType;
import org.jetbrains.java.decompiler.util.collections.ListStack;

public class GenK1Oracle {
  static final String[] VARTYPE_NAMES = {
    "VARTYPE_UNKNOWN", "VARTYPE_INT", "VARTYPE_FLOAT", "VARTYPE_LONG", "VARTYPE_DOUBLE", "VARTYPE_BYTE",
    "VARTYPE_CHAR", "VARTYPE_SHORT", "VARTYPE_BOOLEAN", "VARTYPE_BYTECHAR", "VARTYPE_SHORTCHAR", "VARTYPE_NULL",
    "VARTYPE_STRING", "VARTYPE_CLASS", "VARTYPE_OBJECT", "VARTYPE_INTEGER", "VARTYPE_CHARACTER", "VARTYPE_BYTE_OBJ",
    "VARTYPE_SHORT_OBJ", "VARTYPE_BOOLEAN_OBJ", "VARTYPE_FLOAT_OBJ", "VARTYPE_DOUBLE_OBJ", "VARTYPE_LONG_OBJ",
    "VARTYPE_VOID", "VARTYPE_METHODTYPE"
  };

  static final StringBuilder OUT = new StringBuilder();

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

  static Object getField(Object o, Class<?> cls, String name) {
    try {
      Field f = cls.getDeclaredField(name);
      f.setAccessible(true);
      return f.get(o);
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
  }

  // ---- formatting ----

  static String b(boolean v) {
    return v ? "1" : "0";
  }

  static String hex(String s) {
    StringBuilder sb = new StringBuilder("x");
    for (char c : s.toCharArray()) sb.append(String.format("%04x", (int) c));
    return sb.toString();
  }

  static String vt(VarType t) {
    if (t == null) return "N";
    String s = t.type.name() + ":" + t.arrayDim + ":" + (t.value == null ? "N" : t.value);
    if (t instanceof GenericType g) {
      StringBuilder sb = new StringBuilder(s).append('<');
      for (int i = 0; i < g.getArguments().size(); i++) {
        if (i > 0) sb.append(';');
        sb.append(vt(g.getArguments().get(i)));
      }
      s = sb.append('>').toString();
    }
    return s;
  }

  static String vtc(VarType t) {
    if (t == null) return "N";
    for (String name : VARTYPE_NAMES) {
      try {
        if (VarType.class.getField(name).get(null) == t) return name;
      } catch (ReflectiveOperationException e) {
        throw new RuntimeException(e);
      }
    }
    return "~" + vt(t);
  }

  static String bits(BitSet s) {
    if (s == null) return "N";
    if (s.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (int i = s.nextSetBit(0); i >= 0; i = s.nextSetBit(i + 1)) {
      if (sb.length() > 0) sb.append('.');
      sb.append(i);
    }
    return sb.toString();
  }

  static String valText(Object v) {
    return v == null ? "N" : String.valueOf(v);
  }

  static String desc(Exprent e) {
    StringBuilder sb = new StringBuilder(e.type.name());
    if (e instanceof FunctionExprent f) sb.append(':').append(f.getFuncType().name());
    else if (e instanceof VarExprent v) sb.append(':').append(v.getIndex()).append('_').append(v.getVersion());
    else if (e instanceof ConstExprent c) sb.append(':').append(valText(c.getValue()));
    else if (e instanceof FieldExprent f) sb.append(':').append(f.getName());
    else if (e instanceof InvocationExprent i) sb.append(':').append(i.getName());
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

  static String ids(List<? extends Exprent> lst) {
    if (lst.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (Exprent e : lst) {
      if (sb.length() > 0) sb.append(',');
      sb.append(e.id);
    }
    return sb.toString();
  }

  static void pre(Exprent e, List<Exprent> out) {
    out.add(e);
    for (Exprent c : e.getAllExprents()) pre(c, out);
  }

  static String share(Exprent a, Exprent b) {
    List<Exprent> pa = new ArrayList<>();
    List<Exprent> pb = new ArrayList<>();
    pre(a, pa);
    pre(b, pb);
    if (pa.size() != pb.size()) return "size" + pa.size() + "/" + pb.size();
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < pa.size(); i++) sb.append(pa.get(i) == pb.get(i) ? 'S' : 'D');
    return sb.toString();
  }

  static String guard(Supplier<String> body) {
    try {
      return body.get();
    } catch (RuntimeException e) {
      return "throw:" + e.getClass().getSimpleName();
    }
  }

  // ---- builders (mirrored one to one in k1_exprents.rs) ----

  static BitSet bs(int... b) {
    BitSet s = new BitSet();
    for (int x : b) s.set(x);
    return s;
  }

  static List<Exprent> list(Exprent... es) {
    return new ArrayList<>(Arrays.asList(es));
  }

  static ConstExprent ci(int v) {
    return new ConstExprent(v, false, null);
  }

  static ConstExprent cb(int v) {
    return new ConstExprent(v, true, null);
  }

  static ConstExprent ct(VarType t, Object v) {
    return new ConstExprent(t, v, null);
  }

  static VarExprent var(int index) {
    return new VarExprent(index, VarType.VARTYPE_INT, null);
  }

  static VarExprent var(int index, int version, VarType t) {
    VarExprent v = new VarExprent(index, t, null);
    v.setVersion(version);
    return v;
  }

  static VarExprent defVar(int index) {
    VarExprent v = var(index);
    v.setDefinition(true);
    return v;
  }

  static VarType obj(String cls) {
    return new VarType(CodeType.OBJECT, 0, cls);
  }

  static FunctionExprent fn(FunctionType t, Exprent... ops) {
    return new FunctionExprent(t, list(ops), null);
  }

  static FieldExprent sfield(String name, String desc) {
    return new FieldExprent(name, "a/B", true, null, FieldDescriptor.parseDescriptor(desc), null);
  }

  static FieldExprent ifield(String name, Exprent inst, String desc) {
    return new FieldExprent(name, "a/B", false, inst, FieldDescriptor.parseDescriptor(desc), null);
  }

  static InvocationExprent inv(String name, String desc, Exprent inst, Exprent... params) {
    InvocationExprent e = new InvocationExprent();
    e.setName(name);
    e.setClassname("a/B");
    e.setStringDescriptor(desc);
    e.setDescriptor(MethodDescriptor.parseDescriptor(desc));
    e.setStatic(inst == null);
    e.setInstance(inst);
    e.setLstParameters(list(params));
    return e;
  }

  static AssignmentExprent asg(Exprent l, Exprent r) {
    return new AssignmentExprent(l, r, null);
  }

  static ArrayExprent arr(Exprent a, Exprent i, VarType t) {
    return new ArrayExprent(a, i, t, null);
  }

  static ExitExprent ret(Exprent v, VarType t) {
    return new ExitExprent(ExitExprent.Type.RETURN, v, t, null, null);
  }

  static IfExprent ifx(IfExprent.Type t, Exprent a, Exprent b) {
    ListStack<Exprent> s = new ListStack<>();
    s.push(a);
    s.push(b);
    return new IfExprent(t, s, null);
  }

  static NewExprent newArr(VarType t, List<Exprent> dims, List<Exprent> elems, boolean direct) {
    NewExprent n = new NewExprent(t, dims, null);
    n.setLstArrayElements(elems);
    n.setDirectArrayInit(direct);
    return n;
  }

  static NewExprent newObj(String cls, InvocationExprent ctor) {
    NewExprent n = new NewExprent(obj(cls), list(), null);
    n.setConstructor(ctor);
    return n;
  }

  static AnnotationExprent ann(String cls, List<String> names, List<Exprent> values) {
    return new AnnotationExprent(cls, names, values);
  }

  static PatternExprent pat(String cls, List<Exprent> parts) {
    return new PatternExprent(PatternExprent.recordData(null), obj(cls), parts);
  }

  static SwitchStatement switchStat() {
    try {
      Constructor<SwitchStatement> c = SwitchStatement.class.getDeclaredConstructor();
      c.setAccessible(true);
      return c.newInstance();
    } catch (ReflectiveOperationException e) {
      throw new RuntimeException(e);
    }
  }

  static SwitchHeadExprent switchHead(Exprent value, List<List<Exprent>> cases) {
    SwitchHeadExprent sh = new SwitchHeadExprent(value, null);
    sh.setCaseValues(cases);
    return sh;
  }

  // ---- 1. construction order ----

  static void ctor() {
    reset();
    {
      ConstExprent c1 = ci(5);
      ConstExprent c2 = ct(VarType.VARTYPE_INT, 7);
      VarExprent v = var(3);
      FunctionExprent f = fn(FunctionType.ADD, c1, c2);
      AssignmentExprent a = new AssignmentExprent(v, f, null);
      p("ctor", "basic", desc(a));
    }

    for (IfExprent.Type t : IfExprent.Type.values()) {
      reset();
      ListStack<Exprent> stack = new ListStack<>();
      stack.push(var(1));
      if (t.ordinal() >= IfExprent.Type.ICMPEQ.ordinal() && t != IfExprent.Type.VALUE) stack.push(var(2));
      IfExprent ie = new IfExprent(t, stack, bs(4));
      p("ctor", "if_" + t.name(), desc(ie), stack.size());
    }

    reset();
    {
      ListStack<Exprent> stack = new ListStack<>();
      stack.push(var(1));
      stack.push(var(2));
      stack.push(var(3));
      FunctionExprent add = new FunctionExprent(FunctionType.ADD, stack, null);
      FunctionExprent neg = new FunctionExprent(FunctionType.NEG, stack, bs(2));
      String ternary = guard(() -> desc(new FunctionExprent(FunctionType.TERNARY, stack, null)));
      ConstExprent after = ci(0);
      p("ctor", "fnstack", desc(add), desc(neg), ternary, desc(after), stack.size());
    }

    reset();
    {
      ListStack<Exprent> stack = new ListStack<>();
      stack.push(var(1));
      stack.push(ci(2));
      stack.push(ci(3));
      NewExprent ne = new NewExprent(new VarType(CodeType.INT, 2), stack, 2, null);
      p("ctor", "newstack", desc(ne), stack.size());
    }

    reset();
    {
      InvocationExprent ie = inv("m", "(IJ)V", var(1), ci(2), fn(FunctionType.NEG, var(3)));
      ie.addBytecodeOffsets(bs(7));
      Exprent cp = ie.copy();
      p("ctor", "invcopy", desc(ie), desc(cp));
    }

    reset();
    {
      VarExprent inst = var(1);
      VarExprent p0 = var(1);
      InvocationExprent ie = inv("apply", "(I)V", inst, p0);
      setField(ie, InvocationExprent.class, "invocationType", InvocationExprent.InvocationType.DYNAMIC);
      InvocationExprent cp = (InvocationExprent) ie.copy();
      p("ctor", "invdyn", desc(ie), desc(cp), b(cp.getInstance() == cp.getLstParameters().get(0)));
    }

    reset();
    {
      IfExprent ie = new IfExprent(IfExprent.Type.ICMPLT, stackOf(var(1), var(2)), bs(1));
      ie.negateIf();
      p("ctor", "negate", desc(ie));
    }
  }

  static ListStack<Exprent> stackOf(Exprent... es) {
    ListStack<Exprent> s = new ListStack<>();
    for (Exprent e : es) s.push(e);
    return s;
  }

  // ---- 2. copy ----

  static final String[] COPY_LABELS = {
    "annotation", "array", "assert", "assert_null", "assignment", "const", "const_bool", "const_class",
    "const_condy", "exit_return", "exit_void", "exit_throw", "field_static", "field_instance", "function",
    "function_pp", "if", "invocation", "invocation_static", "monitor", "new_array", "new_array_init",
    "new_object", "pattern", "switch", "switch_head", "var", "yield"
  };

  static Exprent copySubject(String label) {
    switch (label) {
      case "annotation":
        return ann("a/Ann", new ArrayList<>(List.of("value")), list(ci(1)));
      case "array":
        return new ArrayExprent(var(1, 0, new VarType(CodeType.INT, 1)), ci(0), VarType.VARTYPE_INT, bs(2, 5));
      case "assert":
        return new AssertExprent(list(fn(FunctionType.NE, var(1), ci(0)), ct(VarType.VARTYPE_STRING, "m")));
      case "assert_null":
        return new AssertExprent(list((Exprent) null));
      case "assignment":
        return new AssignmentExprent(var(1), fn(FunctionType.ADD, var(1), ci(1)), FunctionType.ADD, bs(3));
      case "const":
        return new ConstExprent(VarType.VARTYPE_INT, 5, bs(1));
      case "const_bool":
        return cb(1);
      case "const_class":
        return ct(VarType.VARTYPE_CLASS, "java/lang/String");
      case "const_condy":
        return new ConstExprent(VarType.VARTYPE_STRING, "s", null, true);
      case "exit_return":
        return new ExitExprent(ExitExprent.Type.RETURN, var(1), VarType.VARTYPE_INT, bs(9), MethodDescriptor.parseDescriptor("()I"));
      case "exit_void":
        return new ExitExprent(ExitExprent.Type.RETURN, null, VarType.VARTYPE_VOID, null, MethodDescriptor.parseDescriptor("()V"));
      case "exit_throw":
        return new ExitExprent(ExitExprent.Type.THROW, var(2, 0, obj("java/lang/Throwable")), null, null, null);
      case "field_static":
        return new FieldExprent("F", "a/B", true, null, FieldDescriptor.parseDescriptor("J"), bs(4));
      case "field_instance":
        return ifield("f", var(1, 0, obj("a/B")), "I");
      case "function":
        return new FunctionExprent(FunctionType.ADD, list(var(1), fn(FunctionType.MUL, ci(2), var(2))), bs(6));
      case "function_pp": {
        FunctionExprent f = fn(FunctionType.IPP, var(1));
        f.setImplicitType(VarType.VARTYPE_INT);
        return f;
      }
      case "if":
        return new IfExprent(IfExprent.Type.ICMPLT, stackOf(var(1), var(2)), bs(8));
      case "invocation":
        return inv("m", "(IJ)V", var(1), ci(2), ct(VarType.VARTYPE_LONG, 3L));
      case "invocation_static":
        return inv("s", "()V", null);
      case "monitor":
        return new MonitorExprent(MonitorExprent.Type.ENTER, var(1), bs(0));
      case "new_array":
        return new NewExprent(new VarType(CodeType.INT, 1), list(ci(3)), bs(2));
      case "new_array_init":
        return newArr(new VarType(CodeType.INT, 1), list(), list(ci(1), ci(2)), true);
      case "new_object": {
        InvocationExprent c = inv("<init>", "(I)V", var(9), ci(5));
        c.setFunctype(InvocationExprent.Type.INIT);
        NewExprent n = new NewExprent(obj("a/B"), list(), bs(11));
        n.setConstructor(c);
        return n;
      }
      case "pattern":
        return pat("a/R", list(var(1), pat("a/S", list(var(2)))));
      case "switch":
        return new SwitchExprent(switchStat(), VarType.VARTYPE_INT, false, true);
      case "switch_head": {
        List<List<Exprent>> cases = new ArrayList<>();
        cases.add(list(ci(1), ci(2)));
        cases.add(list((Exprent) null));
        SwitchHeadExprent sh = new SwitchHeadExprent(var(1), bs(3));
        sh.setCaseValues(cases);
        return sh;
      }
      case "var": {
        VarExprent v = new VarExprent(3, VarType.VARTYPE_LONG, null, bs(5));
        v.setVersion(2);
        v.setDefinition(true);
        v.setStack(true);
        v.setEffectivelyFinal(true);
        v.setCatchTempVar(true);
        v.setClassDef(true);
        return v;
      }
      case "yield":
        return new YieldExprent(fn(FunctionType.ADD, var(1), ci(1)), VarType.VARTYPE_INT);
    }
    throw new IllegalArgumentException(label);
  }

  static String varFlags(VarExprent v) {
    return v.getIndex() + "," + v.getVersion() + "," + b(v.isDefinition()) + b(v.isClassDef()) + b(v.isStack())
      + b(v.isEffectivelyFinal()) + b(v.isCatchTempVar()) + "," + vt(v.getVarType());
  }

  static void copies() {
    for (String label : COPY_LABELS) {
      reset();
      Exprent orig = copySubject(label);
      Exprent cp = orig.copy();
      p("copy", label, desc(orig), desc(cp), b(cp.equals(orig)), b(orig.equals(cp)), share(orig, cp));

      if (orig instanceof ConstExprent c) {
        ConstExprent cc = (ConstExprent) cp;
        p("copyconst", label, vt(c.getConstType()), vt(cc.getConstType()), b(c.isBoolPermitted()) + b(cc.isBoolPermitted()),
          b((Boolean) getField(c, ConstExprent.class, "wasCondy")) + b((Boolean) getField(cc, ConstExprent.class, "wasCondy")));
      }
      if (orig instanceof VarExprent v) {
        p("copyvar", label, varFlags(v), varFlags((VarExprent) cp));
      }
      if (orig instanceof SwitchHeadExprent sh) {
        List<List<Exprent>> a = sh.getCaseValues();
        List<List<Exprent>> c = ((SwitchHeadExprent) cp).getCaseValues();
        StringBuilder sb = new StringBuilder(a == c ? "S" : "D");
        for (int i = 0; i < a.size(); i++) {
          sb.append(':').append(a.get(i) == c.get(i) ? 'S' : 'D');
          for (int j = 0; j < a.get(i).size(); j++) {
            Exprent x = a.get(i).get(j);
            sb.append(x == null ? 'n' : x == c.get(i).get(j) ? 'S' : 'D');
          }
        }
        p("copysh", label, sb);
      }
      if (orig instanceof SwitchExprent sw) {
        p("copysw", label, b(sw.getBacking() == ((SwitchExprent) cp).getBacking()));
      }
      if (orig instanceof InvocationExprent ie) {
        InvocationExprent ic = (InvocationExprent) cp;
        p("copyinv", label, ie.getName() + "," + ie.getClassname() + "," + ie.getStringDescriptor() + "," + b(ie.isStatic()) + "," + ie.getFunctype(),
          ic.getName() + "," + ic.getClassname() + "," + ic.getStringDescriptor() + "," + b(ic.isStatic()) + "," + ic.getFunctype(),
          b(ie.getLstParameters() != ic.getLstParameters()));
      }
    }
  }

  // ---- 3. equals ----

  static void eq(String label, Exprent a, Exprent b) {
    p("eq", label, b(a.equals(b)), b(b.equals(a)));
  }

  static InvocationExprent invNoDesc(String name) {
    InvocationExprent e = new InvocationExprent();
    e.setName(name);
    e.setClassname("a/B");
    e.setStatic(true);
    return e;
  }

  static void equalsTable() {
    reset();
    VarType INT = VarType.VARTYPE_INT;
    VarType LONG = VarType.VARTYPE_LONG;

    eq("const_same", ci(5), ci(5));
    eq("const_value", ci(5), ci(6));
    eq("const_type", ct(INT, 5), ci(5));
    eq("const_null", ct(VarType.VARTYPE_NULL, null), ct(VarType.VARTYPE_NULL, null));
    eq("const_null_vs_value", ct(VarType.VARTYPE_STRING, null), ct(VarType.VARTYPE_STRING, "a"));
    eq("const_boxed", ct(INT, 1), ct(INT, 1L));
    eq("const_bool_permitted_ignored", new ConstExprent(1, true, null), ct(VarType.VARTYPE_BOOLEAN, 1));
    eq("const_vs_var", ci(1), var(1));
    { ConstExprent c = ci(1); eq("const_identity", c, c); }

    eq("var_same", var(1, 2, INT), var(1, 2, INT));
    eq("var_index", var(1), var(2));
    eq("var_version", var(1, 1, INT), var(1, 2, INT));
    eq("var_type", var(1, 0, INT), var(1, 0, LONG));
    eq("var_null_type", new VarExprent(1, null, null), new VarExprent(1, VarType.VARTYPE_UNKNOWN, null));
    eq("var_def_ignored", defVar(1), var(1));

    eq("field_same", ifield("f", var(1), "I"), ifield("f", var(1), "I"));
    eq("field_name", sfield("a", "I"), sfield("b", "I"));
    eq("field_class", new FieldExprent("f", "a/B", true, null, FieldDescriptor.parseDescriptor("I"), null),
      new FieldExprent("f", "a/C", true, null, FieldDescriptor.parseDescriptor("I"), null));
    eq("field_static", new FieldExprent("f", "a/B", true, null, FieldDescriptor.parseDescriptor("I"), null),
      new FieldExprent("f", "a/B", false, null, FieldDescriptor.parseDescriptor("I"), null));
    eq("field_instance", ifield("f", var(1), "I"), ifield("f", var(2), "I"));
    eq("field_instance_null", ifield("f", null, "I"), ifield("f", var(1), "I"));
    eq("field_descriptor", sfield("f", "I"), sfield("f", "J"));

    eq("function_same", fn(FunctionType.ADD, var(1), ci(2)), fn(FunctionType.ADD, var(1), ci(2)));
    eq("function_type", fn(FunctionType.ADD, var(1), ci(2)), fn(FunctionType.SUB, var(1), ci(2)));
    eq("function_operand", fn(FunctionType.ADD, var(1), ci(2)), fn(FunctionType.ADD, var(1), ci(3)));
    eq("function_order", fn(FunctionType.ADD, var(1), ci(2)), fn(FunctionType.ADD, ci(2), var(1)));
    eq("function_arity", fn(FunctionType.ADD, var(1), ci(2)), fn(FunctionType.ADD, var(1)));
    {
      FunctionExprent a = fn(FunctionType.IPP, var(1));
      a.setImplicitType(INT);
      FunctionExprent c = fn(FunctionType.IPP, var(1));
      c.setImplicitType(LONG);
      eq("function_implicit_ignored", a, c);
    }

    eq("assignment_same", asg(var(1), ci(2)), asg(var(1), ci(2)));
    eq("assignment_left", asg(var(1), ci(2)), asg(var(2), ci(2)));
    eq("assignment_right", asg(var(1), ci(2)), asg(var(1), ci(3)));
    eq("assignment_cond", new AssignmentExprent(var(1), ci(2), FunctionType.ADD, null), asg(var(1), ci(2)));
    eq("assignment_cond_same", new AssignmentExprent(var(1), ci(2), FunctionType.ADD, null),
      new AssignmentExprent(var(1), ci(2), FunctionType.ADD, null));

    eq("array_same", arr(var(1), ci(0), INT), arr(var(1), ci(0), INT));
    eq("array_array", arr(var(1), ci(0), INT), arr(var(2), ci(0), INT));
    eq("array_index", arr(var(1), ci(0), INT), arr(var(1), ci(1), INT));
    eq("array_hardtype_ignored", arr(var(1), ci(0), INT), arr(var(1), ci(0), LONG));

    eq("exit_same", ret(var(1), INT), ret(var(1), INT));
    eq("exit_type", ret(var(1), INT), new ExitExprent(ExitExprent.Type.THROW, var(1), null, null, null));
    eq("exit_value", ret(var(1), INT), ret(var(2), INT));
    eq("exit_void", ret(null, VarType.VARTYPE_VOID), ret(null, VarType.VARTYPE_VOID));
    eq("exit_value_null", ret(var(1), INT), ret(null, VarType.VARTYPE_VOID));
    eq("exit_rettype_ignored", ret(var(1), INT), ret(var(1), LONG));

    eq("if_same", ifx(IfExprent.Type.ICMPEQ, var(1), var(2)), ifx(IfExprent.Type.ICMPEQ, var(1), var(2)));
    eq("if_cond", ifx(IfExprent.Type.ICMPEQ, var(1), var(2)), ifx(IfExprent.Type.ICMPNE, var(1), var(2)));
    {
      IfExprent ie = ifx(IfExprent.Type.ICMPEQ, var(1), var(2));
      eq("if_vs_condition", ie, ie.getCondition());
    }

    eq("inv_same", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)V", var(1), ci(2)));
    eq("inv_name", inv("m", "(I)V", var(1), ci(2)), inv("n", "(I)V", var(1), ci(2)));
    {
      InvocationExprent a = inv("m", "(I)V", var(1), ci(2));
      InvocationExprent c = inv("m", "(I)V", var(1), ci(2));
      c.setClassname("a/C");
      eq("inv_class", a, c);
    }
    {
      InvocationExprent a = inv("m", "(I)V", null, ci(2));
      InvocationExprent c = inv("m", "(I)V", null, ci(2));
      c.setStatic(false);
      eq("inv_static", a, c);
    }
    eq("inv_instance", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)V", var(2), ci(2)));
    eq("inv_instance_null", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)V", null, ci(2)));
    eq("inv_descriptor", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)I", var(1), ci(2)));
    eq("inv_nodesc_both", invNoDesc("m"), invNoDesc("m"));
    eq("inv_nodesc_one", invNoDesc("m"), inv("m", "()V", null));
    {
      InvocationExprent a = inv("<init>", "()V", var(1));
      InvocationExprent c = inv("<init>", "()V", var(1));
      c.setFunctype(InvocationExprent.Type.INIT);
      eq("inv_functype", a, c);
    }
    eq("inv_params", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)V", var(1), ci(3)));
    eq("inv_param_count", inv("m", "(I)V", var(1), ci(2)), inv("m", "(I)V", var(1), ci(2), ci(2)));
    {
      InvocationExprent a = inv("m", "(I)V", var(1), ci(2));
      InvocationExprent c = inv("m", "(I)V", var(1), ci(2));
      c.setStringDescriptor("(J)V");
      eq("inv_strdesc_ignored", a, c);
    }
    {
      InvocationExprent a = inv("m", "(I)V", var(1), ci(2));
      InvocationExprent c = inv("m", "(I)V", var(1), ci(2));
      setField(c, InvocationExprent.class, "invocationType", InvocationExprent.InvocationType.DYNAMIC);
      eq("inv_invtype_ignored", a, c);
    }

    eq("monitor_same", new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null), new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null));
    eq("monitor_type", new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null), new MonitorExprent(MonitorExprent.Type.EXIT, var(1), null));
    eq("monitor_value", new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null), new MonitorExprent(MonitorExprent.Type.ENTER, var(2), null));

    VarType intArr = new VarType(CodeType.INT, 1);
    eq("new_same", newArr(intArr, list(ci(3)), list(), false), newArr(intArr, list(ci(3)), list(), false));
    eq("new_type", newArr(intArr, list(ci(3)), list(), false), newArr(new VarType(CodeType.LONG, 1), list(ci(3)), list(), false));
    eq("new_dims", newArr(intArr, list(ci(3)), list(), false), newArr(intArr, list(ci(4)), list(), false));
    eq("new_elements", newArr(intArr, list(), list(ci(1)), true), newArr(intArr, list(), list(ci(2)), true));
    eq("new_direct", newArr(intArr, list(), list(ci(1)), true), newArr(intArr, list(), list(ci(1)), false));
    eq("new_ctor", newObj("a/B", inv("<init>", "()V", null)), newObj("a/B", null));
    eq("new_ctor_same", newObj("a/B", inv("<init>", "(I)V", null, ci(1))), newObj("a/B", inv("<init>", "(I)V", null, ci(1))));
    eq("new_ctor_param", newObj("a/B", inv("<init>", "(I)V", null, ci(1))), newObj("a/B", inv("<init>", "(I)V", null, ci(2))));
    {
      NewExprent a = newArr(intArr, list(ci(3)), list(), false);
      NewExprent c = newArr(intArr, list(ci(3)), list(), false);
      c.setVarArgParam(true);
      eq("new_vararg_ignored", a, c);
    }

    eq("switch_head_same", switchHead(var(1), new ArrayList<>()), switchHead(var(1), new ArrayList<>()));
    eq("switch_head_value", switchHead(var(1), new ArrayList<>()), switchHead(var(2), new ArrayList<>()));
    {
      List<List<Exprent>> cases = new ArrayList<>();
      cases.add(list(ci(1)));
      eq("switch_head_cases_ignored", switchHead(var(1), cases), switchHead(var(1), new ArrayList<>()));
    }

    eq("annotation_same", ann("a/A", new ArrayList<>(List.of("value")), list(ci(1))), ann("a/A", new ArrayList<>(List.of("value")), list(ci(1))));
    eq("annotation_class", ann("a/A", new ArrayList<>(List.of("value")), list(ci(1))), ann("a/C", new ArrayList<>(List.of("value")), list(ci(1))));
    eq("annotation_names", ann("a/A", new ArrayList<>(List.of("value")), list(ci(1))), ann("a/A", new ArrayList<>(List.of("x")), list(ci(1))));
    eq("annotation_values", ann("a/A", new ArrayList<>(List.of("value")), list(ci(1))), ann("a/A", new ArrayList<>(List.of("value")), list(ci(2))));
    eq("annotation_marker", ann("a/A", new ArrayList<>(), list()), ann("a/A", new ArrayList<>(), list()));

    eq("assert_structural", new AssertExprent(list(var(1))), new AssertExprent(list(var(1))));
    { AssertExprent a = new AssertExprent(list(var(1))); eq("assert_identity", a, a); }
    eq("yield_structural", new YieldExprent(var(1), INT), new YieldExprent(var(1), INT));
    { YieldExprent y = new YieldExprent(var(1), INT); eq("yield_identity", y, y); }
    eq("pattern_structural", pat("a/R", list(var(1))), pat("a/R", list(var(1))));
    {
      SwitchStatement backing = switchStat();
      eq("switch_structural", new SwitchExprent(backing, INT, false, true), new SwitchExprent(backing, INT, false, true));
    }

    eq("var_vs_field", var(1), sfield("f", "I"));
    eq("function_vs_assignment", fn(FunctionType.ADD, var(1), ci(2)), asg(var(1), ci(2)));
    eq("monitor_vs_exit", new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null), ret(var(1), INT));
    eq("array_vs_new", arr(var(1), ci(0), INT), newArr(intArr, list(ci(0)), list(), false));
  }

  // ---- 4. getAllExprents orders ----

  static final String[] ALL_LABELS = {
    "tree", "new_object", "new_object_noinst", "new_object_noctor", "new_array", "exit", "exit_void", "if",
    "switch_head", "annotation", "assert", "pattern", "yield", "monitor", "leaf_const", "leaf_var", "switch"
  };

  static Exprent allSubject(String label) {
    switch (label) {
      case "tree":
        return asg(arr(var(1), ci(0), VarType.VARTYPE_INT),
          fn(FunctionType.ADD, inv("m", "(II)I", var(2), ci(1), fn(FunctionType.NEG, var(3))), ifield("f", var(4), "I")));
      case "new_object":
        return newObj("a/B", inv("<init>", "(II)V", var(1), ci(1), fn(FunctionType.ADD, var(2), ci(2))));
      case "new_object_noinst":
        return newObj("a/B", inv("<init>", "(II)V", null, ci(1), ci(2)));
      case "new_object_noctor":
        return newObj("a/B", null);
      case "new_array":
        return newArr(new VarType(CodeType.INT, 2), list(ci(2), var(1)),
          list(newArr(new VarType(CodeType.INT, 1), list(), list(ci(5), var(2)), true)), false);
      case "exit":
        return ret(fn(FunctionType.MUL, var(1), var(2)), VarType.VARTYPE_INT);
      case "exit_void":
        return ret(null, VarType.VARTYPE_VOID);
      case "if":
        return ifx(IfExprent.Type.ICMPGE, fn(FunctionType.ADD, var(1), ci(1)), var(2));
      case "switch_head": {
        List<List<Exprent>> cases = new ArrayList<>();
        cases.add(list(ci(1)));
        return switchHead(fn(FunctionType.ADD, var(1), ci(1)), cases);
      }
      case "annotation":
        return ann("a/A", new ArrayList<>(List.of("x", "y")), list(ci(1), newArr(new VarType(CodeType.INT, 1), list(), list(ci(2)), true)));
      case "assert":
        return new AssertExprent(list(null, ct(VarType.VARTYPE_STRING, "msg")));
      case "pattern":
        return pat("a/R", list(var(1), pat("a/S", list(var(2), var(3))), var(4)));
      case "yield":
        return new YieldExprent(fn(FunctionType.ADD, var(1), var(2)), VarType.VARTYPE_INT);
      case "monitor":
        return new MonitorExprent(MonitorExprent.Type.EXIT, fn(FunctionType.CAST, var(1), ct(VarType.VARTYPE_NULL, null)), null);
      case "leaf_const":
        return ci(1);
      case "leaf_var":
        return var(1);
      case "switch":
        return new SwitchExprent(switchStat(), VarType.VARTYPE_INT, false, false);
    }
    throw new IllegalArgumentException(label);
  }

  static String pairs(Set<VarVersionPair> set) {
    if (set.isEmpty()) return "-";
    StringBuilder sb = new StringBuilder();
    for (VarVersionPair p : set) {
      if (sb.length() > 0) sb.append(',');
      sb.append(p.var).append('.').append(p.version);
    }
    return sb.toString();
  }

  static void allOrders() {
    for (String label : ALL_LABELS) {
      reset();
      Exprent e = allSubject(label);
      p("all", label, desc(e), ids(e.getAllExprents()), ids(e.getAllExprents(false)), ids(e.getAllExprents(true)),
        ids(e.getAllExprents(true, true)), ids(e.getAllExprents(false, true)));
    }

    reset();
    {
      Exprent e = fn(FunctionType.ADD,
        fn(FunctionType.SUB, var(17, 3, VarType.VARTYPE_INT), var(1, 0, VarType.VARTYPE_INT)),
        fn(FunctionType.MUL, var(33, 1, VarType.VARTYPE_INT), fn(FunctionType.NEG, var(1, 0, VarType.VARTYPE_INT))));
      p("vars", "nested", pairs(e.getAllVariables()));
      p("vars", "self", pairs(var(5, 4, VarType.VARTYPE_INT).getAllVariables()));
      p("vars", "none", pairs(ci(1).getAllVariables()));
    }

    reset();
    {
      VarExprent inner = var(2);
      Exprent e = asg(var(1), fn(FunctionType.ADD, inner, ci(3)));
      p("contains", "equal_copy", b(e.containsExprent(var(2))));
      p("contains", "self", b(e.containsExprent(e)));
      p("contains", "self_copy", b(e.containsExprent(asg(var(1), fn(FunctionType.ADD, var(2), ci(3))))));
      p("contains", "absent", b(e.containsExprent(var(9))));
      p("contains", "var_present", b(e.containsVar(new VarVersionPair(2, 0))));
      p("contains", "var_absent", b(e.containsVar(new VarVersionPair(2, 1))));
      p("contains", "var_self", b(var(7, 3, VarType.VARTYPE_INT).containsVar(new VarVersionPair(7, 3))));
    }

    reset();
    {
      PatternExprent pe = pat("a/R", list(var(1), pat("a/S", list(var(2), pat("a/T", list(var(3))))), var(4)));
      p("patvars", "nested", ids(pe.getPatternVars()));
      VarExprent v = var(8);
      p("patvars", "var", ids(v.getPatternVars()));
    }

    reset();
    {
      List<Exprent> lst = list(defVar(5), defVar(2), ci(0), defVar(9), defVar(1), defVar(9), var(4), defVar(3), defVar(-2));
      p("sortidx", "mixed", ids(Exprent.sortIndexed(lst)));
    }
  }

  // ---- 5. replaceExprent ----

  static void repl(String label, Exprent parent, Exprent old, Exprent neu) {
    String before = desc(parent);
    String after = guard(() -> {
      parent.replaceExprent(old, neu);
      return desc(parent);
    });
    p("repl", label, before, after);
  }

  static final String[] REPL_LABELS = {
    "array", "array_equal_not_identical", "assert", "assignment", "exit", "exit_null", "field", "field_static",
    "function", "function_nested", "if_condition", "if_nested", "invocation", "monitor", "new_dims", "new_elems",
    "new_ctor_param", "new_ctor_swap", "new_ctor_badcast", "switch_head", "yield", "base_annotation",
    "base_pattern", "base_const", "base_var", "base_switch"
  };

  static void replaces() {
    for (String label : REPL_LABELS) {
      reset();
      replCase(label);
    }
  }

  static void replCase(String label) {
    VarType INT = VarType.VARTYPE_INT;
    switch (label) {
      case "array": {
        VarExprent x = var(1);
        repl(label, new ArrayExprent(x, x, INT, null), x, ci(9));
        return;
      }
      case "array_equal_not_identical": {
        ArrayExprent a = arr(var(1), ci(0), INT);
        repl(label, a, var(1), ci(9));
        return;
      }
      case "assert": {
        VarExprent x = var(1);
        repl(label, new AssertExprent(list(x, null, x)), x, ci(9));
        return;
      }
      case "assignment": {
        VarExprent x = var(1);
        repl(label, asg(x, x), x, ci(9));
        return;
      }
      case "exit": {
        VarExprent x = var(1);
        repl(label, ret(x, INT), x, ci(9));
        return;
      }
      case "exit_null":
        repl(label, ret(null, VarType.VARTYPE_VOID), var(1), ci(9));
        return;
      case "field": {
        VarExprent x = var(1);
        repl(label, ifield("f", x, "I"), x, var(2));
        return;
      }
      case "field_static":
        repl(label, sfield("f", "I"), var(1), var(2));
        return;
      case "function": {
        VarExprent x = var(1);
        repl(label, fn(FunctionType.ADD, x, x), x, ci(9));
        return;
      }
      case "function_nested": {
        VarExprent x = var(1);
        repl(label, fn(FunctionType.ADD, fn(FunctionType.NEG, x), x), x, ci(9));
        return;
      }
      case "if_condition": {
        IfExprent ie = ifx(IfExprent.Type.ICMPEQ, var(1), var(2));
        repl(label, ie, ie.getCondition(), var(3));
        return;
      }
      case "if_nested": {
        VarExprent x = var(1);
        repl(label, ifx(IfExprent.Type.ICMPEQ, x, var(2)), x, var(3));
        return;
      }
      case "invocation": {
        VarExprent x = var(1);
        repl(label, inv("m", "(II)V", x, x, var(2), x), x, ci(9));
        return;
      }
      case "monitor": {
        VarExprent x = var(1);
        repl(label, new MonitorExprent(MonitorExprent.Type.ENTER, x, null), x, var(2));
        return;
      }
      case "new_dims": {
        VarExprent x = var(1);
        repl(label, newArr(new VarType(CodeType.INT, 2), list(x, x), list(), false), x, ci(9));
        return;
      }
      case "new_elems": {
        VarExprent x = var(1);
        repl(label, newArr(new VarType(CodeType.INT, 1), list(), list(x, ci(1), x), true), x, ci(9));
        return;
      }
      case "new_ctor_param": {
        VarExprent x = var(1);
        repl(label, newObj("a/B", inv("<init>", "(II)V", null, x, x)), x, ci(9));
        return;
      }
      case "new_ctor_swap": {
        InvocationExprent c = inv("<init>", "()V", null);
        NewExprent n = newObj("a/B", c);
        repl(label, n, c, inv("<init>", "(I)V", null, ci(1)));
        return;
      }
      case "new_ctor_badcast": {
        InvocationExprent c = inv("<init>", "()V", null);
        NewExprent n = newObj("a/B", c);
        repl(label, n, c, var(5));
        return;
      }
      case "switch_head": {
        VarExprent x = var(1);
        List<List<Exprent>> cases = new ArrayList<>();
        cases.add(list(x));
        SwitchHeadExprent sh = switchHead(x, cases);
        repl(label, sh, x, var(2));
        p("replsh", label, ids(sh.getCaseValues().get(0)));
        return;
      }
      case "yield": {
        VarExprent x = var(1);
        repl(label, new YieldExprent(x, INT), x, var(2));
        return;
      }
      case "base_annotation": {
        ConstExprent x = ci(1);
        repl(label, ann("a/A", new ArrayList<>(List.of("value")), list(x)), x, ci(2));
        return;
      }
      case "base_pattern": {
        VarExprent x = var(1);
        repl(label, pat("a/R", list(x)), x, var(2));
        return;
      }
      case "base_const":
        repl(label, ci(1), var(1), var(2));
        return;
      case "base_var": {
        VarExprent x = var(1);
        repl(label, x, x, var(2));
        return;
      }
      case "base_switch":
        repl(label, new SwitchExprent(switchStat(), INT, false, true), var(1), var(2));
        return;
    }
    throw new IllegalArgumentException(label);
  }

  // ---- 6. enum tables ----

  static void functionTypes() {
    for (FunctionType t : FunctionType.values()) {
      p("ft", t.ordinal(), t.name(), t.arity, t.operator == null ? "N" : t.operator, t.precedence, vtc(t.castType),
        vtc((VarType) getField(t, FunctionType.class, "type")), b(t.isArithmeticBinaryOperation()), b(t.isMM()), b(t.isPP()),
        b(t.isPPMM()), b(t.isPostfixPPMM()));
    }

    for (FunctionType t : FunctionType.values()) {
      reset();
      Exprent[] pool = {var(1), ct(VarType.VARTYPE_LONG, 2L), sfield("f", "I")};
      List<Exprent> ops = new ArrayList<>();
      for (int i = 0; i < t.arity; i++) ops.add(pool[i]);
      FunctionExprent f = new FunctionExprent(t, ops, null);
      if (t.isPPMM()) f.setImplicitType(VarType.VARTYPE_SHORT);
      String et = guard(() -> vt(f.getExprType()));
      String use = guard(() -> "" + f.getExprentUse());
      String prec = guard(() -> "" + f.getPrecedence());
      f.setNeedsCast(false);
      String precNoCast = guard(() -> "" + f.getPrecedence());
      p("ftinst", t.name(), et, use, prec, precNoCast, vtc(f.getSimpleCastType()), desc(f));
    }

    for (IfExprent.Type t : IfExprent.Type.values()) {
      FunctionType ft = (FunctionType) getField(t, IfExprent.Type.class, "functionType");
      p("it", t.ordinal(), t.name(), ft == null ? "N" : ft.name(), guard(() -> t.getNegative().name()));
    }

    for (Exprent.Type t : Exprent.Type.values()) {
      p("et", t.ordinal(), t.name());
    }
  }

  // ---- 7. ConstExprent hash / equality ----

  static final String[] CONST_LABELS = {
    "b0", "b1", "i0", "i1", "im1", "i97", "i200", "im200", "i40000", "i70000", "imin", "imax", "int1", "char97",
    "l0", "l1", "lm1", "lmin", "l2p40", "l2p32p1", "f0", "fm0", "f1", "f15", "fnan", "fnan2", "finf", "d0", "dm0",
    "d1", "dnan", "dinf", "dmin", "d1e300", "s_empty", "s_a", "s_a2", "s_uni", "s_null", "null", "null2",
    "int_long1", "long_int1", "double_float1", "class_str", "class_arr", "class_null", "int_null"
  };

  static ConstExprent constSubject(String label) {
    VarType INT = VarType.VARTYPE_INT, LONG = VarType.VARTYPE_LONG, FLOAT = VarType.VARTYPE_FLOAT, DOUBLE = VarType.VARTYPE_DOUBLE;
    VarType STRING = VarType.VARTYPE_STRING;
    switch (label) {
      case "b0": return cb(0);
      case "b1": return cb(1);
      case "i0": return ci(0);
      case "i1": return ci(1);
      case "im1": return ci(-1);
      case "i97": return ci(97);
      case "i200": return ci(200);
      case "im200": return ci(-200);
      case "i40000": return ci(40000);
      case "i70000": return ci(70000);
      case "imin": return ci(Integer.MIN_VALUE);
      case "imax": return ci(Integer.MAX_VALUE);
      case "int1": return ct(INT, 1);
      case "char97": return ct(VarType.VARTYPE_CHAR, 97);
      case "l0": return ct(LONG, 0L);
      case "l1": return ct(LONG, 1L);
      case "lm1": return ct(LONG, -1L);
      case "lmin": return ct(LONG, Long.MIN_VALUE);
      case "l2p40": return ct(LONG, 1L << 40);
      case "l2p32p1": return ct(LONG, (1L << 32) + 1);
      case "f0": return ct(FLOAT, 0f);
      case "fm0": return ct(FLOAT, -0f);
      case "f1": return ct(FLOAT, 1f);
      case "f15": return ct(FLOAT, 1.5f);
      case "fnan": return ct(FLOAT, Float.NaN);
      case "fnan2": return ct(FLOAT, Float.intBitsToFloat(0x7fc00001));
      case "finf": return ct(FLOAT, Float.POSITIVE_INFINITY);
      case "d0": return ct(DOUBLE, 0d);
      case "dm0": return ct(DOUBLE, -0d);
      case "d1": return ct(DOUBLE, 1d);
      case "dnan": return ct(DOUBLE, Double.NaN);
      case "dinf": return ct(DOUBLE, Double.NEGATIVE_INFINITY);
      case "dmin": return ct(DOUBLE, Double.MIN_VALUE);
      case "d1e300": return ct(DOUBLE, 1e300);
      case "s_empty": return ct(STRING, "");
      case "s_a": return ct(STRING, "a");
      case "s_a2": return ct(STRING, new String("a"));
      case "s_uni": return ct(STRING, "\u00e9\u4e2d\ud83d\ude00");
      case "s_null": return ct(STRING, null);
      case "null": return ct(VarType.VARTYPE_NULL, null);
      case "null2": return ct(VarType.VARTYPE_NULL, null);
      case "int_long1": return ct(INT, 1L);
      case "long_int1": return ct(LONG, 1);
      case "double_float1": return ct(DOUBLE, 1f);
      case "class_str": return ct(VarType.VARTYPE_CLASS, "java/lang/String");
      case "class_arr": return ct(VarType.VARTYPE_CLASS, "[I");
      case "class_null": return ct(VarType.VARTYPE_CLASS, null);
      case "int_null": return ct(INT, null);
    }
    throw new IllegalArgumentException(label);
  }

  static void consts() {
    reset();
    List<ConstExprent> all = new ArrayList<>();
    for (String label : CONST_LABELS) all.add(constSubject(label));

    for (int i = 0; i < all.size(); i++) {
      ConstExprent c = all.get(i);
      p("cdef", CONST_LABELS[i], vt(c.getConstType()), c.hashCode(), guard(() -> b(c.hasBooleanValue())),
        guard(() -> b(c.hasValueOne())), b(c.isNull()), vt(c.getExprType()), c.getExprentUse());
    }
    for (int i = 0; i < all.size(); i++) {
      StringBuilder sb = new StringBuilder();
      for (ConstExprent o : all) sb.append(b(all.get(i).equals(o)));
      p("ceq", CONST_LABELS[i], sb);
    }

    Set<Exprent> set = new HashSet<>();
    for (ConstExprent c : all) set.add(c);
    StringBuilder order = new StringBuilder();
    for (Exprent e : set) {
      if (order.length() > 0) order.append(',');
      order.append(CONST_LABELS[all.indexOf(e)]);
    }
    p("cset", "order", order);

    Map<Exprent, Integer> map = new HashMap<>();
    for (int i = 0; i < all.size(); i++) map.put(all.get(i), i);
    StringBuilder mapped = new StringBuilder();
    for (Map.Entry<Exprent, Integer> en : map.entrySet()) {
      if (mapped.length() > 0) mapped.append(',');
      mapped.append(CONST_LABELS[all.indexOf(en.getKey())]).append('=').append(CONST_LABELS[en.getValue()]);
    }
    p("cmap", "entries", mapped);

    for (CodeType t : new CodeType[] {CodeType.INT, CodeType.LONG, CodeType.DOUBLE, CodeType.FLOAT, CodeType.BOOLEAN, CodeType.OBJECT}) {
      reset();
      p("czero", t.name(), guard(() -> {
        ConstExprent z = ConstExprent.getZeroConstant(t);
        return vt(z.getConstType()) + "," + valText(z.getValue()) + "," + z.id;
      }));
    }

    String[] adjSubjects = {"i1", "i97", "i10", "i39", "i200", "im1", "char97", "l1", "b1", "int_long1"};
    VarType[] adjTargets = {VarType.VARTYPE_CHAR, VarType.VARTYPE_CHARACTER, VarType.VARTYPE_INT, VarType.VARTYPE_INTEGER,
      VarType.VARTYPE_LONG, VarType.VARTYPE_BYTECHAR};
    for (String s : adjSubjects) {
      StringBuilder sb = new StringBuilder();
      for (VarType target : adjTargets) {
        reset();
        ConstExprent c = s.equals("i10") ? ci(10) : s.equals("i39") ? ci(39) : constSubject(s);
        if (sb.length() > 0) sb.append(',');
        sb.append(guard(() -> {
          c.adjustConstType(target);
          return vtc(c.getConstType());
        }));
      }
      p("cadj", s, sb);
    }

    String[] strs = {"plain", "a\"b\\c'd", "\b\t\n\f\r", "\u0000\u0001\u001f\u007f", "\u00e9\u00a0\u4e2d", "\ud83d\ude00",
      "\uffff", ""};
    for (String s : strs) {
      p("cstr", hex(s), hex(ConstExprent.convertStringToJava(s, false)), hex(ConstExprent.convertStringToJava(s, true)));
    }
  }

  // ---- bytecode ranges ----

  static final String[] BCR_LABELS = {
    "assignment", "array", "function", "if", "invocation", "new_array", "new_object", "exit", "field", "monitor",
    "switch_head", "annotation", "assert", "pattern", "yield", "const", "var", "switch"
  };

  static ConstExprent cbc(int v, int... b) {
    return new ConstExprent(v, false, bs(b));
  }

  static VarExprent vbc(int index, int... b) {
    return new VarExprent(index, VarType.VARTYPE_INT, null, bs(b));
  }

  static Exprent bcrSubject(String label) {
    VarType INT = VarType.VARTYPE_INT;
    switch (label) {
      case "assignment": return new AssignmentExprent(vbc(1, 1), cbc(2, 2), bs(3));
      case "array": return new ArrayExprent(vbc(1, 4), cbc(0, 5), INT, bs(6));
      case "function": return new FunctionExprent(FunctionType.ADD, list(vbc(1, 1), new FunctionExprent(FunctionType.NEG, vbc(2, 9), bs(10))), bs(11));
      case "if": return new IfExprent(IfExprent.Type.ICMPEQ, stackOf(vbc(1, 0), vbc(2, 1)), bs(2));
      case "invocation": {
        InvocationExprent ie = inv("m", "(I)V", vbc(1, 3), cbc(1, 4));
        ie.addBytecodeOffsets(bs(5));
        return ie;
      }
      case "new_array": {
        NewExprent n = new NewExprent(new VarType(CodeType.INT, 1), list(cbc(2, 1)), bs(2));
        n.setLstArrayElements(list(cbc(1, 3), cbc(2, 4)));
        return n;
      }
      case "new_object": {
        InvocationExprent c = inv("<init>", "(I)V", null, cbc(1, 7));
        c.addBytecodeOffsets(bs(8));
        NewExprent n = new NewExprent(obj("a/B"), list(), bs(6));
        n.setConstructor(c);
        return n;
      }
      case "exit": return new ExitExprent(ExitExprent.Type.RETURN, vbc(1, 12), INT, bs(13), null);
      case "field": return new FieldExprent("f", "a/B", false, vbc(1, 2), FieldDescriptor.parseDescriptor("I"), bs(3));
      case "monitor": return new MonitorExprent(MonitorExprent.Type.ENTER, vbc(1, 0), bs(1));
      case "switch_head": {
        List<List<Exprent>> cases = new ArrayList<>();
        cases.add(list(cbc(1, 20), cbc(2, 21)));
        cases.add(list((Exprent) null));
        return switchHead(vbc(1, 3), cases);
      }
      case "annotation": return ann("a/A", new ArrayList<>(List.of("x", "y")), list(cbc(1, 1), cbc(2, 2)));
      case "assert": return new AssertExprent(list(vbc(1, 4), cbc(1, 5)));
      case "pattern": return pat("a/R", list(vbc(1, 6), pat("a/S", list(vbc(2, 7)))));
      case "yield": return new YieldExprent(vbc(1, 8), INT);
      case "const": return cbc(1, 30);
      case "var": return vbc(1, 31);
      case "switch": return new SwitchExprent(switchStat(), INT, false, true);
    }
    throw new IllegalArgumentException(label);
  }

  static void bytecodeRanges() {
    for (String label : BCR_LABELS) {
      reset();
      Exprent e = bcrSubject(label);
      BitSet values = new BitSet();
      e.getBytecodeRange(values);
      p("bcr", label, bits(values), bits(e.bytecode));
    }
  }

  // ---- per-kind queries ----

  static final String[] KQ_LABELS = {
    "annotation", "array", "array_null", "array_2d", "assert", "assignment", "assignment_join", "assignment_obj",
    "const", "exit", "field_static", "field_instance", "field_instance_inv", "function_add", "function_cast",
    "function_cast_nocast", "function_ternary", "function_ternary_obj", "function_and_bool", "function_and_int",
    "function_shl_long", "function_neg_double", "function_pp", "if", "invocation", "monitor", "new_array",
    "new_object", "pattern", "switch", "switch_head", "var", "var_unknown", "yield"
  };

  static Exprent kqSubject(String label) {
    VarType INT = VarType.VARTYPE_INT, LONG = VarType.VARTYPE_LONG, BOOLEAN = VarType.VARTYPE_BOOLEAN;
    switch (label) {
      case "annotation": return ann("a/A", new ArrayList<>(), list());
      case "array": return arr(var(1, 0, new VarType(CodeType.INT, 1)), ci(0), INT);
      case "array_null": return arr(ct(VarType.VARTYPE_NULL, null), ci(0), LONG);
      case "array_2d": return arr(var(1, 0, new VarType(CodeType.OBJECT, 2, "java/lang/String")), ci(0), VarType.VARTYPE_OBJECT);
      case "assert": return new AssertExprent(list(var(1)));
      case "assignment": return asg(var(1, 0, INT), ci(1));
      case "assignment_join": return asg(var(1, 0, LONG), var(2, 0, INT));
      case "assignment_obj": return asg(var(1, 0, obj("java/lang/Object")), var(2, 0, obj("java/lang/String")));
      case "const": return ci(70000);
      case "exit": return ret(var(1), INT);
      case "field_static": return sfield("f", "J");
      case "field_instance": return ifield("f", var(1), "Ljava/lang/String;");
      case "field_instance_inv": return ifield("f", inv("m", "()La/B;", var(1)), "I");
      case "function_add": return fn(FunctionType.ADD, var(1), ci(1));
      case "function_cast": return fn(FunctionType.CAST, var(1, 0, obj("java/lang/Object")), ct(obj("java/lang/String"), null));
      case "function_cast_nocast": {
        FunctionExprent f = fn(FunctionType.CAST, var(1, 0, obj("java/lang/Object")), ct(obj("java/lang/String"), null));
        f.setNeedsCast(false);
        return f;
      }
      case "function_ternary": return fn(FunctionType.TERNARY, cb(1), ci(1), ci(2));
      case "function_ternary_obj": return fn(FunctionType.TERNARY, var(1, 0, BOOLEAN), var(2, 0, obj("java/lang/String")), ct(VarType.VARTYPE_NULL, null));
      case "function_and_bool": return fn(FunctionType.AND, var(1, 0, BOOLEAN), var(2, 0, BOOLEAN));
      case "function_and_int": return fn(FunctionType.AND, var(1, 0, BOOLEAN), var(2, 0, INT));
      case "function_shl_long": return fn(FunctionType.SHL, var(1, 0, LONG), ci(1));
      case "function_neg_double": return fn(FunctionType.NEG, var(1, 0, VarType.VARTYPE_DOUBLE));
      case "function_pp": {
        FunctionExprent f = fn(FunctionType.PPI, var(1));
        f.setImplicitType(INT);
        return f;
      }
      case "if": return ifx(IfExprent.Type.ICMPEQ, var(1), var(2));
      case "invocation": return inv("m", "(I)J", var(1), ci(1));
      case "monitor": return new MonitorExprent(MonitorExprent.Type.ENTER, var(1), null);
      case "new_array": return newArr(new VarType(CodeType.INT, 1), list(ci(1)), list(), false);
      case "new_object": return newObj("a/B", inv("<init>", "()V", null));
      case "pattern": return pat("a/R", list(var(1)));
      case "switch": return new SwitchExprent(switchStat(), VarType.VARTYPE_STRING, false, true);
      case "switch_head": return switchHead(var(1, 0, VarType.VARTYPE_CHAR), new ArrayList<>());
      case "var": return var(1, 0, LONG);
      case "var_unknown": return new VarExprent(1, null, null);
      case "yield": return new YieldExprent(var(1), VarType.VARTYPE_SHORT);
    }
    throw new IllegalArgumentException(label);
  }

  static void queries() {
    for (String label : KQ_LABELS) {
      reset();
      Exprent e = kqSubject(label);
      String prec = e instanceof ConstExprent ? "skip" : guard(() -> "" + e.getPrecedence());
      p("kq", label, guard(() -> vt(e.getExprType())), prec, guard(() -> "" + e.getExprentUse()));
    }

    String[][] boxing = {
      {"int_valueof_const", "java/lang/Integer", "valueOf", "s", "c40000"},
      {"char_valueof_bytechar", "java/lang/Character", "valueOf", "s", "c1"},
      {"short_valueof_shortchar", "java/lang/Short", "valueOf", "s", "c200"},
      {"byte_valueof_bytechar", "java/lang/Byte", "valueOf", "s", "c1"},
      {"int_valueof_var", "java/lang/Integer", "valueOf", "s", "vI"},
      {"long_valueof_int", "java/lang/Long", "valueOf", "s", "vI"},
      {"long_valueof_long", "java/lang/Long", "valueOf", "s", "vJ"},
      {"bool_valueof_bool", "java/lang/Boolean", "valueOf", "s", "vZ"},
      {"valueof_instance", "java/lang/Integer", "valueOf", "i", "vI"},
      {"valueof_two_params", "java/lang/Integer", "valueOf", "s", "vI", "vI"},
      {"string_valueof", "java/lang/String", "valueOf", "s", "vI"},
      {"int_value", "java/lang/Integer", "intValue", "i"},
      {"int_value_on_long", "java/lang/Long", "intValue", "i"},
      {"long_value", "java/lang/Long", "longValue", "i"},
      {"char_value", "java/lang/Character", "charValue", "i"},
      {"int_value_static", "java/lang/Integer", "intValue", "s"},
      {"int_value_param", "java/lang/Integer", "intValue", "i", "vI"}
    };
    for (String[] row : boxing) {
      reset();
      List<Exprent> params = new ArrayList<>();
      for (int i = 4; i < row.length; i++) params.add(boxArg(row[i]));
      InvocationExprent ie = new InvocationExprent();
      ie.setName(row[2]);
      ie.setClassname(row[1]);
      ie.setStatic(row[3].equals("s"));
      if (!ie.isStatic()) ie.setInstance(var(9, 0, obj(row[1])));
      ie.setLstParameters(params);
      p("box", row[0], b(ie.isBoxingCall()), b(ie.isUnboxingCall()));
    }

    reset();
    {
      InvocationExprent unbox = new InvocationExprent();
      unbox.setName("intValue");
      unbox.setClassname("java/lang/Integer");
      unbox.setInstance(new FunctionExprent(FunctionType.CAST, list(var(1, 0, obj("java/lang/Object")), ct(obj("java/lang/Integer"), null)), bs(1)));
      unbox.addBytecodeOffsets(bs(4));
      InvocationExprent plain = new InvocationExprent();
      plain.setName("intValue");
      plain.setClassname("java/lang/Integer");
      plain.setInstance(var(2, 0, obj("java/lang/Integer")));
      FunctionExprent f = fn(FunctionType.ADD, unbox, plain, ci(3));
      String before = desc(f);
      f.unwrapBox();
      p("unwrap", "mixed", before, desc(f));
    }

    reset();
    {
      VarExprent a = var(3);
      VarExprent c = var(3, 2, VarType.VARTYPE_LONG);
      p("varname", "plain", a.getName(), c.getName(), var(10000).getName());
      VarExprent v1 = var(4, 1, VarType.VARTYPE_INT);
      VarExprent v1b = var(4, 1, VarType.VARTYPE_LONG);
      VarExprent v2 = var(4, 2, VarType.VARTYPE_INT);
      p("varname", "equals_versions", b(v1.equalsVersions(v1b)), b(v1.equals(v1b)), b(v1.equalsVersions(v2)), b(v1.equalsVersions(ci(4))));
      VarExprent x = var(5);
      VarExprent y = var(5);
      Exprent tree = asg(x, fn(FunctionType.ADD, y, var(6)));
      p("varname", "referenced", b(x.isVarReferenced(tree)), b(x.isVarReferenced(tree, y)), b(var(7).isVarReferenced(tree)),
        b(x.isVarReferenced(x)), b(y.isVarReferenced(tree, x)));
    }

    reset();
    {
      String[][] names = {{}, {"value"}, {"x"}, {"value", "x"}, {"Value"}};
      StringBuilder sb = new StringBuilder();
      for (String[] n : names) {
        if (sb.length() > 0) sb.append(',');
        sb.append(ann("a/A", new ArrayList<>(Arrays.asList(n)), list()).getAnnotationType().name());
      }
      p("anntype", "all", sb);
    }
  }

  static Exprent boxArg(String spec) {
    switch (spec) {
      case "c1": return ci(1);
      case "c200": return ci(200);
      case "c40000": return ci(40000);
      case "vI": return var(1, 0, VarType.VARTYPE_INT);
      case "vJ": return var(1, 0, VarType.VARTYPE_LONG);
      case "vZ": return var(1, 0, VarType.VARTYPE_BOOLEAN);
    }
    throw new IllegalArgumentException(spec);
  }

  public static void main(String[] args) throws Exception {
    install();
    ctor();
    copies();
    equalsTable();
    allOrders();
    replaces();
    functionTypes();
    consts();
    bytecodeRanges();
    queries();
    System.out.print(OUT);
  }
}

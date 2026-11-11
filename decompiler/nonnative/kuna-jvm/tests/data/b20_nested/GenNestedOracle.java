// Golden vectors for tests/b20_nested.rs: NestedClassProcessor.processClass and
// NestedMemberAccess.propagateMemberAccess over the class_model fixture, with wrappers
// whose methods carry no decompiled code; then NestedMemberAccess over hand-built direct
// graphs for the synthetic static p/Outer.lambda$new$0 (field / method / function bodies)
// and a caller in p/Outer$A.<init>; then NestedClassProcessor placing the local class
// definition in a hand-built statement tree for p/Outer.run. Run through gen.sh.
import java.io.File;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import org.jetbrains.java.decompiler.main.ClassesProcessor.ClassNode;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.Fernflower;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerLogger;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.main.rels.ClassWrapper;
import org.jetbrains.java.decompiler.main.rels.MethodWrapper;
import org.jetbrains.java.decompiler.main.rels.NestedClassProcessor;
import org.jetbrains.java.decompiler.main.rels.NestedMemberAccess;
import org.jetbrains.java.decompiler.modules.decompiler.exps.AssignmentExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ConstExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.ExitExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.Exprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FieldExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.InvocationExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.NewExprent;
import org.jetbrains.java.decompiler.modules.decompiler.exps.VarExprent;
import org.jetbrains.java.decompiler.modules.decompiler.flow.DirectEdge;
import org.jetbrains.java.decompiler.modules.decompiler.flow.DirectGraph;
import org.jetbrains.java.decompiler.modules.decompiler.flow.DirectNode;
import org.jetbrains.java.decompiler.modules.decompiler.flow.DirectNodeType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.DummyExitStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.RootStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.SequenceStatement;
import org.jetbrains.java.decompiler.modules.decompiler.stats.Statement;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarVersionPair;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.InterpreterUtil;

public class GenNestedOracle {
  static final List<String> LOG = new ArrayList<>();

  static String esc(String s) {
    return s == null ? "\\N" : s;
  }

  static String name(ClassNode n) {
    return n == null ? "\\N" : n.classStruct.qualifiedName;
  }

  static void dumpNode(ClassNode n) {
    List<String> nested = new ArrayList<>();
    for (ClassNode c : n.nested) nested.add(c.type.name() + ":" + name(c));
    List<String> enclosing = new ArrayList<>(n.enclosingClasses);
    enclosing.sort(null);
    System.out.println(String.join("\t", "node", n.type.name(), name(n), esc(n.simpleName), name(n.parent),
      String.join(",", nested), esc(n.enclosingMethod), String.join(",", enclosing),
      Integer.toString(n.mapFieldsToVars.size()), Boolean.toString(n.superInvocation != null)));
  }

  static void dumpWrapper(ClassNode n) {
    ClassWrapper w = n.getWrapper();
    List<String> hidden = new ArrayList<>(w.getHiddenMembers());
    hidden.sort(null);
    System.out.println("hidden\t" + name(n) + "\t" + String.join(",", hidden));
    for (MethodWrapper mw : w.getMethods()) {
      String synth = "\\N";
      if (mw.synthParameters != null) {
        List<String> parts = new ArrayList<>();
        for (VarVersionPair p : mw.synthParameters) parts.add(String.valueOf(p));
        synth = String.join(",", parts);
      }
      System.out.println(String.join("\t", "method", name(n),
        mw.methodStruct.getName() + " " + mw.methodStruct.getDescriptor(),
        mw.commentLines == null ? "\\N" : String.join("|", mw.commentLines),
        Boolean.toString(mw.addErrorComment), synth, Boolean.toString(mw.varproc.nestedProcessed)));
    }
  }

  static Map<String, ClassNode> load(File dir) {
    Map<String, Object> options = new HashMap<>();
    options.put(IFernflowerPreferences.INCLUDE_JAVA_RUNTIME, "0");
    options.put(IFernflowerPreferences.DUMP_EXCEPTION_ON_ERROR, "0");
    IFernflowerLogger logger = new IFernflowerLogger() {
      @Override public void writeMessage(String message, Severity severity) {
        if (severity.ordinal() >= Severity.WARN.ordinal()) LOG.add(severity.name() + "\t" + message);
      }
      @Override public void writeMessage(String message, Severity severity, Throwable t) {
        writeMessage(message, severity);
      }
    };
    Fernflower fernflower = new Fernflower(null, options, logger);
    fernflower.addSource(new File(dir, "classes"));
    fernflower.addLibrary(new File(dir, "lib"));
    DecompilerContext.getClassProcessor().loadClasses(null);
    LOG.clear();
    return DecompilerContext.getClassProcessor().getMapRootClasses();
  }

  static final String[] NAMES = {"p/Outer", "p/Outer$1Local", "p/Outer$A", "p/Outer$A$1", "p/Outer$B"};
  static final String ACCESSOR = "lambda$new$0 ()V";
  static final String CALLER = "<init> (Lp/Outer;)V";
  static final String RUN = "run ()V";
  static final Map<String, DirectNode> FIRST = new HashMap<>();
  static final Map<String, Statement> STATS = new HashMap<>();

  static void wrap(Map<String, ClassNode> roots, String kind) throws Exception {
    Field wrapperField = ClassNode.class.getDeclaredField("wrapper");
    wrapperField.setAccessible(true);
    for (String n : NAMES) {
      ClassNode node = roots.get(n);
      ClassWrapper wrapper = new ClassWrapper(node);
      for (StructMethod mt : node.classStruct.getMethods()) {
        MethodDescriptor md = MethodDescriptor.parseDescriptor(mt.getDescriptor());
        String key = InterpreterUtil.makeUniqueKey(mt.getName(), mt.getDescriptor());
        boolean special = kind.equals("access") && ((n.equals("p/Outer") && key.equals(ACCESSOR)) || (n.equals("p/Outer$A") && key.equals(CALLER)))
          || kind.equals("localdef") && n.equals("p/Outer") && key.equals(RUN);
        MethodWrapper mw;
        if (special) {
          Statement head;
          if (kind.equals("localdef")) {
            DummyExitStatement s1 = new DummyExitStatement();
            DummyExitStatement s2 = new DummyExitStatement();
            STATS.put("s1", s1);
            STATS.put("s2", s2);
            head = new SequenceStatement(Arrays.asList(s1, s2));
          } else {
            head = new DummyExitStatement();
          }
          DummyExitStatement exit = new DummyExitStatement();
          RootStatement root = new RootStatement(head, exit, mt);
          mw = new MethodWrapper(root, new VarProcessor(mt, md), mt, node.classStruct, new CounterContainer());
          DirectGraph graph = new DirectGraph();
          DirectNode first = DirectNode.forStat(DirectNodeType.DIRECT, head, null);
          DirectNode last = DirectNode.forStat(DirectNodeType.DIRECT, exit, null);
          graph.nodes.addWithKey(first, first.id);
          graph.nodes.addWithKey(last, last.id);
          graph.first = first;
          first.addSuccessor(DirectEdge.of(first, last));
          mw.graph = graph;
          FIRST.put(n + "." + key, first);
        } else {
          mw = new MethodWrapper(null, new VarProcessor(mt, md), mt, node.classStruct, new CounterContainer());
        }
        wrapper.getMethods().addWithKey(mw, key);
      }
      wrapperField.set(node, wrapper);
    }
  }

  static String desc(Exprent e) {
    if (e == null) return "-";
    String id = "#" + e.id;
    if (e instanceof FieldExprent f) {
      return "Field" + id + "(" + f.getClassname() + "." + f.getName() + "," + f.isStatic() + "," + desc(f.getInstance()) + ")";
    }
    if (e instanceof VarExprent v) {
      return "Var" + id + "(" + v.getIndex() + (v.isDefinition() ? ",def" : "") + (v.isClassDef() ? ",classdef" : "") + ")";
    }
    if (e instanceof NewExprent n) return "New" + id + "(" + n.getNewType().value + ")";
    if (e instanceof ConstExprent) return "Const" + id;
    if (e instanceof InvocationExprent i) {
      List<String> ps = new ArrayList<>();
      for (Exprent p : i.getLstParameters()) ps.add(desc(p));
      return "Inv" + id + "(" + i.getClassname() + "." + i.getName() + i.getStringDescriptor() + "," + i.isStatic() + "," + desc(i.getInstance()) + ",[" + String.join(",", ps) + "])";
    }
    if (e instanceof AssignmentExprent a) return "Assign" + id + "(" + desc(a.getLeft()) + "," + desc(a.getRight()) + ")";
    if (e instanceof FunctionExprent f) {
      List<String> ps = new ArrayList<>();
      for (Exprent p : f.getLstOperands()) ps.add(desc(p));
      return "Func" + id + "(" + f.getFuncType().name() + ",[" + String.join(",", ps) + "])";
    }
    if (e instanceof ExitExprent x) return "Exit" + id + "(" + x.getExitType().name() + "," + desc(x.getValue()) + ")";
    return e.getClass().getSimpleName() + id;
  }

  static InvocationExprent call() {
    InvocationExprent inv = new InvocationExprent();
    inv.setName("lambda$new$0");
    inv.setClassname("p/Outer");
    inv.setStringDescriptor("()V");
    inv.setDescriptor(MethodDescriptor.parseDescriptor("()V"));
    inv.setStatic(true);
    inv.setLstParameters(new ArrayList<>());
    return inv;
  }

  static Exprent staticField() {
    return new FieldExprent("S", "p/Outer", true, null, FieldDescriptor.parseDescriptor("I"), null);
  }

  static void access(File dir, String variant) throws Exception {
    Map<String, ClassNode> roots = load(dir);
    ClassNode outer = roots.get("p/Outer");
    CounterContainer counters = DecompilerContext.getCounterContainer();
    counters.setCounter(CounterContainer.STATEMENT_COUNTER, 50);
    counters.setCounter(CounterContainer.EXPRESSION_COUNTER, 100);
    FIRST.clear();
    wrap(roots, "access");

    Exprent value;
    if (variant.equals("field")) {
      value = staticField();
    } else if (variant.equals("method")) {
      InvocationExprent inv = new InvocationExprent();
      inv.setName("m");
      inv.setClassname("q/Other");
      inv.setStringDescriptor("()I");
      inv.setDescriptor(MethodDescriptor.parseDescriptor("()I"));
      inv.setStatic(true);
      inv.setLstParameters(new ArrayList<>());
      value = inv;
    } else {
      Exprent field = staticField();
      List<Exprent> operands = new ArrayList<>();
      operands.add(field);
      value = new FunctionExprent(FunctionExprent.FunctionType.IPP, operands, null);
    }
    DirectNode accessor = FIRST.get("p/Outer." + ACCESSOR);
    accessor.exprents.add(new ExitExprent(ExitExprent.Type.RETURN, value, VarType.VARTYPE_INT, null, null));

    MethodWrapper callerMethod = roots.get("p/Outer$A").getWrapper().getMethods().getWithKey(CALLER);
    DirectNode caller = FIRST.get("p/Outer$A." + CALLER);
    InvocationExprent first = call();
    VarExprent var = new VarExprent(5, VarType.VARTYPE_INT, callerMethod.varproc);
    InvocationExprent second = call();
    caller.exprents.add(first);
    caller.exprents.add(new AssignmentExprent(var, second, null));

    new NestedMemberAccess().propagateMemberAccess(outer);

    System.out.println("== access " + variant);
    for (String line : LOG) System.out.println(line);
    List<String> hidden = new ArrayList<>(outer.getWrapper().getHiddenMembers());
    hidden.sort(null);
    System.out.println("hidden\t" + String.join(",", hidden));
    for (Exprent e : accessor.exprents) System.out.println("accessor\t" + desc(e));
    for (Exprent e : caller.exprents) System.out.println("caller\t" + desc(e));
  }

  static void localdef(File dir, String variant) throws Exception {
    Map<String, ClassNode> roots = load(dir);
    ClassNode outer = roots.get("p/Outer");
    CounterContainer counters = DecompilerContext.getCounterContainer();
    counters.setCounter(CounterContainer.STATEMENT_COUNTER, 50);
    counters.setCounter(CounterContainer.EXPRESSION_COUNTER, 100);
    FIRST.clear();
    STATS.clear();
    wrap(roots, "localdef");

    MethodWrapper run = outer.getWrapper().getMethods().getWithKey(RUN);
    VarType local = new VarType("p/Outer$1Local", true);
    List<Exprent> first = new ArrayList<>();
    List<Exprent> second = new ArrayList<>();
    VarExprent var1 = new VarExprent(1, VarType.VARTYPE_INT, run.varproc);
    ConstExprent zero = new ConstExprent(0, false, null);
    first.add(new AssignmentExprent(var1, zero, null));
    if (variant.equals("single")) {
      second.add(new NewExprent(local, new ArrayList<>(), null));
    } else {
      VarExprent var2 = new VarExprent(2, local, run.varproc);
      NewExprent created = new NewExprent(local, new ArrayList<>(), null);
      first.add(new AssignmentExprent(var2, created, null));
      second.add(new FieldExprent("x", "p/Outer$1Local", true, null, FieldDescriptor.parseDescriptor("I"), null));
    }
    STATS.get("s1").setExprents(first);
    STATS.get("s2").setExprents(second);

    new NestedClassProcessor().processClass(outer, outer);

    System.out.println("== localdef " + variant);
    for (String line : LOG) System.out.println(line);
    for (String name : new String[] {"s1", "s2"}) {
      for (Exprent e : STATS.get(name).getExprents()) System.out.println(name + "\t" + desc(e));
      System.out.println(name + "\tvardefs\t" + STATS.get(name).getVarDefinitions().size());
    }
    System.out.println("comments\t" + (run.commentLines == null ? "\\N" : String.join("|", run.commentLines)));
  }

  public static void main(String[] args) throws Exception {
    File dir = new File(args[0]);
    Map<String, ClassNode> roots = load(dir);
    ClassNode outer = roots.get("p/Outer");
    StructClass cl = outer.classStruct;

    ClassNode lambda = new ClassNode("p/Outer", "lambda$new$0", "()V", 6, "java/lang/Runnable", "run", "()V", cl);
    lambda.simpleName = "p/Outer##Lambda_0_1";
    lambda.enclosingMethod = "<init> ()V";
    lambda.parent = outer;
    outer.nested.add(lambda);

    roots.get("p/Outer$B").simpleName = null;

    String[] names = NAMES;
    wrap(roots, "plain");

    new NestedClassProcessor().processClass(outer, outer);
    new NestedMemberAccess().propagateMemberAccess(outer);

    System.out.println("== log");
    for (String line : LOG) System.out.println(line);
    System.out.println("== nodes");
    for (String n : names) dumpNode(roots.get(n));
    dumpNode(lambda);
    System.out.println("== wrappers");
    for (String n : names) dumpWrapper(roots.get(n));

    for (String variant : new String[] {"field", "method", "function"}) {
      LOG.clear();
      access(dir, variant);
    }
    for (String variant : new String[] {"single", "both"}) {
      LOG.clear();
      localdef(dir, variant);
    }
  }
}

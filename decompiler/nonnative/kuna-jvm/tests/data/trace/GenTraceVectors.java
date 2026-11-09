// Vectors for tests/trace_vectors.rs: statement trees built by hand with the real Vineflower 1.12.0
// classes, dumped by the Java tracer (tools/vftrace/src/kuna/vftrace/Dumper.java). The Rust test builds
// the same trees in the same order and requires kuna_trace::dump to print the same text. Regenerate
// with gen.sh. Every statement type, every exprent kind and every FunctionType appears at least once.
package org.jetbrains.java.decompiler.modules.decompiler.stats;

import kuna.vftrace.Dumper;
import org.jetbrains.java.decompiler.code.BytecodeVersion;
import org.jetbrains.java.decompiler.code.CodeConstants;
import org.jetbrains.java.decompiler.code.Instruction;
import org.jetbrains.java.decompiler.code.cfg.BasicBlock;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.StatEdge;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.exps.FunctionExprent.FunctionType;
import org.jetbrains.java.decompiler.modules.decompiler.stats.Statement.EdgeDirection;
import org.jetbrains.java.decompiler.modules.decompiler.vars.VarProcessor;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructLocalVariableTableAttribute.LocalVariable;
import org.jetbrains.java.decompiler.struct.consts.LinkConstant;
import org.jetbrains.java.decompiler.struct.consts.PooledConstant;
import org.jetbrains.java.decompiler.struct.consts.PrimitiveConstant;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.FieldDescriptor;
import org.jetbrains.java.decompiler.struct.gen.MethodDescriptor;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.struct.gen.generics.GenericType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.collections.ListStack;

import java.lang.reflect.Field;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;

public class GenTraceVectors {
  static StructMethod MT;
  static StructClass REC;
  static VarProcessor VP;
  static final BytecodeVersion V8 = new BytecodeVersion(52, 0);
  static final StringBuilder OUT = new StringBuilder();

  // ---- reflection and context ----

  static Field field(Class<?> cls, String name) throws Exception {
    Field f = cls.getDeclaredField(name);
    f.setAccessible(true);
    return f;
  }

  static void set(Object o, Class<?> cls, String name, Object value) throws Exception {
    field(cls, name).set(o, value);
  }

  static Object get(Object o, Class<?> cls, String name) throws Exception {
    return field(cls, name).get(o);
  }

  static void install() throws Exception {
    Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    uf.setAccessible(true);
    sun.misc.Unsafe u = (sun.misc.Unsafe) uf.get(null);
    DecompilerContext ctx = (DecompilerContext) u.allocateInstance(DecompilerContext.class);
    set(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    set(ctx, DecompilerContext.class, "properties", new HashMap<String, Object>(IFernflowerPreferences.getDefaults()));
    ClassesProcessor cp = (ClassesProcessor) u.allocateInstance(ClassesProcessor.class);
    set(cp, ClassesProcessor.class, "mapRootClasses", new HashMap<>());
    set(ctx, DecompilerContext.class, "classProcessor", cp);
    DecompilerContext.setCurrentContext(ctx);
  }

  static StructClass load(String path) throws Exception {
    return StructClass.create(new DataInputFullStream(Files.readAllBytes(Paths.get(path))), true);
  }

  /** ClassWrapper.init's per-method setup: a fresh var processor and fresh counters. */
  static void start() {
    VP = new VarProcessor(MT, MT.methodDescriptor());
    DecompilerContext.startMethod(VP);
  }

  static void emit(String name, RootStatement root) {
    OUT.append("### ").append(name).append('\n').append(Dumper.dump(root));
  }

  // ---- builders (mirrored one to one in tests/trace_vectors.rs) ----

  static BitSet bs(int... b) {
    BitSet s = new BitSet();
    for (int x : b) s.set(x);
    return s;
  }

  static <T> List<T> list(T... xs) {
    return new ArrayList<>(Arrays.asList(xs));
  }

  static VarType obj(String cls) {
    return new VarType(CodeType.OBJECT, 0, cls);
  }

  static ConstExprent ci(int v) {
    return new ConstExprent(v, false, null);
  }

  static ConstExprent cs(String v) {
    return new ConstExprent(VarType.VARTYPE_STRING, v, null);
  }

  static VarExprent var(int index) {
    return new VarExprent(index, VarType.VARTYPE_INT, VP);
  }

  static InvocationExprent inv(String name, String desc, Exprent inst, List<Exprent> params) {
    InvocationExprent e = new InvocationExprent();
    e.setName(name);
    e.setClassname("a/B");
    e.setStringDescriptor(desc);
    e.setDescriptor(MethodDescriptor.parseDescriptor(desc));
    e.setStatic(inst == null);
    e.setInstance(inst);
    e.setLstParameters(params);
    return e;
  }

  static Instruction insn(int opcode, int group, int offset, int... operands) {
    return Instruction.create(opcode, false, group, V8, operands.length == 0 ? null : operands, offset, 1);
  }

  static BasicBlockStatement block(Instruction... insns) {
    BasicBlock blk = new BasicBlock(DecompilerContext.getCounterContainer().getCounterAndIncrement(CounterContainer.STATEMENT_COUNTER));
    for (Instruction i : insns) blk.getSeq().addInstruction(i);
    return new BasicBlockStatement(blk);
  }

  static StatEdge edge(int type, Statement src, Statement dst, Statement closure) {
    StatEdge e = new StatEdge(type, src, dst, closure);
    src.addSuccessor(e);
    return e;
  }

  static void add(Statement parent, Statement... children) {
    for (Statement c : children) {
      parent.getStats().addWithKey(c, c.id);
      c.setParent(parent);
    }
  }

  // ---- 1. basic blocks, edges and the statement base fields ----

  static void blocks() throws Exception {
    start();
    BasicBlockStatement b1 = block(insn(CodeConstants.opc_iload, CodeConstants.GROUP_GENERAL, 0, 1),
      insn(CodeConstants.opc_ifeq, CodeConstants.GROUP_JUMP, 1, 5));
    BasicBlockStatement b2 = block(insn(CodeConstants.opc_aload, CodeConstants.GROUP_GENERAL, 4, 0),
      insn(CodeConstants.opc_monitorenter, CodeConstants.GROUP_GENERAL, 5));
    BasicBlockStatement b3 = block(insn(CodeConstants.opc_monitorexit, CodeConstants.GROUP_GENERAL, 6),
      insn(CodeConstants.opc_athrow, CodeConstants.GROUP_GENERAL, 7));
    BasicBlockStatement b4 = block();
    BasicBlockStatement b5 = BasicBlockStatement.create();
    SequenceStatement seq = new SequenceStatement(list(b1, b2, b3, b4));
    DummyExitStatement exit = new DummyExitStatement();
    RootStatement root = new RootStatement(seq, exit, MT);
    seq.setAllParent();

    StatEdge r12 = edge(StatEdge.TYPE_REGULAR, b1, b2, null);
    StatEdge r13 = edge(StatEdge.TYPE_REGULAR, b1, b3, null);
    StatEdge bx = edge(StatEdge.TYPE_BREAK, b1, exit, seq);
    bx.labeled = false;
    b1.changeEdgeType(EdgeDirection.FORWARD, r12, StatEdge.TYPE_BREAK);
    StatEdge exc = new StatEdge(b2, b3, list("java/lang/Exception", "java/io/IOException"));
    b2.addSuccessor(exc);
    b2.addSuccessor(new StatEdge(b2, b4, null));
    StatEdge cont = edge(StatEdge.TYPE_CONTINUE, b3, b1, seq);
    cont.explicit = false;
    cont.canInline = false;
    cont.phantomContinue = true;
    edge(StatEdge.TYPE_FINALLYEXIT, b4, exit, null);
    edge(StatEdge.TYPE_REGULAR, b3, b5, null);
    StatEdge r54 = edge(StatEdge.TYPE_REGULAR, b5, b4, null);
    b4.changeEdgeType(EdgeDirection.BACKWARD, r54, StatEdge.TYPE_BREAK);
    StatEdge stray = new StatEdge(StatEdge.TYPE_REGULAR, b5, b3);
    b4.addEdgeInternal(EdgeDirection.FORWARD, stray);
    b1.addEdgeInternal(EdgeDirection.BACKWARD, stray);
    seq.getLabelEdges().add(new StatEdge(StatEdge.TYPE_REGULAR, b4, b1));
    root.addLabeledEdge(r13);

    seq.getContinueSet().add(b3);
    seq.getContinueSet().add(b1);
    b2.clearTempInformation();
    set(seq, Statement.class, "post", b4);
    seq.setFirst(b2);
    b4.setParent(root);
    b3.setCopied(true);
    b4.setPhantom(true);
    b4.setExprents(list(new AssignmentExprent(var(1), ci(2), bs(3, 4, 5, 9))));
    root.getVarDefinitions().add(var(7));
    exit.addBytecodeOffsets(bs(3, 4, 5, 9, 11, 12));
    root.addComment("first");
    root.addComment("err \"q\" \\ é\n", true);
    emit("blocks", root);
  }

  // ---- 2. every compound statement ----

  static void control() throws Exception {
    start();
    BasicBlockStatement h = block(), t = block(), f = block();
    IfStatement is = new IfStatement();
    add(is, h, t, f);
    is.setFirst(h);
    is.iftype = IfStatement.IFTYPE_IFELSE;
    is.setIfstat(t);
    is.setElsestat(f);
    is.setIfEdge(edge(StatEdge.TYPE_REGULAR, h, t, null));
    is.setElseEdge(edge(StatEdge.TYPE_REGULAR, h, f, null));
    is.setNegated(true);
    is.setPatternMatched(true);
    is.setHasPPMM(true);
    ListStack<Exprent> stack = new ListStack<>();
    stack.push(var(1));
    stack.push(ci(0));
    is.getHeadexprentList().set(0, new IfExprent(IfExprent.Type.NE, stack, bs(2)));

    List<Statement> loops = new ArrayList<>();
    for (DoStatement.Type type : DoStatement.Type.values()) {
      BasicBlockStatement body = block();
      DoStatement ds = new DoStatement();
      add(ds, body);
      ds.setFirst(body);
      ds.setLooptype(type);
      if (type == DoStatement.Type.FOR) {
        ds.getInitExprentList().set(0, new AssignmentExprent(var(2), ci(0), null));
        ds.getConditionExprentList().set(0, new FunctionExprent(FunctionType.LT, list(var(2), ci(10)), null));
        ds.getIncExprentList().set(0, new FunctionExprent(FunctionType.PPI, list(var(2)), null));
      }
      edge(StatEdge.TYPE_CONTINUE, body, ds, ds);
      loops.add(ds);
    }

    BasicBlockStatement sh = block(), c1 = block(), c2 = block();
    SwitchStatement sw = new SwitchStatement();
    add(sw, sh, c1, c2);
    sw.setFirst(sh);
    StatEdge e1 = edge(StatEdge.TYPE_REGULAR, sh, c1, null);
    StatEdge e2 = edge(StatEdge.TYPE_REGULAR, sh, c2, null);
    sw.getCaseStatements().add(c1);
    sw.getCaseStatements().add(c2);
    sw.getCaseEdges().add(list(e1));
    sw.getCaseEdges().add(list(e2, e2));
    sw.getCaseValues().add(list(ci(1), ci(2)));
    sw.getCaseValues().add(list((Exprent) null));
    sw.getCaseGuards().add(null);
    sw.getCaseGuards().add(new FunctionExprent(FunctionType.GT, list(var(3), ci(5)), null));
    @SuppressWarnings("unchecked")
    Set<Statement> scoped = (Set<Statement>) get(sw, SwitchStatement.class, "scopedCaseStatements");
    scoped.add(c2);
    scoped.add(c1);
    sw.setDefaultEdge(e2);
    SwitchHeadExprent head = new SwitchHeadExprent(var(3), bs(8));
    head.setCaseValues(sw.getCaseValues());
    sw.getHeadexprentList().set(0, head);

    BasicBlockStatement yh = block(), yb = block(), ye = block();
    SynchronizedStatement sy = new SynchronizedStatement();
    add(sy, yh, yb, ye);
    sy.setFirst(yh);
    set(sy, SynchronizedStatement.class, "body", yb);
    sy.getHeadexprentList().set(0, new MonitorExprent(MonitorExprent.Type.ENTER, var(4), null));

    BasicBlockStatement th = block(), tc1 = block(), tc2 = block();
    CatchStatement ts = new CatchStatement();
    add(ts, th, tc1, tc2);
    ts.setFirst(th);
    ts.getExctStrings().add(list("java/lang/Exception"));
    ts.getExctStrings().add(list("a/B", "c/D"));
    ts.getVars().add(new VarExprent(5, obj("java/lang/Exception"), VP));
    ts.getVars().add(new VarExprent(6, obj("a/B"), VP));
    ts.getResources().add(new AssignmentExprent(var(8), cs("r"), null));
    th.addSuccessor(new StatEdge(th, tc1, list("java/lang/Exception")));

    BasicBlockStatement ah = block(), ahd = block();
    CatchAllStatement ca = new CatchAllStatement();
    add(ca, ah, ahd);
    ca.setFirst(ah);
    set(ca, CatchAllStatement.class, "handler", ahd);
    ca.setFinally(true);
    ca.setMonitor(new VarExprent(9, VarType.VARTYPE_OBJECT, VP));
    ca.getVars().add(new VarExprent(10, obj("java/lang/Throwable"), VP));

    BasicBlockStatement gh = block(), go = block();
    GeneralStatement gs = new GeneralStatement(gh, list(gh, go), null);
    gs.setAllParent();
    set(gs, GeneralStatement.class, "placeholder", true);

    List<Statement> all = list(is);
    all.addAll(loops);
    all.addAll(list(sw, sy, ts, ca, gs));
    SequenceStatement seq = new SequenceStatement(all);
    seq.setAllParent();
    BasicBlockStatement tail = block();
    tail.setExprents(list(new SwitchExprent(sw, VarType.VARTYPE_INT, true, true),
      new YieldExprent(ci(4), VarType.VARTYPE_INT)));
    SequenceStatement outer = new SequenceStatement(list(seq, tail));
    outer.setAllParent();
    DummyExitStatement exit = new DummyExitStatement();
    RootStatement root = new RootStatement(outer, exit, MT);
    root.buildContentFlags();
    emit("control", root);
  }

  // ---- 3. every exprent kind, every FunctionType and the VarType printer ----

  static void exprents() throws Exception {
    start();
    List<Exprent> l = new ArrayList<>();
    l.add(new AnnotationExprent("a/Ann", list("x", "y"), list(ci(1), cs("v"))));
    l.add(new ArrayExprent(var(1), ci(0), VarType.VARTYPE_INT, bs(2)));
    l.add(new AssertExprent(list(null, cs("msg"))));
    l.add(new AssignmentExprent(var(2), ci(3), FunctionType.ADD, bs(4, 5)));
    l.add(new ConstExprent(1, true, null));
    l.add(new ConstExprent(VarType.VARTYPE_LONG, -5L, null));
    l.add(new ConstExprent(VarType.VARTYPE_FLOAT, Float.intBitsToFloat(0x7fc00001), null));
    l.add(new ConstExprent(VarType.VARTYPE_DOUBLE, -0.0d, null));
    l.add(new ConstExprent(VarType.VARTYPE_CHAR, 65, null));
    l.add(new ConstExprent(VarType.VARTYPE_STRING, "a\"b\\c\né😀~", null, true));
    l.add(new ConstExprent(VarType.VARTYPE_STRING, "x\ud800y\udc00\udc00\ud800", null));
    l.add(new ConstExprent(VarType.VARTYPE_NULL, null, null));
    l.add(new ExitExprent(ExitExprent.Type.RETURN, var(3), VarType.VARTYPE_INT, bs(6), null));
    l.add(new ExitExprent(ExitExprent.Type.RETURN, null, VarType.VARTYPE_VOID, null, null));
    FieldExprent sf = new FieldExprent("S", "a/B", true, null, FieldDescriptor.parseDescriptor("[Ljava/lang/String;"), null);
    sf.forceQualified(true);
    set(sf, FieldExprent.class, "wasCondy", true);
    l.add(sf);
    FieldExprent inf = new FieldExprent("f", "a/B", false, var(0), FieldDescriptor.parseDescriptor("J"), bs(1));
    inf.setIsQualifier();
    l.add(inf);
    for (FunctionType t : FunctionType.values()) {
      if (t == FunctionType.OTHER) continue;
      l.add(new FunctionExprent(t, list(var(1), ci(2)), null));
    }
    FunctionExprent cast = new FunctionExprent(FunctionType.CAST, list(var(1), new ConstExprent(obj("a/B"), null, null)), null);
    cast.setImplicitType(VarType.VARTYPE_LONG);
    cast.setNeedsCast(false);
    set(cast, FunctionExprent.class, "disableNewlineGroupCreation", true);
    l.add(cast);
    ListStack<Exprent> stack = new ListStack<>();
    stack.push(var(1));
    l.add(new IfExprent(IfExprent.Type.NULL, stack, null));

    InvocationExprent st = inv("m", "(I)V", null, list(ci(1)));
    st.setFunctype(InvocationExprent.Type.GENERAL);
    l.add(st);
    InvocationExprent dyn = inv("apply", "(Ljava/lang/Object;)La/F;", var(0), list(var(1)));
    set(dyn, InvocationExprent.class, "invocationType", InvocationExprent.InvocationType.DYNAMIC);
    set(dyn, InvocationExprent.class, "invokeDynamicClassSuffix", "##Lambda_1_2");
    set(dyn, InvocationExprent.class, "bootstrapMethod",
      new LinkConstant(CodeConstants.CONSTANT_MethodHandle, "java/lang/invoke/LambdaMetafactory", "metafactory", "()V"));
    List<PooledConstant> bsa = new ArrayList<>();
    bsa.add(new PrimitiveConstant(CodeConstants.CONSTANT_Integer, (Object) 7));
    bsa.add(new PrimitiveConstant(CodeConstants.CONSTANT_String, "s"));
    bsa.add(null);
    bsa.add(new LinkConstant(CodeConstants.CONSTANT_Methodref, "a/B", "c", "()V"));
    set(dyn, InvocationExprent.class, "bootstrapArguments", bsa);
    set(dyn, InvocationExprent.class, "desc", MT);
    set(dyn, InvocationExprent.class, "canIgnoreBoxing", false);
    set(dyn, InvocationExprent.class, "remappedInstType", obj("a/R"));
    dyn.getGenericArgs().add(obj("java/lang/String"));
    dyn.getGenericArgs().add(VarType.VARTYPE_INT);
    dyn.forceGenericQualfication = true;
    dyn.getGenericsMap().put(GenericType.parse("TT;"), obj("a/X"));
    dyn.getGenericsMap().put(GenericType.parse("TU;"), null);
    dyn.setInvocationInstance();
    dyn.setIsQualifier();
    dyn.forceBoxing(true);
    dyn.forceUnboxing(true);
    dyn.setSyntheticNullCheck();
    dyn.markWasLazyCondy();
    l.add(dyn);
    InvocationExprent init = inv("<init>", "()V", var(0), list());
    init.setFunctype(InvocationExprent.Type.INIT);
    set(init, InvocationExprent.class, "invocationType", InvocationExprent.InvocationType.SPECIAL);
    l.add(init);

    MonitorExprent mon = new MonitorExprent(MonitorExprent.Type.EXIT, var(4), bs(9));
    mon.setRemove(true);
    l.add(mon);

    NewExprent nobj = new NewExprent(obj("a/B"), list(), null);
    nobj.setConstructor(inv("<init>", "(I)V", null, list(ci(3))));
    set(nobj, NewExprent.class, "anonymous", true);
    set(nobj, NewExprent.class, "lambda", true);
    set(nobj, NewExprent.class, "methodReference", true);
    nobj.setEnumConst(true);
    set(nobj, NewExprent.class, "inferredLambdaType", obj("a/F"));
    @SuppressWarnings("unchecked")
    List<VarType> nga = (List<VarType>) get(nobj, NewExprent.class, "genericArgs");
    nga.add(GenericType.parse("Ljava/util/List<*>;"));
    l.add(nobj);
    l.add(new NewExprent(new VarType(CodeType.INT, 2), list(ci(3), var(1)), bs(10)));
    NewExprent ninit = new NewExprent(new VarType(CodeType.INT, 1), list(), null);
    ninit.setLstArrayElements(list(ci(1), ci(2)));
    ninit.setDirectArrayInit(true);
    ninit.setVarArgParam(true);
    l.add(ninit);

    PatternExprent inner = new PatternExprent(PatternExprent.recordData(REC), obj("a/In"), list(var(11)));
    PatternExprent pat = new PatternExprent(PatternExprent.recordData(REC), obj("a/Out"), list(var(12), inner));
    pat.getBoundTypes().set(0, VarType.VARTYPE_INT);
    l.add(pat);

    VarExprent noproc = new VarExprent(13, null, null);
    noproc.setVersion(2);
    l.add(noproc);
    VarExprent rich = new VarExprent(14, obj("a/B"), VP, bs(0, 1, 2));
    rich.setDefinition(true);
    rich.setClassDef(true);
    rich.setStack(true);
    LocalVariable lvt = new LocalVariable(3, 10, "loc", "Ljava/util/List;", 14);
    set(lvt, LocalVariable.class, "signature", "Ljava/util/List<Ljava/lang/String;>;");
    rich.setLVT(lvt);
    rich.setBackingInstr(insn(CodeConstants.opc_astore, CodeConstants.GROUP_GENERAL, 12, 14));
    rich.setEffectivelyFinal(true);
    rich.setWritingPattern();
    rich.setBoundType(GenericType.parse("Ljava/lang/Number;", GenericType.WILDCARD_EXTENDS));
    rich.setIntersectionType(true);
    rich.setCatchTempVar(true);
    l.add(rich);
    l.add(new YieldExprent(var(1), VarType.VARTYPE_BOOLEAN));

    VarType[] types = {
      VarType.VARTYPE_BYTE, VarType.VARTYPE_CHAR, VarType.VARTYPE_DOUBLE, VarType.VARTYPE_FLOAT, VarType.VARTYPE_INT,
      VarType.VARTYPE_LONG, VarType.VARTYPE_SHORT, VarType.VARTYPE_BOOLEAN, VarType.VARTYPE_VOID,
      VarType.VARTYPE_BYTECHAR, VarType.VARTYPE_SHORTCHAR, VarType.VARTYPE_UNKNOWN, VarType.VARTYPE_NULL,
      new VarType(CodeType.GROUP2EMPTY), new VarType(CodeType.NOTINITIALIZED), new VarType(CodeType.ADDRESS),
      new VarType(CodeType.OBJECT, 3, "a/B"), new VarType(CodeType.INT, 2).resizeArrayDim(0),
      new VarType(CodeType.INT, 0, "Q"), new VarType("TT;"),
      GenericType.parse("Ljava/util/Map<TK;+Ljava/lang/Number;>;"),
      GenericType.parse("Ljava/util/List<-La/B;>;"),
      GenericType.parse("[Ljava/util/List<*>;"),
      GenericType.parse("La/Outer<TT;>.Inner<TU;>;"),
    };
    for (int i = 0; i < types.length; i++) {
      l.add(new VarExprent(20 + i, types[i], null));
    }

    ConstExprent shared = ci(42);
    l.add(new FunctionExprent(FunctionType.ADD, list(shared, shared), null));
    l.add(shared);

    BasicBlockStatement b = block();
    b.setExprents(l);
    DummyExitStatement exit = new DummyExitStatement();
    RootStatement root = new RootStatement(b, exit, MT);
    emit("exprents", root);
  }

  // ---- 4. the trace file: DecompileRecord.add through kuna.vftrace.Trace ----

  static RootStatement tiny(StructMethod mt) {
    BasicBlockStatement b = block();
    return new RootStatement(b, new DummyExitStatement(), mt);
  }

  static void record(String path) throws Exception {
    kuna.vftrace.Trace.start(Paths.get(path), "engine=vineflower", null);
    start();
    RootStatement root = tiny(MT);
    org.jetbrains.java.decompiler.main.rels.DecompileRecord rec = new org.jetbrains.java.decompiler.main.rels.DecompileRecord(MT);
    rec.add("Initial", root);
    rec.add("CondenseSequences", root);
    root.getFirst().setExprents(list(ci(1)));
    rec.incrementMainLoop();
    rec.add("Start", root);
    rec.add("Start", root);
    rec.incrementMergeLoop();
    rec.add("MergeLoopStart", root);
    rec.resetMergeLoop();
    rec.resetMainLoop();
    rec.add("MainLoopEnd", root);
    rec.add("NoTree", null);
    start();
    RootStatement root2 = tiny(MT2);
    new org.jetbrains.java.decompiler.main.rels.DecompileRecord(MT2).add("Initial", root2);
    new org.jetbrains.java.decompiler.main.rels.DecompileRecord(MT).add("FinalSimplifySwitches", root);
    new org.jetbrains.java.decompiler.main.rels.DecompileRecord(MT2).add("FinalPrettifyIfs", root);
    kuna.vftrace.Trace.finish();
  }

  static StructMethod MT2;

  public static void main(String[] args) throws Exception {
    install();
    StructClass shapes = load(args[0]);
    MT = shapes.getMethod("straight", "(II)I");
    MT2 = shapes.getMethod("ifElse", "(I)I");
    REC = load(args[1]);
    blocks();
    control();
    exprents();
    Files.writeString(Paths.get(args[2]), OUT.toString());
    record(args[3]);
  }
}

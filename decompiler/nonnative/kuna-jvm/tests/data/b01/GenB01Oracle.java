// Golden vectors for the B01 slice (InvocationExprent's opcode constructor,
// checkExprTypeBounds and copy), computed by the real Vineflower 1.12.0 classes
// over every invoke instruction of B01Calls.class. Regenerate with
// `sh gen.sh <vineflower-1.12.0.jar>`.

import java.io.File;
import java.lang.reflect.Field;
import java.nio.file.Files;
import java.util.*;

import org.jetbrains.java.decompiler.code.FullInstructionSequence;
import org.jetbrains.java.decompiler.code.Instruction;
import org.jetbrains.java.decompiler.main.ClassesProcessor;
import org.jetbrains.java.decompiler.main.DecompilerContext;
import org.jetbrains.java.decompiler.main.collectors.CounterContainer;
import org.jetbrains.java.decompiler.main.extern.IFernflowerPreferences;
import org.jetbrains.java.decompiler.modules.decompiler.exps.*;
import org.jetbrains.java.decompiler.modules.decompiler.vars.CheckTypesResult;
import org.jetbrains.java.decompiler.struct.StructClass;
import org.jetbrains.java.decompiler.struct.StructMethod;
import org.jetbrains.java.decompiler.struct.attr.StructBootstrapMethodsAttribute;
import org.jetbrains.java.decompiler.struct.attr.StructGeneralAttribute;
import org.jetbrains.java.decompiler.struct.consts.ConstantPool;
import org.jetbrains.java.decompiler.struct.consts.LinkConstant;
import org.jetbrains.java.decompiler.struct.consts.PooledConstant;
import org.jetbrains.java.decompiler.struct.gen.CodeType;
import org.jetbrains.java.decompiler.struct.gen.VarType;
import org.jetbrains.java.decompiler.util.DataInputFullStream;
import org.jetbrains.java.decompiler.util.collections.ListStack;

public class GenB01Oracle {
  static final VarType[] TYPES = {
    VarType.VARTYPE_INT, VarType.VARTYPE_LONG, VarType.VARTYPE_STRING,
    VarType.VARTYPE_DOUBLE, VarType.VARTYPE_BOOLEAN, new VarType(CodeType.OBJECT, 0, "java/util/List")
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

  static void setField(Object o, Class<?> cls, String name, Object value) throws Exception {
    Field f = cls.getDeclaredField(name);
    f.setAccessible(true);
    f.set(o, value);
  }

  static void install() throws Exception {
    Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
    uf.setAccessible(true);
    sun.misc.Unsafe u = (sun.misc.Unsafe) uf.get(null);
    DecompilerContext ctx = (DecompilerContext) u.allocateInstance(DecompilerContext.class);
    setField(ctx, DecompilerContext.class, "counterContainer", new CounterContainer());
    setField(ctx, DecompilerContext.class, "properties", new HashMap<String, Object>(IFernflowerPreferences.getDefaults()));
    ClassesProcessor cp = (ClassesProcessor) u.allocateInstance(ClassesProcessor.class);
    setField(cp, ClassesProcessor.class, "mapRootClasses", new HashMap<>());
    setField(ctx, DecompilerContext.class, "classProcessor", cp);
    DecompilerContext.setCurrentContext(ctx);
  }

  static String vt(VarType t) {
    if (t == null) return "N";
    return t.type.name() + ":" + t.arrayDim + ":" + (t.value == null ? "N" : t.value);
  }

  static String var(Exprent e) {
    if (e == null) return "N";
    return e instanceof VarExprent v ? "v" + v.getIndex() : "?" + e.type;
  }

  static String vars(List<Exprent> es) {
    if (es.isEmpty()) return "-";
    StringJoiner j = new StringJoiner(",");
    for (Exprent e : es) j.add(var(e));
    return j.toString();
  }

  static String bounds(List<CheckTypesResult.ExprentTypePair> ps) {
    if (ps.isEmpty()) return "-";
    StringJoiner j = new StringJoiner(",");
    for (CheckTypesResult.ExprentTypePair p : ps) j.add(var(p.exprent) + "=" + vt(p.type));
    return j.toString();
  }

  static String bits(BitSet s) {
    if (s == null) return "N";
    if (s.isEmpty()) return "-";
    StringJoiner j = new StringJoiner(",");
    for (int i = s.nextSetBit(0); i >= 0; i = s.nextSetBit(i + 1)) j.add(Integer.toString(i));
    return j.toString();
  }

  static String args(List<PooledConstant> args) {
    if (args == null) return "N";
    if (args.isEmpty()) return "-";
    StringJoiner j = new StringJoiner(",");
    for (PooledConstant c : args) j.add(c == null ? "N" : (c instanceof LinkConstant l ? "L" + l.index1 : "P" + c.type));
    return j.toString();
  }

  public static void main(String[] argv) throws Exception {
    install();
    StructGeneralAttribute.init();
    byte[] bytes = Files.readAllBytes(new File(argv[0]).toPath());
    StructClass cl = StructClass.create(new DataInputFullStream(bytes), true);
    ConstantPool pool = cl.getPool();
    StructBootstrapMethodsAttribute bootstrap = cl.getAttribute(StructGeneralAttribute.ATTRIBUTE_BOOTSTRAP_METHODS);

    for (StructMethod mt : cl.getMethods()) {
      mt.expandData(cl);
      FullInstructionSequence seq = mt.getInstructionSequence();
      if (seq == null) continue;
      for (int i = 0; i < seq.length(); i++) {
        Instruction instr = seq.getInstr(i);
        int op = instr.opcode;
        if (op < 182 || op > 186) continue;

        DecompilerContext.getCounterContainer().setCounter(CounterContainer.EXPRESSION_COUNTER, 1);
        ListStack<Exprent> stack = new ListStack<>();
        for (int k = 0; k < TYPES.length; k++) stack.push(new VarExprent(k, TYPES[k], null));

        LinkConstant cn = pool.getLinkConstant(instr.operand(0));
        LinkConstant bm = null;
        List<PooledConstant> ba = null;
        if (op == 186 && bootstrap != null) {
          bm = bootstrap.getMethodReference(cn.index1);
          ba = bootstrap.getMethodArguments(cn.index1);
        }
        BitSet offsets = new BitSet();
        offsets.set(instr.startOffset);

        InvocationExprent e = new InvocationExprent(op, cn, bm, ba, stack, offsets);
        String at = mt.getName() + "@" + instr.startOffset;
        p("inv", at, op, e.id, e.getName(), e.getClassname(), e.isStatic() ? 1 : 0, e.getFunctype(),
          e.getInvocationType(), e.getInvokeDynamicClassSuffix(), e.getStringDescriptor(),
          vt(e.getExprType()), var(e.getInstance()), vars(e.getLstParameters()), stack.size(),
          e.getBootstrapMethod() == null ? "N" : e.getBootstrapMethod().elementname, args(e.getBootstrapArguments()),
          bits(e.bytecode));

        CheckTypesResult r = e.checkExprTypeBounds();
        p("bnd", at, bounds(r.getLowerBounds()), bounds(r.getUpperBounds()));

        InvocationExprent c = (InvocationExprent) e.copy();
        boolean instIsP0 = c.getInstance() != null && !c.getLstParameters().isEmpty() && c.getInstance() == c.getLstParameters().get(0);
        p("cpy", at, c.id, var(c.getInstance()), vars(c.getLstParameters()), instIsP0 ? 1 : 0,
          c.getInvokeDynamicClassSuffix(), bits(c.bytecode), e.equals(c) ? 1 : 0);
      }
    }
    System.out.print(OUT);
  }
}

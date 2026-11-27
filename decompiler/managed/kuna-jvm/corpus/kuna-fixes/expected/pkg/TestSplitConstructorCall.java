package pkg;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import kotlin.Metadata;
import kotlin.jvm.internal.Intrinsics;
import kotlin.jvm.internal.SourceDebugExtension;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

@Metadata(
   mv = {2, 0, 0},
   k = 1,
   xi = 48,
   d1 = "\u0000L\n\u0002\u0018\u0002\n\u0002\u0010\u0000\n\u0000\n\u0002\u0010(\n\u0002\u0010\u000e\n\u0002\b\u000b\n\u0002\u0010\b\n\u0002\b\u0005\n\u0002\u0010\u000b\n\u0002\b\u0005\n\u0002\u0018\u0002\n\u0002\b\b\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0018\u0002\n\u0002\b\u0003\n\u0002\u0010 \n\u0000\n\u0002\u0010\u0011\n\u0002\b\u0003\u0018\u00002\u00020\u0001B\u0015\u0012\f\u0010\u0002\u001a\b\u0012\u0004\u0012\u00020\u00040\u0003¢\u0006\u0004\b\u0005\u0010\u0006J\u0006\u0010$\u001a\u00020%J\u0006\u0010&\u001a\u00020%J\u0006\u0010'\u001a\u00020%J\u0006\u0010(\u001a\u00020)J\b\u0010*\u001a\u0004\u0018\u00010%J\u0006\u0010+\u001a\u00020\u0004J!\u0010,\u001a\b\u0012\u0004\u0012\u00020%0-2\u000e\u0010.\u001a\n\u0012\u0006\u0012\u0004\u0018\u00010\u00040/¢\u0006\u0002\u00100J\u0011\u00101\u001a\u00020\u00102\u0006\u00102\u001a\u00020\u0010H\u0082\bR\u0014\u0010\u0002\u001a\b\u0012\u0004\u0012\u00020\u00040\u0003X\u0082\u0004¢\u0006\u0002\n\u0000R\u001c\u0010\u0007\u001a\u0004\u0018\u00010\u0004X\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\b\u0010\t\"\u0004\b\n\u0010\u000bR\u001c\u0010\f\u001a\u0004\u0018\u00010\u0004X\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\r\u0010\t\"\u0004\b\u000e\u0010\u000bR\u001a\u0010\u000f\u001a\u00020\u0010X\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\u0011\u0010\u0012\"\u0004\b\u0013\u0010\u0014R\u001a\u0010\u0015\u001a\u00020\u0016X\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\u0017\u0010\u0018\"\u0004\b\u0019\u0010\u001aR\u001c\u0010\u001b\u001a\u0004\u0018\u00010\u001cX\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\u001d\u0010\u001e\"\u0004\b\u001f\u0010 R\u001c\u0010!\u001a\u0004\u0018\u00010\u001cX\u0086\u000e¢\u0006\u000e\n\u0000\u001a\u0004\b\"\u0010\u001e\"\u0004\b#\u0010 ",
   d2 = {
         "Lpkg/TestSplitConstructorCall;",
         "",
         "names",
         "",
         "",
         "<init>",
         "(Ljava/util/Iterator;)V",
         "name",
         "getName",
         "()Ljava/lang/String;",
         "setName",
         "(Ljava/lang/String;)V",
         "domain",
         "getDomain",
         "setDomain",
         "index",
         "",
         "getIndex",
         "()I",
         "setIndex",
         "(I)V",
         "upper",
         "",
         "getUpper",
         "()Z",
         "setUpper",
         "(Z)V",
         "bytes",
         "Lpkg/SplitPart$Builder;",
         "getBytes",
         "()Lpkg/SplitPart$Builder;",
         "setBytes",
         "(Lpkg/SplitPart$Builder;)V",
         "number",
         "getNumber",
         "setNumber",
         "build",
         "Lpkg/SplitEntry;",
         "both",
         "next",
         "format",
         "Lpkg/SplitFormat;",
         "orNull",
         "guarded",
         "collect",
         "",
         "keys",
         "",
         "([Ljava/lang/String;)Ljava/util/List;",
         "checkIndex",
         "i"
   }
)
@SourceDebugExtension(
   "SMAP\nTestSplitConstructorCall.kt\nKotlin\n*S Kotlin\n*F\n+ 1 TestSplitConstructorCall.kt\npkg/TestSplitConstructorCall\n*L\n1#1,58:1\n54#1,2:59\n*S KotlinDebug\n*F\n+ 1 TestSplitConstructorCall.kt\npkg/TestSplitConstructorCall\n*L\n28#1:59,2\n*E\n"
)
public final class TestSplitConstructorCall {
   @NotNull
   private final Iterator<String> names;
   @Nullable
   private String name;
   @Nullable
   private String domain;
   private int index;
   private boolean upper;
   @Nullable
   private SplitPart.Builder bytes;
   @Nullable
   private SplitPart.Builder number;

   public TestSplitConstructorCall(@NotNull Iterator<String> names) {
      Intrinsics.checkNotNullParameter(names, "names");
      super();
      this.names = names;
   }

   @Nullable
   public final String getName() {
      return this.name;
   }

   public final void setName(@Nullable String var1) {
      this.name = var1/* $VF was: <set-?> */;
   }

   @Nullable
   public final String getDomain() {
      return this.domain;
   }

   public final void setDomain(@Nullable String var1) {
      this.domain = var1/* $VF was: <set-?> */;
   }

   public final int getIndex() {
      return this.index;
   }

   public final void setIndex(int var1) {
      this.index = var1/* $VF was: <set-?> */;
   }

   public final boolean getUpper() {
      return this.upper;
   }

   public final void setUpper(boolean var1) {
      this.upper = var1/* $VF was: <set-?> */;
   }

   @Nullable
   public final SplitPart.Builder getBytes() {
      return this.bytes;
   }

   public final void setBytes(@Nullable SplitPart.Builder var1) {
      this.bytes = var1/* $VF was: <set-?> */;
   }

   @Nullable
   public final SplitPart.Builder getNumber() {
      return this.number;
   }

   public final void setNumber(@Nullable SplitPart.Builder var1) {
      this.number = var1/* $VF was: <set-?> */;
   }

   @NotNull
   public final SplitEntry build() {
      String var10003 = this.name;
      if (var10003 == null) {
         throw new IllegalStateException("name == null");
      } else {
         return new SplitEntry(this.index, var10003);
      }
   }

   @NotNull
   public final SplitEntry both() {
      String var10002 = this.domain;
      if (var10002 == null) {
         throw new IllegalStateException("domain == null");
      } else {
         int var1 = var10002.length();
         String var10003 = this.name;
         if (var10003 == null) {
            throw new IllegalStateException("name == null");
         } else {
            return new SplitEntry(var1, var10003);
         }
      }
   }

   @NotNull
   public final SplitEntry next() {
      int i$iv = this.index++;
      i$iv = i$iv;
      int $i$f$checkIndex = 0;
      if (i$iv < 0) {
         throw new ArithmeticException("Index overflow has happened.");
      } else {
         return new SplitEntry(i$iv, (String)this.names.next());
      }
   }

   @NotNull
   public final SplitFormat format() {
      SplitPart var1;
      boolean var10002;
      label19: {
         var10002 = this.upper;
         SplitPart.Builder var10003 = this.bytes;
         if (var10003 != null) {
            var1 = var10003.build();
            if (var1 != null) {
               break label19;
            }
         }

         var1 = new SplitPart(1);
      }

      SplitPart.Builder var10004 = this.number;
      if (var10004 != null) {
         SplitPart var2 = var10004.build();
         if (var2 != null) {
            return new SplitFormat(var10002, var1, var2);
         }
      }

      return new SplitFormat(var10002, var1, new SplitPart(2));
   }

   @Nullable
   public final SplitEntry orNull() {
      String var10003 = this.name;
      return var10003 == null ? null : new SplitEntry(this.index, var10003);
   }

   @NotNull
   public final String guarded() {
      try {
         String var10003 = this.name;
         if (var10003 == null) {
            throw new IllegalArgumentException("no name");
         }

         SplitEntry e = new SplitEntry(this.index, var10003);
         return e.getName();
      } catch (IllegalArgumentException e) {
         return "caught " + e.getMessage();
      }
   }

   @NotNull
   public final List<SplitEntry> collect(@NotNull String[] keys) {
      Intrinsics.checkNotNullParameter(keys, "keys");
      ArrayList out = new ArrayList();
      int var3 = 0;

      for (int var4 = keys.length; var3 < var4; var3++) {
         String k = keys[var3];
         int var6 = this.index++;
         if (k != null) {
            String var7 = k;
            int var8 = var6;
            out.add(new SplitEntry(var8, var7));
         }
      }

      return (List<SplitEntry>)out;
   }

   private final int checkIndex(int i) {
      int $i$f$checkIndex = 0;
      if (i < 0) {
         throw new ArithmeticException("Index overflow has happened.");
      } else {
         return i;
      }
   }
}

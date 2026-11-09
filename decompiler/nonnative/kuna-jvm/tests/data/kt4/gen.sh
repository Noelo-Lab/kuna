#!/bin/sh
# Regenerates the KT4 fixture classes (kotlinc 2.0.0) and the vectors made by
# Vineflower 1.12.0's own Kotlin plugin through KtMembersDump.java:
#   members.golden   KFunction/KProperty/KConstructor parse facts, object and
#                    enum processors, accessor wiring, default arguments
#   ctors.golden     KConstructor.parse + writePrimaryConstructor/stringify of
#                    the classes whose constructors take no parameters
#   isolated.golden  KEnumProcessor, markObjectInstance and DefaultArgsMap.from
#                    with synthetic parameters (no KType/KContract needed)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
JDK=${JDK:-$HOME/kwt/jvm/jdks/jdk17}
VF=${VF:-$HOME/kwt/jvm/oracle/vineflower-1.12.0.jar}
KOTLINC=${KOTLINC:-$HOME/kwt/jvm/jdks/kotlin/x200/kotlinc}
WORK=${WORK:-$HOME/kwt/jvm/scratch/kt4/gen}

mkdir -p "$WORK/kc"
if [ ! -d "$HERE/classes/kt4" ]; then
  JAVA_HOME=$JDK "$KOTLINC/bin/kotlinc" -jvm-target 17 -Xcontext-receivers -Xlambdas=class \
    -Xsam-conversions=class -module-name kt4members -d "$WORK/kc" "$HERE"/src/*.kt 2>/dev/null
  mkdir -p "$HERE/classes"
  cp -r "$WORK/kc/kt4" "$HERE/classes/"
fi

(cd "$WORK" && unzip -o -q "$VF" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$VF:$WORK/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$WORK" "$HERE/KtMembersDump.java"
run() { "$JDK/bin/java" -cp "$CP:$WORK" KtMembersDump "$@"; }

run members "$HERE/classes" > "$HERE/members.golden"
run ctors "$HERE/classes" kt4/CtorOnly kt4/InitOnly kt4/Obj kt4/E2 > "$HERE/ctors.golden"
run isolated "$HERE/classes" > "$HERE/isolated.golden"

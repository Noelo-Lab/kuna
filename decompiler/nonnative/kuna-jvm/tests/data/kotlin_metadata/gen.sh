#!/bin/sh
# Regenerates crafted.txt with Vineflower 1.12.0's own Kotlin plugin (BitEncoding + shaded
# protobuf + MetadataNameResolver): KtMetadataDump dumps each gen_cases.py value; crafted.txt
# keeps a case's raw values, the first message or exception of its dump, and for the
# string-table cases (names_*) every name the resolver gives.
# Env: KUNA_JVM_ORACLE (decompiler/nonnative/kuna-jvm/tools/oracle/README.md), or the overrides JDK
# (default $KUNA_JVM_ORACLE/jdks/jdk17), VF (the release jar) and WORK (default $KUNA_JVM_ORACLE/work/kt1-gen).
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
NO_ORACLE='set it to the JVM oracle workspace (decompiler/nonnative/kuna-jvm/tools/oracle/README.md)'
JDK=${JDK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/jdks/jdk17}
VF=${VF:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/oracle/vineflower-1.12.0.jar}
WORK=${WORK:-${KUNA_JVM_ORACLE:?$NO_ORACLE}/work/kt1-gen}
mkdir -p "$WORK"
(cd "$WORK" && unzip -o -q "$VF" META-INF/plugins/vineflower-kotlin-0.1.0.jar)
CP="$WORK/META-INF/plugins/vineflower-kotlin-0.1.0.jar"
"$JDK/bin/javac" -nowarn -cp "$CP" -d "$WORK" "$HERE/KtMetadataDump.java"
python3 "$HERE/gen_cases.py" > "$WORK/cases.in"
"$JDK/bin/java" -cp "$CP:$WORK" KtMetadataDump cases "$WORK/cases.in" | awk '
  /^=== / { label = substr($0, 5); print label; done = 0; names = 0; next }
  /^raw / { print "  " substr($0, 5); next }
  !done && (/^msg / || /^!/ || /^nok$/ || /^nod1$/) { print "= " $0; done = 1 }
  /^names / { names = label ~ /^case!names_/; next }
  names && /^ -?[0-9]+ / { print "name" $0 }
' > "$HERE/crafted.txt"

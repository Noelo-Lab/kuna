#!/usr/bin/env bash
# Regenerates src/p4_calls/classpath_helper_jdk.txt, the JDK facts ClasspathHelper.findMethod
# answers from (see JdkVarargs.java). JAVA_HOME picks the JDK (default: OpenJDK 21).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
crate=$(cd "$here/../../.." && pwd)
java_home=${JAVA_HOME:-/usr/lib/jvm/java-21-openjdk-amd64}
work=$(mktemp -d)
"$java_home/bin/javac" -d "$work" "$here/JdkVarargs.java"
"$java_home/bin/java" -cp "$work" JdkVarargs > "$crate/src/p4_calls/classpath_helper_jdk.txt"

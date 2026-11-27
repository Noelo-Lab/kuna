"""Pinned dataset definition: the five most-starred real Java projects on GitHub,
the two held-out sets, and where the benchmark keeps its files.

Selection (``gh api search/repositories -f q='language:java stars:>20000'
-f sort=stars``, queried 2026-11-09) is documented in ``docs/dataset.md``; this
module only pins what is fetched.

Every path is under one workspace directory, ``--workspace`` or
``$JAVABENCH_WORKSPACE``: ``dataset/`` (``$JAVABENCH_DATASET`` overrides),
``results/`` (``$JAVABENCH_RESULTS``), ``tools/`` (coursier, Temurin 25, the
Vineflower jar, the compile server) and ``locks/`` (locks of projects without a
pin in ``pins/``). The JDK homes are ``$JAVABENCH_JDK<N>``; 17, 19 and 21 default
to the Debian/Ubuntu packages, 25 to the Temurin build ``fetch`` installs.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from pathlib import Path

PACKAGE_DIR = Path(__file__).resolve().parent
PINS_DIR = PACKAGE_DIR / "pins"
NO_WORKSPACE = ("JAVABENCH_WORKSPACE is not set: pass --workspace DIR or set it to the benchmark's "
                "workspace directory (scripts/javabench/README.md)")
_CONFIGURED = ("WORKSPACE", "DATASET_DIR", "RESULTS_DIR", "TOOLS_DIR", "LOCK_DIR", "JDKS")

MAVEN = "https://repo1.maven.org/maven2"


def configure(workspace: str | os.PathLike | None = None) -> Path:
    """Resolve every path from ``workspace`` (else ``$JAVABENCH_WORKSPACE``), and export
    it so worker processes resolve the same; exits with ``NO_WORKSPACE`` when neither is set."""
    root = workspace or os.environ.get("JAVABENCH_WORKSPACE")
    if not root:
        raise SystemExit(NO_WORKSPACE)
    root = Path(root).resolve()
    os.environ["JAVABENCH_WORKSPACE"] = str(root)
    tools = root / "tools"
    g = globals()
    g["WORKSPACE"] = root
    g["DATASET_DIR"] = Path(os.environ.get("JAVABENCH_DATASET") or root / "dataset")
    g["RESULTS_DIR"] = Path(os.environ.get("JAVABENCH_RESULTS") or root / "results")
    g["TOOLS_DIR"] = tools
    g["LOCK_DIR"] = root / "locks"
    g["JDKS"] = {
        17: os.environ.get("JAVABENCH_JDK17") or "/usr/lib/jvm/java-17-openjdk-amd64",
        19: os.environ.get("JAVABENCH_JDK19") or "/usr/lib/jvm/java-19-openjdk-amd64",
        21: os.environ.get("JAVABENCH_JDK21") or "/usr/lib/jvm/java-21-openjdk-amd64",
        25: os.environ.get("JAVABENCH_JDK25") or str(tools / "jdk25-x/jdk-25.0.5+8"),
    }
    return root


def __getattr__(name: str):
    if name in _CONFIGURED:
        configure()
        return globals()[name]
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


# Temurin 25 is fetched by `javabench fetch` (Ghidra 12.2 ships Java 25 class
# files, and Spring Framework 7 / Guava 33 were built by a JDK 25 javac).
JDK25_URL = (
    "https://github.com/adoptium/temurin25-binaries/releases/download/"
    "jdk-25.0.5%2B8/OpenJDK25U-jdk_x64_linux_hotspot_25.0.5_8.tar.gz"
)
JDK25_SHA256 = "68982529869257d0ee6b824ec5aa270ff8b118bb438670da0b97ebdab178d7fb"
COURSIER_URL = "https://github.com/coursier/coursier/releases/download/v2.1.26/cs-x86_64-pc-linux.gz"
COURSIER_SHA256 = "348e37bc2a8c706640e6b032c4551a0e055a7f1537485b4a91540e9d3598ec6d"


@dataclass
class Project:
    name: str
    repo: str
    stars: int
    version: str
    kind: str  # "maven" | "ghidra"
    modules: list[str]  # maven coords g:a:v, or ghidra module paths
    # Maven-only dependency spec: BOM + coords (``g:a:`` = version from BOM).
    bom: str | None = None
    dep_coords: list[str] = field(default_factory=list)
    # Artifacts (g:a) that are part of the project itself and must not be deps.
    self_artifacts: list[str] = field(default_factory=list)
    dist_url: str | None = None
    dist_sha256: str | None = None
    # javac used for recompilation: JDK major matching the original build JDK.
    javac_jdk: int = 21
    build_jdk_note: str = ""
    # "release": javac --release N ; "source-target": -source N -target N against the
    # full JDK (what the project's own build does, e.g. Guava uses Java 9+ APIs from 8 code)
    release_mode: str = "release"
    # Default scored set: stratified sample size (0 = everything).
    sample_size: int = 3000

    @property
    def dir(self) -> Path:
        return DATASET_DIR / self.name


PROJECTS: dict[str, Project] = {
    "ghidra": Project(
        name="ghidra",
        repo="NationalSecurityAgency/ghidra",
        stars=98836,
        version="12.2",
        kind="ghidra",
        modules=["Framework/SoftwareModeling"],
        dist_url=(
            "https://github.com/NationalSecurityAgency/ghidra/releases/download/"
            "Ghidra_12.2_build/ghidra_12.2_PUBLIC_20261016.zip"
        ),
        dist_sha256=None,  # pinned in pins/ghidra.json
        javac_jdk=25,
        build_jdk_note="class files are major 69 (Java 25); recompiled with Temurin javac 25.0.5",
    ),
    "spring-boot": Project(
        name="spring-boot",
        repo="spring-projects/spring-boot",
        stars=81787,
        version="4.1.2",
        kind="maven",
        modules=["org.springframework.boot:spring-boot:4.1.2"],
        bom="org.springframework.boot:spring-boot-dependencies:4.1.2",
        # core/spring-boot/build.gradle @ v4.1.2: api + optional deps
        dep_coords=[
            "org.springframework:spring-core:", "org.springframework:spring-context:",
            "ch.qos.logback:logback-classic:", "com.google.code.gson:gson:",
            "io.projectreactor:reactor-core:", "jakarta.servlet:jakarta.servlet-api:",
            "jakarta.validation:jakarta.validation-api:", "org.apache.groovy:groovy:",
            "org.apache.logging.log4j:log4j-api:", "org.apache.logging.log4j:log4j-core:",
            "org.apache.logging.log4j:log4j-jul:", "org.crac:crac:",
            "org.jetbrains.kotlin:kotlin-reflect:", "org.jetbrains.kotlin:kotlin-stdlib:",
            "org.slf4j:jul-to-slf4j:", "org.springframework:spring-test:",
            "org.springframework:spring-web:", "org.yaml:snakeyaml:",
            "tools.jackson.core:jackson-databind:",
        ],
        self_artifacts=["org.springframework.boot:spring-boot"],
        javac_jdk=25,
        build_jdk_note=("manifest Build-Jdk-Spec 17 (Gradle JVM), class files major 61; javac chosen by source-oracle "
                        "calibration: byte-exact 99.13% (17), 99.97% (21), 100.0% (25) -- the compiling toolchain was >=18 "
                        "(JDK-8272564 invokeinterface for Object methods on interfaces)"),
    ),
    "elasticsearch": Project(
        name="elasticsearch",
        repo="elastic/elasticsearch",
        stars=78530,
        version="9.5.6",
        kind="maven",
        modules=["org.elasticsearch:elasticsearch:9.5.6"],
        dep_coords=["org.elasticsearch:elasticsearch:9.5.6"],
        self_artifacts=["org.elasticsearch:elasticsearch"],
        javac_jdk=21,
        build_jdk_note="manifest Created-By 21.0.7 (Temurin); class files major 65",
    ),
    "spring-framework": Project(
        name="spring-framework",
        repo="spring-projects/spring-framework",
        stars=60307,
        version="7.0.10",
        kind="maven",
        modules=[
            "org.springframework:spring-core:7.0.10",
            "org.springframework:spring-beans:7.0.10",
            "org.springframework:spring-context:7.0.10",
        ],
        bom="org.springframework.boot:spring-boot-dependencies:4.1.2",  # Boot 4.1.2 pins Framework 7.0.10
        # {spring-core,spring-beans,spring-context}.gradle @ v7.0.10: api/optional/compileOnly;
        # explicit versions are framework-platform.gradle @ v7.0.10 pins absent from / different in the Boot BOM
        dep_coords=[
            "org.springframework:spring-aop:", "org.springframework:spring-expression:",
            "org.springframework:spring-instrument:",
            "commons-logging:commons-logging:", "org.jspecify:jspecify:",
            "com.google.code.findbugs:jsr305:3.0.2", "io.projectreactor.tools:blockhound:1.0.8.RELEASE",
            "org.graalvm.sdk:graal-sdk:22.3.1",
            "io.micrometer:context-propagation:", "io.netty:netty-buffer:",
            "io.projectreactor:reactor-core:", "io.reactivex.rxjava3:rxjava:3.1.12",
            "io.smallrye.reactive:mutiny:1.10.0", "net.sf.jopt-simple:jopt-simple:5.0.4",
            "org.aspectj:aspectjweaver:", "org.eclipse.jetty:jetty-io:",
            "org.jetbrains.kotlin:kotlin-reflect:", "org.jetbrains.kotlin:kotlin-stdlib:",
            "org.jetbrains.kotlinx:kotlinx-coroutines-core:",
            "org.jetbrains.kotlinx:kotlinx-coroutines-reactor:",
            "jakarta.inject:jakarta.inject-api:2.0.1", "org.apache.groovy:groovy-xml:",
            "org.reactivestreams:reactive-streams:", "org.yaml:snakeyaml:",
            "io.micrometer:micrometer-observation:",
            "jakarta.annotation:jakarta.annotation-api:", "jakarta.ejb:jakarta.ejb-api:4.0.1",
            "jakarta.enterprise.concurrent:jakarta.enterprise.concurrent-api:3.1.1",
            "jakarta.interceptor:jakarta.interceptor-api:2.2.0",
            "jakarta.validation:jakarta.validation-api:", "javax.money:money-api:1.1",
            "org.apache.groovy:groovy:", "org.apache-extras.beanshell:bsh:2.0b6",
            "org.crac:crac:", "org.hibernate.validator:hibernate-validator:",
            "com.palantir.javapoet:javapoet:0.7.0", "org.objenesis:objenesis:3.4",
        ],
        self_artifacts=[
            "org.springframework:spring-core",
            "org.springframework:spring-beans",
            "org.springframework:spring-context",
        ],
        javac_jdk=25,
        build_jdk_note="manifest Build-Jdk-Spec 25; class files major 61 (--release 17)",
    ),
    "guava": Project(
        name="guava",
        repo="google/guava",
        stars=51920,
        version="33.8.0-jre",
        kind="maven",
        modules=["com.google.guava:guava:33.8.0-jre"],
        dep_coords=["com.google.guava:guava:33.8.0-jre"],
        self_artifacts=["com.google.guava:guava"],
        javac_jdk=25,
        release_mode="source-target",
        build_jdk_note="manifest Build-Jdk-Spec 25; guava-parent pom: -source 1.8 -target 1.8; class files major 52",
    ),
}


def _held_out(name: str, repo: str, modules: list[str], deps: tuple[str, ...] = (), jdk: int = 17,
              mode: str = "release") -> Project:
    """A held-out project: Maven coordinates with explicit dependency versions, no BOM."""
    return Project(name=name, repo=repo, stars=0, version=modules[0].split(":")[2], kind="maven", modules=modules,
                   dep_coords=modules + list(deps), self_artifacts=[":".join(c.split(":")[:2]) for c in modules], javac_jdk=jdk,
                   release_mode=mode, build_jdk_note="javac chosen by source-oracle calibration (docs/dataset.md)")


# The first held-out set: eleven projects chosen by a rule written before any decompiler ran on
# them; fixes were later developed against them (docs/dataset.md, "The first held-out set").
HELDOUT1: dict[str, Project] = {
    "antlr4": _held_out("antlr4", "antlr/antlr4", ["org.antlr:antlr4-runtime:4.13.2"]),
    "h2": _held_out("h2", "h2database/h2database", ["com.h2database:h2:2.5.252"], (
        "org.apache.lucene:lucene-core:9.12.0", "org.apache.lucene:lucene-analysis-common:9.12.0",
        "org.apache.lucene:lucene-queryparser:9.12.0", "javax.servlet:javax.servlet-api:4.0.1",
        "jakarta.servlet:jakarta.servlet-api:6.1.0", "org.osgi:osgi.core:8.0.0",
        "org.osgi:org.osgi.service.jdbc:1.1.0", "org.locationtech.jts:jts-core:1.20.0",
        "org.slf4j:slf4j-api:2.0.16")),
    "httpcore5": _held_out("httpcore5", "apache/httpcomponents-core", [
        "org.apache.httpcomponents.core5:httpcore5:5.5", "org.apache.httpcomponents.core5:httpcore5-h2:5.5",
    ], ("org.conscrypt:conscrypt-openjdk-uber:2.8.0",), mode="source-target"),
    "fastjson2": _held_out("fastjson2", "alibaba/fastjson2", ["com.alibaba.fastjson2:fastjson2:2.0.66"], (
        "org.jetbrains.kotlin:kotlin-stdlib:2.1.20", "org.jetbrains.kotlin:kotlin-reflect:2.1.20",
    ), mode="source-target"),
    "tink": _held_out("tink", "tink-crypto/tink-java", ["com.google.crypto.tink:tink:1.24.0"], (
        "com.google.http-client:google-http-client:1.43.3", "org.conscrypt:conscrypt-openjdk-uber:2.7.0"), jdk=21),
    "janino": _held_out("janino", "janino-compiler/janino", [
        "org.codehaus.janino:janino:3.1.12", "org.codehaus.janino:commons-compiler:3.1.12",
    ], ("org.apache.ant:ant:1.10.11",)),
    "rhino": _held_out("rhino", "mozilla/rhino", ["org.mozilla:rhino:1.9.2"], jdk=21),
    "joda-time": _held_out("joda-time", "JodaOrg/joda-time", ["joda-time:joda-time:2.15.1"],
                           ("org.joda:joda-convert:1.9.2",)),
    "commons-math3": _held_out("commons-math3", "apache/commons-math", ["org.apache.commons:commons-math3:3.6.1"]),
    "jenetics": _held_out("jenetics", "jenetics/jenetics", ["io.jenetics:jenetics:9.1.0"], jdk=25),
    "nashorn": _held_out("nashorn", "openjdk/nashorn", ["org.openjdk.nashorn:nashorn-core:15.7"]),
}

# The second held-out set: one project per style slot, chosen by a rule written before any
# decompiler ran on them, measured once (docs/dataset.md, "The second held-out set").
HELDOUT2: dict[str, Project] = {
    "parboiled": _held_out("parboiled", "sirthias/parboiled", ["org.parboiled:parboiled-java:1.4.1"]),
    "hsqldb": _held_out("hsqldb", "sourceforge.net/projects/hsqldb", ["org.hsqldb:hsqldb:2.7.4"]),
    "undertow": _held_out("undertow", "undertow-io/undertow", ["io.undertow:undertow-core:2.4.4.Final"]),
    "dsl-json": _held_out("dsl-json", "ngs-doo/dsl-json", ["com.dslplatform:dsl-json:2.1.0"], (
        "javax.json.bind:javax.json.bind-api:1.0", "javax.json:javax.json-api:1.1.2",
        "com.thoughtworks.paranamer:paranamer:2.8", "com.google.code.findbugs:jsr305:3.0.2"), jdk=21),
    "jasypt": _held_out("jasypt", "jasypt/jasypt", ["org.jasypt:jasypt:1.9.3"], (
        "com.ibm.icu:icu4j:3.4.4", "javax.servlet:servlet-api:2.4")),
    "ecj": _held_out("ecj", "eclipse-jdt/eclipse.jdt.core", ["org.eclipse.jdt:ecj:3.46.100"], jdk=25,
                     mode="source-target"),
    "luaj": _held_out("luaj", "luaj/luaj", ["org.luaj:luaj-jse:3.0.1"]),
    "junit4": _held_out("junit4", "junit-team/junit4", ["junit:junit:4.13.2"]),
    "ojalgo": _held_out("ojalgo", "optimatika/ojAlgo", ["org.ojalgo:ojalgo:58.0.0"], jdk=25),
    "langchain4j": _held_out("langchain4j", "langchain4j/langchain4j", ["dev.langchain4j:langchain4j-core:1.24.1"],
                             jdk=25),
}
GROUPS: dict[str, dict[str, Project]] = {"insample": PROJECTS, "heldout1": HELDOUT1, "heldout2": HELDOUT2}
KNOWN: dict[str, Project] = {**PROJECTS, **HELDOUT1, **HELDOUT2}


def get(name: str) -> Project:
    if name not in KNOWN:
        raise SystemExit(f"unknown project {name!r}; known: {', '.join(KNOWN)}, or a set: {', '.join(GROUPS)}, all")
    return KNOWN[name]


def selected(names: list[str] | None) -> list[Project]:
    """The projects ``--project`` names: project names and the sets ``insample`` (the
    default), ``heldout1``, ``heldout2`` and ``all``, in the order given, each project once."""
    out: dict[str, Project] = {}
    for n in names or ["insample"]:
        group = KNOWN if n == "all" else GROUPS.get(n)
        for proj in (group.values() if group is not None else [get(n)]):
            out.setdefault(proj.name, proj)
    return list(out.values())


def resolve_run(r: str) -> Path:
    """A run directory, given as a path or as a name under the results directory."""
    p = Path(r)
    if not p.exists():
        p = RESULTS_DIR / r
    if not (p / "run.json").exists():
        raise SystemExit(f"not a javabench run: {r}")
    return p

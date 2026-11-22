import ObjectiveC

precedencegroup AssignmentPrecedence { assignment: true associativity: right }
public struct Int { var v: Builtin.Int64 }
@_silgen_name("ext_sink") func ext_sink(_ x: Int)
@_silgen_name("ext_get") func ext_get() -> Int
@_silgen_name("ext_obj") func ext_obj(_ o: NSObject)

class Gone: NSObject {
  var hits: Int = Int(v: Builtin.zeroInitializer())
  @inline(never) func handle() { ext_sink(hits); ext_sink(hits); hits = ext_get() }
  @objc func tap() { handle() }
}

class Kept: NSObject {
  var hits: Int = Int(v: Builtin.zeroInitializer())
  @inline(never) func handle() { ext_sink(hits); ext_sink(hits); hits = ext_get() }
  @objc func tap() { handle() }
}

class Kept2: Kept {
  @objc override func tap() { ext_sink(hits) }
}

class Pure: NSObject {
  var hits: Int = Int(v: Builtin.zeroInitializer())
  @objc func run() { ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); ext_sink(hits); }
}

public class Sym: NSObject {
  @objc @inline(never) public func act(_ o: NSObject) { ext_obj(o); ext_obj(o); ext_obj(o) }
}

@_silgen_name("ext_gone") func ext_gone() -> Gone
@_silgen_name("ext_kept") func ext_kept() -> Kept
public func use() { ext_gone().handle(); ext_kept().tap(); ext_kept().handle() }

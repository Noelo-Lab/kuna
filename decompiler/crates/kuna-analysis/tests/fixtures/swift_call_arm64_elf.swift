precedencegroup AssignmentPrecedence { assignment: true associativity: right }
public protocol Error {}
public struct Bad: Error { public var c: Builtin.Int64 }
@_silgen_name("cond") public func cond(_ x: Builtin.Int64) -> Builtin.Int1
@_silgen_name("sinkv") public func sinkv(_ x: Builtin.Int64)
@_silgen_name("altv") public func altv(_ x: Builtin.Int64) -> Builtin.Int64
public struct Bool { var v: Builtin.Int1
  public init(_ v: Builtin.Int1) { self.v = v }
  public var _value: Builtin.Int1 { return v } }
@inline(never) public func risky(_ x: Builtin.Int64) throws -> Builtin.Int64 {
  if Bool(cond(x)) { throw Bad(c: x) }
  return x
}
@inline(never) public func doCatch(_ x: Builtin.Int64) -> Builtin.Int64 {
  do { let r = try risky(x); sinkv(r); return r } catch { return altv(x) }
}

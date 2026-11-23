// Fixture for the Swift helper arm (`swifthelpers`): a conditional
// conformance of a generic struct used at one concrete type, so swiftc emits
// `__swift_instantiateConcreteTypeFromMangledName` (and its `Abstract` twin)
// and a lazy protocol witness table accessor for `Box<Unit> : Shape`; and
// generic callers of the generic types' metadata accessors (`swifttypes`).
precedencegroup AssignmentPrecedence { assignment: true associativity: right }
public protocol Shape { func area() -> Builtin.Int64 }
public struct Unit: Shape { public var u: Builtin.Int64
  public init(_ u: Builtin.Int64) { self.u = u }
  public func area() -> Builtin.Int64 { return u } }
public struct Box<T> { public var t: T
  public init(_ t: T) { self.t = t } }
extension Box: Shape where T: Shape {
  public func area() -> Builtin.Int64 { return t.area() } }
@inline(never) @_semantics("optimize.sil.specialize.generic.never")
public func measure<S: Shape>(_ s: S) -> Builtin.Int64 { return s.area() }
@inline(never) @_semantics("optimize.sil.specialize.generic.never")
public func useBox(_ v: Builtin.Int64) -> Builtin.Int64 { return measure(Box<Unit>(Unit(v))) }
@inline(never) @_semantics("optimize.sil.specialize.generic.never")
public func wrap<T: Shape>(_ t: T) -> Builtin.Int64 { return measure(Box<T>(t)) }
public final class Cell<T> { public init() {} }
@inline(never) @_semantics("optimize.sil.specialize.generic.never")
public func pair<T>(_ t: T.Type) -> (Cell<T>, Cell<T>) { return (Cell<T>(), Cell<T>()) }

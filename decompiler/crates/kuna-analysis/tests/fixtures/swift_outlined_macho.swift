// Fixture for the outlined value-witness helpers (`valuewitness`): values of
// types whose layout is only known at run time (they hold resilient
// Foundation values), copied, moved and destroyed through the outlined
// helpers swiftc emits, which fetch the type's metadata and call one value
// witness. Identical helpers of several types are merged into one that takes
// the metadata accessor as an argument; `URL?`'s helpers fetch its metadata
// through `__swift_instantiateConcreteTypeFromMangledName`.
import Foundation
public struct Stamp { public var date: Date; public var tag: String
  public init(date: Date, tag: String) { self.date = date; self.tag = tag } }
public struct Link { public var url: URL; public var title: String
  public init(url: URL, title: String) { self.url = url; self.title = title } }
public struct Ident { public var id: UUID; public var name: String
  public init(id: UUID, name: String) { self.id = id; self.name = name } }
public enum Slot { case stamp(Stamp), link(Link), none }
@inline(never) public func keep(_ s: Stamp) -> [Stamp] { var a: [Stamp] = []; a.append(s); a.append(s); return a }
@inline(never) public func keepLink(_ s: Link) -> [Link] { var a: [Link] = []; a.append(s); a.append(s); return a }
@inline(never) public func keepIdent(_ s: Ident) -> [Ident] { var a: [Ident] = []; a.append(s); a.append(s); return a }
@inline(never) public func pick(_ s: Slot) -> Stamp? { if case .stamp(let v) = s { return v }; return nil }
@inline(never) public func pickLink(_ s: Slot) -> Link? { if case .link(let v) = s { return v }; return nil }
@inline(never) public func swapOut(_ a: inout Stamp, _ b: Stamp) -> Stamp { let o = a; a = b; return o }
@inline(never) public func swapLink(_ a: inout Link, _ b: Link) -> Link { let o = a; a = b; return o }
@inline(never) public func swapIdent(_ a: inout Ident, _ b: Ident) -> Ident { let o = a; a = b; return o }
public enum Moment { case at(Date), never }
@inline(never) public func firstDate(_ d: [Date]) -> Date? { return d.first }
@inline(never) public func moment(_ m: Moment) -> Date? { if case .at(let d) = m { return d }; return nil }
@inline(never) public func swapURL(_ a: inout URL?, _ b: URL?) -> URL? { let o = a; a = b; return o }

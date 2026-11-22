// Fixture for the value-witness arm (`valuewitness`): a struct and an enum
// whose value witnesses are compiled into the image (each holds a reference),
// a generic struct whose witnesses come from its metadata pattern, and a
// generic function that copies, moves and destroys a `T` through the value
// witness table of its metadata argument.
public final class Box { public var n: Int; public init(_ n: Int) { self.n = n } }
public struct Pair { public var name: String; public var box: Box; public init(name: String, box: Box) { self.name = name; self.box = box } }
public enum Shape { case circle(Box), square(String), empty }
public struct Holder<T> { public var item: T; public var count: Int; public init(item: T, count: Int) { self.item = item; self.count = count } }
@inline(never) public func swapIn<T>(_ a: inout T, _ b: T) -> T {
    let old = a
    a = b
    return old
}
let n = CommandLine.arguments.count
var p = Pair(name: "p", box: Box(n))
let q = swapIn(&p, Pair(name: "q", box: Box(2)))
var s = n > 1 ? Shape.circle(Box(n)) : n > 2 ? Shape.square("sq") : .empty
let t = swapIn(&s, .empty)
var h = Holder(item: p, count: n)
let g = swapIn(&h, Holder(item: q, count: 3))
print(p.name, q.box.n, s, t, h.count, g.item.name)

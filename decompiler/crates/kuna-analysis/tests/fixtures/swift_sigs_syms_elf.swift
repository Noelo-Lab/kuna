// Fixture for declared Swift prototypes (`swiftcall`, an image that kept
// its symbols): a class whose methods return a three-word struct (in
// registers, where C would read it through the indirect-result register),
// a five-word struct (indirect), a payload-less enum, and an `async`
// method, with a struct method whose by-value `self` stays recovered.
public struct Triple { public var a: Int; public var b: Int; public var c: Int; public init(a: Int, b: Int, c: Int) { self.a = a; self.b = b; self.c = c } }
public struct Wide { public var a: Int; public var b: Int; public var c: Int; public var d: Int; public var e: Int; public init(a: Int, b: Int, c: Int, d: Int, e: Int) { self.a = a; self.b = b; self.c = c; self.d = d; self.e = e } }
public enum Color { case red, green, blue }
open class Box {
    public var base: Int = 1
    public var color: Color = .green
    public init() {}
    @inline(never) open func triple(_ k: Int) -> Triple { Triple(a: base, b: base &+ k, c: k) }
    @inline(never) open func wide(_ k: Int) -> Wide { Wide(a: base, b: k, c: base &* k, d: 2, e: 3) }
    @inline(never) open func scaled(_ k: Int, by factor: Int) -> Int { base &* k &+ factor }
    @inline(never) open func slow(_ k: Int) async -> Int { await Task.yield(); return base &+ k }
    @inline(never) open func tint(_ k: Int) -> Color { k > 1 ? color : .red }
}
extension Triple {
    @inline(never) public func sum() -> Int { a &+ b &+ c }
    @inline(never) public mutating func bump(_ k: Int) { a &+= k; c &-= k }
}
@inline(never) public func twice<T>(_ x: T, _ n: Int) -> Int { n &* 2 }
@inline(never) public func hashed<T: Hashable>(_ x: T, _ n: Int) -> Int { x.hashValue &+ n }
@inline(never) public func swap2(_ p: (Int, Int)) -> (Int, Int) { (p.1, p.0) }
@inline(never) public func sum3(_ t: Triple) -> Int { t.a &+ t.b &+ t.c }
@inline(never) public func sum5(_ w: Wide) -> Int { w.a &+ w.e }
@main struct Main {
    static func main() async {
        let n = CommandLine.arguments.count
        let box = Box()
        let t = box.triple(n)
        let w = box.wide(n)
        var t2 = t; t2.bump(n)
        let sw = swap2((n, 7))
        print(t.sum(), w.e, box.scaled(n, by: 2), box.tint(n), twice(w, n), hashed(n, 3), sw.0 &+ sw.1, sum3(t2), sum5(w))
        let r = await box.slow(n)
        print(r)
    }
}

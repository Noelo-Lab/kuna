// Fixture for the Swift calling-convention arm (`swiftcall`): a mutating
// struct method called after a runtime helper (`swift_beginAccess`) that
// runs between the caller setting the swiftself register and the call, once
// straight-line and once in a loop.
public final class Box { public var n = 0; public init() {} }
public struct Counter {
    public var a: Int
    public var b: Box
    @inline(never) public mutating func bump() { a &+= b.n }
}
@inline(never) public func once(_ c: inout Counter, _ o: Box) -> Int { o.n &+= 1; c.bump(); return o.n }
@inline(never) public func loop(_ c: inout Counter, _ o: Box, _ m: Int) -> Int {
    var t = 0
    for _ in 0..<m { o.n &+= 1; c.bump(); t &+= o.n }
    return t
}
var c0 = Counter(a: CommandLine.arguments.count, b: Box())
print(once(&c0, Box()), loop(&c0, Box(), 3))

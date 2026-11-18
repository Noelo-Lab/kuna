// Fixture for the Swift calling-convention arm (`swiftcall`): a mutating
// struct method whose signature has no declared form (its argument is an
// enum) called after a Swift leaf function that runs between the caller
// setting the swiftself register and the call and never reads it.
public enum Step { case by(Int), none }
public final class Box { public var n = 0; public init() {} }
public struct Counter {
    public var a: Int
    public var b: Box
    @inline(never) public mutating func bump(_ s: Step) { if case .by(let k) = s { a &+= k &+ b.n } }
}
@inline(never) public func tick(_ x: Int) -> Int { x &* 3 &+ 1 }
@inline(never) public func once(_ c: inout Counter, _ x: Int) -> Int { let t = tick(x); c.bump(.by(t)); return t }
@inline(never) public func twice(_ c: inout Counter, _ x: Int) { c.bump(.by(x)) }
var c0 = Counter(a: CommandLine.arguments.count, b: Box())
print(once(&c0, 5)); twice(&c0, 2); print(c0.a)

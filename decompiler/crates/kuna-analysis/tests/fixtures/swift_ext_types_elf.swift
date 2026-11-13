// Fixture for `swifttypes`: a struct nested in an extension of another
// module's type, whose context descriptor's parent is the extension
// context, used as a stored property of a local struct.
extension Int {
    public struct Pair { public var lo: Int; public var hi: Int; public init(lo: Int, hi: Int) { self.lo = lo; self.hi = hi } }
}
public struct Holder {
    public var pair: Int.Pair
    public var count: Int
    public init(pair: Int.Pair, count: Int) { self.pair = pair; self.count = count }
}
@inline(never) public func span(_ h: Holder) -> Int { h.pair.hi - h.pair.lo + h.count }

let n = CommandLine.arguments.count
print(span(Holder(pair: Int.Pair(lo: n, hi: 9), count: n)))

// Fixture for merged-code naming (`swiftmetadata` under identical code
// folding): built with `-Xfrontend -function-sections -Xlinker --icf=all`,
// so the linker folds functions with identical bodies and one address is
// claimed by several metadata records: the `tag` getters of `Alpha` and
// `Beta` (one protocol member, two types), and the `tag` getter and
// `describe()` of `Delta` (one type, two members). `Gamma`'s `key` getter
// stays its own.
public protocol Tagged { var tag: Int { get }; func describe() -> Int }
public protocol Keyed { var key: Int { get } }
public struct Alpha: Tagged { public var n: Int; public init(n: Int) { self.n = n }; public var tag: Int { 7 }; public func describe() -> Int { n * 5 + 1 } }
public struct Beta: Tagged { public var m: Int; public init(m: Int) { self.m = m }; public var tag: Int { 7 }; public func describe() -> Int { m * 9 } }
public struct Gamma: Keyed { public var k: Int; public init(k: Int) { self.k = k }; public var key: Int { k + 11 } }
public struct Delta: Tagged { public var d: Int; public init(d: Int) { self.d = d }; public var tag: Int { d * 3 }; public func describe() -> Int { d * 3 } }
open class Widget { public var width: Int = 1; public init() {}; open var span: Int { width } }
@inline(never) public func use(_ t: Tagged, _ k: Keyed) -> Int { t.tag + t.describe() + k.key }
let n = CommandLine.arguments.count
let w = Widget()
print(use(Alpha(n: n), Gamma(k: n)), use(Beta(m: n), Gamma(k: 1)), use(Delta(d: n), Gamma(k: 2)), w.span)

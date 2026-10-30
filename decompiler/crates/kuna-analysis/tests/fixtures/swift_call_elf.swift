// Fixture for the Swift calling-convention arm (`swiftcall`): a class
// method taking two arguments, a throwing method, a mutating struct method
// and a non-mutating one, and protocol witnesses of a struct and a class,
// called from one function so the call sites carry `self` and the error.
public struct Counter {
    public var value: Int
    public var step: Int
    public init(value: Int, step: Int) { self.value = value; self.step = step }
    @inline(never) public mutating func advance(by n: Int, times t: Int) { value += step * n + t }
    @inline(never) public func peek(_ k: Int) -> Int { value * k + step }
}
public enum Oops: Error { case bad }
open class Account {
    public var balance: Int
    public var owner: String
    public init(balance: Int, owner: String) { self.balance = balance; self.owner = owner }
    @inline(never) open func deposit(_ amount: Int, fee: Int) -> Int { balance += amount - fee; return balance }
    @inline(never) open func withdraw(_ amount: Int) throws -> Int {
        if amount > balance { throw Oops.bad }
        balance -= amount; return balance
    }
}
public protocol Pingable { func ping(_ x: Int) -> Int; mutating func bump(_ k: Int) }
extension Counter: Pingable {
    public func ping(_ x: Int) -> Int { value &+ x }
    public mutating func bump(_ k: Int) { step &+= k }
}
extension Account: Pingable {
    public func ping(_ x: Int) -> Int { balance &+ x }
    public func bump(_ k: Int) { balance &+= k }
}
@inline(never) public func run(_ a: Account, _ c: inout Counter, _ n: Int) -> Int {
    c.advance(by: n, times: 3)
    let r = a.deposit(n, fee: 2)
    let w = (try? a.withdraw(n * 5)) ?? -1
    return r + w + c.peek(n)
}
@inline(never) public func pingAll(_ ps: [any Pingable]) -> Int { var s = 0; for p in ps { s &+= p.ping(3) }; return s }

let n = CommandLine.arguments.count
let acct = Account(balance: 100, owner: "x")
var ctr = Counter(value: n, step: 2)
print(run(acct, &ctr, n), ctr.value, acct.balance, pingAll([ctr, acct]))

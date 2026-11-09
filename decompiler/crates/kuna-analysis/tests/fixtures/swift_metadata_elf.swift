// Fixture for the Swift metadata pass (`swiftmetadata`): one module with a
// struct, a class hierarchy, an enum, a protocol with a requirement and a
// default, conformances to a local and a stdlib protocol, and an override.
public struct Point { public var x: Int; public var y: Int; public init(x: Int, y: Int) { self.x = x; self.y = y } }
public enum Shade { case light, dark(Int) }
public protocol Shape { func area() -> Int; var name: String { get } }
extension Shape { public var name: String { "shape" } }
open class Animal {
    public var legs: Int = 4
    public init() {}
    open func sound() -> String { "..." }
}
public final class Dog: Animal {
    public override func sound() -> String { "woof" }
}
public struct Square: Shape, Hashable {
    public var side: Int
    public init(side: Int) { self.side = side }
    public func area() -> Int { side * side }
}
public func describe(_ s: Square) -> String { s.name + " " + String(s.area()) }

let d: Animal = Dog()
let sq = Square(side: Int(CommandLine.arguments.count))
print(describe(sq), d.sound(), sq.hashValue, Point(x: 1, y: 2).x, Shade.dark(3))

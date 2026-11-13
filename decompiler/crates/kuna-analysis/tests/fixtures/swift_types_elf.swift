// Fixture for the Swift type pass (`swifttypes`): fixed-layout structs with
// mixed field types (integers, a double, a bool, a string, a nested struct,
// a class reference, an optional class), a class hierarchy whose stored
// properties are read through `self` in vtable getters, and a struct read
// through `self` in a protocol witness.
public struct Point { public var x: Int; public var y: Int; public init(x: Int, y: Int) { self.x = x; self.y = y } }
public protocol Shape { func area() -> Int; var name: String { get } }
extension Shape { public var name: String { "shape" } }
open class Animal {
    public var legs: Int = 4
    public var name: String = "animal"
    public init() {}
    open func sound() -> String { "..." }
}
public final class Dog: Animal {
    public var tricks: Int = 0
    public override func sound() -> String { "woof" }
}
public struct Pixel: Shape {
    public var at: Point
    public var alpha: Double
    public var on: Bool
    public var tag: UInt8
    public var label: String
    public var owner: Animal
    public var shadow: Dog?
    public init(at: Point, alpha: Double, on: Bool, tag: UInt8, label: String, owner: Animal, shadow: Dog?) {
        self.at = at; self.alpha = alpha; self.on = on; self.tag = tag; self.label = label; self.owner = owner; self.shadow = shadow
    }
    public func area() -> Int { at.x * at.y + Int(tag) }
}
open class Sprite {
    public var origin: Point
    public var pixel: Pixel
    public var count: Int
    public var mate: Sprite?
    public init(origin: Point, pixel: Pixel) { self.origin = origin; self.pixel = pixel; self.count = 1; self.mate = nil }
    open func total() -> Int { count + origin.x + pixel.area() }
}
@inline(never) public func measure(_ s: Shape) -> Int { s.area() + s.name.utf8.count }
@inline(never) public func centerX(_ s: Sprite) -> Int { s.origin.x + s.count }

let n = CommandLine.arguments.count
let dog = Dog()
let px = Pixel(at: Point(x: n, y: 2), alpha: 0.5, on: n > 1, tag: 7, label: "px", owner: dog, shadow: n > 2 ? dog : nil)
let sp = Sprite(origin: Point(x: 3, y: n), pixel: px)
sp.mate = n > 3 ? sp : nil
print(measure(px), sp.total(), centerX(sp), dog.sound(), dog.tricks, sp.pixel.owner.legs, sp.origin.y)

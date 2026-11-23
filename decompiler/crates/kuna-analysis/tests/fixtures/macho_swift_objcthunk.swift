import Foundation

class Replay: NSObject {
    @objc var scrollView: NSObject? = nil
    @objc var count: Int = 0
    @objc func addFrame(timestamp: Double, label: NSObject?) -> Int {
        return Int(timestamp) + (label == nil ? 1 : 2) + count
    }
    @objc class func make() -> Replay { return Replay() }
    func native(_ x: Int) -> Int { return x * 3 + count }
}

class Sub: Replay {
    @objc override func addFrame(timestamp: Double, label: NSObject?) -> Int { return 7 + count }
}

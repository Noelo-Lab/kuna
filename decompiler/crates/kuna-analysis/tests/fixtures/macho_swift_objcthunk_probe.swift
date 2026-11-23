import Foundation

private class Probe: NSObject {
    var hits: Int = 0
    @inline(never) func refresh() { hits += 1 }
    override func doesNotRecognizeSelector(_ aSelector: Selector!) {
        super.doesNotRecognizeSelector(aSelector)
        refresh()
    }
}

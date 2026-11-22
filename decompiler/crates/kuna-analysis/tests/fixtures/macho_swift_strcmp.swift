// Swift string equality against literals: `s == "lit"` tests the literal's
// two words (count and object) inline before it calls the runtime compare.
@inline(never)
func model(_ s: String) -> Int {
    if s == "iPod5,1" { return 1 }
    if s == "iPhone3,1" { return 2 }
    if s == "iPad2,1" { return 3 }
    return 0
}

print(model(CommandLine.arguments.last ?? ""))

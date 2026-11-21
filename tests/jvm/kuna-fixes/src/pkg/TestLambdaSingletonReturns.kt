package pkg

val clamp: (Int) -> Int = clamp@{ x ->
    if (x < 0) return@clamp 0
    if (x > 100) return@clamp 100
    x
}

val firstNegative: (IntArray) -> Int = { values ->
    var found = -1
    for (v in values) {
        if (v < 0) {
            found = v
            break
        }
    }
    found
}

val sign: (Int) -> String = { x ->
    if (x < 0) {
        println(x)
        "negative"
    } else if (x == 0) {
        "zero"
    } else {
        "positive"
    }
}

val search: (List<String>, String) -> Int = search@{ items, key ->
    for ((i, item) in items.withIndex()) {
        if (item == key) return@search i
    }
    -1
}

package pkg

fun pathKind(name: String): Int {
    when (name) {
        "." -> return 1
        "../" -> return 2
        "-x-" -> return 3
        "2nd" -> return 4
    }
    return 0
}

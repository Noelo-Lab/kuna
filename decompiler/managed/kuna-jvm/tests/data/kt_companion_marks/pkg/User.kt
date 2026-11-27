package pkg

class User {
    fun use(r: TestKtCompanionOverride): Int = r.next() + r.next(3) + r.size + r.label.length
}

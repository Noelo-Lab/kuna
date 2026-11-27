package pkg

class TestKtLegacyToTypedArray {
    var items: Array<String> = emptyArray()

    fun names(list: List<String>): Array<String> {
        return list.toTypedArray()
    }

    fun joined(list: List<String>): String {
        return join(*list.toTypedArray())
    }

    fun join(vararg parts: String): String {
        var res = ""
        for (p in parts) {
            res += p
        }
        return res
    }

    fun keep(list: Collection<String>) {
        items = list.toTypedArray()
    }

    fun objects(list: List<String>): Array<Any> {
        return list.toTypedArray<Any>()
    }

    class Modern {
        fun names(list: List<String>): Array<String> {
            return list.toTypedArray()
        }

        fun objects(list: List<String>): Int {
            return list.toTypedArray<Any>().size
        }
    }

    fun count(set: Set<Int>): Int {
        val array = set.toTypedArray()
        return array.size + array[0]
    }
}

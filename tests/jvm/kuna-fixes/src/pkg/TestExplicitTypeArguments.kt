package pkg

class TestExplicitTypeArguments {
    class Item(private val label: String) {
        fun label(): String = label
    }

    object Seqs {
        @JvmStatic fun <T> lastOf(items: List<T>): T = items[items.size - 1]
        @JvmStatic fun <T> firstOf(items: Array<T>): T = items[0]
    }

    fun lastLabel(items: List<Item>): String = Seqs.lastOf(items).label()

    fun firstLabel(items: Array<Item>): String = Seqs.firstOf(items).label()
}

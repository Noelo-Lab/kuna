package pkg

import pkg.pick as pickFrom

class TestKtExtensionMemberType {
    abstract class Dice {
        abstract fun roll(until: Int): Int

        abstract fun mark(label: String): Int

        abstract fun pick(item: Any): Int

        abstract fun scale(factor: Long): Long
    }

    fun rollIn(dice: Dice, range: IntRange): Int = dice.roll(range)

    fun markCount(dice: Dice, count: Int): Int = dice.mark(count)

    fun pickFirst(dice: Dice, range: IntRange): Int = dice.pickFrom(range)

    fun scaled(dice: Dice, step: Int): Long = dice.scale(step)
}

fun TestKtExtensionMemberType.Dice.roll(range: IntRange): Int = range.first + roll(range.last - range.first + 1)

fun TestKtExtensionMemberType.Dice.mark(count: Int): Int = mark("#$count")

fun TestKtExtensionMemberType.Dice.pick(range: IntRange): Int = range.first

fun TestKtExtensionMemberType.Dice.scale(step: Int): Long = scale(step.toLong() * 2)

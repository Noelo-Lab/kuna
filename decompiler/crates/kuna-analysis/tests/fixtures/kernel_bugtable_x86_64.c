// SPDX-License-Identifier: GPL-2.0
// The source of kernel_bugtable_x86_64.ko. Build against a kernel tree with
// `obj-m += kernel_bugtable_x86_64.o` in a Makefile beside it:
//   make -C /lib/modules/$(uname -r)/build M=$PWD modules
//   strip --strip-debug -o kernel_bugtable_x86_64.ko kernel_bugtable_x86_64.ko
// The fixture was built as kwarn_bugsite.c, which is the file name its
// __bug_table entries carry.
#include <linux/module.h>
#include <linux/bug.h>

int kwarn_counter;

noinline int kwarn_check(int value)
{
	WARN_ON(value < 0);
	kwarn_counter += value;
	BUG_ON(value > 100);
	return kwarn_counter;
}
EXPORT_SYMBOL(kwarn_check);

static int __init kwarn_init(void)
{
	return kwarn_check(1) > 0 ? 0 : -EINVAL;
}

static void __exit kwarn_exit(void)
{
}

module_init(kwarn_init);
module_exit(kwarn_exit);
MODULE_LICENSE("GPL");

/* Separate pointer stores populate one 16-byte range. The empty assembly
   barrier preserves a pointer-slot merge for the emitted-member regression. */
#include <stdbool.h>
#include <stdint.h>

#ifndef KUNA_EMITTED_TREE_TYPES
typedef struct Node {
  struct Node *left;
  struct Node *parent;
  struct Node *right;
  unsigned char color;
  bool sentinel;
  unsigned char padding[2];
  int key;
  int tag;
} Node;
typedef struct Tree { Node *head; uint64_t size; } Tree;
typedef struct Range { Node *first; Node *last; } Range;
#endif

static Range observed_range;

__attribute__((ms_abi, noinline)) void consume_range(Tree *tree, const Range *range) {
  observed_range = *range;
  tree->size = range->first->key + range->last->key;
}

#ifndef KUNA_EMITTED_TREE_BODY
__attribute__((ms_abi, noinline)) void tree_range(Tree *tree, int key, int tag) {
  Node *head = tree->head;
  Node *first = head;
  Node *last = head;
  Node *cursor = head->parent;
  while (!cursor->sentinel) {
    if (cursor->key < key || (cursor->key == key && cursor->tag < tag)) {
      cursor = cursor->right;
    } else {
      if (last->sentinel && (cursor->key > key || cursor->tag > tag)) last = cursor;
      first = cursor;
      cursor = cursor->left;
    }
  }
  Node **link = last->sentinel ? &head->parent : &last->left;
  __asm__ volatile("" : "+r"(link));
  cursor = *link;
  while (!cursor->sentinel) {
    if (key < cursor->key || (key == cursor->key && tag < cursor->tag)) {
      last = cursor;
      cursor = cursor->left;
    } else {
      cursor = cursor->right;
    }
  }
  Range range = {first, last};
  consume_range(tree, &range);
}
#else
__attribute__((ms_abi)) void tree_range(Tree *tree, int key, int tag);
#endif

int main(void) {
  Node nil = {.sentinel = true, .key = 0};
  Node right = {.left = &nil, .right = &nil, .key = 30, .tag = 9};
  Node left = {.left = &nil, .right = &nil, .key = 10, .tag = 5};
  Node root = {.left = &left, .right = &right, .key = 20, .tag = -4};
  Node *ordered[] = {&left, &root, &right};
  Tree tree = {&nil, 0};
  int keys[] = {0, 10, 15, 20, 25, 30, 40};
  int tags[] = {-5, -4, 0, 5, 9, 10};
  for (int empty = 0; empty < 2; ++empty) {
    nil.parent = empty ? &nil : &root;
    for (unsigned i = 0; i < sizeof(keys)/sizeof(keys[0]); ++i) {
      for (unsigned j = 0; j < sizeof(tags)/sizeof(tags[0]); ++j) {
        Range expected = {&nil, &nil};
        if (!empty) {
          for (unsigned n = 0; n < 3; ++n) {
            Node *node = ordered[n];
            if (expected.first == &nil &&
                (node->key > keys[i] || (node->key == keys[i] && node->tag >= tags[j])))
              expected.first = node;
            if (expected.last == &nil &&
                (node->key > keys[i] || (node->key == keys[i] && node->tag > tags[j])))
              expected.last = node;
          }
        }
        observed_range = (Range){0};
        tree_range(&tree, keys[i], tags[j]);
        if (observed_range.first != expected.first || observed_range.last != expected.last)
          return 1;
      }
    }
  }
  return 0;
}

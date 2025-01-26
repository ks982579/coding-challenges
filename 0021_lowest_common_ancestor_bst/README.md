# Lowest Common Ancestor of a Binary Search Tree

Can be found on LeetCode.

Suppose we are given a binary search tree (BST).
The goal is to find the lowest common ancestor (LCA) node between any two given nodes in the tree.
You can search Wikipedia for the definition of LCA.
Note that the lowest common ancestor between two nodes can be one of the nodes itself.

Examples help:

```txt
  2
 /\
3  4
```

If the nodes were `p=3` and `q=4`, their common ancestor is `a=2`.
And if the nodes were `p=3` and `q=2`, their common ancestor is still `a=2`.

I think the major constraints are that the descendant nodes must exist,
and you cannot ask for the ancestor of the same 2 descendant nodes.
I suppose the answer would be itself.

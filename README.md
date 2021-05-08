# i3-pretty-tree

Prints out the tree of the [i3](https://i3wm.org/) containers of the current workspace in [dot](https://www.graphviz.org/) format.

Example output:

```dot
digraph g {
n94027044503440 [ label = "WORKSPACE: 5" ];
n94027044503440 -> n94027044013552;
n94027044013552 [ label = "CONTAINER: SplitH" ];
n94027044013552 -> n94027044558032;
n94027044013552 -> n94027044047920;
n94027044047920 [ label = "CONTAINER: SplitV" ];
n94027044047920 -> n94027044338176;
n94027044047920 -> n94027043894912;
n94027043894912 [ label = "shahn : ...Konsole" ];
n94027044338176 [ label = "shahn : ...Konsole" ];
n94027044558032 [ label = "Mozilla Firefox" ];
}
```

Rendered:

![dot graph](example.png)

## Motivation

`i3` puts windows in a tree structure.
When I started using it, I found it difficult to wrap my head around
that though:
which window was in which container,
which container was in which parent container,
which keyboard shortcuts to use to navigate that tree
or move nodes around in it.
So I wrote `i3-pretty-tree` to help with that.
I added [this keyboard shortcut](https://github.com/soenkehahn/dotfiles/blob/c11e56cfe3b4c1cace8320540aea1e9c0c96e576/.config/i3/config#L61) to be able to quickly bring up the current window structure on screen and look at it.
That helped me get used to `i3`'s tree structure.
(Now that I'm used to it, I hardly use `i3-pretty-tree` anymore.)

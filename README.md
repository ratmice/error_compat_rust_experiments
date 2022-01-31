This is a repository containing experiments on the usage of nightly/unstable rust
implementing various traits on a c style enum error where enum Error { NoError = 0, ... }

The primary questions this experiment seeks to answer is:

1. Can we keep the error ABI compatible with the existing C ABI,
While still writing rust code which is ergonomic in the way that rust users expect, and if not exactly
how close can we get?
2. If we choose to use a enum which is ABI compatible/using stable, in the future if the necessary requirements are stablized will we be able to implement the kind of error handling rust users expect?


Reliance is on:
* no_std: [try_trait_v2](https://rust-lang.github.io/rfcs/3058-try-trait-v2.html)
* std:    [termination_trait_lib](https://doc.rust-lang.org/beta/unstable-book/library-features/termination-trait-lib.html)

It also uses a number of compilation time procedural macros, on the various enums, in some cases these provide more generated code than is needed.

I have not yet looked at the underlying codegen to understand exactly the toll or overhead of doing it this way requires.

# aoc23-rust [![.github/workflows/ci.yml](https://github.com/n8henrie/aoc23-rust/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/n8henrie/aoc23-rust/actions/workflows/ci.yml)

I had a blast doing [Advent of Code 2018 in Rust][0], though I barely finished
in time for AoC 2019. I was only able to complete a handfull of [AoC
2021](https://github.com/n8henrie/aoc21-rust), and I didn't get terribly
far on [2022](https://github.com/n8henrie/aoc22-rust) either.

Like 2022, for 2023 I'm going to try not to spend *too* much time on AoC, but
I'll go through a few of them at least.

1. Have fun.
1. Look for opportunities to practice my many weak spots and new features I've
   been hoping to try out:
    - Generics
    - Const generics
    - Declarative macros (probably not procedural)
    - Parallelism / Rayon
    - Documenting my crates
    - Workspaces
    - Async? Doubt there is much opportunity in AoC
1. Continue exploring the incredibly addicting time-sink of [nix +
   rust](https://n8henrie.com/tags/#nix-ref)
1. Try to finally sort out a reasonably time-efficient, performant, and safe
way to tackle the numerous problems that are best solved with trees and linked
lists, as these are notoriously difficult to do in Rust due to the memory
model (arenas vs reference counting vs ??)

Anything I've cared to document is at <https://n8henrie.com/aoc23-rust/aoc/>.

I may also try a few problems in Go, Swift, or maybe something else entirely.
Time will tell.

## Other AoC '23 in Rust repos:

#TODO

You might also consider exploring a few using the GitHub API:

```console
$ gh api \
    -X GET search/repositories \
    --paginate \
    -f q='language:rust "advent of code"' \
    -f sort=stars \
    --jq '.items[] | .html_url'

https://github.com/warycat/rustgym
https://github.com/BurntSushi/advent-of-code
https://github.com/fspoettel/advent-of-code-rust
https://github.com/timvisee/advent-of-code-2021
https://github.com/scarvalhojr/aoc-cli
https://github.com/timvisee/advent-of-code-2020
https://github.com/simonw/advent-of-code-2022-in-rust
https://github.com/timvisee/advent-of-code-2022
https://github.com/aldanor/aoc-2021
https://github.com/AxlLind/AdventOfCode2021
...
```

[0]: https://github.com/n8henrie/advent2018-rust
[1]: https://github.com/n8henrie/aoc21-rust

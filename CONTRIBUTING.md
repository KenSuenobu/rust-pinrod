# Contributing to rust-pushrod

First of all, **thank you for helping on rust-pushrod!**

Here are a few guidelines.

## Bugs

If you find a bug, and you can make a screenshot, please take one!  If it's a bug with
the library itself, please run the library with `RUST_BACKTRACE=1`, and get a copy of
the stack trace.

Once you have enough information on how to recreate the bug, please file an issue with
the project [here](https://github.com/KenSuenobu/rust-pushrod/issues).  Make sure to apply
a label of "bug" to the project, and identify the version that was affected by selecting
the project milestone version.

## Ideas

If you have ideas on how to make the library better, have an idea for a better `Widget`,
or just see something in the core library that can be improved, please let us know!

[Contribute your issue here](https://github.com/KenSuenobu/rust-pushrod/issues) and
provide a usage pattern if possible.  If you have a design for the `Widget`, please
take a picture (or a sketch) of what you'd like the `Widget` to look like.  Please keep
in mind, the `Widget` should be simple.

Please make sure to label as many labels as you can so that it helps categorize the work
that needs to be done.  Any new ideas should be in the "Backlog" milestone, as the
developers will determine which milestone to include the change to.

## What will get your pull request rejected

Just running a `cargo format` or `cargo clippy`.  While I understand your intentions are
in the right place, this isn't enough to constitute a valid pull request, as this is
part of the rules in the next section.

Undocumented code will get a swift rejection.  Please comment your code so that others
can understand what you're doing: especially if it's complicated!

Any `unsafe { }` blocks unless there is a **damned** good reason as to why it's being
used.

## Pull requests

Great!  Thank you for your contribution, you magnificent developer, you!

* Please make sure to refer to the [issue](https://github.com/KenSuenobu/rust-pushrod/issues) that this PR references.
* Please format your code using `cargo fmt`
* Make sure your tests pass using `cargo test`
* Make sure you **document your code**; this is extremely important.  Use `sh scripts/docs.sh` to build top-level docs.
* Only use features in the latest Rust stable (2018) branch.
* Do not make the pull request include a huge number of changes - keep it simple, short, and sweet.


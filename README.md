# Learn Rust, one exercise at a time

You've heard about Rust, but you never had the chance to try it out?\
This course is for you!

You'll learn Rust by solving 100 exercises.\
You'll go from knowing nothing about Rust to being able to start
writing your own programs, one exercise at a time.

> [!NOTE]
> This course has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting started

Go to [rust-exercises.com](https://rust-exercises.com) and follow the instructions there
to get started with the course.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how
  you installed Rust on your system)
  to make sure you're running on the latest stable version.
- _(Optional but recommended)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with
    the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Repo-local git identity

If you're working across multiple machines, you can set your git identity for this
repository only:

```bash
./scripts/setup-git-identity.sh
```

You can also run it non-interactively:

```bash
GIT_USER_NAME="Your Name" GIT_USER_EMAIL="you@example.com" ./scripts/setup-git-identity.sh
```

## Solutions

You can find the solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions) of this repository.

# License

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).

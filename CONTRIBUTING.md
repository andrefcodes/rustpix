# Contributing to rustpix

First off, thank you for considering contributing to Rustpix!

## Development Workflow

### Getting Started

1. Clone the repository:
   ```
   git clone https://github.com/andrefcodes/rustpix
   cd rustpix
   ```

2. Install dependencies:
   - Rust toolchain (via [rustup](https://rustup.rs/))
   - System dependencies: `pkg-config` and `libheif-dev` >= 1.17 (for HEIF support)

### Making Changes

1. Create a new branch for your feature or bugfix:
   ```
   git checkout -b my-new-feature
   ```

2. Write your code, following the project's code style.
   - Keep your changes focused and related to a single issue.
   - Add tests for new functionality.
   - Update documentation as needed.

3. Commit your changes with meaningful commit messages:
   ```
   git commit -am "Add new feature: description of what changed"
   ```

   Format your commit messages with a short (50 chars or less) summary line,
   followed by a blank line and a more thorough description if needed.

## Submitting Changes

### Via Pull Request (Preferred Method)

1. Fork the repository on GitHub

2. Push your changes to your fork:
   ```
   git push origin my-new-feature
   ```

3. Open a pull request on [GitHub](https://github.com/andrefcodes/rustpix/pulls)

## Code Guidelines

- Follow Rust best practices and idioms.
- Use the `rustfmt` tool before submitting (configured in the repo).
- Run `cargo clippy` to catch common mistakes
- Add comments for complex logic.
- Write tests for new functionality
- Update documentation for public APIs.
- Keep the codebase modular as per the current structure.

## Reporting Bugs

When reporting a bug, please include:

- A clear and descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your operating system and terminal information
- Any relevant logs or error messages

## Feature Requests

Feature requests are welcome! Please:

1. Open an issue on [GitHub](https://github.com/andrefcodes/rustpix/issues) with the title "Feature Request: [Your Feature]"
2. Describe the feature and why it would be valuable
3. If possible, provide examples of how it might work

All feature requests are welcome and will be considered based on how they align with the project's goals.

## License

By contributing, you agree that your contributions will be licensed under the project's [GNU Affero General Public License v3.0](LICENSE).

### Copyright Headers

All new source files must include the AGPL-3.0 copyright header:

```rust
// Copyright (C) 2023-2024 [Your Name] <your.email@example.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Questions?

Feel free to open an issue on [GitHub](https://github.com/andrefcodes/rustpix/issues) if you have any questions or need assistance with the contribution process.
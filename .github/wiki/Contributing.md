# 📗 Contribution Guidelines

First off, thank you for your interest in contributing to cord.nvim! We welcome contributions from everyone. This document provides guidelines for contributing to the project.

## How Can I Contribute?

### Reporting Bugs

- Before submitting a bug report, make sure to go through the [**🔧 Troubleshooting**](./Troubleshooting.md) section of the wiki, and check for [existing issues](https://github.com/vyfor/cord.nvim/issues) to see if the problem has already been reported.
- If you're unable to find an open issue addressing the problem, [open a new one](https://github.com/vyfor/cord.nvim/issues/new). Be sure to include a clear title and description, along with as much relevant information as possible.

### Suggesting Enhancements

- Open a new issue with your suggestion, providing as much detail as possible.
- Explain why this enhancement would be useful and whether it could introduce breaking changes to the codebase.

### Pull Requests

1. Fork the repo and create your branch from `master`.
2. Add your changes and give it a proper testing.
3. Make sure your code is formatted and lints.
4. Issue that pull request!

## Styleguides

### Git Commit Messages

We use the [Conventional Commits](https://www.conventionalcommits.org/) specification for our commit messages. This leads to more readable messages that are easy to follow when looking through the project history. Please adhere to this convention for your commit messages.

The commit message should be structured as follows:
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

For breaking changes, add a `!` after the type/scope. Make sure to include the details in the commit body.

Examples:

`fix: correctly parse Git config`

`feat(icons): provide an icon for Rust`

`docs(readme): fix typo`

`refactor!: remove config.timer.enable`

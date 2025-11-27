Thank you for considering contributing to this project! This document explains how to report issues, propose changes, and submit pull requests so your contribution can be reviewed and merged quickly.

**Code of Conduct**: Be respectful and constructive in all interactions. If this repository doesn't include a `CODE_OF_CONDUCT.md`, follow the Contributor Covenant principles and treat others with courtesy.

How to contribute

- **Report bugs**: Open an issue titled "bug: short description" and include steps to reproduce, expected vs actual behavior, and environment details.
- **Request features**: Open an issue titled "feat: short description" describing the motivation, proposed behavior, and any UI/UX notes or examples.
- **Discuss**: Use issues for design questions or to propose a larger change before opening a PR.

Branching & Pull Requests

- Fork the repository and create a branch from `main` named using the pattern `feat/<short-desc>` or `fix/<short-desc>`.
- Keep changes focused and small; one logical change per PR.
- Add tests for new behavior (unit/integration) and update documentation if needed.
- Run linters/formatters and ensure the project builds locally before opening a PR.
- Open a pull request against `main` with a clear title and description. Reference related issues (e.g., "fixes #12").

Commit messages

- Use concise, descriptive commit messages. Prefer the Conventional Commits style, for example:
  - `feat(parser): add support for X`
  - `fix(api): handle missing field`
  - `chore(deps): bump dependency`

Tests, CI, and Quality

- If the project includes a test suite, run tests locally and ensure they pass before submitting a PR.
- Add tests for bug fixes and new features.
- Continuous integration will run on PRs; ensure all checks pass before requesting a merge.

Review process

- Maintainers will review PRs and may request changes or clarifications.
- Respond to review comments promptly. If requested, update your branch and push new commits.
- Maintainers may squash, rebase, or edit commits when merging.

Security

- For security vulnerabilities, do not open a public issue. Instead, contact the repository owner directly or use the recommended private disclosure method. If none is provided, label details as sensitive when contacting `ant-vu` off-band.

License

- By contributing, you agree that your contributions will be licensed under the same license as the project (`LICENSE.md`).

Thank you

- We appreciate contributions large and small. If you'd like help getting started, open an issue and we'll point you to a good first task.

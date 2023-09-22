# Contributing to try-runtime-cli

Thanks for taking the time to contribute! ❤️

## Versioning and Releases
Versioning for try-runtime-cli presents unique challenges due to its dependence on runtimes. Our approach to versioning is a mix of traditional semantic versioning and custom practices tailored to supporting old polkadot releases.

### Semantic Versioning for Master
The latest code on the master branch is maintained to be compatible with polkadot master, and follows a standard Semantic Versioning (SemVer) approach. This means:

- Major releases (X.0.0) include breaking changes.
- Minor releases (0.X.0) add new features without breaking existing functionality.
- Patch releases (0.0.X) include fixes and minor improvements.

It's important to note that while we try to guarantee master compatibility with the latest polkadot versions, there's no assurance that older versions will work with any specific runtime and should be considered deprecated.

### Tags for Polkadot SDK Releases
In addition to SemVer on master, we maintain Git tags or branches corresponding to every Polkadot SDK release >=v1.0.0. This ensures that:

- We provide versions of `try-runtime` that are guaranteed to be compatible with specific Polkadot SDK releases.
- In the case of patches or minor changes, we can move or adjust these tags or branches, ensuring continued compatibility with their counterparts in the Substrate repo.
- While this method requires manual maintenance, it offers clarity and ensures compatibility for users working with different runtime versions.

### What to Expect as a Contributor
When contributing, consider the following:

- If your changes are general improvements or fixes and don't tie specifically to a Polkadot SDK release, they will be integrated into the main codebase and subject to semantic versioning.
- If your contribution pertains to compatibility with a specific Polkadot SDK release, it may be integrated into the corresponding tagged branch or version.
- Always indicate in your pull request or issue any specific version considerations or compatibility issues you're aware of.

## Tips

Non-trivial contributions are eligible for DOT tips. To express interest include your Polkadot address in your PR description.

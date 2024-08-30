# Dev README

## Update dependencies
To bump dependencies, run:

```
cargo update
```

## Bump version

Bump version in Cargo.toml and update references to the current version in all documentation.

## Tag
To create a new tag, run:

```
git tag -a v0.3.1 -m "xxx" 
```

To push the tag and trigger build and release on Github, run:

```
git push origin v0.3.1
```

## Update crate on crates.io

To dry run the publishing, run:

```
cargo publish --dry-run
```
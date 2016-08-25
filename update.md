# Updating precedure

Since I keep forgetting to update version numbers and URLs here's the things required before `cargo publish`:

* Update libc dependency with `cargo update`.

* Commit, test and push all the code and let it simmer for a few days.

* Push updated docs to `github.io`.

* Update version and docs link in `Cargo.toml` and `readme.md`.

* Create update commit and tag it with its version number.

* Push to github and finally publish on crates.

# Community repos

## Setup Debian bookworm to use these packages

```sh
# copy the community-repos.asc key to /etc/apt/keyrings/community-repos.asc

echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/community-repos.asc] https://pub-d8a49fc4a4cf482a8e6dc323cde06026.r2.dev bookworm main" | sudo tee /etc/apt/sources.list.d/community-repos.list > /dev/null
```

## Developer setup

```sh
R2_ACCESS_KEY_ID=redacted R2_SECRET_ACCESS_KEY=redacted R2_ENDPOINT=redacted cat rclone.conf.template | envsubst > ~/.config/rclone/rclone.conf
```

```sh
$ ./scripts/mount

# If you are bootstrapping a new repo, copy the conf directory from the root of this
# repository into debpkgs.
#
# You can also just `mkdir debpkgs` rather than mounting an external one, which is
# useful for testing.

$ reprepro -S unknown -b debpkgs includedeb bookworm path/to.deb

# Packages can be removed, which is often useful for testing.
#
# This should only be run against a local instance of the package repo, unless
# the intent is to actually remove the package from the online repo.
$ reprepro -b debpkgs remove bookworm $PACKAGE_NAME

# Run pkgr to package up all known packages.
cd pkgr
cargo run
cd ..

$ ./scripts/unmount
```


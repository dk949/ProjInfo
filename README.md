# ProjInfo

Language stats for your projects.

## Build and install

Requires the rust tool chain and make (doesn't actually require make, you can
just copy the files by hand).

```sh
make
make install
```

or

```sh
cargo build --release
cp target/release/projinfo $SOME_DIRECTORY_IN_PATH
cp src/res/langs.json $XDG_CONFIG_HOME
```

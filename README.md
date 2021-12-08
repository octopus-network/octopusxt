# octopusxt

Octopusxt used to call substrate node, include substrate-ibc. 
Now include astar runtime, will be support more runtime.

## Used build metadata file

fisrt you need compile metadata.scale file 

### Dowlodaing metada from a Substrate node

Use the suxt-cli tool to download the metadata for your target runtime from a node.

1. install:
```shell
cargo install subxt-cli
```
2. Save the encoded metadata to a file:
```shell
cd ./metadata_file

subxt metadata -f bytes --url http://localhost:8545 > metadata.scale
```

This default to querying the metadata of locally running node on the default `http://locahost:9933/`.
If querying a different node then the `metadata` command accepts a `--url` argument.

### Generating the runtime API from the downloaded metadata

Declare a module and decorate it with the subxt attribute which points at the downloaded metadata for the target 
runtime:

```shell
#[subxt::subxt(runtime_metadata_path = "metadata_file/metadata.scale")]
pub mod ibc_node_runtime { }
```

Important: `runtime_metadata_path` resolves to a path relative to the directory where your crate's `Cargo.tom;`
resides( CARGO_MANIFEST_DIR), not relative to the source file.

## Generate file

```shell
subxt codegen --url http://localhost:8545 | rustfmt --edition=2018 --emit=stdout > generate.rs
```

## Then you can `Cargo run`

```shell
cargo run
```




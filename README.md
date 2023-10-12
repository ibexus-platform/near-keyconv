# NEAR Key Converter

This is a CLI wrapper for the `near-seed-phrase` crate of [cornflower](https://crates.io/users/hanakannzashi). Additionally, you can generate a NEAR seed phrase and store the seed, secret key and public key in 1Password. You can also add the account id, which then gives you the ability to authorize NEAR CLI using 1Password:

```console
near send foo.near bar.near 1 --keyPath <(op read "op:://<VAULT NAME>/<KEY NAME>/Key Pair JSON")
```

## Usage

To be used as CLI tool, with integrated help using `clap`. To learn about all commands, use

```console
near-keyconv --help
```

## License

MIT

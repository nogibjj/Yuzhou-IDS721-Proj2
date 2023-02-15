# rust-new-project-template
1. Install rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Configure rustup nightly
```
rustup default nightly
```
3. use per-directory overrides to use the nightly version only for your Rocket project
```
rustup override set nightly
```


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-bert-translate](https://docs.rs/rust-bert/latest/rust_bert/index.html)
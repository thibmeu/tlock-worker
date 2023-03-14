# tlock-worker

Demonstration worker supporting timelock encryption. Deployed at `https://tlock_worker.crypto-team.workers.dev`.

## Usage

If you'd like to perform local encryption and decryption, you can install `dee`. Pre-build binaries are available on [GitHub](https://github.com/thibmeu/drand-rs/releases/tag/v0.0.4).

### Encryption

**Only supported for a fixed round number when using the worker.**

* local: `dee crypt -r 1000 original.png > encrypted.pem`
* remote: `curl -X POST --data-binary @original.png https://tlock_worker.crypto-team.workers.dev/encrypt/1000 > encrypted.pem`

### Decryption

* local: `dee crypt --decrypt encrypted.pem > decrypted.png`
* remote: `curl -X POST --data-binary @encrypted.pem https://tlock_worker.crypto-team.workers.dev/decrypt > decrypted.png`


## Development

This template starts you off with a `src/lib.rs` file, acting as an entrypoint for requests hitting your Worker. Feel free to add more code in this file, or create Rust modules anywhere else for this project to use.

With `wrangler`, you can build, test, and deploy your Worker with the following commands:

```sh
# compiles your project to WebAssembly and will warn of any issues
$ npm run build

# run your Worker in an ideal development workflow (with a local server, file watcher & more)
$ npm run dev

# deploy your Worker globally to the Cloudflare network (update your wrangler.toml file for configuration)
$ npm run deploy
```

Read the latest `worker` crate documentation here: https://docs.rs/worker

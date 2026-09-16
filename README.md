# <p align="center">Juncto</p>

Juncto is a set of Open Source projects which empower users to use and deploy
video conferencing platforms with state-of-the-art video quality and features.

<hr />

<p align="center">
<img src="https://raw.githubusercontent.com/juncto/juncto/master/readme-img1.png" width="900" />
</p>

<hr />

Amongst others here are the main features Juncto offers:

* Support for all current browsers
* HD audio and video
* Content sharing
* Raise hand and reactions
* Chat with private conversations
* Polls
* Virtual backgrounds

And many more!

## Using Juncto

Using Juncto is straightforward, as it's browser based. Head over to [meet.juncto.net](https://meet.juncto.net) and give it a try. It's scalable and free to use. All you need is a Google, Facebook or GitHub account in order to start a meeting. All browsers are supported!

## Running your own instance

This repository is a single Rust workspace: a Leptos (WASM) frontend backed by an Axum
server, with shared types in the `shared` crate. The previous React/Webpack implementation
has been removed.

Prerequisites: a Rust toolchain (with the `wasm32-unknown-unknown` target) and
`wasm-bindgen-cli`.

```sh
bash build.sh                     # builds the WASM frontend and copies static assets
cargo run --release -p backend    # serves the app on :3000
```

Rust unit tests and the single consolidated Playwright suite:

```sh
cargo test --workspace                     # unit tests
cd tests/e2e && npx playwright test        # end-to-end parity suite
```

## Juncto as a Service

If you like the branding capabilities of running your own instance but you'd like
to avoid dealing with the complexity of monitoring, scaling and updates, JunctoService might be
for you.

[Juncto Juncto as a Service (JunctoService)](https://jaas.Juncto.vc) is an enterprise-ready video meeting platform that allows developers, organizations and businesses to easily build and deploy video solutions. With Juncto as a Service we now give you all the power of Juncto running on our global platform so you can focus on building secure and branded video experiences.

## Documentation

All the Juncto documentation is available in [the handbook](https://juncto.github.io/handbook/).

## Security

For a comprehensive description of all Juncto's security aspects, please check [this link](https://juncto.org/security).

For a detailed description of Juncto's End-to-End Encryption (E2EE) implementation,
please check [this link](https://juncto.org/e2ee-whitepaper/).

For information on reporting security vulnerabilities in Juncto, see [SECURITY.md](./SECURITY.md).

## Contributing

If you are looking to contribute to Juncto, first of all, thank you! Please
see our [guidelines for contributing](CONTRIBUTING.md).

<br />
<br />

<footer>
<p align="center" style="font-size: smaller;">
Built with ❤️ by the Juncto team at <a href="https://Juncto.com" target="_blank">Juncto</a> and our community.
</p>
</footer>

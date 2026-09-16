# Follow Our Updated Guide to See How You Can Contribute

**Hello there! 👋**

We're thrilled that you're eager to contribute to **Juncto! ❤️** 

Your interest in improving our platform means a lot to us. To ensure your contributions align seamlessly with our goals and processes, we've recently updated our guide. This guide will provide you with clear instructions on how to get involved effectively.

### 📖 Get Started

Ready to get started? Head over to our [Juncto Handbook](https://juncto.github.io/handbook/docs/dev-guide/dev-guide-contributing/) and let's make **Juncto** even better together!

### 💬 Join the Discussion

Have questions or need help? Join our community discussions on the [Juncto Forum](https://community.juncto.org/) where contributors and maintainers can assist you.

### ❗️Additional Note
Before sending us your code, double-check that it meets our coding standards. This is
a Rust workspace, so verify your changes with the following commands from the repo root:

```sh
cargo fmt --all -- --check        # formatting
cargo clippy --workspace -- -D warnings   # linting
cargo test --workspace           # unit tests
cd tests/e2e && npx playwright test       # end-to-end tests (as applicable)
```

Once your code passes these checks, feel free to submit your pull request.

**Happy coding!**

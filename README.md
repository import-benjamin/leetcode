# leetcode

This repository is a collections of small challenges I've completed on leetcode in rust.
Each challenge is available as a Rust library in the `problems/` folder. The solution is a function alongside some tests.
You can run tests directly from a container using a solution like `podman`. To do so, use the following command:

```bash
podman run --rm --user root --tty --interactive --workdir /tmp/leetcode --volume $PWD:/tmp/leetcode:z docker.io/rust:1.90-alpine
```

This should provide an environment with the current working folder under the `/tmp/leetcode` path.
Once you get a prompt, you should be able to use `cargo` to run tests with the command below:

```bash
cargo test
```

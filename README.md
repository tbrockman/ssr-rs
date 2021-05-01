# 🚀  Rust server side rendering

[![Valerioageno](https://circleci.com/gh/Valerioageno/ssr-rs.svg?style=svg)](https://github.com/Valerioageno/ssr-rs)

The project aims to enable server side rendering on rust servers in the simplest and lightest way possible.

Actually it works with ReactJS. It isn't tested yet with other frontend frameworks.

The all logic is stored inside the `render_to_string()` function.

```rust
use ssr_rs::Ssr;

fn main() {
   let html = Ssr::render_to_string("./path/to/build.js", "entryPoint", "renderFunction", None);
    
    assert_eq!(html, "<!doctype html><html>...</html>".to_string());
}
```

There are included examples with the most famous server frameworks and a default frontend react app made using `npx create-react-app` and the typescript `--template` flag.


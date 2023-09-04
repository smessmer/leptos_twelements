leptos-twelements
-----------------

A UI component library for [Leptos](https://leptos.dev/), based on [Tailwind Elements](https://tailwind-elements.com/).

Installation (for projects using [cargo leptos](https://github.com/leptos-rs/cargo-leptos))
-----------------

1. Use the nightly rust compiler. This crate doesn't work with stable rust yet.

2. Add the following to the `Cargo.toml` of your Leptos project:

```toml
[dependencies]
leptos-twelements = "^0.1.0"

[build-dependencies]
leptos-twelements = "^0.1.0"

[features]
ssr = [
    # ... leptos probably already has some other entries here
    "leptos-twelements/ssr",
    # ...
]
```

3. Add the following to your `tailwind.config.js`:
```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: [
      ...
      "./target/.leptos-twelements/rs/**/*.rs",
      "./target/.leptos-twelements/js/**/*.js",
    ],
  },
  plugins: [require("./target/.leptos-twelements/plugin.cjs")]
}
```
Note that `plugin.cjs` and the `js` directory aren't there just yet,
but we'll add a build script generating them in the next step.

4. Add the following `build.rs` to your Leptos project to generate those files:

```rust
use std::path::Path;

fn main() {
    let target_dir = Path::new(&env!("CARGO_MANIFEST_DIR")).join("target");
    leptos_twelements::install_files_to(&target_dir);
}
```

5. Add a call to `.setup_twelements()` to the axum router setup:
```rust
    use leptos_twelements::AxumRouterExt;
    let app = Router::new()
        // ...
        .setup_twelements()
        // ...
```
This function call will add the necessary routes to your axum app to serve the JavaScript required by Tailwind Elements.

Note that this code, including the `use` statement, should be guarded by a `#[cfg(feature = "ssr")]` attribute.
The default leptos setup should already do this correctly.

6. Add `<TwElementsSetup />` to your `App` component:
```rust
use leptos_twelements::TwElementsSetup;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // ...
    view! {
        cx,
        // ...
        <TwElementsSetup />
        // ...
    }
}
```

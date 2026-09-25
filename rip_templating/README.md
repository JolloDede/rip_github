# rip_templating

A small Rust HTML templating library built around a proc-macro `html!` macro for composing HTML fragments and components without writing raw string concatenation.

## Features

- Declarative HTML generation with Rust-like syntax
- Component support via the `#[Component]` attribute
- Automatic attribute rendering for HTML and HTMX attributes
- Works in plain Rust and with `axum::response::Html`
- Lightweight, no runtime templating engine required

## Installation

Add this crate to your project:

```toml
[dependencies]
rip_templating = { version = "0.2.2" }
```

If you want Axum integration, enable the feature:

```toml
[dependencies]
rip_templating = { version = "0.2.2", features = ["axum"] }
```

## Usage

```rust
use rip_templating::html;

let page = html! {
    html {
        head {
            title { "Welcome" }
        }
        body {
            div {
                class: "container",
                p { "Hello from rip_templating!" }
            }
        }
    }
};

assert!(page.contains("<div class=\"container\">"));
```

### HTMX attributes

```rust
use rip_templating::html;

let button = html! {
    button {
        hx_get: "/items",
        hx_trigger: "click",
        "Load items"
    }
};

assert!(button.contains("hx-get=\"/items\""));
```

### Conditionals

Use Rust-like `if` and `else` blocks to render content conditionally:

```rust
use rip_templating::html;

let logged_in = true;

let page = html! {
    div {
        if logged_in {
            p { "Welcome back!" }
        } else {
            p { "Please sign in." }
        }
    }
};

assert!(page.contains("Welcome back!"));
```

### For loops

Use `for` loops to render repeated content from an iterable:

```rust
use rip_templating::html;

let items = vec!["One", "Two", "Three"];

let list = html! {
    ul {
        for item in items {
            li { {item} }
        }
    }
};

assert!(list.contains("<li>One</li>"));
assert!(list.contains("<li>Three</li>"));
```

### Components

```rust
use rip_templating::{Component, html};

#[Component]
fn card(children: Element) {
    html! {
        div {
            class: "card",
            {children}
        }
    }
}

let output = html! {
    card {
        p { "Hello" }
    }
};

assert!(output.contains("<div class=\"card\"><p>Hello</p></div>"));
```

## Axum support

With the `axum` feature enabled, the `html!` macro returns `axum::response::Html`:

```rust
use rip_templating::html;

async fn handler() -> axum::response::Html {
    html! {
        div { "Rendered by Axum" }
    }
}
```

## Running tests

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

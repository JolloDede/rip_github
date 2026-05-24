use templating::html;

#[cfg(feature = "axum")]
fn into_string(value: axum::response::Html<String>) -> String {
    value.0
}

#[cfg(not(feature = "axum"))]
fn into_string(value: String) -> String {
    value
}

#[test]
fn basic() {
    let test = into_string(html!("Hello World"));
    assert_eq!(test, "Hello World");
}

#[test]
fn basic_html() {
    let test = into_string(html! {
        div { "Test" }
    });
    assert_eq!(test, "<div>Test</div>");
}

#[test]
fn nested_html() {
    let test = into_string(html! {
        div { div { "Test" } }
    });
    assert_eq!(test, "<div><div>Test</div></div>");
}

#[test]
fn class_html() {
    let test = into_string(html! {
        div {
            class: "bg-red-600",
            "Test"
        }
    });
    assert_eq!(test, "<div class=\"bg-red-600\">Test</div>");
}

#[test]
fn htmx_tags() {
    let test = into_string(html! {
        div {
            hx_trigger: "click",
            "Test"
        }
    });
    assert_eq!(test, "<div hx-trigger=\"click\">Test</div>");
}

#[test]
fn special_elements() {
    let test = into_string(html! {
        hr {}
    });
    assert_eq!(test, "<hr>");

    let test = into_string(html! {
        img {
            src: "bla",
            alt: "bla"
        }
    });
    assert_eq!(test, "<img src=\"bla\" alt=\"bla\">");
}

use templating::html;

#[test]
fn basic() {
    let test = html!("Hello World");
    assert_eq!(test, "Hello World");
}

#[test]
fn basic_html() {
    let test = html! {
        div { "Test" }
    };
    assert_eq!(test, "<div>Test</div>");
}

#[test]
fn nested_html() {
    let test = html! {
        div { div { "Test" } }
    };
    assert_eq!(test, "<div><div>Test</div></div>");
}

#[test]
fn class_html() {
    let test = html! {
        div {
            class: "bg-red-600",
            "Test"
        }
    };
    assert_eq!(test, "<div class=\"bg-red-600\">Test</div>");
}

#[test]
fn htmx_tags() {
    let test = html! {
        div {
            hx_trigger: "click",
            "Test"
        }
    };
    assert_eq!(test, "<div hx-trigger=\"click\">Test</div>");
}

#[test]
fn special_elements() {
    let test = html! {
        hr {}
    };
    assert_eq!(test, "<hr>");

    let test = html! {
        img {
            src: "bla",
            alt: "bla"
        }
    };
    assert_eq!(test, "<img src=\"bla\" alt=\"bla\">");
}

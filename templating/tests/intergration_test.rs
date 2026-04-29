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

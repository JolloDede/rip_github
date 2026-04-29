use templating::rsx;

#[test]
fn basic() {
    let test = rsx!("Hello World");
    assert_eq!(test, "Hello World");
}

#[test]
fn basic_html() {
    let test = rsx! {
        div { "Test" }
    };
    assert_eq!(test, "div { \"Test\" }");
}

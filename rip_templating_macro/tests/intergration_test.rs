use rip_templating_macro::{Component, html};

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

#[Component]
fn html_comp(children: Element) {
    html! {
       html {
          {children}
       }
    }
}

#[test]
fn component_call() {
    let test = html! {
        html_comp {
           p {
               "test"
           }
        }
    };

    assert_eq!(test, "<html><p>test</p></html>");
}

#[test]
fn conditionals() {
    let test_true = html! {
        p {
            {
                if true {
                    "test"
                }else{""}
            }
        }
    };

    let test_false = html! {
        p {
            {
                if false{
                    "test"
                }else{""}
            }
        }
    };

    assert_eq!(test_true, "<p>test</p>");
    assert_eq!(test_false, "<p></p>");
}

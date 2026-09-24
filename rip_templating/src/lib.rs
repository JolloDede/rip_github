pub use rip_templating_macro::{Component, component_html as __component_html, html as __html};

#[cfg(feature = "axum")]
#[macro_export]
macro_rules! html {
    ($($tokens:tt)*) => {
        ::axum::response::Html($crate::__html!($($tokens)*))
    };
}

#[cfg(not(feature = "axum"))]
#[macro_export]
macro_rules! html {
    ($($tokens:tt)*) => {
        $crate::__html!($($tokens)*)
    };
}

pub const DOCTYPE: &str = "<!DOCTYPE html>";

#[test]
fn doctype() {
    let test = html! {
        {DOCTYPE}
    };

    assert_eq!(test, "<!DOCTYPE html>");
}

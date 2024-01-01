mod app;

use app::App;
use leptos::*;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|| view! { <App /> });
}

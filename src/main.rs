use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="bg-blue-500 text-white p-4">
            "Hello, Tailwind in Leptos!"
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn app() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click={move |_| {
                set_count.update(|n| *n += 10);
            }}
            style="position: relative"
            style:left=move || format!("{}px", count() + 10)
            style:background-color=move || format!("rgb({}, {}, 205)", count(), 90)
            style:max-width="400px"
            style=("--columns", count)
            class:red= move || count() % 2 == 0 
        >
            "Click me: "
            {move || count()}
        </button>
    }
}
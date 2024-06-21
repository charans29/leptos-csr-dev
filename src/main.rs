use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
fn app() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let _html = "<p>This is inner HTML injected.</p>";

    view! {
        <div style="text-align: center" inner_html=_html />
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center">
            <progress
                max="300"
                value=move || count() / 2
            />
            <button
                on:click={move |_| {
                    set_count.update(|n| *n += 10);
                }}          
                style:background-color=move || format!("rgb({}, {}, 205)", count(), 90)
                class:red= move || count() % 2 == 0 
            >
                "Click me: "
                {move || count()}
            </button>
        </div>
    }
}
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App)
}

#[component]
fn app() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let html = "<p>This is inner HTML injected.</p>";
    let double_count = move || count() * 2;
    let values = vec![0, 1, 2];
    
    view! {
        <div style="text-align: center" inner_html=html />
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center">
            <ProgressBar max=10 progress=count />
            <ProgressBar progress=count />
            <ProgressBar max=110 progress=count />
            <ProgressBar max=300 progress=Signal::derive(double_count)/>
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
            <br/>
            <p>{values.clone()}</p>
            <ul style="margin-left: -25px">
                {values.into_iter()
                    .map(|n| view! {<li>{n}</li>})
                    .collect_view()
                }
            </ul>
            <br/>
            <DynamicButtons />
        </div>
    }
}

#[component]
fn progress_bar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max={max}
            value=progress
        />
        <br/>
    }
}

#[component]
fn dynamic_buttons() -> impl IntoView {
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}
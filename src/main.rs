use leptos::{ev::{self, SubmitEvent}, html::{self, button, div, span}, prelude::*};
use gloo_timers::future::TimeoutFuture;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let values = vec![0,1,2];
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();
    let (count, set_count) = signal(0);

    let double_count = move || count.get() * 2;
    let (name, set_name) = signal("Controlled".to_string());
    let input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;
    let message = move || is_odd().then(|| "Ding ding ding!");
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);
    let (ok_value, set_ok_value) = signal(Ok(0));

    let (num, set_num) = signal(0);

    // this will reload every time `name` changes
    let async_data = LocalResource::new(move || important_api_call(name.get()));


    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        class:red=move || count.get() % 2 == 1
        class=("button-20", move || count.get() % 3 == 1)
        // class=(["button-20", "rounded"], move || count.get() % 3 == 1)
        style="position: absolute"
        // and toggle individual CSS properties with `style:`
        style:left=move || format!("{}px", count.get() + 100)
        style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
        style:max-width="400px"
        // Set a CSS variable for stylesheet use
        style=("--columns", move || count.get().to_string())
        >
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            {move || count.get() * 1}
        </p>
        // <ProgressBar progress=double_count/> // only nightly
        // non nightly
        <ProgressBar progress=Signal::derive(double_count)/>
        <ProgressBar progress=count/>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
                //.collect_view()} // leptos helper method
        </ul>
        <ul>{counter_buttons}</ul>
        <input type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                // .value() returns the current value of an HTML input element
                set_name.set(ev.target().value());
            }

            // the `prop:` syntax lets you update a DOM property,
            // rather than an attribute.
            prop:value=name
        />

        // INPUT
        <p>"Name is: " {name}</p>
        <input type="text"
            bind:value=(name, set_name)
        />
        <input type="email"
            bind:value=email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <fieldset>
            <legend>"Favorite color"</legend>
            <label>
                "Red"
                <input
                    type="radio"
                    name="color"
                    value="red"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Green"
                <input
                    type="radio"
                    name="color"
                    value="green"
                    bind:group=favorite_color
                />
            </label>
            <label>
                "Blue"
                <input
                    type="radio"
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                />
            </label>
        </fieldset>
        <p>"Your favorite color is " {favorite_color} "."</p>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>

        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
        <form on:submit=on_submit> // on_submit defined below
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
        <button
            on:click=move |_| {
                *set_value.write() += 1;
            }
        >
            "Valuebutton:"
            {value}
        </button>

            // CONTROL FLOW
        <br/>
        <Show
          when=move || { value.get() > 5 }
          fallback=|| view! { Under 5 } // somewhat ugly    
        >
        {None::<String>}
        </Show>

            <br/>
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or something that's not a number!)"
            <input type="number" on:input:target=move |ev| {
                // when input changes, try to parse a number from the input
                set_ok_value.set(ev.target().value().parse::<i32>())
            }/>
            // If an `Err(_) had been rendered inside the <ErrorBoundary/>,
            // the fallback will be displayed. Otherwise, the children of the
            // <ErrorBoundary/> will be displayed.
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|errors| view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors
                        // as strings, if we'd like
                        <ul>
                            {move || errors.get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                .collect::<Vec<_>>()
                            }
                        </ul>
                    </div>
                }
            >
                <p>
                    "You entered "
                    // because `ok_value` is `Result<i32, _>`,
                    // it will render the `i32` if it is `Ok`,
                    // and render nothing and trigger the error boundary
                    // if it is `Err`. It's a signal, so this will dynamically
                    // update when `ok_value` changes
                    <strong>{ok_value}</strong>
                </p>
            </ErrorBoundary>
        </label>
        // BUILDER PATTERN
        {counter(0, 20)} // builder pattern instead of view!
        <br/>
        <input
            on:change:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name
        />
        <p><code>"name:"</code> {name}</p>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            // Suspend allows you use to an async block in the view
            <p>
                "Your shouting name is "
                {move || Suspend::new(async move {
                    async_data.await
                })}
            </p>
        </Suspense>
        <Suspense
            // the fallback will show whenever a resource
            // read "under" the suspense is loading
            fallback=move || view! { <p>"Loading..."</p> }
        >
            // the children will be rendered once initially,
            // and then whenever any resources has been resolved
            <p>
                "Which should be the same as... "
                {move || async_data.get().as_deref().map(ToString::to_string)}
            </p>
        </Suspense>
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar(
    // you can specify it or not when you use <ProgressBar/>
    // #[prop(optional)] would make it optional
    /// The maximum value of the progress bar
    #[prop(default = 100)]
    max: u16,
    // progress: ReadSignal<i32> // (would not take double_count
    /// How much progress should be displayed
    #[prop(into)] // not nightly
    progress: Signal<i32>
    // progress: impl Fn() -> i32 + Send + Sync + 'static // nightly
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br/>
    }
}

/// Builder pattern instead of view!
pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            // typed events found in leptos::ev
            // 1) prevent typos in event names
            // 2) allow for correct type inference in callbacks
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),
        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),
        span().child(("Value: ", move || count.get(), "!")),
        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
    ))
}

async fn important_api_call(name: String) -> String {
    TimeoutFuture::new(1_000).await;
    name.to_ascii_uppercase()
}

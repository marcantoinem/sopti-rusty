use leptos::*;

#[component]
pub fn NumberInput<F>(
    #[prop(optional, into)] value: RwSignal<u8>,
    max: u8,
    label: &'static str,
    submit: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let on_input = move |ev| {
        value.set(event_target_value(&ev).parse::<u8>().unwrap());
        submit();
    };

    let minus = move |_| {
        let v = value.get();
        if v > 0 {
            value.set(v - 1);
            submit();
        }
    };

    let plus = move |_| {
        let v = value.get();
        if v != u8::MAX {
            value.set(v + 1);
            submit();
        }
        submit();
    };

    view! {
        <div class="flex flex-row gap-8 items-center">
            <p class="text-white font-sans font-medium tracking-tight">{label}</p>
            <div class="relative flex items-center max-w-20">
                <button on:pointerdown=minus type="button" class="bg-gray-100 hover:bg-gray-200 border border-gray-300 rounded-s-lg p-1 h-7 focus:ring-gray-100 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 2">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h16"/>
                    </svg>
                </button>
                <input
                    type="numeric"
                    class="bg-gray-50 border-x-0 border-gray-300 h-7 text-center text-gray-900 text-sm focus:ring-amber-500 focus:border-amber-500 block w-full py-2.5"
                    placeholder="0"
                    on:input=on_input
                    type="number"
                    min="0"
                    max=max
                    prop:value=value
                    required />
                <button on:pointerdown=plus type="button" class="bg-gray-100 hover:bg-gray-200 border border-gray-300 rounded-e-lg p-1 h-7 focus:ring-gray-100 focus:ring-2 focus:outline-none">
                    <svg class="w-3 h-3 text-gray-900" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 18 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 1v16M1 9h16"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}

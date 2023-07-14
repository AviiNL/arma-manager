use leptos::*;
// https://github.com/leptos-rs/leptos/blob/main/examples/login_with_token_csr_only/client/src/pages/login.rs

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="relative flex flex-col items-center justify-center h-screen overflow-hidden">
            <div class="w-full p-6 bg-base-200 border-t-4 border-accent rounded-md shadow-md border-top lg:max-w-lg">
                <form class="space-y-4" on:submit=|ev| ev.prevent_default()>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Email"</span>
                        </label>
                        <input type="text" placeholder="Email Address" class="w-full input input-bordered" />
                    </div>
                    <div>
                        <label class="label">
                            <span class="text-base label-text">"Password"</span>
                        </label>
                        <input type="password" placeholder="Enter Password"
                            class="w-full input input-bordered" />
                    </div>
                    <a href="#" class="text-xs label-text hover:underline float-right">"Forget Password?"</a>
                    <div>
                        <button class="btn btn-block">"Login"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}

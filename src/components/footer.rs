use yew::prelude::*;
use crate::components::slider::Slider; // adjust path as needed

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class={classes!("select-none")}>
            <Slider />
            <div class={classes!("py-7", "font-Canopee", "text-2xl", "flex", "flex-row", "justify-between", "px-6")}>
                <a
                    class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                    href="#"
                    target="_blank"
                >
                    {"Sanjay N ©"}
                </a>

                <div class={classes!("flex", "gap-2")}>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://github.com/RNS-Forge"
                        target="_blank"
                    >
                        {"Github"}
                    </a>
                     <span>{"•"}</span>
                    <a
                        class={classes!("hover:opacity-75", "transition-colors", "duration-300")}
                        href="https://www.linkedin.com/in/sanjay--n"
                        target="_blank"
                    >
                        {"LinkedIn"}
                    </a>
                </div>
            </div>
        </div>
    }
}


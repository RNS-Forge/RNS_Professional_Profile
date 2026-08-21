use yew::prelude::*;

#[function_component(Slider)]
pub fn slider() -> Html {
    html! {
        <div class={classes!("marquee-container", "select-none")}>
            <div class={classes!("marquee")}>
                {"WANTED: OPPORTUNITIES IN AI & FULL STACK ENGINEERING — INQUIRE WITHIN — 2005sanjaynrs@gmail.com — "}
                <a href="https://github.com/RNS-Forge" target="_blank">
                    {"GitHub"}
                </a>
                {" & "}
                <a href="https://linkedin.com/in/sanjay--n" target="_blank">
                    {"LinkedIn"}
                </a>
            </div>
        </div>
    }
}


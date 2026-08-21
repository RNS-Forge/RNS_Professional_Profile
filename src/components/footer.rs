use yew::prelude::*;
use stylist::style;
use crate::components::slider::Slider;

#[function_component(Footer)]
pub fn footer() -> Html {
    let container_style = style!(
        r#"
        margin-top: 3rem;
        border-top: 5px double #18181b;
        padding-top: 2rem;
        padding-bottom: 2rem;
        font-family: 'Times New Roman', serif;
        color: #18181b;
        "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: 2fr 1fr 2fr;
        gap: 2rem;
        width: 100%;
        margin-bottom: 2rem;
        @media (max-width: 768px) {
            grid-template-columns: 1fr;
            gap: 1.5rem;
            text-align: center;
        }
        "#
    ).unwrap();

    let sub_footer_style = style!(
        r#"
        border-top: 1px solid rgba(24, 24, 27, 0.2);
        padding-top: 1rem;
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 0.85rem;
        text-transform: uppercase;
        font-weight: bold;
        letter-spacing: 0.5px;
        @media (max-width: 768px) {
            flex-direction: column;
            gap: 0.75rem;
        }
        "#
    ).unwrap();

    html! {
        <div class={classes!("select-none", container_style.get_class_name().to_string())}>
            <div class={grid_style.get_class_name().to_string()}>
                // Left Column: Editorial Info
                <div style="text-align: justify; padding-right: 1rem;">
                    <h3 style="font-family: 'OldLondon', serif; font-size: 1.5rem; border-bottom: 1px solid #18181b; padding-bottom: 0.25rem; margin-bottom: 0.75rem;">
                        {"The Sanjay Times"}
                    </h3>
                    <p style="font-size: 0.85rem; line-height: 1.5; font-style: italic;">
                        {"Established in 2026. Published as an interactive portfolio ledger. Transmitted to bureaus globally from Coimbatore and Bengaluru. Structured autonomously using the Rust language and compiled into WebAssembly binary instructions for standard client render engines."}
                    </p>
                </div>

                // Middle Column: Quick Index Links
                <div style="display: flex; flex-direction: column; align-items: center; justify-content: flex-start;">
                    <h3 style="font-weight: bold; text-transform: uppercase; font-size: 1rem; border-bottom: 1px solid #18181b; width: 100%; text-align: center; padding-bottom: 0.25rem; margin-bottom: 0.75rem;">
                        {"Gazette Index"}
                    </h3>
                    <div style="display: flex; flex-direction: column; gap: 0.35rem; font-size: 0.9rem; font-weight: 600;">
                        <a onclick={Callback::from(move |_| {
                            if let Some(window) = web_sys::window() {
                                let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                            }
                        })} style="cursor: pointer; text-decoration: underline; hover:color: #B93C12;">
                            {"I. Masthead Cover"}
                        </a>
                        <a onclick={Callback::from(move |_| {
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(elem) = doc.get_element_by_id("career-desk") {
                                        elem.scroll_into_view();
                                    }
                                }
                            }
                        })} style="cursor: pointer; text-decoration: underline; hover:color: #B93C12;">
                            {"II. Career Desk"}
                        </a>
                        <a onclick={Callback::from(move |_| {
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(elem) = doc.get_element_by_id("projects-section") {
                                        elem.scroll_into_view();
                                    }
                                }
                            }
                        })} style="cursor: pointer; text-decoration: underline; hover:color: #B93C12;">
                            {"III. Projects Supplement"}
                        </a>
                        <a onclick={Callback::from(move |_| {
                            if let Some(window) = web_sys::window() {
                                if let Some(doc) = window.document() {
                                    if let Some(elem) = doc.get_element_by_id("education-section") {
                                        elem.scroll_into_view();
                                    }
                                }
                            }
                        })} style="cursor: pointer; text-decoration: underline; hover:color: #B93C12;">
                            {"IV. Scholastic Ledger"}
                        </a>
                    </div>
                </div>

                // Right Column: Registry and Badges
                <div style="text-align: right; padding-left: 1rem;">
                    <h3 style="font-weight: bold; text-transform: uppercase; font-size: 1rem; border-bottom: 1px solid #18181b; padding-bottom: 0.25rem; margin-bottom: 0.75rem;">
                        {"Bureau Registry"}
                    </h3>
                    <p style="font-size: 0.85rem; line-height: 1.5; margin-bottom: 1rem;">
                        {"All dispatches registered and managed under open-source software licenses. Powered by Yew Wasm, Stylist CSS grid models, and Tailwind Utility templates."}
                    </p>
                    <div style="display: flex; gap: 0.75rem; justify-content: flex-end; font-size: 0.9rem;">
                        <a href="https://github.com/RNS-Forge" target="_blank" style="text-decoration: underline; font-weight: bold; color: #B93C12;">
                            {"GitHub Registry"}
                        </a>
                        <span>{"•"}</span>
                        <a href="https://www.linkedin.com/in/sanjay--n" target="_blank" style="text-decoration: underline; font-weight: bold; color: #B93C12;">
                            {"LinkedIn Node"}
                        </a>
                    </div>
                </div>
            </div>

            // Running Marquee slider inside the footer
            <Slider />

            // Bottom Copyright info
            <div class={sub_footer_style.get_class_name().to_string()}>
                <div>
                    {"THE SANJAY TIMES © 2026. ALL RIGHTS RESERVED."}
                </div>
                <div>
                    {"VOL. I • BENGALURU & COIMBATORE EDITION"}
                </div>
            </div>
        </div>
    }
}

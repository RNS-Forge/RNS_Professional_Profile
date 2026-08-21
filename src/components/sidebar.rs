use yew::prelude::*;
use stylist::style;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let is_open = use_state(|| false);
    let show_nav = use_state(|| true);
    let last_scroll_y = use_state(|| 0.0);

    let toggle_sidebar = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    // Scroll listener to toggle navbar visibility on scroll directions
    {
        let show_nav = show_nav.clone();
        let last_scroll_y = last_scroll_y.clone();
        use_effect(move || {
            let listener = gloo::events::EventListener::new(&web_sys::window().unwrap(), "scroll", move |_event| {
                if let Some(win) = web_sys::window() {
                    let current_scroll_y = win.scroll_y().unwrap_or(0.0);
                    let previous_y = *last_scroll_y;
                    
                    if current_scroll_y > previous_y && current_scroll_y > 50.0 {
                        // Scrolling down - hide navbar
                        show_nav.set(false);
                    } else if current_scroll_y < previous_y {
                        // Scrolling up - show navbar
                        show_nav.set(true);
                    }
                    
                    last_scroll_y.set(current_scroll_y);
                }
            });
            move || drop(listener)
        });
    }

    let nav_container_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        display: flex;
        justify-content: center;
        align-items: center;
        background: url('/public/bg/texture.jpg') !important;
        background-repeat: no-repeat;
        background-size: cover;
        border-bottom: 4px double #18181b;
        padding: 0.35rem 0;
        z-index: 150;
        pointer-events: auto;
        transition: transform 0.3s ease-in-out;
        "#
    ).unwrap();

    let trigger_style = style!(
        r#"
        background: none;
        border: none;
        color: #18181b;
        padding: 0.1rem 1.5rem;
        font-family: 'OldLondon', 'Times New Roman', serif;
        font-weight: bold;
        font-size: 1.8rem;
        text-transform: uppercase;
        cursor: pointer;
        transition: color 0.2s ease-in-out;
        &:hover {
            color: #B93C12;
        }
        "#
    ).unwrap();

    let overlay_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: url('/public/bg/texture.jpg') !important;
        background-repeat: no-repeat;
        background-size: cover;
        color: #18181b;
        z-index: 200;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        font-family: 'OldLondon', 'Times New Roman', serif;
        border: 10px double #18181b;
        box-sizing: border-box;
        overflow-y: auto;
        padding: 6rem 2rem 3rem 2rem;
        "#
    ).unwrap();

    let close_btn_style = style!(
        r#"
        position: absolute;
        top: 2rem;
        right: 2rem;
        background: none;
        border: 2px solid #18181b;
        padding: 0.4rem 1rem;
        font-family: 'Times New Roman', serif;
        font-size: 1.2rem;
        font-weight: bold;
        cursor: pointer;
        box-shadow: 2px 2px 0px #18181b;
        &:hover {
            box-shadow: 3px 3px 0px #18181b;
        }
        "#
    ).unwrap();

    let nav_list_style = style!(
        r#"
        list-style: none;
        text-align: center;
        font-size: 3.5rem;
        line-height: 1.5;
        "#
    ).unwrap();

    let nav_item_style = style!(
        r#"
        margin: 1.5rem 0;
        cursor: pointer;
        transition: color 0.2s ease;
        &:hover {
            color: #B93C12;
            text-decoration: underline;
        }
        "#
    ).unwrap();

    let container_transform = if *show_nav { "transform: translateY(0);" } else { "transform: translateY(-100%);" };

    html! {
        <>
            <div class={nav_container_style.get_class_name().to_string()} style={container_transform}>
                <button class={trigger_style.get_class_name().to_string()} onclick={toggle_sidebar.clone()}>
                    {"🗞️ Dispatch Menu"}
                </button>
            </div>

            if *is_open {
                <div class={overlay_style.get_class_name().to_string()}>
                    <button class={close_btn_style.get_class_name().to_string()} onclick={toggle_sidebar.clone()}>
                        {"[ Close Edition ]"}
                    </button>
                    <div style="text-align: center; margin-bottom: 2rem;">
                        <h1 style="font-size: 5rem; border-bottom: 3px solid #18181b; padding-bottom: 1rem; text-transform: uppercase;">
                            {"The Sanjay Times"}
                        </h1>
                        <p style="font-family: 'Times New Roman', serif; font-style: italic; font-size: 1.25rem;">
                            {"Select an edition section below to navigate"}
                        </p>
                    </div>
                    <ul class={nav_list_style.get_class_name().to_string()}>
                        <li class={nav_item_style.get_class_name().to_string()} onclick={
                            let toggle = toggle_sidebar.clone();
                            Callback::from(move |e| {
                                toggle.emit(e);
                                if let Some(window) = web_sys::window() {
                                    let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                                }
                            })
                        }>
                            {"I. Masthead & Cover"}
                        </li>
                        <li class={nav_item_style.get_class_name().to_string()} onclick={
                            let toggle = toggle_sidebar.clone();
                            Callback::from(move |e| {
                                toggle.emit(e);
                                if let Some(window) = web_sys::window() {
                                    if let Some(doc) = window.document() {
                                        if let Some(body) = doc.body() {
                                            let height = body.scroll_height() as f64;
                                            let _ = window.scroll_to_with_x_and_y(0.0, height * 0.2);
                                        }
                                    }
                                }
                            })
                        }>
                            {"II. Career Desk"}
                        </li>
                        <li class={nav_item_style.get_class_name().to_string()} onclick={
                            let toggle = toggle_sidebar.clone();
                            Callback::from(move |e| {
                                toggle.emit(e);
                                if let Some(window) = web_sys::window() {
                                    if let Some(doc) = window.document() {
                                        if let Some(body) = doc.body() {
                                            let height = body.scroll_height() as f64;
                                            let _ = window.scroll_to_with_x_and_y(0.0, height * 0.45);
                                        }
                                    }
                                }
                            })
                        }>
                            {"III. The Lab Log"}
                        </li>
                        <li class={nav_item_style.get_class_name().to_string()} onclick={
                            let toggle = toggle_sidebar.clone();
                            Callback::from(move |e| {
                                toggle.emit(e);
                                if let Some(window) = web_sys::window() {
                                    if let Some(doc) = window.document() {
                                        if let Some(body) = doc.body() {
                                            let height = body.scroll_height() as f64;
                                            let _ = window.scroll_to_with_x_and_y(0.0, height);
                                        }
                                    }
                                }
                            })
                        }>
                            {"IV. Classifieds & Contacts"}
                        </li>
                    </ul>
                </div>
            }
        </>
    }
}

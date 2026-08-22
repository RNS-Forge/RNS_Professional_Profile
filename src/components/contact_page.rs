use yew::prelude::*;
use stylist::style;

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    let container_style = style!(
        r#"
        margin: 2rem 0;
        text-align: justify;
        "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
        width: 100%;
        @media (max-width: 768px) {
            grid-template-columns: 1fr;
        }
        "#
    ).unwrap();

    let border_box = style!(
        r#"
        border: 4px double #18181b;
        padding: 1.5rem;
        background: rgba(24, 24, 27, 0.01);
        "#
    ).unwrap();

    let input_style = style!(
        r#"
        width: 100%;
        padding: 0.5rem;
        border: 1px solid #18181b;
        background: transparent;
        font-family: 'Courier New', Courier, monospace;
        font-size: 0.9rem;
        color: #18181b;
        margin-top: 0.25rem;
        margin-bottom: 0.75rem;
        &:focus {
            outline: none;
            border: 2px solid #B93C12;
        }
        "#
    ).unwrap();

    let submit_btn_style = style!(
        r#"
        width: 100%;
        padding: 0.75rem;
        border: 2px solid #18181b;
        background: #18181b;
        color: #fff;
        font-family: 'Times New Roman', serif;
        font-weight: bold;
        font-size: 1rem;
        text-transform: uppercase;
        cursor: pointer;
        box-shadow: 3px 3px 0px #B93C12;
        transition: all 0.2s ease;
        &:hover {
            background: #B93C12;
            box-shadow: 3px 3px 0px #18181b;
        }
        "#
    ).unwrap();

    let form_submitted = use_state(|| false);
    
    // NodeRefs to read inputs
    let name_ref = use_node_ref();
    let email_ref = use_node_ref();
    let subject_ref = use_node_ref();
    let body_ref = use_node_ref();

    let on_submit = {
        let form_submitted = form_submitted.clone();
        let name_ref = name_ref.clone();
        let email_ref = email_ref.clone();
        let subject_ref = subject_ref.clone();
        let body_ref = body_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Extract values
            let name = name_ref.cast::<web_sys::HtmlInputElement>().map(|input| input.value()).unwrap_or_default();
            let email = email_ref.cast::<web_sys::HtmlInputElement>().map(|input| input.value()).unwrap_or_default();
            let subject = subject_ref.cast::<web_sys::HtmlInputElement>().map(|input| input.value()).unwrap_or_default();
            let body = body_ref.cast::<web_sys::HtmlTextAreaElement>().map(|input| input.value()).unwrap_or_default();

            // Construct template
            let template = format!(
                "Hello Sanjay,\n\nI filed a dispatch from your portfolio site:\n- Name: {}\n- Email: {}\n- Subject: {}\n- Message: {}\n\nLooking forward to connecting!",
                name, email, subject, body
            );

            // Construct mailto link
            let mailto_url = format!(
                "mailto:2005sanjaynrs@gmail.com?subject={}&body={}",
                js_sys::encode_uri_component(&subject),
                js_sys::encode_uri_component(&template)
            );

            // Copy to Clipboard & open mail link using web_sys
            if let Some(window) = web_sys::window() {
                let navigator = window.navigator();
                let clipboard = navigator.clipboard();
                let _ = clipboard.write_text(&template);
                let _ = window.open_with_url_and_target(&mailto_url, "_self");
            }

            form_submitted.set(true);
        })
    };

    html! {
        <div id="contact-section" class={container_style.get_class_name().to_string()}>
            <h1 style="font-size: 2rem; border-bottom: 2px solid #18181b; padding-bottom: 0.5rem; margin-bottom: 1rem; text-align: center; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                {"Classifieds Desk — Contacts & Dispatch submissions"}
            </h1>

            <div class={grid_style.get_class_name().to_string()}>
                // Contact Details
                <div class={border_box.get_class_name().to_string()} style="display: flex; flex-direction: column; justify-content: space-between;">
                    <div>
                        <h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                            {"I. Official Contacts"}
                        </h2>
                        <p style="font-size: 0.95rem; line-height: 1.5; margin-bottom: 1.5rem;">
                            {"Do you have inquiries, engineering feedback, or telemetry suggestions? You can file a dispatch through the contact form, or connect directly through our direct telegraph lines."}
                        </p>

                        <div style="display: flex; flex-direction: column; gap: 0.75rem; font-size: 0.95rem;">
                            <div>
                                <strong style="color: #B93C12;">{"Telegraph Email: "}</strong>
                                <a href="mailto:2005sanjaynrs@gmail.com" style="text-decoration: underline;">
                                    {"2005sanjaynrs@gmail.com"}
                                </a>
                            </div>
                            <div>
                                <strong style="color: #B93C12;">{"Bureau Location: "}</strong>
                                <span>{"Coimbatore, Tamil Nadu, India"}</span>
                            </div>
                            <div>
                                <strong style="color: #B93C12;">{"GitHub Registry: "}</strong>
                                <a href="https://github.com/RNS-Forge" target="_blank" style="text-decoration: underline;">
                                    {"github.com/RNS-Forge"}
                                </a>
                            </div>
                            <div>
                                <strong style="color: #B93C12;">{"LinkedIn Node: "}</strong>
                                <a href="https://www.linkedin.com/in/sanjay--n" target="_blank" style="text-decoration: underline;">
                                    {"linkedin.com/in/sanjay--n"}
                                </a>
                            </div>
                        </div>
                    </div>

                    <div style="margin-top: 2rem; border-top: 1px dashed rgba(24, 24, 27, 0.25); padding-top: 1rem; font-style: italic; font-size: 0.85rem; color: #71717a;">
                        {"* Submissions are received and logged autonomously by multi-agent dispatch listeners."}
                    </div>
                </div>

                // Contact Form
                <div class={border_box.get_class_name().to_string()}>
                    <h2 style="font-size: 1.5rem; font-weight: bold; margin-bottom: 1rem; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                        {"II. File a Dispatch"}
                    </h2>

                    {
                        if *form_submitted {
                            html! {
                                <div style="border: 2px dashed #B93C12; padding: 1.5rem; text-align: center; background: rgba(185, 60, 18, 0.05);">
                                    <h3 style="font-size: 1.25rem; font-weight: bold; color: #B93C12; font-family: 'Times New Roman', serif;">
                                        {"DISPATCH COPIED & EMAIL LAUNCHED!"}
                                    </h3>
                                    <p style="font-size: 0.9rem; margin-top: 0.5rem; line-height: 1.4;">
                                        {"We have copied the message draft to your clipboard and launched your default email client prefilled to 2005sanjaynrs@gmail.com. Feel free to paste or review the draft and click send!"}
                                    </p>
                                    <button 
                                        onclick={
                                            let form_submitted = form_submitted.clone();
                                            Callback::from(move |_| form_submitted.set(false))
                                        }
                                        style="margin-top: 1rem; border: 1px solid #18181b; padding: 0.25rem 0.75rem; background: transparent; cursor: pointer; font-size: 0.85rem; font-weight: bold;"
                                    >
                                        {"Send Another"}
                                    </button>
                                </div>
                            }
                        } else {
                            html! {
                                <form onsubmit={on_submit}>
                                    <label style="font-size: 0.8rem; font-weight: bold; text-transform: uppercase;">{"Name / Identifier"}</label>
                                    <input type="text" ref={name_ref} required=true class={input_style.get_class_name().to_string()} placeholder="e.g. Inspector Sterling" />

                                    <label style="font-size: 0.8rem; font-weight: bold; text-transform: uppercase;">{"Telegraph Address (Email)"}</label>
                                    <input type="email" ref={email_ref} required=true class={input_style.get_class_name().to_string()} placeholder="e.g. sterling@bureau.org" />

                                    <label style="font-size: 0.8rem; font-weight: bold; text-transform: uppercase;">{"Dispatch Subject"}</label>
                                    <input type="text" ref={subject_ref} required=true class={input_style.get_class_name().to_string()} placeholder="e.g. Project Consultation" />

                                    <label style="font-size: 0.8rem; font-weight: bold; text-transform: uppercase;">{"Dispatch Body"}</label>
                                    <textarea ref={body_ref} required=true rows=4 class={input_style.get_class_name().to_string()} style="resize: vertical;" placeholder="Type your telegram message here..." />

                                    <button type="submit" class={submit_btn_style.get_class_name().to_string()}>
                                        {"Transmit telegram →"}
                                    </button>
                                </form>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}

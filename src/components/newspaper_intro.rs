use yew::prelude::*;
use stylist::style;

#[derive(Properties, PartialEq)]
pub struct IntroProps {
    pub on_complete: Callback<()>,
}

#[function_component(NewspaperIntro)]
pub fn newspaper_intro(props: &IntroProps) -> Html {
    let container_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: #000;
        z-index: 500;
        overflow: hidden;
        display: flex;
        justify-content: center;
        align-items: center;
        "#
    ).unwrap();

    let video_style = style!(
        r#"
        width: 100vw;
        height: 100vh;
        object-fit: cover;
        display: block;
        "#
    ).unwrap();

    let skip_btn_style = style!(
        r#"
        position: absolute;
        bottom: 2rem;
        right: 2rem;
        z-index: 510;
        border: 2px solid #ffffff;
        padding: 0.5rem 1.5rem;
        background: rgba(24, 24, 27, 0.85);
        color: #ffffff;
        font-weight: bold;
        font-family: 'Times New Roman', serif;
        font-size: 1rem;
        cursor: pointer;
        text-transform: uppercase;
        box-shadow: 3px 3px 0px #ffffff;
        transition: all 0.2s ease;
        &:hover {
            background: #B93C12;
            border-color: #B93C12;
            box-shadow: 3px 3px 0px #B93C12;
            transform: translateY(-2px);
        }
        "#
    ).unwrap();

    let video_ref = use_node_ref();

    // Programmatic play trigger on mount to bypass strict browser autoplay limits
    {
        let video_ref = video_ref.clone();
        use_effect(move || {
            if let Some(video) = video_ref.cast::<web_sys::HtmlVideoElement>() {
                video.set_muted(true);
                video.set_autoplay(true);
                let _ = video.play();
            }
            || ()
        });
    }

    // Fallback liveness timer (auto skip after 12 seconds in case video fails to load/play)
    {
        let on_complete = props.on_complete.clone();
        use_effect(move || {
            let timeout = gloo::timers::callback::Timeout::new(12000, move || {
                on_complete.emit(());
            });
            move || drop(timeout)
        });
    }

    let on_video_ended = {
        let on_complete = props.on_complete.clone();
        Callback::from(move |_: Event| {
            on_complete.emit(());
        })
    };

    let on_skip_click = {
        let on_complete = props.on_complete.clone();
        Callback::from(move |_: MouseEvent| {
            on_complete.emit(());
        })
    };

    html! {
        <div class={container_style.get_class_name().to_string()}>
            <video 
                ref={video_ref}
                src="/public/Intro.mp4"
                autoplay=true
                muted=true
                playsinline=true
                preload="auto"
                onended={on_video_ended}
                class={video_style.get_class_name().to_string()}
            />

            <button onclick={on_skip_click} class={skip_btn_style.get_class_name().to_string()}>
                {"Skip Intro →"}
            </button>
        </div>
    }
}

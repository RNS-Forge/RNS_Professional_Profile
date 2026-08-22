use yew::prelude::*;
use stylist::style;

struct ProjectItem {
    title: &'static str,
    subtitle: &'static str,
    tech: &'static str,
    desc: &'static str,
    img: &'static str,
    link: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsPageProps {
    pub on_view_archive: Callback<()>,
}

#[function_component(ProjectsPage)]
pub fn projects_page(props: &ProjectsPageProps) -> Html {
    let container_style = style!(
        r#"
        margin: 2rem 0;
        text-align: justify;
        "#
    ).unwrap();

    let scroll_container_style = style!(
        r#"
        display: flex;
        gap: 1.5rem;
        overflow-x: auto;
        padding: 1rem 0.5rem;
        scroll-behavior: smooth;
        scrollbar-width: thin;
        scrollbar-color: #B93C12 transparent;
        &::-webkit-scrollbar {
            height: 6px;
        }
        &::-webkit-scrollbar-thumb {
            background-color: #B93C12;
            border-radius: 3px;
        }
        "#
    ).unwrap();

    let card_style = style!(
        r#"
        flex: 0 0 350px;
        border: 4px double #18181b;
        background: rgba(24, 24, 27, 0.01);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        padding: 1rem;
        box-shadow: 4px 4px 0px rgba(24, 24, 27, 0.1);
        transition: transform 0.2s ease, box-shadow 0.2s ease;
        &:hover {
            transform: translateY(-4px);
            box-shadow: 6px 6px 0px rgba(185, 60, 18, 0.25);
        }
        "#
    ).unwrap();

    let view_more_btn_style = style!(
        r#"
        display: block;
        margin: 1.5rem auto 0 auto;
        border: 2px solid #18181b;
        padding: 0.5rem 2rem;
        background: transparent;
        font-family: 'Times New Roman', serif;
        font-weight: bold;
        cursor: pointer;
        text-transform: uppercase;
        box-shadow: 3px 3px 0px #18181b;
        &:hover {
            background: #B93C12;
            color: #fff;
            box-shadow: 3px 3px 0px #B93C12;
        }
        "#
    ).unwrap();

    let container_ref = use_node_ref();

    // Auto-scroll bounce loop
    {
        let container_ref = container_ref.clone();
        use_effect(
            move || {
                let scroll_right = std::rc::Rc::new(std::cell::Cell::new(true));
                let interval = gloo::timers::callback::Interval::new(30, move || {
                    if let Some(elem) = container_ref.cast::<web_sys::HtmlElement>() {
                        let max_scroll = elem.scroll_width() - elem.client_width();
                        let current_scroll = elem.scroll_left();
                        
                        if scroll_right.get() {
                            if current_scroll >= max_scroll - 1 {
                                scroll_right.set(false); // Change direction to left
                                elem.set_scroll_left(current_scroll - 1);
                            } else {
                                elem.set_scroll_left(current_scroll + 1);
                            }
                        } else {
                            if current_scroll <= 0 {
                                scroll_right.set(true); // Change direction to right
                                elem.set_scroll_left(current_scroll + 1);
                            } else {
                                elem.set_scroll_left(current_scroll - 1);
                            }
                        }
                    }
                });
                move || drop(interval)
            }
        );
    }

    let projects = vec![
        ProjectItem {
            title: "Agentium Engine",
            subtitle: "Agentic Multi-Agent Framework",
            tech: "Python, LangChain, CrewAI",
            desc: "Designed and published a modular python library for structuring complex agent communications and automating business workflows. Cuts pipeline setup by 55%.",
            img: "/public/IMG/UIComponent.png",
            link: "https://github.com/RNS-Forge",
        },
        ProjectItem {
            title: "Researcher AgentX",
            subtitle: "Autonomous Research Dispatcher",
            tech: "Python, AutoGen, LangGraph",
            desc: "An intelligent autonomous literature crawler that searches publications, filters relevant research papers, and structures research reviews by 18% higher speed.",
            img: "/public/IMG/blog.png",
            link: "https://github.com/RNS-Forge",
        },
        ProjectItem {
            title: "Audioscape: Spotify App",
            subtitle: "Responsive Media Player Interface",
            tech: "HTML, CSS, Web API, JS",
            desc: "Replicates high-fidelity Spotify desktop designs and dynamic search queries, integrating client audio playback loops and custom visual sliders.",
            img: "/public/IMG/spotify.png",
            link: "https://github.com/RNS-Forge",
        },
        ProjectItem {
            title: "AI Exam Paper Analyzer",
            subtitle: "Academic Evaluation Tool",
            tech: "Python, PyTorch, LangChain",
            desc: "Parses handwritten exam sheets, extracts answers using computer vision, and runs analysis against strict rubrics to cut manual grading time by 30%.",
            img: "/public/IMG/scholarship.png",
            link: "https://github.com/RNS-Forge",
        },
        ProjectItem {
            title: "Corporate Placement HUB",
            subtitle: "Job Search Portal & Tracker",
            tech: "Node.js, React, Express, MongoDB",
            desc: "Coordinates student profiles and schedules recruiter rounds, displaying success analytics metrics and automated email dispatch templates.",
            img: "/public/IMG/placement.png",
            link: "https://github.com/RNS-Forge",
        },
    ];

    html! {
        <div id="projects-section" class={container_style.get_class_name().to_string()}>
            <h1 style="font-size: 2rem; border-bottom: 2px solid #18181b; padding-bottom: 0.5rem; margin-bottom: 0.5rem; text-align: center; font-family: 'Times New Roman', serif; text-transform: uppercase;">
                {"Sunday Supplement — Portfolio & Project Gallery"}
            </h1>
            <p style="font-size: 1.1rem; font-style: italic; text-align: center; margin-bottom: 1rem;">
                {"Scroll horizontally to view details of our latest technical dispatches."}
            </p>

            <div ref={container_ref} class={scroll_container_style.get_class_name().to_string()}>
                {
                    projects.into_iter().map(|p| {
                        html! {
                            <div class={card_style.get_class_name().to_string()}>
                                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                                    <div style="height: 180px; width: 100%; overflow: hidden; border: 1px solid #18181b; margin-bottom: 0.5rem;">
                                        <img src={p.img} style="width: 100%; height: 100%; object-fit: cover;" />
                                    </div>
                                    <span style="font-size: 0.75rem; text-transform: uppercase; font-weight: bold; color: #B93C12;">
                                        {p.subtitle}
                                    </span>
                                    <h3 style="font-size: 1.3rem; font-weight: bold; font-family: 'Times New Roman', serif; line-height: 1.1;">
                                        {p.title}
                                    </h3>
                                    <p style="font-size: 0.8rem; font-weight: bold; text-transform: uppercase; letter-spacing: 0.5px; color: #3f3f46;">
                                        {p.tech}
                                    </p>
                                    <p style="font-size: 0.85rem; color: #18181b; line-height: 1.4; margin-top: 0.25rem;">
                                        {p.desc}
                                    </p>
                                </div>
                                <div style="margin-top: 1rem; border-top: 1px dashed rgba(24, 24, 27, 0.25); padding-top: 0.75rem; text-align: right;">
                                    <a href={p.link} target="_blank" style="font-family: 'Times New Roman', serif; font-size: 0.9rem; font-weight: bold; color: #B93C12; text-decoration: underline;">
                                        {"View Repository →"}
                                    </a>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            <button 
                onclick={let on_view = props.on_view_archive.clone(); Callback::from(move |_: MouseEvent| on_view.emit(()))}
                class={view_more_btn_style.get_class_name().to_string()}
            >
                {"View Full Projects Archive"}
            </button>
        </div>
    }
}

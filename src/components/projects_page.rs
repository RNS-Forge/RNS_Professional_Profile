use yew::prelude::*;
use stylist::style;
use wasm_bindgen::JsCast;

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
        cursor: grab;
        touch-action: pan-y;
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
        cursor: pointer;
        text-decoration: none;
        color: inherit;
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

    {
        let container_ref = container_ref.clone();
        use_effect(
            move || {
                let mut listeners = Vec::new();
                let last_interaction = std::rc::Rc::new(std::cell::Cell::new(0.0));
                let is_dragging = std::rc::Rc::new(std::cell::Cell::new(false));
                let start_x = std::rc::Rc::new(std::cell::Cell::new(0.0));
                let start_scroll = std::rc::Rc::new(std::cell::Cell::new(0));
                let is_pointer_down = std::rc::Rc::new(std::cell::Cell::new(false));
                let mut cleanup_interval = None;

                if let Some(elem) = container_ref.cast::<web_sys::HtmlElement>() {
                    let update_interaction = {
                        let last_interaction = last_interaction.clone();
                        move || {
                            last_interaction.set(js_sys::Date::now());
                        }
                    };

                    // Pointer Down (captures both mouse drag and touch swipe)
                    {
                        let is_pointer_down = is_pointer_down.clone();
                        let is_dragging = is_dragging.clone();
                        let start_x = start_x.clone();
                        let start_scroll = start_scroll.clone();
                        let elem_for_closure = elem.clone();
                        let update_interaction = update_interaction.clone();
                        let listener = gloo::events::EventListener::new(&elem, "pointerdown", move |event| {
                            if let Some(pe) = event.dyn_ref::<web_sys::PointerEvent>() {
                                is_pointer_down.set(true);
                                is_dragging.set(false);
                                start_x.set(pe.client_x() as f64);
                                start_scroll.set(elem_for_closure.scroll_left());
                                update_interaction();
                            }
                        });
                        listeners.push(listener);
                    }

                    // Pointer Move (drags the container)
                    {
                        let is_pointer_down = is_pointer_down.clone();
                        let is_dragging = is_dragging.clone();
                        let start_x = start_x.clone();
                        let start_scroll = start_scroll.clone();
                        let elem_for_closure = elem.clone();
                        let update_interaction = update_interaction.clone();
                        let listener = gloo::events::EventListener::new(&elem, "pointermove", move |event| {
                            if is_pointer_down.get() {
                                if let Some(pe) = event.dyn_ref::<web_sys::PointerEvent>() {
                                    let current_x = pe.client_x() as f64;
                                    let dx = current_x - start_x.get();
                                    
                                    if !is_dragging.get() && dx.abs() > 5.0 {
                                        is_dragging.set(true);
                                        let _ = elem_for_closure.style().set_property("cursor", "grabbing");
                                        let _ = elem_for_closure.style().set_property("user-select", "none");
                                        let _ = elem_for_closure.set_pointer_capture(pe.pointer_id());
                                    }
                                    
                                    if is_dragging.get() {
                                        elem_for_closure.set_scroll_left(start_scroll.get() - dx as i32);
                                        update_interaction();
                                    }
                                }
                            }
                        });
                        listeners.push(listener);
                    }

                    // Pointer Up / Cancel / Leave
                    {
                        let is_pointer_down = is_pointer_down.clone();
                        let is_dragging = is_dragging.clone();
                        let elem_for_closure = elem.clone();
                        let update_interaction = update_interaction.clone();
                        let end_drag = move |event: &web_sys::Event| {
                            if is_pointer_down.get() {
                                is_pointer_down.set(false);
                                if is_dragging.get() {
                                    is_dragging.set(false);
                                    let _ = elem_for_closure.style().remove_property("cursor");
                                    let _ = elem_for_closure.style().remove_property("user-select");
                                    if let Some(pe) = event.dyn_ref::<web_sys::PointerEvent>() {
                                        let _ = elem_for_closure.release_pointer_capture(pe.pointer_id());
                                    }
                                }
                                update_interaction();
                            }
                        };

                        {
                            let end_drag = end_drag.clone();
                            listeners.push(gloo::events::EventListener::new(&elem, "pointerup", move |e| end_drag(e)));
                        }
                        {
                            let end_drag = end_drag.clone();
                            listeners.push(gloo::events::EventListener::new(&elem, "pointercancel", move |e| end_drag(e)));
                        }
                    }

                    // Wheel event (resets timer on manual trackpad/mouse wheel scroll)
                    {
                        let update_interaction = update_interaction.clone();
                        let listener = gloo::events::EventListener::new(&elem, "wheel", move |_| {
                            update_interaction();
                        });
                        listeners.push(listener);
                    }

                    // Auto-scroll loop
                    let scroll_right = std::rc::Rc::new(std::cell::Cell::new(true));
                    let elem_for_scroll = elem.clone();
                    let last_interaction = last_interaction.clone();
                    let is_dragging_check = is_dragging.clone();
                    let interval = gloo::timers::callback::Interval::new(30, move || {
                        if !is_dragging_check.get() && (js_sys::Date::now() - last_interaction.get() > 3000.0) {
                            let max_scroll = elem_for_scroll.scroll_width() - elem_for_scroll.client_width();
                            let current_scroll = elem_for_scroll.scroll_left();
                            
                            if scroll_right.get() {
                                if current_scroll >= max_scroll - 1 {
                                    scroll_right.set(false);
                                    elem_for_scroll.set_scroll_left(current_scroll - 1);
                                } else {
                                    elem_for_scroll.set_scroll_left(current_scroll + 1);
                                }
                            } else {
                                if current_scroll <= 0 {
                                    scroll_right.set(true);
                                    elem_for_scroll.set_scroll_left(current_scroll + 1);
                                } else {
                                    elem_for_scroll.set_scroll_left(current_scroll - 1);
                                }
                            }
                        }
                    });
                    cleanup_interval = Some(interval);
                }

                move || {
                    if let Some(interval) = cleanup_interval {
                        drop(interval);
                    }
                    drop(listeners);
                }
            }
        );
    }

    let projects = vec![
        ProjectItem {
            title: "Agentic Code Generator",
            subtitle: "Autonomous AI Code Builder",
            tech: "HTML, Tailwind CSS, JavaScript",
            desc: "An interactive developer workbench generating clean, semantic HTML templates on demand, leveraging AI models to accelerate layout prototyping.",
            img: "public/projects/AgenticCodeGenerator.jpg",
            link: "https://github.com/RNS-Forge/Agentic-Code-Generator",
        },
        ProjectItem {
            title: "AgriBridge AI",
            subtitle: "Smart Agriculture Telemetry",
            tech: "TypeScript, React, Node.js",
            desc: "An enterprise agricultural supply-chain and telemetry portal connecting smallholder farmers with export networks, ensuring instant payout validation.",
            img: "public/projects/AgriBridgeAI.jpg",
            link: "https://github.com/RNS-Forge/AgriBridge-AI",
        },
        ProjectItem {
            title: "AI-Based Market Research Analyst",
            subtitle: "Market Intelligence Agent",
            tech: "JavaScript, Node.js, LLMs",
            desc: "An autonomous research agent parsing keyword volumes, scraping competitor directories, and outputting targeted audience reports.",
            img: "public/projects/MarketResearchAnalyst.jpg",
            link: "https://github.com/RNS-Forge/AI-Based-Market-Research-Analyst",
        },
        ProjectItem {
            title: "Automated AI-powered API Pentesting",
            subtitle: "Security Automation Scanner",
            tech: "TypeScript, OWASP API, security",
            desc: "Dynamic penetration scanner mimicking cybersecurity attack scripts, scanning REST endpoints for injection flaws, and reporting CVE fixes.",
            img: "public/projects/APIPentesting.jpg",
            link: "https://github.com/RNS-Forge/Automated-AI-powered-API-Pentesting",
        },
        ProjectItem {
            title: "RNS Exam Paper Analyzer",
            subtitle: "Academic Evaluation Tool",
            tech: "JavaScript, Vision APIs, PyTorch",
            desc: "Parses handwritten exam sheets, extracts answers using computer vision, and runs analysis against strict rubrics to cut manual grading time by 30%.",
            img: "public/projects/ExamPaperAnalyzer.jpg",
            link: "https://github.com/RNS-Forge/RNS_Exam-Papper-Analyzer",
        },
        ProjectItem {
            title: "Agentium Engine",
            subtitle: "Agentic Multi-Agent Framework",
            tech: "Python, LangChain, CrewAI",
            desc: "Designed and published a modular python library for structuring complex agent communications and automating business workflows. Cuts pipeline setup by 55%.",
            img: "public/projects/Agentium.png",
            link: "https://pypi.org/project/agentium/",
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
                            <a href={p.link} target="_blank" class={card_style.get_class_name().to_string()}>
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
                                    <span style="font-family: 'Times New Roman', serif; font-size: 0.9rem; font-weight: bold; color: #B93C12; text-decoration: underline;">
                                        {"View Repository →"}
                                    </span>
                                </div>
                            </a>
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

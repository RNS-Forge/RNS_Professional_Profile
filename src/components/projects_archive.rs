use yew::prelude::*;
use stylist::style;

struct ArchiveProject {
    title: &'static str,
    subtitle: &'static str,
    tech: &'static str,
    desc: &'static str,
    link: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct ArchiveProps {
    pub on_close: Callback<()>,
}

#[function_component(ProjectsArchive)]
pub fn projects_archive(props: &ArchiveProps) -> Html {
    let container_style = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: url('/public/bg/texture.jpg') !important;
        background-repeat: no-repeat;
        background-size: cover;
        z-index: 250;
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 2rem;
        box-sizing: border-box;
        overflow-y: auto;
        font-family: 'Times New Roman', serif;
        color: #18181b;
        "#
    ).unwrap();

    let grid_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
        gap: 1.5rem;
        width: 100%;
        max-width: 1200px;
        margin: 2rem 0;
        "#
    ).unwrap();

    let card_style = style!(
        r#"
        border: 4px double #18181b;
        background: rgba(24, 24, 27, 0.02);
        padding: 1.25rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        min-height: 200px;
        box-shadow: 4px 4px 0px rgba(24, 24, 27, 0.1);
        transition: transform 0.2s ease, box-shadow 0.2s ease;
        &:hover {
            transform: translateY(-3px);
            box-shadow: 6px 6px 0px rgba(185, 60, 18, 0.25);
        }
        "#
    ).unwrap();

    let btn_style = style!(
        r#"
        border: 2px solid #18181b;
        padding: 0.5rem 2rem;
        background: transparent;
        font-weight: bold;
        cursor: pointer;
        text-transform: uppercase;
        box-shadow: 3px 3px 0px #18181b;
        font-family: 'Times New Roman', serif;
        margin-top: 1rem;
        &:hover {
            background: #B93C12;
            color: #fff;
            box-shadow: 3px 3px 0px #B93C12;
        }
        "#
    ).unwrap();
    let on_close = props.on_close.clone();

    let projects = vec![
        ArchiveProject {
            title: "Agentium Engine",
            subtitle: "Agentic Multi-Agent Framework",
            tech: "Python, LangChain, CrewAI",
            desc: "Designed and published a modular python library for structuring complex agent communications and automating business workflows. Cuts pipeline setup by 55%.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Researcher AgentX",
            subtitle: "Autonomous Research Dispatcher",
            tech: "Python, AutoGen, LangGraph",
            desc: "An intelligent autonomous literature crawler that searches publications, filters relevant research papers, and structures research reviews by 18% higher speed.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Audioscape: Spotify App",
            subtitle: "Responsive Media Player Interface",
            tech: "HTML, CSS, Web API, JS",
            desc: "Replicates high-fidelity Spotify desktop designs and dynamic search queries, integrating client audio playback loops and custom visual sliders.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "AI Exam Paper Analyzer",
            subtitle: "Academic Evaluation Tool",
            tech: "Python, PyTorch, LangChain",
            desc: "Parses handwritten exam sheets, extracts answers using computer vision, and runs analysis against strict rubrics to cut manual grading time by 30%.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Corporate Placement HUB",
            subtitle: "Job Search Portal & Tracker",
            tech: "Node.js, React, Express, MongoDB",
            desc: "Coordinates student profiles and schedules recruiter rounds, displaying success analytics metrics and automated email dispatch templates.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Smart Agritech Telemetry",
            subtitle: "IOT ML Yield Forecast Engine",
            tech: "Python, scikit-learn, FastAPI",
            desc: "An IoT dashboard integrating remote soil humidity sensor data streams with ML forecasting models to predict harvest yield dates with 92% reliability.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "OmniSearch vectorDB",
            subtitle: "Fast Contextual Embedding Parser",
            tech: "Go, Rust, gRPC, Pinecone",
            desc: "Designed a vector database proxy wrapper that standardizes high-volume contextual database queries with optimized multi-threaded client threads.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Crypto Ledger Telemetry",
            subtitle: "Realtime Blockchain Tracker Node",
            tech: "Rust, Web3.rs, TypeScript",
            desc: "Compiles a live distributed blockchain event monitor capturing transaction rates and contract calls, compiling automated alerts.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "Neural Style Painter",
            subtitle: "Adversarial GAN Editor UI",
            tech: "Python, TensorFlow, React",
            desc: "Implements convolutional neural style transfers on custom images, displaying styled art grids with adjustable latency settings.",
            link: "https://github.com/RNS-Forge",
        },
        ArchiveProject {
            title: "RAG Knowledge Assistant",
            subtitle: "Large PDF Semantic Chatbot",
            tech: "Python, LlamaIndex, OpenAI API",
            desc: "Extracts metadata across document repositories, constructing chunk structures and indexes to deliver precise search answers.",
            link: "https://github.com/RNS-Forge",
        },
    ];

    html! {
        <div class={container_style.get_class_name().to_string()}>
            <div style="text-align: center; max-width: 800px; margin-bottom: 1rem;">
                <h1 style="font-size: 3rem; font-family: 'OldLondon', serif; border-bottom: 3px solid #18181b; padding-bottom: 0.5rem; text-transform: uppercase;">
                    {"The Sanjay Times — Complete Projects Archive"}
                </h1>
                <p style="font-size: 1.15rem; font-style: italic; margin-top: 0.25rem;">
                    {"Cataloging technical dispatches, open source developments, and software releases."}
                </p>
            </div>

            <div class={grid_style.get_class_name().to_string()}>
                {
                    projects.into_iter().map(|p| {
                        html! {
                            <div class={card_style.get_class_name().to_string()}>
                                <div style="display: flex; flex-direction: column; gap: 0.35rem;">
                                    <span style="font-size: 0.7rem; text-transform: uppercase; font-weight: bold; color: #B93C12;">
                                        {p.subtitle}
                                    </span>
                                    <h3 style="font-size: 1.25rem; font-weight: bold; font-family: 'Times New Roman', serif; line-height: 1.1;">
                                        {p.title}
                                    </h3>
                                    <span style="font-size: 0.75rem; font-weight: bold; color: #71717a; text-transform: uppercase;">
                                        {p.tech}
                                    </span>
                                    <p style="font-size: 0.85rem; color: #27272a; line-height: 1.4; margin-top: 0.5rem; text-align: justify;">
                                        {p.desc}
                                    </p>
                                </div>
                                <div style="margin-top: 1rem; border-top: 1px dashed rgba(24, 24, 27, 0.25); padding-top: 0.5rem; text-align: right;">
                                    <a href={p.link} target="_blank" style="font-size: 0.85rem; font-weight: bold; color: #B93C12; text-decoration: underline;">
                                        {"Inspect Repository →"}
                                    </a>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            <button onclick={Callback::from(move |_: MouseEvent| on_close.emit(()))} class={btn_style.get_class_name().to_string()}>
                {"Return to Gazette"}
            </button>
        </div>
    }
}

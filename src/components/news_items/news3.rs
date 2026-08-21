use yew::prelude::*;
use stylist::style;

#[function_component(News3)]
pub fn news3() -> Html {
    let container_style = style!(
        r#"
        margin-top: 1rem;
        display: flex;
        flex-direction: row;
        gap: 1.25rem;
        text-align: justify;
        "#).unwrap();
    
    let imgtype_style = style!(r#"
        border-radius: 20px;
        filter: grayscale(80%);
        "#).unwrap(); 

    let container =  container_style.get_class_name().to_string();
    let imgtype = imgtype_style.get_class_name().to_string();    

    html! {
        <div >
            // Section 1
            <h1 style="font-size: 2rem;text-align: center;">{"FROM THE LAB: FIVE EXPERIMENTS IN AGENTIC ENGINEERING"}</h1>
            <div class={container.clone()}>
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                     <h1 class={classes!("text-4xl","mb-1")}>
                        {"AGENTIUM"}
                    </h1>
                    <p>{"A Python Library for Agentic AI Systems"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"A"}</span>{"gentium enables modular, high-efficiency orchestration workflows."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Python, LangChain, CrewAI"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Designed and published Agentium, a lightweight Python engine built to structure multi-agent communications and minimize integration overhead. Standardizing agent interfaces cut design and development time by 55%."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/RNSsanjay">{" GitHub Repository [1]"}</a>
                        </p>
                    </div>
                </div>
            </div>

            // Section 2
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div><h1 class={classes!("text-4xl", "mt-1", "mb-1")}>{"RESEARCHER AGENTX"}</h1><p>{"Multi-Agent AI Research System"}</p></div>
                <div class={classes!("flex","gap-3","text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"R"}</span>{"unning autonomous web retrieval pipelines for academic literature reviews."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Python, AutoGen, LangGraph"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Developed an autonomous researcher dispatch tool utilizing layered agentic frameworks. Agents crawl database portals, validate paper relevance, summarize key findings, and output structured markdown digests. Research efficiency rose by 18%."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/RNSsanjay">{" GitHub Repository [2]"}</a>
                        </p>

                    </div>
                </div>
            </div>

            // Section 3
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-3xl","mb-1")}>
                        {"PROJECT MANAGEMENT AGENT"}
                    </h1>
                    <p>{"AI-Driven Automation for Modern Teams"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"P"}</span>{"roductivity tool automating task tracking, messaging, and follow-ups."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Node.js, CrewAI, RESTful APIs"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Engineered an automation layer integrating slack logs, calendar tasks, and incoming emails to autonomously coordinate schedules, project statuses, and updates. Lifted team-wide productivity benchmarks by 40%."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/RNSsanjay">{" GitHub Repository [3]"}</a>
                        </p>
                    </div>
                </div>
            </div>

            
            </div>
        </div>
    }
}


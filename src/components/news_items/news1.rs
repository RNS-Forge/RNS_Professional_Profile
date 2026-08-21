use yew::prelude::*;
use stylist::style;

#[function_component(News1)]
pub fn news1() -> Html {
    let container_style = style!(
        r#"
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        gap: 1.25rem;
        margin-top: 2rem;
        margin-bottom: 2rem;
    "#).unwrap();

    let title_style = style!(
        r#"
        font-family: 'Times New Roman', serif;
        font-weight: 800;
        font-size: 3rem;
        padding-bottom: 0.75rem;
    "#).unwrap();

    let subheading_style = style!(
        r#"
        font-size: 1.25rem; 
        font-weight: bold;
        "#).unwrap();
    
    let section_style = style!(
        r#" 
        display: flex; 
        flex-direction: column; 
        gap: 0.5rem;"#).unwrap();
    
    let section_style2 = style!(
        r#"
        flex: 1; 
        display: flex; 
        flex-direction: column; 
        gap: 1rem;"#
    ).unwrap();

    let container = container_style.get_class_name().to_string();
    let title = title_style.get_class_name().to_string();
    let subheading = subheading_style.get_class_name().to_string();
    let section = section_style.get_class_name().to_string();
    let section2 = section_style2.get_class_name().to_string();

    html! {
        <div class={container.clone()}>
            // Left Column
            <div class={classes!("basis-1/2")}>
                <div class={title.clone()}>
                    {"LOCAL ENGINEER BUILDS AGENTIC AI SYSTEMS"} 
                    <span class={classes!("not-italic", "font-bold")}>{" — FULL STACK PLATFORMS BEFORE GRADUATION"}</span>
                </div>
                <div class={classes!("text-sm", "font-bold", "mb-3", "uppercase")}>
                    {"B.Tech AI & ML Graduate Ships Production AI Tools Across Three Companies, Five Independent Projects"}
                </div>
                <div class={classes!("flex", "flex-row", "gap-3", "text-sm", "text-justify")}>
                    // First Text Block
                    <div class={section2.clone()}>
                        <div class={section.clone()}>
                            <p>
                                <span class={classes!("high")}>{"S"}</span>
                                {"anjay N, an AI & Full Stack Engineering graduate (B.Tech, Artificial Intelligence & Machine Learning, SNS College of Technology, CGPA 8.38/10), has spent the last three years embedded in agentic AI and scalable web development."}
                            </p>
                            <p>
                                {"He has focused on building production-ready systems across academic workflow platforms, enterprise assessment tools, and intelligent automation engines."}
                            </p>
                        </div>
                    </div>
                    <div class={section2.clone()}>
                        <div class={section.clone()}>
                            <p>
                                {"Sources close to the developer confirm a deep specialization in LangChain, LangGraph, CrewAI, and AutoGen, backed by a robust full-stack toolkit spanning Python, TypeScript, React, and Node.js."}
                            </p>
                            <p>
                                {"His approach bridges the gap between sophisticated agentic orchestrations and responsive, clean frontend interfaces built to scale and deliver business value."}
                            </p>
                        </div>
                    </div>
                    // Second Text Block with Image
                    
                    // Third Text Block
                </div>
            </div>

            // Right Column with image and two text blocks
            <div class={classes!("flex", "flex-col", "gap-4", "basis-1/2")}>
                <img src="/public/IMG/crimeGIFS/a-3.gif" style="height: 500px; object-fit: cover;" />
            </div>
        </div>
    }
}


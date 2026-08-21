use stylist::style;

use yew::prelude::*;

#[function_component(News4)]
pub fn news4() -> Html {
    let container_style = style!(
            r#"
            display: flex; 
            gap: 0.5rem;
            "#).unwrap();
    
    let column_left_style = style!(
        r#"
        flex-basis: 80%;
        text-align: justify;
        border-right: 2px solid #3f3f46; /* zinc-700 */
        padding-right: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    "#).unwrap();

    let column_main_style = style!(
            r#"
            flex-basis: 30%; 
            display: flex; 
            gap: 1.25rem;
            "#).unwrap();

    let left_main_style = style!(
            r#"
            flex-basis: 60%; 
            display: flex; 
            flex-direction: column; 
            gap: 0.75rem;
            "#).unwrap();
    
    let mid_text_block_style = style!(
            r#"
            flex: 1; 
            display: flex; 
            flex-direction: column; 
            gap: 0.5rem; 
            text-align: justify; 
            font-size: 0.875rem;"#).unwrap();
    
    let right_img_text_style = style!(
            r#"
            flex-basis: 40%; 
            border: 2px solid #3f3f46; 
            padding: 0.5rem; 
            text-align: justify; 
            display: flex; 
            flex-direction: column; 
            gap: 1rem;"#).unwrap();
    
    let tech_img_style = style!(
            r#"
            width: 24px;
            height: 24px;
            margin-right: 0.5rem;
            filter: grayscale(60%);
            "#).unwrap();
    
    let tech_img_heading_style = style!(
            r#"
            width: 35px;
            height: 35px;
            margin-right: 0.5rem;
            filter: grayscale(60%);
            "#).unwrap();

    let tech_text_heading_style = style!(
        r#"
        font-size: 30px;
        margin-top: 5px;
        font-weight: bold;
        text-transform: uppercase;
        "#).unwrap();


    let tech_text_style = style!(
        r#"
        margin-top: 2.5px;
        font-weight: bold;
        text-transform: uppercase;
        "#).unwrap();
    
    let container = container_style.get_class_name().to_string();
    let column_left = column_left_style.get_class_name().to_string();
    let column_main = column_main_style.get_class_name().to_string();
    let left_main = left_main_style.get_class_name().to_string();
    let mid_text_block = mid_text_block_style.get_class_name().to_string();
    let right_img_text = right_img_text_style.get_class_name().to_string();
    let tech_img = tech_img_style.get_class_name().to_string();
    let tech_text = tech_text_style.get_class_name().to_string();
    let tech_img_heading = tech_img_heading_style.get_class_name().to_string();
    let tech_text_heading = tech_text_heading_style.get_class_name().to_string();



    html! {
        <div>
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">{"NEXUS HORIZON COMMISSIONS AI DEVELOPER FOR FACULTIES.AI OVERHAUL"}</h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                            <p class="dropcap">
                                <span class="first-letter">{"F"}</span>{"aculties.ai required scalable, responsive AI-driven academic workflows."}
                            </p>
                            <p>
                                {"Between "}<strong>{"September 2025 and April 2026"}</strong>{", Sanjay N served as "}<strong>{"AI Developer & Tester"}</strong>{" to deliver production-ready solutions."}
                            </p>
                            <p>
                                {"He built user-friendly interfaces integrated with backend and AI services, cutting API response handling errors by 30%. He developed and integrated AI-driven applications using Python, LLMs, and agentic AI workflows, collaborating across UI/UX, backend, and AI teams."}
                            </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Sep 2025 - Apr 2026"}</div>
                    </div>
                </div>
            </div>
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">{"SNS SQUARE DEPLOYS FULL-STACK AI ACROSS THREE ENTERPRISE PLATFORMS"}</h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                            <p class="dropcap">
                                <span class="first-letter">{"M"}</span>{"ultiple AI-based enterprise platforms needed full-stack development and precise requirement analysis."}
                            </p>
                            <p>
                                {"From "}<strong>{"August 2024 to September 2025"}</strong>{", as a "}<strong>{"Full Stack AI Developer & Tester"}</strong>{", he built modules, implemented AI logic, and ran testing aligned to business needs across three key projects: "}<em>{"AI Exam Analyzer, Gen AI Suite, and Aggregator"}</em>{"."}
                            </p>
                            <p>
                                {"His interventions lifted assessment accuracy by 15% and boosted project success rates by 10% through comprehensive deployment of frontend interfaces and structured backend APIs."}
                            </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Aug 2024 - Sep 2025"}</div>
                    </div>
                </div>
            </div>
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">{"COGNIFYZ TECHNOLOGIES TRIALS FRAUD-DETECTION, RAG-POWERED BOTS"}</h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                            <p class="dropcap">
                                <span class="first-letter">{"S"}</span>{"ystems required intelligent automation and sharper AI response accuracy."}
                            </p>
                            <p>
                                {"During his tenure from "}<strong>{"October 2023 to June 2024"}</strong>{" as an "}<strong>{"AIML Engineer"}</strong>{", Sanjay N developed and optimized models for personalization, fraud detection, and knowledge retrieval — building recommendation models and training small language models for projects like "}<em>{"Electro Bot"}</em>{" and "}<em>{"Collexa.ai"}</em>{"."}
                            </p>
                            <p>
                                {"By implementing retrieval-augmented generation (RAG) pipelines, response relevance grew by 30%, while average query resolution time fell by 40%."}
                            </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Oct 2023 - Jun 2024"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}


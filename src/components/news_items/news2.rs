use stylist::style;
use yew::prelude::*;

#[function_component(News2)]
pub fn news2() -> Html {
    let container_style = style!(
        r#"
        display: grid;
        grid-template-columns: repeat(6, 1fr);
        gap: 1.5rem;
        width: 100%;
        margin-top: 1rem;
        "#
    ).unwrap();
    
    let column_main_style = style!(
        r#"
        display: flex; 
        flex-direction: column; 
        gap: 0.75rem;
        background: rgba(24, 24, 27, 0.02);
        padding: 0.75rem;
        border: 1px dashed rgba(24, 24, 27, 0.15);
        border-radius: 4px;
        "#
    ).unwrap();

    let tech_text_heading_style = style!(
        r#"
        font-size: 1.2rem;
        font-weight: 800;
        text-transform: uppercase;
        color: #18181b;
        font-family: 'Helvetica', 'Arial', sans-serif;
        "#
    ).unwrap();

    let tech_text_style = style!(
        r#"
        font-size: 0.75rem;
        font-weight: 700;
        text-transform: uppercase;
        color: #3f3f46;
        "#
    ).unwrap();

    let tech_subtext_style = style!(
        r#"
        font-size: 0.75rem;
        font-weight: 500;
        color: #71717a;
        line-height: 1.4;
        "#
    ).unwrap();
    
    let container = container_style.get_class_name().to_string();
    let column_main = column_main_style.get_class_name().to_string();
    let tech_text = tech_text_style.get_class_name().to_string();
    let tech_subtext = tech_subtext_style.get_class_name().to_string();
    let tech_text_heading = tech_text_heading_style.get_class_name().to_string();

    html! {
        <div>
            <div>
                <h1 style="font-size: 2rem; border-bottom: 2px solid #18181b; padding-bottom: 0.5rem; margin-bottom: 0.5rem;">
                    {"PRESSES & TYPE — TOOLS OF THE TRADE"}
                </h1>
            </div>
            <p style="font-size: 1.25rem; font-style: italic; margin-bottom: 1rem;">
                {"Sanjay N, an AI & Full Stack specialist, combines agentic AI orchestrations with robust, modern web stacks. Below are the tools he commands."}
            </p>
            
            <div class={container.clone()}>
                
                // Column 1: Python
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"Python"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Languages & AI"}</span>
                        <span class={tech_subtext.clone()}>{"Python, C#, JS, TS, SQL"}</span>
                        <span class={tech_subtext.clone()}>{"Java (Basics)"}</span>
                    </div>
                </div>

                // Column 2: Agentic AI
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M18 18.75a3 3 0 00-3-3h-5.25a3 3 0 00-3 3M12 4.5a3.75 3.75 0 100 7.5 3.75 3.75 0 000-7.5zM12 11.25V15m0 0v3.75m0-3.75h3.75M12 15H8.25" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"Agentic AI"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Frameworks"}</span>
                        <span class={tech_subtext.clone()}>{"LangChain"}</span>
                        <span class={tech_subtext.clone()}>{"LangGraph"}</span>
                        <span class={tech_subtext.clone()}>{"CrewAI, AutoGen"}</span>
                    </div>
                </div>

                // Column 3: ML & Data
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 6a7.5 7.5 0 107.5 7.5h-7.5V6z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 10.5H21A7.5 7.5 0 0013.5 3v7.5z" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"ML & Data"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Libraries"}</span>
                        <span class={tech_subtext.clone()}>{"NumPy, Pandas"}</span>
                        <span class={tech_subtext.clone()}>{"Scikit-learn"}</span>
                        <span class={tech_subtext.clone()}>{"TensorFlow, PyTorch"}</span>
                        <span class={tech_subtext.clone()}>{"OpenCV, Matplotlib"}</span>
                    </div>
                </div>

                // Column 4: Web Stack
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 01-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0115 18.257V17.25m6-12V15a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 15V5.25m18 0A2.25 2.25 0 0018.75 3H5.25A2.25 2.25 0 003 5.25m18 0V12a2.25 2.25 0 01-2.25 2.25H5.25A2.25 2.25 0 013 12V5.25" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"Web Stack"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Front & Back"}</span>
                        <span class={tech_subtext.clone()}>{"Node.js"}</span>
                        <span class={tech_subtext.clone()}>{"React.js"}</span>
                        <span class={tech_subtext.clone()}>{"HTML/CSS"}</span>
                    </div>
                </div>

                // Column 5: Infra & Tools
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.43l-1.003.828c-.293.241-.438.613-.43.992a7.723 7.723 0 010 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.43l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.991l-1.004-.827a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.28z" />
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"Infra"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Testing & APIs"}</span>
                        <span class={tech_subtext.clone()}>{"Git, RESTful APIs"}</span>
                        <span class={tech_subtext.clone()}>{"Postman, JMeter"}</span>
                        <span class={tech_subtext.clone()}>{"Zap, Lighthouse"}</span>
                        <span class={tech_subtext.clone()}>{"CI/CD"}</span>
                    </div>
                </div>

                // Column 6: Databases
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "items-center", "gap-2", "border-b", "border-zinc-700/25", "pb-2")}>
                        <svg class="w-5 h-5 text-themeOrange shrink-0" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125m16.5 0v3.75m-16.5-3.75v3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125" />
                        </svg>
                        <span class={tech_text_heading.clone()}>{"Databases"}</span>
                    </div>
                    <div class={classes!("flex", "flex-col", "gap-1")}>
                        <span class={tech_text.clone()}>{"Storage"}</span>
                        <span class={tech_subtext.clone()}>{"MySQL"}</span>
                        <span class={tech_subtext.clone()}>{"MongoDB"}</span>
                    </div>
                </div>

            </div>
        </div>
    }
}

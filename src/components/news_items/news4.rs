use stylist::style;
use yew::prelude::*;

#[function_component(News4)]
pub fn news4() -> Html {
    let container_style = style!(
        r#"
        display: flex; 
        gap: 0.5rem;
        "#
    ).unwrap();
    
    let column_left_style = style!(
        r#"
        flex-basis: 80%;
        text-align: justify;
        border-right: 2px solid #3f3f46; /* zinc-700 */
        padding-right: 0.5rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        "#
    ).unwrap();

    let column_main_style = style!(
        r#"
        flex-basis: 30%; 
        display: flex; 
        gap: 1.25rem;
        "#
    ).unwrap();
    
    let container = container_style.get_class_name().to_string();
    let column_left = column_left_style.get_class_name().to_string();
    let column_main = column_main_style.get_class_name().to_string();

    html! {
        <div id="career-desk">
            // Entry 1: Axodian
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">
                            <a href="https://www.axodian.com/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"AXODIAN"}</a>
                            {" APPOINTS FULL STACK DEVELOPER FOR GLOBAL TRADE TECH PLATFORMS"}
                        </h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p class="dropcap">
                            <span class="first-letter">{"A"}</span>
                            <a href="https://www.axodian.com/" target="_blank" style="color: #B93C12; font-weight: bold; text-decoration: underline;">{"xodian"}</a>
                            {" required advanced full stack infrastructure to support global trade pipelines."}
                        </p>
                        <p>
                            {"Since "}<strong>{"June 2026 to Present"}</strong>{", Sanjay N serves as a "}<strong>{"Full Stack Developer"}</strong>{", exploring technologies and optimizing systems related to international export and import procedures."}
                        </p>
                        <p>
                            {"He builds and maintains web applications, implements secure backend endpoints, and integrates trade logistics telemetry tracking systems to streamline operational compliance."}
                        </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Jun 2026 - Present"}</div>
                    </div>
                </div>
            </div>

            // Entry 2: Nexus Horizon / Faculties.ai
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">
                            <a href="https://www.faculties.ai/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"NEXUS HORIZON"}</a>
                            {" COMMISSIONS AI DEVELOPER FOR FACULTIES.AI OVERHAUL"}
                        </h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p class="dropcap">
                            <span class="first-letter">{"F"}</span>
                            <a href="https://www.faculties.ai/" target="_blank" style="color: #B93C12; font-weight: bold; text-decoration: underline;">{"aculties.ai"}</a>
                            {" required scalable, responsive AI-driven academic workflows."}
                        </p>
                        <p>
                            {"Between "}<strong>{"September 2025 and April 2026"}</strong>{", Sanjay N served as "}<strong>{"AI Developer & Tester"}</strong>{" at "}<strong>{"Nexus Horizon"}</strong>{" to deliver production-ready solutions."}
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

            // Entry 3: SNS Square
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">
                            <a href="https://www.snssquare.com/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"SNS SQUARE"}</a>
                            {" DEPLOYS FULL-STACK AI ACROSS THREE ENTERPRISE PLATFORMS"}
                        </h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p class="dropcap">
                            <span class="first-letter">{"M"}</span>
                            {"ultiple AI-based enterprise platforms needed full-stack development and precise requirement analysis."}
                        </p>
                        <p>
                            {"From "}<strong>{"August 2024 to September 2025"}</strong>{", as a "}<strong>{"Full Stack AI Developer & Tester"}</strong>{" at "}<strong><a href="https://www.snssquare.com/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"SNS Square"}</a></strong>{", he built modules, implemented AI logic, and ran testing aligned to business needs across three key projects: "}<em>{"AI Exam Analyzer, Gen AI Suite, and Aggregator"}</em>{"."}
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

            // Entry 4: Cognifyz
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">
                            <a href="https://cognifyz.com/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"COGNIFYZ TECHNOLOGIES"}</a>
                            {" TRIALS FRAUD-DETECTION, RAG-POWERED BOTS"}
                        </h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p class="dropcap">
                            <span class="first-letter">{"S"}</span>
                            {"ystems required intelligent automation and sharper AI response accuracy."}
                        </p>
                        <p>
                            {"During his tenure from "}<strong>{"October 2023 to June 2024"}</strong>{" as an "}<strong>{"AIML Engineer"}</strong>{" at "}<strong><a href="https://cognifyz.com/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"Cognifyz Technologies"}</a></strong>{", Sanjay N developed and optimized models for personalization, fraud detection, and knowledge retrieval — building recommendation models and training small language models for projects like "}<em>{"Electro Bot"}</em>{" and "}<em>{"Collexa.ai"}</em>{"."}
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

            // Entry 5: Mindful AI
            <div class={container.clone()}>
                <div class={column_left.clone()}>
                    <div>
                        <h1 style="font-size: 2rem;">
                            <a href="https://www.mindfulai.co.in/about/" target="_blank" style="color: #B93C12; text-decoration: underline;">{"MINDFUL AI"}</a>
                            {" INDUCTS AI ENGINEER TRAINEE FOR MODEL DEVELOPMENT & PORTAL INTEGRATION"}
                        </h1>
                    </div>

                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p class="dropcap">
                            <span class="first-letter">{"M"}</span>
                            <a href="https://www.mindfulai.co.in/about/" target="_blank" style="color: #B93C12; font-weight: bold; text-decoration: underline;">{"indful AI"}</a>
                            {" required optimization in predictive model pipelines and third-party web interface integrations."}
                        </p>
                        <p>
                            {"From "}<strong>{"June 2023 to October 2023"}</strong>{", he served as an "}<strong>{"AI Engineer Trainee"}</strong>{". In this capacity, he designed models and developed AI integration endpoints to connect AI intelligence across numerous client websites."}
                        </p>
                        <p>
                            {"He optimized feature engineering methodologies to increase model prediction speeds and implemented secure API protocols, establishing smooth data routing flows to remote sites."}
                        </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex","justify-center","items-center","text-center","w-full")}>
                        <div>{"TIMELINE"}<br />{"Jun 2023 - Oct 2023"}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}

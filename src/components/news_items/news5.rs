use yew::prelude::*;
use stylist::style;

#[function_component(News5)]
pub fn news5() -> Html {
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
            <h1 style="font-size: 2rem;text-align: center;">{"FROM THE LAB OF SANJAY N"}</h1>
            <div class={container.clone()}>
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl","mb-1")}>
                        {"EXAM PAPER ANALYZER"}
                    </h1>
                    <p>{"AI-Powered Evaluation Engine"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"E"}</span>{"valuation workflows optimized using agentic vision models."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Python, PyTorch, LangChain"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Developed an automated solution to digitize, parse, and score physical exam scripts. This AI pipeline extracted written answers, cross-referenced grading rubrics, and reduced standard evaluation time by 30%."}</p>
                         <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/RNS-Forge/RNS_Exam-Papper-Analyzer" target="_blank">{" GitHub Repository [4]"}</a>
                         </p>
                    </div>
                </div>
            </div>

            // Section 2
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div><h1 class={classes!("text-3xl","mb-1")}>{"LOAN ELIGIBILITY CHECKER"}</h1><p>{"AI-Based Approval System"}</p></div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"L"}</span>{"everaging predictive intelligence to assess credit risk profiles."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"🛠️ Tech Stack: Python, Scikit-learn, FastAPI"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}>{"Built a deployment-ready screening system assessing demographic and credit parameters. Decreased manual verification queues by 12% and lifted initial screening accuracy by 15%."}</p>
                        <p class={classes!("mt-2","mb-2")}>
                            <a href="https://github.com/RNS-Forge">{" GitHub Repository [5]"}</a>
                        </p>

                    </div>
                </div>
            </div>

            // Section 3
            <div class={classes!("flex-1", "flex", "flex-col", "gap-2")}>
                <div>
                    <h1 class={classes!("text-4xl")}>
                        {"HONORS DESK"}
                    </h1>
                    <p>{"Academic & Professional Credentials"}</p>
                </div>
                <div class={classes!("flex", "gap-3", "text-sm")}>
                    <div class={classes!("flex-1", "flex", "flex-col", "gap-1")}>
                        <p class={classes!("mt-1","mb-1")}><span class="high">{"P"}</span>{"rofessional badges and validation certificates."}</p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"• Salesforce AI Associate & Agentforce Specialist (80%+)"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"• Oracle AI Foundations Associate (92%)"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"• Postman AI Student Expert | NPTEL IoT 4.0"}</strong></p>
                        <p class={classes!("mt-1","mb-1")}><strong>{"• NASSCOM Digital Edge (81%)"}</strong></p>
                    </div>
                </div>
            </div>

            
            </div>
        </div>
    }
}


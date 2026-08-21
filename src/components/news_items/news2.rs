use stylist::style;

use yew::prelude::*;

#[function_component(News2)]
pub fn news2() -> Html {
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
            flex-basis: 80%; 
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
            <div>
                <h1 style="font-size: 2rem;">{"PRESSES & TYPE — TOOLS OF THE TRADE"}</h1>
            </div>
            <p class={classes!("")} style="font-size: 1.5rem;"> {"Sanjay N, an AI & Full Stack specialist, combines agentic AI orchestrations with robust, modern web stacks. Below are the tools he commands."} </p>
            <div class={container.clone()}>
                
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <img src="https://raw.githubusercontent.com/devicons/devicon/master/icons/python/python-original.svg" alt="Python Logo" class={tech_img_heading.clone()} />
                                <span class={tech_text_heading.clone()}>{"Python"}</span> 
                            </p>
                            
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Languages & AI:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Python, C#, JS, TS, SQL"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Java (Basics)"}</span> 
                            </p>
                        </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <span class={tech_text_heading.clone()}>{"Agentic AI"}</span> 
                            </p>
                            <p class={classes!("flex")}>           
                                <span class={tech_text.clone()}>{"Frameworks:"}</span> 
                            </p>

                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"LangChain"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"LangGraph"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"CrewAI, AutoGen"}</span> 
                            </p>
                        </p>
                    </div>
                </div>
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <span class={tech_text_heading.clone()}>{"ML & Data"}</span> 
                            </p>
                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Libraries:"}</span> 
                            </p>

                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"NumPy, Pandas"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Scikit-learn"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"TensorFlow, PyTorch"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"OpenCV, Matplotlib"}</span> 
                            </p>
                        </p>
                    </div>
                </div> 
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <span class={tech_text_heading.clone()}>{"Web Stack"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Front & Back:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Node.js"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"React.js"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"HTML/CSS"}</span> 
                            </p>
                        </p>
                    </div>
                </div>           
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <span class={tech_text_heading.clone()}>{"Infra / Tools"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Testing & APIs:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Git, RESTful APIs"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Postman, JMeter"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"Zap, Lighthouse"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"CI/CD"}</span> 
                            </p>
                        </p>
                    </div>
                </div>                       
                <div class={column_main.clone()}>
                    <div class={classes!("flex", "flex-col", "gap-0.5")}>
                        <p>
                            <p class={classes!("flex","justify-center","items-center")}>
                                <span class={tech_text_heading.clone()}>{"Databases"}</span>
                            </p>

                            <p class={classes!("flex")}>
                                <span class={tech_text.clone()}>{"Storage:"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"MySQL"}</span> 
                            </p>
                            <p class={classes!("flex","ml-5")}>
                                <span class={tech_text.clone()}>{"MongoDB"}</span> 
                            </p>
                        </p>
                    </div>
                </div>                       
             
            </div>
        </div>
    }
}


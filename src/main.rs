use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct AnswerListProps {
    answers: Vec<String>,
    on_add: Callback<String>,
}

#[function_component(AnswerList)]
fn answer_list(props: &AnswerListProps) -> Html {
    props
        .answers
        .iter()
        .map(|answer| {
            html! {
                <li>{ answer }</li>
            }
        })
        .collect()
}

#[function_component(App)]
fn app() -> Html {
    let answers = use_state(Vec::<String>::new);
    let input_value = use_state(String::new);

    let onclick = {
        let answers = answers.clone();
        let input_value = input_value.clone();
        Callback::from(move |_: MouseEvent| {
            let mut new_answers = (*answers).clone();
            new_answers.push((*input_value).clone());
            answers.set(new_answers);
            input_value.set(String::new());
        })
    };

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let value = e
                .target_unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            input_value.set(value);
        })
    };

    let onkeydown = {
        let onclick = onclick.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                onclick.emit(MouseEvent::new("click").unwrap());
            }
        })
    };

    html! {
        <>
            <p>{ "Once you have identified something something something lorem ipsum dolor sit amet" }</p>
            <div class="level-container">
            <h1>{"Level 1"}</h1>
            <h2>{"Minimal level"}</h2>
            <AnswerList answers={(*answers).clone()} on_add={Callback::from(move |answer| {
                let mut new_answers = (*answers).clone();
                new_answers.push(answer);
                answers.set(new_answers);
            })} />
            <input type="text" value={(*input_value).clone()} id="answer-input" oninput={oninput} onkeydown={onkeydown}/>
            <button type="button" onclick={onclick}>{ "Add" }</button>
            </div>
            <div class="level container">
            <h1>{"Level 2"}</h1>
            <h2>{"Normal level"}</h2>
            <input type="text" value={(*input_value).clone()} id="answer-input" />
            <button type="button">{ "Add" }</button>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

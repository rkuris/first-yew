use log::info;
use yew::prelude::*;

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
pub enum Msg {
    AddAnswer(String),
    DeleteAnswerById(usize),
    UpdateInput(String),
}

#[derive(PartialEq, Clone)]
struct Answer {
    id: usize,
    text: Rc<str>,
}

static ID: AtomicUsize = AtomicUsize::new(0);

impl<T: AsRef<str>> From<T> for Answer {
    fn from(text: T) -> Self {
        Answer {
            id: ID.fetch_add(1, Ordering::Relaxed),
            text: Rc::from(text.as_ref()),
        }
    }
}
#[derive(Properties, PartialEq)]
struct App {
    answers: Vec<Answer>,
    input_value: String,
}

#[derive(Properties, PartialEq)]
struct Props {
    question: String,
    answer: String,
}

#[derive(Properties, PartialEq)]
struct AnswerListProps {
    answers: Vec<Answer>,
    on_add: Callback<String>,
    on_delete: Callback<usize>,
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            answers: Vec::new(),
            input_value: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddAnswer(answer) => {
                info!("Added answer: {}", answer);
                self.answers.push(answer.into());
                self.input_value.clear();
                true
            }
            Msg::DeleteAnswerById(id) => {
                if let Some(pos) = self.answers.iter().position(|answer| answer.id == id) {
                    self.answers.remove(pos);
                    true
                } else {
                    false
                }
            }
            Msg::UpdateInput(value) => {
                self.input_value = value;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let question = ctx.props().question.clone();
        let input_value = self.input_value.clone();
        let input_value_for_keypress = input_value.clone();

        html! {
            <div class="container">
                <p>{ question }</p>
                <div class="level-container">
                    <h1>{"Level 1"}</h1>
                    <h2>{"Minimal level"}</h2>
                    <AnswerList
                        answers={self.answers.clone()}
                        on_add={ctx.link().callback(Msg::AddAnswer)}
                        on_delete={ctx.link().callback(Msg::DeleteAnswerById)}
                    />
                    <input
                        type="text"
                        value={self.input_value.clone()}
                        id="answer-input"
                        oninput={ctx.link().callback(|e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            Msg::UpdateInput(value)
                        })}
                        onkeydown={ctx.link().callback(move |e: KeyboardEvent| {
                            if e.key() == "Enter" {
                                Msg::AddAnswer(input_value_for_keypress.clone())
                            } else {
                                Msg::UpdateInput(input_value_for_keypress.clone())
                            }
                        })}
                    />
                    <button type="button" onclick={ctx.link().callback(move |_| Msg::AddAnswer(input_value.clone()))}>{ "Add" }</button>
                </div>
            </div>
        }
    }
}

#[function_component(AnswerList)]
fn answer_list(props: &AnswerListProps) -> Html {
    let on_delete = props.on_delete.clone();
    props
        .answers
        .iter()
        .map(|answer| {
            let id = answer.id;
            html! {
                <p>{ answer.text.clone() }<span class="delete-icon" onclick={on_delete.reform(move |_| id)}>{ "❌" }</span></p>
            }
        })
        .collect()
}

#[function_component(Root)]
fn root() -> Html {
    let input_value = use_state(String::new);

    html! {
        <App question={"Once you have identified the cause of your stress, enter the items on this form".to_string()} answer={(*input_value).clone()} />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::Renderer::<Root>::new().render();
}

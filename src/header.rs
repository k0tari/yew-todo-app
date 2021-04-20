use std::mem;

use yew::prelude::*;
pub struct HeaderComponent {
    link: ComponentLink<Self>,
    props: Props,
    input_value: String,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_submit: Callback<String>,
}

pub enum Msg {
    Input(String),
    Submit,
}

impl Component for HeaderComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: _props,
            input_value: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => self.input_value = value,
            Msg::Submit => {
                let text = mem::replace(&mut self.input_value, "".to_string());
                self.props.on_submit.emit(text);
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onkeyup = self.link.batch_callback(move |e: KeyboardEvent| {
            e.stop_propagation();
            if e.key() == "Enter" {
                Some(Msg::Submit)
            } else {
                None
            }
        });
        html! {
            <header class="header">
                <h1>{"todos"}</h1>
                <input class="new-todo"
                       value=self.input_value
                       placeholder="What needs to be done?"
                       oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                       onkeyup=onkeyup />
            </header>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

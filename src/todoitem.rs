use serde::{Deserialize, Serialize};
use yew::{prelude::*, web_sys::HtmlInputElement};
use yewtil::*;
#[derive(Properties, Clone, PartialEq, Default, Debug, Deserialize, Serialize)]
pub struct TodoItem {
    pub content: String,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(content: String) -> Self {
        TodoItem {
            content,
            completed: false,
        }
    }
}
pub struct TodoItemComponent {
    link: ComponentLink<Self>,
    props: Props,
    is_editing: bool,
    node_ref: NodeRef,
}

pub enum Msg {
    Delete,
    Edit,
    Focus,
    Submit,
}
#[derive(Properties, Clone, PartialEq, Default)]
pub struct Props {
    pub id: usize,
    pub todoitem: TodoItem,
    pub on_edit: Callback<(usize, String)>,
    pub on_toggle: Callback<usize>,
    pub on_destroy: Callback<usize>,
}
impl Component for TodoItemComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props: _props,
            is_editing: false,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Delete => {
                self.props.on_destroy.emit(self.props.id);
            }
            Msg::Edit => {
                self.is_editing = true;
            }
            Msg::Focus => {
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                    input.focus().unwrap();
                }
            }
            Msg::Submit => {
                if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
                    self.props.on_edit.emit((self.props.id, input.value()));
                    self.is_editing = false;
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let id = self.props.id;

        let mut classes = vec![];
        if self.props.todoitem.completed {
            classes.push("completed");
        }
        if self.is_editing {
            classes.push("editing")
        }

        let onkeyup = self.link.batch_callback(move |e: KeyboardEvent| {
            e.stop_propagation();
            if e.key() == "Enter" || e.key() == "Escape" {
                Some(Msg::Submit)
            } else {
                None
            }
        });

        html! {
            <li class=classes!(classes)>
                <div class="view">
                    <input
                        class="toggle"
                        type="checkbox"
                        checked=self.props.todoitem.completed
                        onchange=self.props.on_toggle.reform(move |_| id) />
                    <label ondblclick=self.link.callback(|_| Msg::Edit) >
                        { &self.props.todoitem.content }
                    </label>
                    <button
                        class="destroy"
                        onclick=self.link.callback(|_| Msg::Delete) />
                </div>
                <input
                    class="edit"
                    ref=self.node_ref.clone()
                    value=self.props.todoitem.content
                    onmouseover=self.link.callback(|_| Msg::Focus)
                    onkeyup=onkeyup
                    onblur=self.link.callback(|_| Msg::Submit) />
            </li>
        }
    }
}

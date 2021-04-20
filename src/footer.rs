use crate::Filter;

use yewtil::*;

use yew::prelude::*;
pub struct FooterComponent {
    props: Props,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub cnt_active: usize,
    pub on_clear_completed: Callback<()>,
    pub on_change_filter: Callback<Filter>,
}

impl Component for FooterComponent {
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props: _props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let on_clear_completed = self.props.on_clear_completed.reform(|_| ());
        let on_change_filter = &self.props.on_change_filter;
        html! {
            <footer class="footer">
                <span class="todo-count">{format!("{} items left", self.props.cnt_active)}</span>
                <ul class="filters">
                    <li>
                        <a
                            href="#/"
                            class="selected"
                            onclick=on_change_filter.reform(|_| Filter::All)>
                            {"All"}
                        </a>
                    </li>
                    <li>
                        <a
                            href="#/active"
                            class="selected"
                            onclick=on_change_filter.reform(|_| Filter::Active)>
                            {"Active"}
                        </a>
                    </li>
                    <li>
                        <a
                            href="#/completed"
                            class="selected"
                            onclick=on_change_filter.reform(|_| Filter::Completed)>
                            {"completed"}
                        </a>
                    </li>
                </ul>
                <button class="clear-completed" onclick=on_clear_completed>{"Clear completed"}</button>
            </footer>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

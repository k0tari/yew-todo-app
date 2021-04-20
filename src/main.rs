mod footer;
mod header;
mod todoitem;

use footer::FooterComponent;
use header::HeaderComponent;
use todoitem::{TodoItem, TodoItemComponent};
use yew::format::Json;
use yew::prelude::*;
use yew_services::storage::Area;
use yew_services::StorageService;

#[derive(Debug)]
pub enum Filter {
    All,
    Active,
    Completed,
}
pub enum Msg {
    AddTodo(String),
    Destroy(usize),
    ToggleCompleted(usize),
    ToggleAllCompleted,
    UpdateContent(usize, String),
    ClearCompleted,
    ChangeFilter(Filter),
    Save,
}

#[derive(Properties, Clone, PartialEq, Default)]
struct Props {
    todolist: Vec<TodoItem>,
}
struct Model {
    link: ComponentLink<Self>,
    todolist: Vec<TodoItem>,
    filter: Filter,
    storage: StorageService,
}

const KEY: &str = "yew-todo-app";

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        let Json(todolist) = storage.restore(KEY);
        let todolist = todolist.ok().unwrap_or_else(Vec::new);
        Self {
            link,
            todolist,
            filter: Filter::All,
            storage,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo(content) => {
                self.todolist.push(TodoItem::new(content));
                self.link.send_message(Msg::Save);
                true
            }
            Msg::Destroy(id) => {
                self.todolist.remove(id);
                self.link.send_message(Msg::Save);
                true
            }
            Msg::Save => {
                self.storage.store(KEY, Json(&self.todolist));
                false
            }
            Msg::ToggleCompleted(id) => {
                if let Some(todo) = self.todolist.get_mut(id) {
                    todo.completed = !todo.completed;
                    self.link.send_message(Msg::Save);
                    true
                } else {
                    false
                }
            }
            Msg::UpdateContent(id, content) => {
                if let Some(todo) = self.todolist.get_mut(id) {
                    todo.content = content;
                    self.link.send_message(Msg::Save);
                    true
                } else {
                    false
                }
            }
            Msg::ToggleAllCompleted => {
                let checked = self.todolist.iter().all(|e| e.completed);
                if checked {
                    self.todolist.iter_mut().for_each(|e| e.completed = false);
                } else {
                    self.todolist.iter_mut().for_each(|e| e.completed = true);
                }
                true
            }
            Msg::ClearCompleted => {
                self.todolist = self
                    .todolist
                    .drain(..)
                    .filter(|e| !e.completed)
                    .collect::<Vec<_>>();
                true
            }
            Msg::ChangeFilter(filter) => {
                self.filter = filter;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let on_submit = self.link.callback(|content: String| Msg::AddTodo(content));
        let render_item = |(id, todoitem): (usize, &TodoItem)| {
            html! {
                    <TodoItemComponent
                                    id=id
                                    todoitem=todoitem
                                    on_edit=self.link.callback(|(id, content)| Msg::UpdateContent(id, content))
                                    on_toggle=self.link.callback(|id| Msg::ToggleCompleted(id))
                                    on_destroy=self.link.callback(|id| Msg::Destroy(id)) />
            }
        };
        let shown_todos = self.todolist.iter().filter(|t| match self.filter {
            Filter::Active => !t.completed,
            Filter::Completed => t.completed,
            Filter::All => true,
        });
        html! {
            <section class="todoapp">
                <HeaderComponent on_submit=on_submit/>
                <section class="main">
                    <input
                        id="toggle-all"
                        class="toggle-all"
                        type="checkbox"
                        checked={ self.todolist.iter().all(|t| t.completed) }
                        onchange=self.link.callback(|_| Msg::ToggleAllCompleted) />
                    <label for="toggle-all"/>
                    <ul class="todo-list">
                        { shown_todos.enumerate().map(render_item).collect::<Html>() }
                    </ul>
                </section>
                <FooterComponent
                    cnt_active={ self.todolist.iter().filter(|t| !t.completed).count() }
                    on_clear_completed=self.link.callback(|()| Msg::ClearCompleted)
                    on_change_filter=self.link.callback(Msg::ChangeFilter) />
            </section>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

use yew::prelude::*;

pub struct App {
    todos: Vec<String>,
    input_ref: NodeRef,
}

impl App {
    fn add_todo(&mut self, text: String) {
        if text.is_empty() {
            return;
        }
        self.todos.push(text);
    }
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            todos: vec![],
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::AddTodo(text) => {
                self.input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .set_value("");
                self.add_todo(text);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_ref = self.input_ref.clone();
        let on_click = ctx
            .link()
            .callback(move |_| Msg::AddTodo(input_value(&input_ref)));
        let input_ref = self.input_ref.clone();
        let on_key_press = ctx.link().callback(move |e: KeyboardEvent| {
            // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/keyCode
            if e.key_code() == 0x0D {
                // 0x0D is the code for the Enter key
                Msg::AddTodo(input_value(&input_ref))
            } else {
                Msg::None
            }
        });
        html! {
            <div>
                <h1>{"Todo Web"}</h1>
                <p>{"This is a web app to manage your todo list."}</p>
                <ul>
                    {
                        for self.todos.iter().map(|todo| {
                            html! {
                                <li>{todo}</li>
                            }
                        })
                    }
                </ul>
                <div>
                    <input ref={ self.input_ref.clone() } placeholder="What needs to be done?" onkeypress={ on_key_press } />
                    // <input ref={ self.input_ref.clone() } placeholder="What needs to be done?" />
                    <button onclick={ on_click }> {"Add Todo"} </button>
                </div>
            </div>
        }
    }
}

fn input_value(input_ref: &NodeRef) -> String {
    input_ref
        .cast::<web_sys::HtmlInputElement>()
        .unwrap()
        .value()
}

pub enum Msg {
    None,
    AddTodo(String),
}

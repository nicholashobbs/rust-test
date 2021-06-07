use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;

const ENTER_KEY: &str = "Enter";

#[derive(Default)]
struct Model {
    items: Vec<Todo>,
    error: Option<String>,
    new_todo_title: String,
}

struct Todo {
    value: String,
    completed: Bool,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<Todo>>),
    CreateTodo,
    TodoChanged(String),
    ClearAll,
    RemoveTodo,
}


fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        Msg::FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        }

        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            let new_todo = Todo { value: title.to_owned(), completed: False };
            if not(title.is_empty()) {
                model.items.append(new_todo);
            }
        }
        Msg::TodoChanged(title) => {
            model.new_todo_title = title;
        }
        Msg::ClearAll => {
          model.items.clear();
        }

        Msg::RemoveTodo => {
            model.items.clear();
          }
    }
    
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        view_input(&model.new_todo_title),
        view_main(model)
    ]
}

// ------ header ------

fn view_input(new_todo_title: &str) -> Node<Msg> {
    header![C!["header"],
        h1!["todos"],
        input![C!["new-todo"],
            attrs!{
                At::Placeholder => "What needs to be done?", 
                At::AutoFocus => AtValue::None,
                At::Value => new_todo_title}     
        ],
        //button!["save"],
        button![
            "save", 
            ev(Ev::Click, |_| Msg::CreateTodo),
        ],
        button![
            "clear all",
            ev(Ev::Click, |_| Msg::ClearAll),],
        input_ev(Ev::Input, Msg::TodoChanged),
        keyboard_ev(Ev::KeyDown, |keyboard_event| {
            IF!(keyboard_event.key() == ENTER_KEY => Msg::CreateTodo)
        }),
    ]
}

// ------ main ------

fn view_main(model: &Model) -> Node<Msg> {
    div![
        ul![
            model.items.iter().map(|item| {
                li![item.value, "1234"
                ]
            })
        ]
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<Todo>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

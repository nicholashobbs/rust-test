use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;

const ENTER_KEY: &str = "Enter";

#[derive(Default)]
struct Model {
    items: Vec<String>,
    error: Option<String>,
    new_todo_title: String,
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    CreateTodo,
    TodoChanged(String),
}


fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        }
        Msg::CreateTodo => {
            let title = model.new_todo_title.trim();
            if not(title.is_empty()) {
                model.items.insert(0,title.to_owned());
            }
        }
        Msg::TodoChanged(title) => {
            model.new_todo_title = title;
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
                At::Value => new_todo_title},     
        ],
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
                li![item]
            })
        ]
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
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

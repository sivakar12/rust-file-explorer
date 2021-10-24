use iced::{Column, Row, Text, Button, Sandbox, Settings, Element};
use std::path::{PathBuf};

mod utils;
pub enum Node {
    Folder(PathBuf),
    File(PathBuf),
}
impl Node {
    fn get_name(&self) -> &str {
        match self {
            Node::File(p) => p.to_str().unwrap(),
            Node::Folder(p) => p.to_str().unwrap()
        }
    }
}



fn display_nodes<'a> (nodes: &'a Vec<Node>) -> Element<Message> {
    let mut column = Column::new();
    for node in nodes {
        column = column.push(Text::new(node.get_name()));
    }
    column.into()
    
}

#[derive(Default)]
struct FileExplorer {
    path: PathBuf,
    loading: bool,
    files: Vec<Node>,
    up_button: iced::button::State
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    OpenNode(usize),
    GoUp,
}

impl Sandbox for FileExplorer {
    type Message = Message;

    fn new() -> FileExplorer {
        let home_dir = utils::get_home_directory().unwrap();
        let files = utils::get_nodes_in_path(&home_dir).unwrap();
        FileExplorer { 
            path: home_dir.clone(), 
            loading: false, 
            files: files, 
            up_button: iced::button::State::new() 
        }
    }
    fn title(&self) -> String {
        String::from("Rust File Explorer")
    }
    fn view(&mut self) -> Element<Message> {
        Column::new()
            .push(
                Button::new(&mut self.up_button, Text::new("up"))
                    .on_press(Message::GoUp)
            )
            .push(
                Row::new().push(display_nodes(&self.files))
            )
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenNode(i) => {
                // self.path += self.files[i].get_path();
            },
            Message::GoUp => {
                self.path = self.path.parent().unwrap().into();
                self.files = utils::get_nodes_in_path(&self.path).unwrap();
            }
        }
    }
}



fn main() -> iced::Result {
    FileExplorer::run(Settings::default())
}
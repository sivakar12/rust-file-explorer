use iced::{Column, Row, Text, Button, Sandbox, Settings, Element};

enum Node {
    Folder(String),
    File(String),
}
impl Node {
    pub fn get_path(&self) -> &str {
        match self {
            Node::Folder(path) => &path,
            Node::File(path) => &path 
        }
    }
}

#[derive(Default)]
struct FileExplorer {
    path: String,
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
        Self::default()
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
                Row::new()
            )
            .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OpenNode(i) => {
                self.path += self.files[i].get_path();
            },
            Message::GoUp => {

            }
        }
    }
}



fn main() -> iced::Result {
    FileExplorer::run(Settings::default())
}
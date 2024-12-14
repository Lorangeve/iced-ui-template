use std::thread::sleep;

use iced::{
    widget::{button, column, row, text},
    window, Element, Font, Task,
};

#[derive(Default)]
struct App {
    // 路由
    router: Router,
    // 状态
    state: State,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            App {
                router: Router::default(),
                state: State::default(),
            },
            Task::perform(async {}, |_| {
                // 需要等待的操作放在这里，会显示一个loading界面
                sleep(std::time::Duration::from_millis(1000));
                Message::RouterChanged(Router::Home)
            }),
        )
    }

    fn update(&mut self, message: Message) {
        #[cfg(debug_assertions)]
        println!("Message: {:?}, Router: {:?}", message.clone(), &self.router);

        match message {
            Message::RouterChanged(router) => {
                self.router = router;
            }
            Message::CounterIncremented => self.state.count += 1,
            Message::WindowResized(..) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        match self.router {
            Router::Loadding => text("Loading...").into(),
            Router::Home => column![
                text("Hello, Iced!").size(32),
                text("你好，Iced!").size(32),
                row![button("Go to Counter").on_press(Message::RouterChanged(Router::Counter)),]
                    .align_y(iced::Alignment::Center)
            ]
            .into(),
            Router::Counter => row![
                text(format!("Count: {}", self.state.count)),
                button("Increment/增加").on_press(Message::CounterIncremented),
                button("BackHome/返回").on_press(Message::RouterChanged(Router::Home))
            ]
            .align_y(iced::Alignment::Center)
            .into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
enum Router {
    #[default]
    Loadding,
    Home,
    Counter,
}

#[derive(Debug, Clone, Default)]
struct State {
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    RouterChanged(Router),
    CounterIncremented,
    WindowResized(f32, f32),
}

fn main() -> iced::Result {
    iced::application("hello_iced", App::update, App::view)
        .default_font(Font::with_name("Microsoft YaHei"))
        .subscription(|app| {
            window::resize_events().map(|e| Message::WindowResized(e.1.width, e.1.height))
        })
        .window_size(iced::Size::new(350.0, 170.0))
        .run_with(App::new)
}

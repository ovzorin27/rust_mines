use iced::{button, executor, time, Align, Application, Button, Column, Command, Container, Element, HorizontalAlignment, Length, Row, Settings, Subscription, Text, Clipboard, VerticalAlignment};
use std::time::{Duration, Instant};

const N: usize = 5;

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    duration: Duration,
    state: State,
    toggle: button::State,
    reset: button::State,
    buttons: Vec<button::State>,
    cells: Vec<Cell>,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Clone)]
struct Cell {
    is_opened: bool,
    state: CellState,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            is_opened: false,
            state: CellState::MinesAround(0),
        }
    }
}

#[derive(Clone)]
enum CellState {
    Mine,
    MinesAround(i32),
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
    Pressed(usize),
}

impl Application for Stopwatch {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                duration: Duration::default(),
                state: State::Idle,
                toggle: button::State::new(),
                reset: button::State::new(),
                buttons: Vec::from([button::State::new(); N * N]),
                cells: generate_cells(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Stopwatch - Iced")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => match &mut self.state {
                State::Ticking { last_tick } => {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                _ => {}
            },
            Message::Reset => {
                self.duration = Duration::default();
            },
            Message::Pressed(index) => {
                let cell = self.cells.get_mut(index).unwrap();
                open_empty_cells(index, &mut self.cells);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => {
                time::every(Duration::from_millis(10)).map(Message::Tick)
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // const MINUTE: u64 = 60;
        // const HOUR: u64 = 60 * MINUTE;
        //
        // let seconds = self.duration.as_secs();
        //
        // let duration = Text::new(format!(
        //     "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
        //     seconds / HOUR,
        //     (seconds % HOUR) / MINUTE,
        //     seconds % MINUTE,
        //     self.duration.subsec_millis() / 10,
        // ))
        //     .size(40);
        //
        // let button = |state, label, style| {
        //     Button::new(
        //         state,
        //         Text::new(label)
        //             .horizontal_alignment(HorizontalAlignment::Center),
        //     )
        //         .min_width(80)
        //         .padding(10)
        //         .style(style)
        // };
        //
        // let toggle_button = {
        //     let (label, color) = match self.state {
        //         State::Idle => ("Start", style::Button::Primary),
        //         State::Ticking { .. } => ("Stop", style::Button::Destructive),
        //     };
        //
        //     button(&mut self.toggle, label, color).on_press(Message::Toggle)
        // };
        //
        // let reset_button =
        //     button(&mut self.reset, "Reset", style::Button::Secondary)
        //         .on_press(Message::Reset);
        //
        // let controls = Row::new()
        //     .spacing(20)
        //     .push(toggle_button)
        //     .push(reset_button);
        //
        // let content = Column::new()
        //     .align_items(Align::Center)
        //     .spacing(20)
        //     .push(duration)
        //     .push(controls);
        //
        // Container::new(content)
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        //     .center_x()
        //     .center_y()
        //     .into()

        let mut row  = Row::new()
            .spacing(0);
        let mut col = Column::new()
            .align_items(Align::Center)
            .spacing(4);
        for (index, button) in self.buttons.iter_mut().enumerate() {
            let (_, j) = (index / N, index % N);

            if j == 0 {
                col = col.push(row);
                row  = Row::new()
                    .spacing(4);
            }

            let cell = self.cells.get(index).unwrap();
            if cell.is_opened {
                let text;
                match cell.state {
                    CellState::Mine => {
                        text = format!("X");
                    }
                    CellState::MinesAround(count) => {
                        text = format!("{}", count);
                    }
                }
                let text_view = Text::new(text)
                    .size(28)
                    .width(Length::Units(40))
                    .height(Length::Units(40))
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center)
                    .color(iced::Color::from_rgb(0.5, 0.5, 0.5),);
                row = row.push(text_view);
            } else {
                let button_view = Button::new(
                    button,
                    Text::new(format!(""))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                    .min_width(40)
                    .min_height(40)
                    .padding(10)
                    .on_press(Message::Pressed(index));
                row = row.push(button_view);
            }
        }
        col = col.push(row);

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn generate_cells() -> Vec<Cell> {

    fn inc_mines_count(i: i32, j: i32, cells: &mut [Cell]) {
        if i < 0 || i >= N as i32 || j < 0 || j >= N as i32 {
            return;
        }

        let index = (i * N as i32 + j) as usize;
        let cell: &mut Cell = cells.get_mut(index).unwrap();
        match cell.state {
            CellState::Mine => {}
            CellState::MinesAround(count) => {
                cell.state = CellState::MinesAround(count + 1);
            }
        }
    }

    let mut cells: Vec<Cell> = std::iter::repeat(Cell::default()).take(N * N).collect();
    for _ in 0..N {
        loop {
            let mine_index = rand::random::<usize>() % (N * N);
            let cell = cells.get_mut(mine_index).unwrap();
            match cell.state {
                CellState::Mine => continue,
                CellState::MinesAround(_) => {
                    cell.state = CellState::Mine;

                    let (i, j) = ((mine_index / N) as i32, (mine_index % N) as i32);
                    inc_mines_count(i - 1, j, &mut cells);
                    inc_mines_count(i - 1, j + 1, &mut cells);
                    inc_mines_count(i, j + 1, &mut cells);
                    inc_mines_count(i + 1, j + 1, &mut cells);
                    inc_mines_count(i + 1, j, &mut cells);
                    inc_mines_count(i + 1, j - 1, &mut cells);
                    inc_mines_count(i, j - 1, &mut cells);
                    inc_mines_count(i - 1, j - 1, &mut cells);

                    break;
                }
            }
        }
    }

    cells
}

fn open_empty_cells(index: usize, cells: &mut [Cell]) {
    fn open_empty_cells_recursive(i: i32, j: i32, cells: &mut [Cell]) {
        if i < 0 || i >= N as i32 || j < 0 || j >= N as i32 {
            return;
        }

        let index = (i * N as i32 + j) as usize;
        let cell: &mut Cell = cells.get_mut(index).unwrap();
        if cell.is_opened {
            return;
        } else {
            cell.is_opened = true;
        }
        match cell.state {
            CellState::Mine => {}
            CellState::MinesAround(count) if count == 0 => {
                open_empty_cells_recursive(i - 1, j, cells);
                open_empty_cells_recursive(i - 1, j + 1, cells);
                open_empty_cells_recursive(i, j + 1, cells);
                open_empty_cells_recursive(i + 1, j + 1, cells);
                open_empty_cells_recursive(i + 1, j, cells);
                open_empty_cells_recursive(i + 1, j - 1, cells);
                open_empty_cells_recursive(i, j - 1, cells);
                open_empty_cells_recursive(i - 1, j - 1, cells);
            }
            CellState::MinesAround(_) => {}
        }
    }

    let (i, j) = ((index / N) as i32, (index % N) as i32);
    open_empty_cells_recursive(i, j, cells);
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
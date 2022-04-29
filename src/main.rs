use iced::{Color, button, executor, time, Align, Application, Button, Column, Command, Container, Element, HorizontalAlignment, Length, Row, Settings, Subscription, Text, Clipboard, VerticalAlignment, Font};
use std::time::{Duration, Instant};

// The side of the square and the number of mines
const N: usize = 10;
const MINES_COUNT: usize = 20;

pub fn main() -> iced::Result {
    Stopwatch::run(Settings::default())
}

struct Stopwatch {
    state: State,
    reset: button::State,
    buttons: Vec<button::State>,
    cells: Vec<Cell>,
}

enum State {
    Game,
    Win,
    Fail,
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

#[derive(Clone, PartialEq)]
enum CellState {
    Mine,
    MinesAround(i32),
}

#[derive(Debug, Clone)]
enum Message {
    Reset,
    Pressed(usize),
}

impl Application for Stopwatch {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                state: State::Game,
                reset: button::State::new(),
                buttons: Vec::from([button::State::new(); N * N]),
                cells: generate_cells(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Mines - Iced")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Reset => {
                self.state = State::Game;
                self.buttons = Vec::from([button::State::new(); N * N]);
                self.cells = generate_cells();
            },
            Message::Pressed(index) => match self.state {
                State::Fail | State::Win => { },
                State::Game => {
                    let cell = self.cells.get_mut(index).unwrap();
                    match cell.state {
                        CellState::Mine => {
                            self.state = State::Fail;
                            open_mines(&mut self.cells);
                        }
                        CellState::MinesAround(_) => {
                            open_empty_cells(index, &mut self.cells);
                            if is_win(&self.cells) {
                                self.state = State::Win;
                            }
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<Message> {
        let mut row  = Row::new()
            .spacing(0);
        let mut col = Column::new()
            .align_items(Align::Center)
            .spacing(4);

        let reset_button = Button::new(
            &mut self.reset,
            Text::new(format!("Reset"))
                .horizontal_alignment(HorizontalAlignment::Center),
        )
            .min_height(40)
            .padding(10)
            .on_press(Message::Reset);
        let reset_row = Row::new()
            .padding(20)
            .push(reset_button);

        row = row.push(reset_row);

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
                let style;
                match cell.state {
                    CellState::Mine => {
                        text = format!("x");
                        style = style::Button::Mine;
                    }
                    CellState::MinesAround(count) => {
                        if count == 0 {
                            text = format!("");
                        } else {
                            text = format!("{}", count);
                        }
                        style = style::Button::Empty(count);
                    }
                }
                let open_cell_view = Button::new(
                    button,
                    Text::new(format!("{}", text))
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                    .min_width(40)
                    .min_height(40)
                    .padding(10)
                    .style(style);
                row = row.push(open_cell_view);
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

        let (text_state, text_color) = match self.state {
            State::Win => ("You win!", Color::from_rgb(0.2, 0.2, 0.8)),
            State::Fail => ("You lose :(", Color::from_rgb(0.8, 0.2, 0.2)),
            _ => (" ", Color::default()),
        };
        let label = Text::new(format!("{}", text_state))
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(24)
            .color(text_color);
        let label_row = Row::new()
            .padding(20)
            .push(label);

        col = col.push(label_row);

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
    for _ in 0..MINES_COUNT {
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

fn open_mines(cells: &mut [Cell]) {
    cells.iter_mut()
        .filter(|c| c.state == CellState::Mine)
        .for_each(|c| c.is_opened = true);
}

fn is_win(cells: &[Cell]) -> bool {
    let mut is_one_closed = false;

    for cell in cells.iter() {
        match cell.state {
            CellState::Mine => continue,
            CellState::MinesAround(_) if !cell.is_opened => {
                is_one_closed = true;
                break;
            }
            CellState::MinesAround(_) => continue,
        }
    }

    !is_one_closed
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

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn edit_icon() -> Text {
    icon('\u{F303}')
}

fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        NotOpened,
        Mine,
        Empty(i32),
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: match self {
                    Button::NotOpened => None,
                    Button::Empty(_) => Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
                    Button::Mine => Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))),
                },
                border_radius: 2.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: match self {
                    Button::NotOpened => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Mine => Color::from_rgb(0.0, 0.0, 0.0),
                    Button::Empty(count) => match *count {
                        1 => Color::from_rgb(0.9, 0.0, 0.0),
                        2 => Color::from_rgb(0.0, 0.9, 0.0),
                        3 => Color::from_rgb(0.0, 0.0, 0.9),
                        4 => Color::from_rgb(0.9, 0.6, 0.6),
                        5 => Color::from_rgb(0.6, 0.9, 0.6),
                        6 => Color::from_rgb(0.6, 0.6, 0.9),
                        7 => Color::from_rgb(0.6, 0.0, 0.6),
                        8 => Color::from_rgb(0.0, 0.6, 0.6),
                        _ => Color::from_rgb(0.0, 0.0, 0.0),
                    }
                },
                ..button::Style::default()
            }
        }
    }
}
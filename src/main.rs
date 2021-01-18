use std::ops::Not;
use yew::prelude::*;

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}
impl Not for Color {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone)]
struct Cell {
    color: Option<Color>,
}
impl Cell {
    fn new() -> Self {
        Self { color: None }
    }
    fn fill(&mut self, color: Color) {
        self.color = Some(color);
    }
}

pub enum Msg {
    FillCell(usize),
}

pub struct Board {
    link: ComponentLink<Self>,
    cells: Vec<Cell>,
    turn: Color,
}
impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // changing this size requires to also change the grid-template-columns in the CSS
        let size: usize = 19;
        Self {
            link,
            cells: vec![Cell::new(); size.pow(2)],
            turn: Color::Black,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FillCell(id) => {
                let cell = self.cells.get_mut(id).unwrap();
                if cell.color.is_some() {
                    false
                } else {
                    cell.fill(self.turn);
                    self.turn = !self.turn;
                    true
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cells = self.cells.iter().enumerate().map(|(id, cell)| {
            let class = match (&cell.color, self.turn) {
                (Some(Color::White), _) => "grid-item-white-stone",
                (Some(Color::Black), _) => "grid-item-black-stone",
                (None, Color::White) => "grid-item-white",
                (None, Color::Black) => "grid-item-black",
            };
            html! {<div class=class onclick=self.link.callback(move |_| Msg::FillCell(id)) />}
        });

        html! {
            <div class="grid-container">
                {for cells}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Board>();
}

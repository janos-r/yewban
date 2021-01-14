use yew::prelude::*;

#[derive(Clone)]
struct Cell {
    filled: bool,
}
impl Cell {
    fn new() -> Self {
        Self { filled: false }
    }
    fn fill(&mut self) {
        self.filled = true;
    }
}

pub enum Msg {
    FillCell(usize),
}

pub struct Board {
    link: ComponentLink<Self>,
    cells: Vec<Cell>,
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FillCell(id) => {
                let cell = self.cells.get_mut(id).unwrap();
                if cell.filled {
                    false
                } else {
                    cell.fill();
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
            let class = if cell.filled {
                "grid-item-with-stone"
            } else {
                "grid-item"
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

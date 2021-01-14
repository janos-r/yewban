use yew::prelude::*;

#[derive(Clone)]
struct Cell {
    filled: bool,
}
impl Cell {
    pub fn new() -> Self {
        Self { filled: false }
    }
}

pub struct Board {
    cells: Vec<Cell>,
}
impl Component for Board {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let size: usize = 19;
        Self {
            cells: vec![Cell::new(); size.pow(2)],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cells = self.cells.iter().map(|_| html! {<div class="grid-item"/>});

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

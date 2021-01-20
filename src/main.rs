use goban::{
    pieces::stones::Color,
    rules::{GobanSizes, Move, Player, JAPANESE},
};
use goban::{pieces::util::coord::one_to_2dim, rules::game::Game};
use yew::prelude::*;

pub enum Msg {
    FillCell(usize),
}

fn to_move(id: usize) -> Move {
    let m = one_to_2dim((19, 19), id);
    Move::Play(m.0, m.1)
}

pub struct Board {
    link: ComponentLink<Self>,
    game: Game,
}
impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // changing this size requires to also change the grid-template-columns in the CSS
        let size = GobanSizes::Nineteen;
        let rule = JAPANESE;
        let game = Game::new(size, rule);
        Self { link, game }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FillCell(id) => {
                if self.game.try_play(to_move(id)).is_err() {
                    false
                } else {
                    true
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let turn: Player = self.game.turn();
        let game_cells: Vec<Color> = self.game.goban().raw();
        let cells = game_cells.iter().enumerate().map(|(id, color)| {
            let class = match (color, turn) {
                (Color::White, _) => "grid-item-white-stone",
                (Color::Black, _) => "grid-item-black-stone",
                (Color::None, Player::White) => "grid-item-white",
                (Color::None, Player::Black) => "grid-item-black",
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

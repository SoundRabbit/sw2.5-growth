use super::molecule::growth::{self, Growth};
use super::template::basic_page::{self, BasicPage};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {
    SetGrowthDice(String),
    SetGrowth([[i32; 6]; 6]),
}

pub enum On {}

pub struct Sheet {
    growth_dice: [[i32; 6]; 6],
    growth: [[i32; 6]; 6],
}

impl Component for Sheet {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Sheet {}

impl Constructor for Sheet {
    fn constructor(_props: Self::Props) -> Self {
        Self {
            growth_dice: [
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
            ],
            growth: [
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
            ],
        }
    }
}

impl Update for Sheet {
    fn update(mut self: Pin<&mut Self>, msg: Msg) -> Cmd<Self> {
        match msg {
            Msg::SetGrowthDice(text) => {
                let mut dices = vec![];
                for maybe_number in text.chars() {
                    if let Ok(n) = String::from(maybe_number).parse::<usize>() {
                        if 1 <= n && n <= 6 {
                            dices.push(n - 1);
                        }
                    }
                }

                for p in 0..6 {
                    for s in 0..6 {
                        self.growth_dice[p][s] = 0;
                    }
                }

                for i in 0..(dices.len() / 2) {
                    let p = dices[i * 2];
                    let s = dices[i * 2 + 1];
                    self.growth_dice[usize::min(p, s)][usize::max(p, s)] += 1;
                }

                Cmd::none()
            }
            Msg::SetGrowth(growth) => {
                self.growth = growth;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Sheet {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        BasicPage::new(
            self,
            None,
            basic_page::Props {},
            Sub::none(),
            (
                Attributes::new(),
                Events::new(),
                vec![
                    Html::text("成長ダイス"),
                    Html::textarea(
                        Attributes::new(),
                        Events::new().on_input(self, |text| Msg::SetGrowthDice(text)),
                        vec![],
                    ),
                    Growth::empty(
                        self,
                        None,
                        growth::Props {
                            growth_dice: self.growth_dice.clone(),
                            growth: self.growth.clone(),
                        },
                        Sub::map(|sub| match sub {
                            growth::On::Growth(growth) => Msg::SetGrowth(growth),
                        }),
                    ),
                ],
            ),
        )
    }
}

impl Styled for Sheet {
    fn style() -> Style {
        style! {}
    }
}

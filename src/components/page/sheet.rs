use super::atom::btn::Btn;
use super::molecule::growth::{self, Growth};
use super::template::basic_page::{self, BasicPage};
use crate::model::attr_growth::AttrGrowth;
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
    SetAttrGrowth(AttrGrowth),
}

pub enum On {}

pub struct Sheet {
    attr: AttrGrowth,
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
            attr: AttrGrowth {
                attrs: [0, 0, 0, 0, 0, 0],
                attr_mods: [0, 0, 0, 0, 0, 0],
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
            },
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
                        self.attr.growth_dice[p][s] = 0;
                    }
                }

                for i in 0..(dices.len() / 2) {
                    let p = dices[i * 2];
                    let s = dices[i * 2 + 1];
                    self.attr.growth_dice[usize::min(p, s)][usize::max(p, s)] += 1;
                }

                Cmd::none()
            }
            Msg::SetAttrGrowth(attr) => {
                self.attr = attr;
                Cmd::none()
            }
            Msg::SetGrowth(growth) => {
                self.attr.growth = growth;
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for Sheet {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(BasicPage::new(
            self,
            None,
            basic_page::Props {},
            Sub::none(),
            (
                Attributes::new().class(Self::class("base")),
                Events::new(),
                vec![
                    Html::div(
                        Attributes::new().class(Self::class("growth-dice")),
                        Events::new(),
                        vec![
                            Html::text("成長ダイス"),
                            Html::textarea(
                                Attributes::new().class(Self::class("growth-dice-input")),
                                Events::new().on_input(self, |text| Msg::SetGrowthDice(text)),
                                vec![],
                            ),
                        ],
                    ),
                    Growth::empty(
                        self,
                        None,
                        growth::Props {
                            attr: self.attr.clone(),
                        },
                        Sub::map(|sub| match sub {
                            growth::On::SetAttrGrowth(attr) => Msg::SetAttrGrowth(attr),
                        }),
                    ),
                    Btn::with_valiant(
                        "primary",
                        Attributes::new().class(Self::class("reset-btn")),
                        Events::new().on_click(self, |_| {
                            Msg::SetGrowth([
                                [0, 0, 0, 0, 0, 0],
                                [0, 0, 0, 0, 0, 0],
                                [0, 0, 0, 0, 0, 0],
                                [0, 0, 0, 0, 0, 0],
                                [0, 0, 0, 0, 0, 0],
                                [0, 0, 0, 0, 0, 0],
                            ])
                        }),
                        vec![Html::text("成長をリセット")],
                    ),
                ],
            ),
        ))
    }
}

impl Styled for Sheet {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-template-rows": "max-content max-content max-content";
                "row-gap": "1rem";
            }

            ".growth-dice" {
                "display": "grid";
                "grid-template-columns": "1fr";
                "grid-template-rows": "max-content max-content";
            }

            ".growth-dice-input" {
                "width": "100%";
                "height": "5em";
                "resize": "none";
            }

            ".reset-btn" {
                "justify-self": "end";
            }
        }
    }
}

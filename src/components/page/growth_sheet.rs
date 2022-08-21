use super::atom::{
    btn::Btn,
    growth_log::{self, GrowthLog},
};
use super::molecule::growth_alloc::{self, GrowthAlloc};
use super::template::basic_page::{self, BasicPage};
use crate::model::{attr::Attrs, attr_growth::AttrGrowth};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {
    SetGrowthDice(String),
    ResetGrowth,
    SetAttrGrowth(AttrGrowth),
    SetBirth(Attrs),
    SetInit(Attrs),
    SetMods(Attrs),
}

pub enum On {}

pub struct GrowthSheet {
    birth: Attrs,
    init: Attrs,
    mods: Attrs,
    growth: AttrGrowth,
}

impl Component for GrowthSheet {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for GrowthSheet {}

impl Constructor for GrowthSheet {
    fn constructor(_props: Self::Props) -> Self {
        Self {
            birth: Attrs::new(3),
            init: Attrs::new(3),
            mods: Attrs::new(3),
            growth: AttrGrowth::new(),
        }
    }
}

impl Update for GrowthSheet {
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

                self.growth.raw_growth_dice.clear();
                for p in 0..6 {
                    for s in 0..6 {
                        self.growth.growth_dice[p][s] = 0;
                        self.growth.growth[p][s] = 0;
                    }
                }

                for i in 0..(dices.len() / 2) {
                    let p = dices[i * 2];
                    let s = dices[i * 2 + 1];
                    self.growth
                        .raw_growth_dice
                        .push([usize::min(p, s), usize::max(p, s)]);
                    self.growth.growth_dice[usize::min(p, s)][usize::max(p, s)] += 1;
                }

                for i in 0..6 {
                    self.growth.growth[i][i] = self.growth.growth_dice[i][i];
                }

                Cmd::none()
            }
            Msg::SetAttrGrowth(attr) => {
                self.growth = attr;
                Cmd::none()
            }
            Msg::SetBirth(birth) => {
                self.birth = birth;
                Cmd::none()
            }
            Msg::SetInit(init) => {
                self.init = init;
                Cmd::none()
            }
            Msg::SetMods(mods) => {
                self.mods = mods;
                Cmd::none()
            }
            Msg::ResetGrowth => {
                self.growth.growth = [
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                    [0, 0, 0, 0, 0, 0],
                ];
                for i in 0..6 {
                    self.growth.growth[i][i] = self.growth.growth_dice[i][i];
                }
                Cmd::none()
            }
        }
    }
}

impl Render<Html> for GrowthSheet {
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
                        Attributes::new().class(Self::class("GrowthSheet")),
                        Events::new(),
                        vec![
                            Html::div(
                                Attributes::new().class(Self::class("growth-dice")),
                                Events::new(),
                                vec![
                                    Html::text("成長ダイス"),
                                    Html::textarea(
                                        Attributes::new().class(Self::class("growth-dice-input")),
                                        Events::new()
                                            .on_input(self, |text| Msg::SetGrowthDice(text)),
                                        vec![],
                                    ),
                                ],
                            ),
                            GrowthAlloc::empty(
                                self,
                                None,
                                growth_alloc::Props {
                                    birth: self.birth.clone(),
                                    init: self.init.clone(),
                                    mods: self.mods.clone(),
                                    growth: self.growth.clone(),
                                },
                                Sub::map(|sub| match sub {
                                    growth_alloc::On::SetAttrGrowth(attr) => {
                                        Msg::SetAttrGrowth(attr)
                                    }
                                    growth_alloc::On::SetBirth(birth) => Msg::SetBirth(birth),
                                    growth_alloc::On::SetInit(init) => Msg::SetInit(init),
                                    growth_alloc::On::SetMods(mods) => Msg::SetMods(mods),
                                }),
                            ),
                            Btn::with_valiant(
                                "primary",
                                Attributes::new().class(Self::class("reset-btn")),
                                Events::new().on_click(self, |_| Msg::ResetGrowth),
                                vec![Html::text("成長をリセット")],
                            ),
                        ],
                    ),
                    GrowthLog::empty(
                        self,
                        None,
                        growth_log::Props {
                            growth: self.growth.clone(),
                        },
                        Sub::none(),
                    ),
                ],
            ),
        ))
    }
}

impl Styled for GrowthSheet {
    fn style() -> Style {
        style! {
            ".base" {
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-template-rows": "max-content max-content";
                "column-gap": "1rem";
            }

            ".GrowthSheet" {
                "display": "grid";
                "grid-template-columns": "max-content";
                "grid-auto-rows": "max-content";
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

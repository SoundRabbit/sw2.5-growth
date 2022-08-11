use super::atom::btn::Btn;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub growth_dice: [[i32; 6]; 6],
    pub growth: [[i32; 6]; 6],
}

pub enum Msg {
    Growth(usize, usize),
}

pub enum On {
    Growth([[i32; 6]; 6]),
}

pub struct Growth {
    growth_dice: [[i32; 6]; 6],
    growth: [[i32; 6]; 6],
}

impl Component for Growth {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Growth {}

impl Constructor for Growth {
    fn constructor(props: Self::Props) -> Self {
        Self {
            growth_dice: props.growth_dice,
            growth: props.growth,
        }
    }
}

impl Update for Growth {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.growth_dice = props.growth_dice;
        self.growth = props.growth;

        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::Growth(p, s) => {
                if self.growth([p, s], [false, false, false, false, false, false]) {
                    Cmd::submit(On::Growth(self.growth.clone()))
                } else {
                    Cmd::none()
                }
            }
        }
    }
}

impl Render<Html> for Growth {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("tabular")),
            Events::new(),
            self.render_tabular(),
        ))
    }
}

impl Growth {
    fn empty() -> Html {
        Html::div(Attributes::new(), Events::new(), vec![])
    }

    fn text(text: impl Into<String>) -> Html {
        Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    fn attr_text(attr: usize) -> String {
        match attr {
            0 => String::from("器用"),
            1 => String::from("敏捷"),
            2 => String::from("筋力"),
            3 => String::from("生命"),
            4 => String::from("知力"),
            5 => String::from("精神"),
            _ => String::from(""),
        }
    }
    fn render_tabular(&self) -> Vec<Html> {
        let mut cells = vec![];

        cells.push(Self::empty());
        cells.push(Self::text("成長回数"));
        cells.push(Self::text("器用"));
        cells.push(Self::text("敏捷"));
        cells.push(Self::text("筋力"));
        cells.push(Self::text("生命"));
        cells.push(Self::text("知力"));
        cells.push(Self::text("精神"));

        for p in 0..6 {
            cells.push(Self::text(Self::attr_text(p)));
            cells.push(Self::text(
                self.growth[p].iter().fold(0, |g, sum| sum + g).to_string(),
            ));

            for s in 0..6 {
                let count = (
                    self.growth_dice[usize::min(p, s)][usize::max(p, s)] - self.count_used([p, s]),
                    self.growth_dice[usize::min(p, s)][usize::max(p, s)],
                );
                let growthable = (
                    self.count_growthable(true, [p, s], [false, false, false, false, false, false]),
                    self.count_growthable(
                        false,
                        [p, s],
                        [false, false, false, false, false, false],
                    ),
                );

                cells.push(Btn::with_valiant(
                    if count.1 == 0 && growthable.1 == 0 {
                        "light"
                    } else if count.0 == 0 {
                        "secondary"
                    } else {
                        "primary"
                    },
                    Attributes::new().class(Self::class("btn")),
                    Events::new().on_click(self, move |_| Msg::Growth(p, s)),
                    vec![
                        Html::text(self.growth[p][s].to_string()),
                        Html::element("br", Attributes::new(), Events::new(), vec![]),
                        Html::text(format!(
                            "残：{}({})/{}({})",
                            count.0, growthable.0, count.1, growthable.1
                        )),
                    ],
                ));
            }
        }

        cells
    }
}

impl Growth {
    fn validate_attr(attr: usize) -> bool {
        attr < 6
    }

    fn count_used(&self, attr: [usize; 2]) -> i32 {
        let [p, s] = attr;
        if p == s {
            self.growth[p][s]
        } else {
            self.growth[p][s] + self.growth[s][p]
        }
    }

    fn count_growthable(&self, count_used: bool, attr: [usize; 2], mut exclude: [bool; 6]) -> i32 {
        let [p, s] = attr;
        if !Self::validate_attr(p) || !Self::validate_attr(s) {
            return -1;
        }

        exclude[p] = true;
        exclude[s] = true;

        let mut count = self.growth_dice[usize::min(p, s)][usize::max(p, s)]
            - if count_used {
                self.count_used([p, s])
            } else {
                0
            };

        if p != s {
            for i in 0..6 {
                if !exclude[i] {
                    count += i32::min(
                        self.growth_dice[usize::min(p, i)][usize::max(p, i)],
                        self.count_growthable(count_used, [i, s], exclude.clone()),
                    );
                }
            }
        }

        count
    }

    fn growth(self: &mut Pin<&mut Self>, attr: [usize; 2], mut exclude: [bool; 6]) -> bool {
        let [p, s] = attr;
        if !Self::validate_attr(p) || !Self::validate_attr(s) {
            return false;
        }

        exclude[p] = true;
        exclude[s] = true;

        if self.growth_dice[usize::min(p, s)][usize::max(p, s)] > self.count_used([p, s]) {
            self.growth[p][s] += 1;
            return true;
        }

        if self.growth[s][p] > 0 {
            self.growth[p][s] += 1;
            self.growth[s][p] -= 1;
            return true;
        }

        if p != s {
            for i in 0..6 {
                if !exclude[i] {
                    if self.count_growthable(false, [p, i], exclude.clone()) > 0
                        && self.count_growthable(false, [i, s], exclude.clone()) > 0
                    {
                        self.growth([i, s], exclude.clone());
                        self.growth([p, i], exclude.clone());
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

impl Styled for Growth {
    fn style() -> Style {
        style! {
            ".tabular" {
                "display": "grid";
                "grid-template-columns": "repeat(8, max-content)";
                "grid-template-rows": "repeat(7, max-content)";
                "align-items": "center";
                "justify-items": "center";
                "row-gap": "0.2rem";
                "column-gap": "0.1rem";
            }

            ".btn" {
                "line-height": "1";
            }
        }
    }
}

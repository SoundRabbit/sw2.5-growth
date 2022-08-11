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
    attrs: [i32; 6],
    attr_mods: [i32; 6],
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
            attrs: [0, 0, 0, 0, 0, 0],
            attr_mods: [0, 0, 0, 0, 0, 0],
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

    fn cell_text(attrs: Attributes, text: impl Into<String>) -> Html {
        Html::div(
            attrs.class(Self::class("cell")),
            Events::new(),
            vec![Self::text(text)],
        )
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

        let sum_of_growth = self
            .growth
            .iter()
            .fold(0, |sum, gs| sum + gs.iter().fold(0, |sum, g| sum + g));

        cells.push(Self::empty());
        cells.push(Self::text("自然能力値"));
        cells.push(Self::empty());
        cells.push(Self::text("能力値修正"));
        cells.push(Self::empty());
        cells.push(Self::text(format!("成長回数（合計{}回）", sum_of_growth)));
        cells.push(Self::text("器用"));
        cells.push(Self::text("敏捷"));
        cells.push(Self::text("筋力"));
        cells.push(Self::text("生命"));
        cells.push(Self::text("知力"));
        cells.push(Self::text("精神"));
        cells.push(Self::empty());
        cells.push(Self::text("能力値合計"));

        for p in 0..6 {
            let growth_count = self.growth[p].iter().fold(0, |g, sum| sum + g);
            cells.push(Self::cell_text(
                Attributes::new().class(Self::class(format!("n{}", p).as_str())),
                Self::attr_text(p),
            ));
            cells.push(Html::input(
                Attributes::new()
                    .class(Self::class("cell-input"))
                    .class(Self::class(format!("n{}", p).as_str()))
                    .value(self.attrs[p].to_string())
                    .type_("number")
                    .nut("step", 1),
                Events::new(),
                vec![],
            ));
            cells.push(Self::text("＋"));
            cells.push(Html::input(
                Attributes::new()
                    .class(Self::class("cell-input"))
                    .class(Self::class(format!("n{}", p).as_str()))
                    .value(self.attr_mods[p].to_string())
                    .type_("number")
                    .nut("step", 1),
                Events::new(),
                vec![],
            ));
            cells.push(Self::text("＋"));
            cells.push(Self::cell_text(
                Attributes::new().class(if growth_count > 0 {
                    Self::class(format!("g{}", p).as_str())
                } else {
                    Self::class(format!("n{}", p).as_str())
                }),
                growth_count.to_string(),
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

                cells.push(Btn::no_valiant(
                    Attributes::new()
                        .class(Self::class("btn"))
                        .class(Self::class(
                            format!(
                                "{}{}",
                                if count.1 == 0 && growthable.1 == 0 {
                                    "n"
                                } else if self.growth[p][s] == 0 {
                                    "t"
                                } else {
                                    "g"
                                },
                                p
                            )
                            .as_str(),
                        )),
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

            cells.push(Self::text("＝"));
            cells.push(Self::cell_text(
                Attributes::new().class(Self::class(format!("n{}", p).as_str())),
                (self.attrs[p] + self.attr_mods[p] + growth_count).to_string(),
            ));
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
                "grid-template-columns": "repeat(14, max-content)";
                "grid-template-rows": "repeat(8, max-content)";
                "align-items": "center";
                "justify-items": "center";
                "row-gap": "0.2rem";
                "column-gap": "0.1rem";
            }

            ".btn" {
                "line-height": "1";
                "justify-self": "stretch";
            }

            ".cell" {
                "display": "grid";
                "justify-self": "stretch";
                "align-self": "stretch";
                "justify-content": "center";
                "align-content": "center";
            }

            ".cell-input" {
                "justify-self": "stretch";
                "align-self": "stretch";
                "max-width": "6em";
            }

            ".n0" {
                "background-color": "#e9fac8";
                "color": "#212529";
            }

            ".t0" {
                "background-color": "#94d82d";
                "color": "#212529";
            }

            ".g0" {
                "background-color": "#5c940d";
                "color": "#f8f9fa";
            }

            ".n1" {
                "background-color": "#d3f9d8";
                "color": "#212529";
            }

            ".t1" {
                "background-color": "#51cf66";
                "color": "#212529";
            }

            ".g1" {
                "background-color": "#2b8a3e";
                "color": "#f8f9fa";
            }

            ".n2" {
                "background-color": "#c5f6fa";
                "color": "#212529";
            }

            ".t2" {
                "background-color": "#22b8cf";
                "color": "#212529";
            }

            ".g2" {
                "background-color": "#0b7285";
                "color": "#f8f9fa";
            }

            ".n3" {
                "background-color": "#d0ebff";
                "color": "#212529";
            }

            ".t3" {
                "background-color": "#339af0";
                "color": "#212529";
            }

            ".g3" {
                "background-color": "#1864ab";
                "color": "#f8f9fa";
            }

            ".n4" {
                "background-color": "#e5dbff";
                "color": "#212529";
            }

            ".t4" {
                "background-color": "#845ef7";
                "color": "#212529";
            }

            ".g4" {
                "background-color": "#5f3dc4";
                "color": "#f8f9fa";
            }

            ".n5" {
                "background-color": "#ffdeeb";
                "color": "#212529";
            }

            ".t5" {
                "background-color": "#f06595";
                "color": "#212529";
            }

            ".g5" {
                "background-color": "#a61e4d";
                "color": "#f8f9fa";
            }
        }
    }
}

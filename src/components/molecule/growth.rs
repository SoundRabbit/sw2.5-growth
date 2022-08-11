use super::atom::btn::Btn;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub growth_dice: [[usize; 6]; 6],
    pub growth: [[usize; 6]; 6],
}

pub enum Msg {}

pub enum On {}

pub struct Growth {
    growth_dice: [[usize; 6]; 6],
    growth: [[usize; 6]; 6],
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
                cells.push(Btn::secondary(
                    Attributes::new().class(Self::class("btn")),
                    Events::new(),
                    vec![
                        Html::text(
                            (self.growth_dice[usize::min(p, s)][usize::max(p, s)]
                                - self.growth[p][s]
                                - self.growth[s][p])
                                .to_string(),
                        ),
                        Html::element("br", Attributes::new(), Events::new(), vec![]),
                        Html::text(format!(
                            "({}/{})",
                            self.growth_dice[usize::min(p, s)][usize::max(p, s)].to_string(),
                            self.count_growthable(
                                [p, s],
                                [false, false, false, false, false, false]
                            )
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

    fn count_growthable(&self, attr: [usize; 2], mut exclude: [bool; 6]) -> i32 {
        let [p, s] = attr;
        if !Self::validate_attr(p) || !Self::validate_attr(s) {
            return -1;
        }

        exclude[p] = true;
        exclude[s] = true;

        let mut count = self.growth_dice[usize::min(p, s)][usize::max(p, s)] as i32
            - self.growth[p][s] as i32
            - self.growth[s][p] as i32;

        if p != s {
            for i in 0..6 {
                if !exclude[i] {
                    count += i32::min(
                        self.growth_dice[usize::min(p, i)][usize::max(p, i)] as i32,
                        self.count_growthable([i, s], exclude.clone()),
                    );
                }
            }
        }

        count
    }

    fn growth(&mut self, attr: [usize; 2]) {}
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

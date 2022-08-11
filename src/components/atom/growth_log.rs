use crate::model::attr_growth::AttrGrowth;
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub attr: AttrGrowth,
}

pub enum Msg {}

pub enum On {}

pub struct GrowthLog {
    attr: AttrGrowth,
}

impl Component for GrowthLog {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for GrowthLog {}

impl Constructor for GrowthLog {
    fn constructor(props: Self::Props) -> Self {
        Self { attr: props.attr }
    }
}

impl Update for GrowthLog {
    fn on_load(mut self: Pin<&mut Self>, props: Self::Props) -> Cmd<Self> {
        self.attr = props.attr;
        Cmd::none()
    }
}

impl Render<Html> for GrowthLog {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(self.render_lists())
    }
}

impl GrowthLog {
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

    fn empty() -> Html {
        Html::span(Attributes::new(), Events::new(), vec![])
    }

    fn text(text: impl Into<String>) -> Html {
        Html::span(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    fn attr(attr: usize) -> Html {
        Html::span(
            Attributes::new().class(Self::class(format!("a{}", attr).as_str())),
            Events::new(),
            vec![Html::text(Self::attr_text(attr))],
        )
    }

    fn render_lists(&self) -> Html {
        let mut lists = vec![];
        let mut growth = self.attr.growth.clone();

        for n in (0..self.attr.raw_growth_dice.len()).step_by(20) {
            let mut items = vec![];
            for i in n..(n + 20) {
                if let Some(a) = self.attr.raw_growth_dice.get(i) {
                    items.push(Some(vec![
                        Self::text(format!("#{}", i + 1)),
                        Self::text(":"),
                        Self::text(format!("[{}, {}]", a[0] + 1, a[1] + 1)),
                        if growth[a[0]][a[1]] > 0 {
                            growth[a[0]][a[1]] -= 1;
                            Self::attr(a[0])
                        } else if growth[a[1]][a[0]] > 0 {
                            growth[a[1]][a[0]] -= 1;
                            Self::attr(a[1])
                        } else {
                            Self::empty()
                        },
                    ]));
                } else {
                    items.push(Some(vec![
                        Self::empty(),
                        Self::empty(),
                        Self::empty(),
                        Self::empty(),
                    ]));
                }
            }
            lists.push(items);
        }

        let cols = lists.len();
        let mut elements = vec![];
        for j in 0..20 {
            for i in 0..lists.len() {
                let item = lists[i][j].take();
                if let Some(item) = item {
                    elements.push(item);
                }
            }
        }

        Html::div(
            Attributes::new()
                .class(Self::class("base"))
                .class(Self::class("list"))
                .style(
                    "grid-template-columns",
                    format!("repeat({}, max-content)", cols * 4),
                ),
            Events::new(),
            elements.into_iter().flatten().collect(),
        )
    }
}

impl Styled for GrowthLog {
    fn style() -> Style {
        style! {
            ".base" {
                "height": "100%";
                "padding-left": "0.5em";
                "padding-right": "0.5em";
            }

            ".list" {
                "display": "grid";
                "grid-auto-rows": "max-content";
                "column-gap": "0.25em";
            }

            ".a0" {
                "background-color": "#e9fac8";
                "color": "#212529";
            }

            ".a1" {
                "background-color": "#d3f9d8";
                "color": "#212529";
            }

            ".a2" {
                "background-color": "#c5f6fa";
                "color": "#212529";
            }

            ".a3" {
                "background-color": "#d0ebff";
                "color": "#212529";
            }

            ".a4" {
                "background-color": "#e5dbff";
                "color": "#212529";
            }

            ".a5" {
                "background-color": "#ffdeeb";
                "color": "#212529";
            }
        }
    }
}

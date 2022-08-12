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
        Self::styled(self.render_pre())
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

    fn render_pre(&self) -> Html {
        let mut lists = vec![];
        let mut growth = self.attr.growth.clone();

        for n in (0..self.attr.raw_growth_dice.len()).step_by(20) {
            let mut items = vec![];
            for i in n..(n + 20) {
                if let Some(a) = self.attr.raw_growth_dice.get(i) {
                    items.push(format!(
                        "#{:>03}：[{}, {}]…{}",
                        i + 1,
                        a[0] + 1,
                        a[1] + 1,
                        if growth[a[0]][a[1]] > 0 {
                            growth[a[0]][a[1]] -= 1;
                            Self::attr_text(a[0])
                        } else if growth[a[1]][a[0]] > 0 {
                            growth[a[1]][a[0]] -= 1;
                            Self::attr_text(a[1])
                        } else {
                            String::from("　　")
                        }
                    ));
                } else {
                    items.push(String::new());
                }
            }
            lists.push(items);
        }

        let mut text = String::new();
        for j in 0..20 {
            let mut cols = vec![];
            for i in 0..lists.len() {
                cols.push(lists[i][j].drain(..).collect::<String>());
            }
            text += cols.join("　").as_str();
            text += "\n";
        }

        Html::pre(
            Attributes::new().class(Self::class("base")),
            Events::new(),
            vec![Html::text(text)],
        )
    }
}

impl Styled for GrowthLog {
    fn style() -> Style {
        style! {
            ".base" {
                "padding-left": "0.5em";
                "padding-right": "0.5em";
            }
        }
    }
}

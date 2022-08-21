use super::atom::btn::Btn;
use crate::model::{attr::Attrs, attr_growth::AttrGrowth};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {
    pub birth: Attrs,
    pub init: Attrs,
    pub mods: Attrs,
    pub growth: AttrGrowth,
}

pub enum Msg {
    GrowthAlloc(usize, usize),

    SetBirthSelection(usize),
    SetInitSelection(usize),
    SetModsSelection(usize),

    SetBirth(usize, usize, i32),
    SetInit(usize, usize, i32),
    SetMods(usize, usize, i32),
}

pub enum On {
    SetBirth(Attrs),
    SetInit(Attrs),
    SetMods(Attrs),
    SetAttrGrowth(AttrGrowth),
}

pub struct GrowthAlloc {
    birth: Attrs,
    init: Attrs,
    mods: Attrs,
    growth: AttrGrowth,
}

impl Component for GrowthAlloc {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for GrowthAlloc {}

impl Constructor for GrowthAlloc {
    fn constructor(props: Self::Props) -> Self {
        Self {
            birth: props.birth,
            init: props.init,
            mods: props.mods,
            growth: props.growth,
        }
    }
}

impl Update for GrowthAlloc {
    fn on_load(mut self: Pin<&mut Self>, props: Props) -> Cmd<Self> {
        self.birth = props.birth;
        self.init = props.init;
        self.mods = props.mods;
        self.growth = props.growth;

        Cmd::none()
    }

    fn update(mut self: Pin<&mut Self>, msg: Self::Msg) -> Cmd<Self> {
        match msg {
            Msg::GrowthAlloc(p, s) => {
                if self.growth.growth([p, s]) {
                    Cmd::submit(On::SetAttrGrowth(self.growth.clone()))
                } else {
                    Cmd::none()
                }
            }

            Msg::SetBirthSelection(n) => {
                self.birth.selecting = n;
                Cmd::submit(On::SetBirth(self.birth.clone()))
            }

            Msg::SetInitSelection(n) => {
                self.init.selecting = n;
                Cmd::submit(On::SetInit(self.init.clone()))
            }

            Msg::SetModsSelection(n) => {
                self.mods.selecting = n;
                Cmd::submit(On::SetMods(self.mods.clone()))
            }

            Msg::SetBirth(n, i, v) => {
                if let Some(attrs) = self.birth.attrs_list.get_mut(n) {
                    attrs[i] = v;
                }
                Cmd::submit(On::SetBirth(self.birth.clone()))
            }
            Msg::SetInit(n, i, v) => {
                if let Some(attrs) = self.init.attrs_list.get_mut(n) {
                    attrs[i] = v;
                }
                Cmd::submit(On::SetInit(self.init.clone()))
            }
            Msg::SetMods(n, i, v) => {
                if let Some(attrs) = self.mods.attrs_list.get_mut(n) {
                    attrs[i] = v;
                }
                Cmd::submit(On::SetMods(self.mods.clone()))
            }
        }
    }
}

impl Render<Html> for GrowthAlloc {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Self::styled(Html::div(
            Attributes::new().class(Self::class("tabular")).style(
                "grid-template-columns",
                format!(
                    "repeat({}, max-content)",
                    13 + self.birth.attrs_list.len()
                        + self.init.attrs_list.len()
                        + self.mods.attrs_list.len()
                ),
            ),
            Events::new(),
            self.render_tabular(),
        ))
    }
}

impl GrowthAlloc {
    fn empty() -> Html {
        Html::div(Attributes::new(), Events::new(), vec![])
    }

    fn text(text: impl Into<String>) -> Html {
        Html::div(Attributes::new(), Events::new(), vec![Html::text(text)])
    }

    fn text_select(is_selecting: bool, events: Events, text: impl Into<String>) -> Html {
        Html::div(
            Attributes::new()
                .class(Self::class("selectable"))
                .string("data-selecting", is_selecting.to_string()),
            events,
            vec![Self::text(text)],
        )
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
            .growth
            .iter()
            .fold(0, |sum, gs| sum + gs.iter().fold(0, |sum, g| sum + g));

        cells.push(Self::empty());
        for n in 0..self.birth.attrs_list.len() {
            cells.push(Self::text_select(
                n == self.birth.selecting,
                Events::new().on_click(self, move |_| Msg::SetBirthSelection(n)),
                format!("生まれ {}", n + 1),
            ));
        }
        cells.push(Self::empty());
        for n in 0..self.init.attrs_list.len() {
            cells.push(Self::text_select(
                n == self.init.selecting,
                Events::new().on_click(self, move |_| Msg::SetInitSelection(n)),
                format!("初期値 {}", n + 1),
            ));
        }
        cells.push(Self::empty());
        for n in 0..self.mods.attrs_list.len() {
            cells.push(Self::text_select(
                n == self.mods.selecting,
                Events::new().on_click(self, move |_| Msg::SetModsSelection(n)),
                format!("修正 {}", n + 1),
            ));
        }
        cells.push(Self::empty());
        cells.push(Html::div(
            Attributes::new(),
            Events::new(),
            vec![
                Html::text("成長回数"),
                Html::element("br", Attributes::new(), Events::new(), vec![]),
                Html::text(format!("(合計{}回)", sum_of_growth)),
            ],
        ));
        cells.push(Self::text("器用"));
        cells.push(Self::text("敏捷"));
        cells.push(Self::text("筋力"));
        cells.push(Self::text("生命"));
        cells.push(Self::text("知力"));
        cells.push(Self::text("精神"));
        cells.push(Self::empty());
        cells.push(Self::text("能力値合計"));

        for p in 0..6 {
            let growth_count = self.growth.growth[p].iter().fold(0, |g, sum| sum + g);
            cells.push(Self::cell_text(
                Attributes::new().class(Self::class(format!("n{}", p).as_str())),
                Self::attr_text(p),
            ));
            for n in 0..self.birth.attrs_list.len() {
                let birth = self.birth.attrs_list[n][p];
                cells.push(Html::input(
                    Attributes::new()
                        .class(Self::class("cell-input"))
                        .class(Self::class(format!("n{}", p).as_str()))
                        .value(birth.to_string())
                        .type_("number")
                        .nut("step", 1),
                    Events::new().on_input(self, move |a| {
                        a.parse::<i32>()
                            .ok()
                            .map(move |a| Msg::SetBirth(n, p, a))
                            .unwrap_or(Msg::SetBirth(n, p, birth))
                    }),
                    vec![],
                ));
            }
            cells.push(Self::text("＋"));
            for n in 0..self.init.attrs_list.len() {
                let init = self.init.attrs_list[n][p];
                cells.push(Html::input(
                    Attributes::new()
                        .class(Self::class("cell-input"))
                        .class(Self::class(format!("n{}", p).as_str()))
                        .value(init.to_string())
                        .type_("number")
                        .nut("step", 1),
                    Events::new().on_input(self, move |a| {
                        a.parse::<i32>()
                            .ok()
                            .map(move |a| Msg::SetInit(n, p, a))
                            .unwrap_or(Msg::SetInit(n, p, init))
                    }),
                    vec![],
                ));
            }
            cells.push(Self::text("＋"));
            for n in 0..self.mods.attrs_list.len() {
                let mods = self.mods.attrs_list[n][p];
                cells.push(Html::input(
                    Attributes::new()
                        .class(Self::class("cell-input"))
                        .class(Self::class(format!("n{}", p).as_str()))
                        .value(mods.to_string())
                        .type_("number")
                        .nut("step", 1),
                    Events::new().on_input(self, move |a| {
                        a.parse::<i32>()
                            .ok()
                            .map(move |a| Msg::SetMods(n, p, a))
                            .unwrap_or(Msg::SetMods(n, p, mods))
                    }),
                    vec![],
                ));
            }
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
                    self.growth.growth_dice[usize::min(p, s)][usize::max(p, s)]
                        - self.growth.count_used([p, s]),
                    self.growth.growth_dice[usize::min(p, s)][usize::max(p, s)],
                );
                let growthable = (
                    self.growth.count_growthable(true, [p, s]),
                    self.growth.count_growthable(false, [p, s]),
                );

                cells.push(Btn::no_valiant(
                    Attributes::new()
                        .class(Self::class("btn"))
                        .class(Self::class(
                            format!(
                                "{}{}",
                                if count.1 == 0 && growthable.1 == 0 {
                                    "n"
                                } else if self.growth.growth[p][s] == 0 {
                                    "t"
                                } else {
                                    "g"
                                },
                                p
                            )
                            .as_str(),
                        )),
                    Events::new().on_click(self, move |_| Msg::GrowthAlloc(p, s)),
                    vec![
                        Html::text(self.growth.growth[p][s].to_string()),
                        Html::element("br", Attributes::new(), Events::new(), vec![]),
                        Html::text(format!(
                            "{}/{}({}/{})",
                            count.0, count.1, growthable.0, growthable.1
                        )),
                    ],
                ));
            }

            cells.push(Self::text("＝"));
            cells.push(Self::cell_text(
                Attributes::new().class(Self::class(format!("n{}", p).as_str())),
                (self.birth.attrs().map(|a| a[p]).unwrap_or(0)
                    + self.init.attrs().map(|a| a[p]).unwrap_or(0)
                    + self.mods.attrs().map(|a| a[p]).unwrap_or(0)
                    + growth_count)
                    .to_string(),
            ));
        }

        cells
    }
}

impl GrowthAlloc {}

impl Styled for GrowthAlloc {
    fn style() -> Style {
        style! {
            ".tabular" {
                "display": "grid";
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

            ".selectable" {
                "justify-self": "stretch";
                "align-self": "stretch";
                "display": "flex";
                "justify-content": "center";
                "align-items": "center";
            }

            ".selectable:hover" {
                "cursor": "pointer";
            }

            r#".selectable[data-selecting="false"]"# {
                "background-color": "#c5f6fa";
            }

            r#".selectable[data-selecting="true"]"# {
                "background-color": "#22b8cf";
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

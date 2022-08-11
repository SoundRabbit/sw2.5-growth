use super::molecule::growth::{self, Growth};
use super::template::basic_page::{self, BasicPage};
use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct Sheet {}

impl Component for Sheet {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for Sheet {}

impl Constructor for Sheet {
    fn constructor(_props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for Sheet {}

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
                vec![Growth::empty(
                    self,
                    None,
                    growth::Props {
                        growth_dice: [
                            [1, 1, 1, 0, 1, 0],
                            [0, 1, 0, 1, 0, 1],
                            [0, 0, 0, 0, 0, 0],
                            [0, 0, 0, 1, 1, 1],
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
                    Sub::none(),
                )],
            ),
        )
    }
}

impl Styled for Sheet {
    fn style() -> Style {
        style! {}
    }
}

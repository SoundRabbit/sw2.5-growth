use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct BasicPage {}

impl Component for BasicPage {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for BasicPage {}

impl Constructor for BasicPage {
    fn constructor(_props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for BasicPage {}

impl Render<Html> for BasicPage {
    type Children = (Attributes, Events, Vec<Html>);
    fn render(&self, (attrs, events, children): Self::Children) -> Html {
        Self::styled(Html::div(
            attrs.class(Self::class("base")),
            events,
            children,
        ))
    }
}

impl Styled for BasicPage {
    fn style() -> Style {
        style! {
            ".base" {
                "position": "fixed";
                "top": "0";
                "left": "0";
                "width": "100vw";
                "height": "100vh";
                "overflow": "hidden";
            }
        }
    }
}

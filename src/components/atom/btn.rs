use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Btn {}

impl Btn {
    pub fn secondary(attrs: Attributes, events: Events, children: Vec<Html>) -> Html {
        Html::button(
            attrs.class("pure-button").class(Self::class("primary")),
            events,
            children,
        )
    }
}

impl Styled for Btn {
    fn style() -> Style {
        style! {}
    }
}

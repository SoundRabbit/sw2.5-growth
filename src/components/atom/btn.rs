use isaribi::{
    style,
    styled::{Style, Styled},
};
use kagura::prelude::*;
use nusa::prelude::*;

pub struct Btn {}

impl Btn {
    pub fn with_valiant(
        valiant: &str,
        attrs: Attributes,
        events: Events,
        children: Vec<Html>,
    ) -> Html {
        Self::styled(Html::button(
            attrs.class("pure-button").class(Self::class(valiant)),
            events,
            children,
        ))
    }
}

impl Styled for Btn {
    fn style() -> Style {
        style! {
            ".primary" {
                "background-color": "#1971c2";
                "color": "#f8f9fa";
            }

            ".secondary" {
                "background-color": "#adb5bd";
                "color": "#212529";
            }

            ".light" {
                "background-color": "#f1f3f5";
                "color": "#212529";
            }
        }
    }
}

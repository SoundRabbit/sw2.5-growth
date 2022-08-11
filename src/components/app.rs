use kagura::prelude::*;
use nusa::prelude::*;

use super::page::sheet::{self, Sheet};

pub struct Props {}

pub enum Msg {}

pub enum On {}

pub struct App {}

impl Component for App {
    type Props = Props;
    type Msg = Msg;
    type Event = On;
}

impl HtmlComponent for App {}

impl Constructor for App {
    fn constructor(_props: Self::Props) -> Self {
        Self {}
    }
}

impl Update for App {}

impl Render<Html> for App {
    type Children = ();
    fn render(&self, _children: Self::Children) -> Html {
        Sheet::empty(self, None, sheet::Props {}, Sub::none())
    }
}

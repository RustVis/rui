// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::color::Color;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disable_gutters: bool,

    #[prop_or(false)]
    pub disable_sticky: bool,

    #[prop_or(false)]
    pub inset: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(ListSubheader)]
pub fn list_subheader(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}

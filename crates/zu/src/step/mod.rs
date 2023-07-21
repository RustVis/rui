// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or(false)]
    pub active: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub completed: bool,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub expanded: bool,

    #[prop_or_default]
    pub index: i32,

    #[prop_or(false)]
    pub last: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Step)]
pub fn step(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}

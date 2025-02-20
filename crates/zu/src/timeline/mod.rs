// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Children, Classes, Html, Properties};

use crate::styles::position::HorizontalPosition;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub class_name: AttrValue,

    #[prop_or_default]
    pub position: HorizontalPosition,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(Timeline)]
pub fn timeline(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}

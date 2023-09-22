// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Classes, Html, Properties};

use crate::styles::color::Color;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub auto_complete: AttrValue,

    #[prop_or(false)]
    pub auto_focus: bool,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub color: Color,

    // TODO(Shaohua): Add components
    #[prop_or_default]
    pub default_value: AttrValue,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disable_injecting_global_styles: bool,

    #[prop_or_default]
    pub end_adornment: Option<Html>,

    #[prop_or(false)]
    pub error: bool,

    #[prop_or(false)]
    pub full_width: bool,

    #[prop_or_default]
    pub id: AttrValue,
    // TODO(Shaohua): Add input component
    #[prop_or(true)]
    pub dense_margin: bool,

    #[prop_or_default]
    pub max_rows: Option<i32>,

    #[prop_or_default]
    pub min_rows: Option<i32>,

    #[prop_or(false)]
    pub multiline: bool,

    #[prop_or_default]
    pub name: AttrValue,

    #[prop_or_default]
    pub on_blur: Option<Callback<()>>,
}

#[function_component(InputBase)]
pub fn input_base(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}

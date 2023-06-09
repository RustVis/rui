// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub open_icon: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(SpeedDialIcon)]
pub fn speed_dial_icon(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilterNoneSharp)]
pub fn filter_none_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilterNoneSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 5H1v18h18v-2H3V5zm20-4H5v18h18V1zm-2 16H7V3h14v14z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BoyRounded)]
pub fn boy_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BoyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,7.5c0.97,0,1.75-0.78,1.75-1.75S12.97,4,12,4s-1.75,0.78-1.75,1.75S11.03,7.5,12,7.5z M14,19c0,0.55-0.45,1-1,1h-2 c-0.55,0-1-0.45-1-1v-4c-0.55,0-1-0.45-1-1v-3.5c0-1.1,0.9-2,2-2h2c1.1,0,2,0.9,2,2V14c0,0.55-0.45,1-1,1V19z"/>
        </SvgIcon>
    }
}

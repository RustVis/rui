// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CalculateSharp)]
pub fn calculate_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CalculateSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M13.03,7.06L14.09,6l1.41,1.41L16.91,6l1.06,1.06l-1.41,1.41l1.41,1.41l-1.06,1.06L15.5,9.54l-1.41,1.41 l-1.06-1.06l1.41-1.41L13.03,7.06z M6.25,7.72h5v1.5h-5V7.72z M11.5,16h-2v2H8v-2H6v-1.5h2v-2h1.5v2h2V16z M18,17.25h-5v-1.5h5 V17.25z M18,14.75h-5v-1.5h5V14.75z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourtyTwoMpSharp)]
pub fn fourty_two_mp_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourtyTwoMpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,3v18h18V3H3z M6.5,8h3V7h-3V5.5H11V9H8v1h3v1.5H6.5V8z M12.5,18.5H11V14h-1v3H8.5v-3h-1v4.5H6v-6h6.5V18.5z M13,8h3V7 h-3V5.5h4.5V9h-3v1h3v1.5H13V8z M18,17h-3v1.5h-1.5v-6H18V17z"/>
        </SvgIcon>
    }
}

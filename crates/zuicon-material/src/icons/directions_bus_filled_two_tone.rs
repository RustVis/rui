// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirectionsBusFilledTwoTone)]
pub fn directions_bus_filled_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirectionsBusFilledTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,15c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2v-3H6V15z M15.5,13 c0.83,0,1.5,0.67,1.5,1.5S16.33,16,15.5,16S14,15.33,14,14.5S14.67,13,15.5,13z M8.5,13c0.83,0,1.5,0.67,1.5,1.5S9.33,16,8.5,16 S7,15.33,7,14.5S7.67,13,8.5,13z" enable-background="new" opacity=".3"/><path d="M12,4C8.48,4,7.03,4.48,6.43,5h11.24C17.13,4.46,15.71,4,12,4z" enable-background="new" opacity=".3"/><path d="M12,2C8,2,4,2.5,4,6v9.5c0,0.95,0.38,1.81,1,2.44V20c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h8v1c0,0.55,0.45,1,1,1h1 c0.55,0,1-0.45,1-1v-2.06c0.62-0.63,1-1.49,1-2.44V6C20,2.5,16.42,2,12,2z M12,4c3.71,0,5.13,0.46,5.67,1H6.43 C7.03,4.48,8.48,4,12,4z M18,15c0,1.1-0.9,2-2,2H8c-1.1,0-2-0.9-2-2v-3h12V15z M18,10H6V7h12V10z"/>
        </SvgIcon>
    }
}

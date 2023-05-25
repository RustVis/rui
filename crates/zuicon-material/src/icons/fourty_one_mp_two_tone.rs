// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourtyOneMpTwoTone)]
pub fn fourty_one_mp_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourtyOneMpTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,19h14V5H5V19z M13,5.5h3v6h-1.5V7H13V5.5z M13.5,12.5H17c0.55,0,1,0.45,1,1V16c0,0.55-0.45,1-1,1h-2v1.5 h-1.5V12.5z M7.5,9c0-0.55,0.45-1,1-1h2V7h-3V5.5H11c0.55,0,1,0.45,1,1V8c0,0.55-0.45,1-1,1H9v1h3v1.5H7.5V9z M6,13.5 c0-0.55,0.45-1,1-1h4.5c0.55,0,1,0.45,1,1v5H11V14h-1v3H8.5v-3h-1v4.5H6V13.5z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M12,10H9V9h2c0.55,0,1-0.45,1-1V6.5c0-0.55-0.45-1-1-1H7.5V7h3v1h-2c-0.55,0-1,0.45-1,1v2.5H12V10z"/><path d="M7.5,14h1v3H10v-3h1v4.5h1.5v-5c0-0.55-0.45-1-1-1H7c-0.55,0-1,0.45-1,1v5h1.5V14z"/><path d="M15,17h2c0.55,0,1-0.45,1-1v-2.5c0-0.55-0.45-1-1-1h-3.5v6H15V17z M15,14h1.5v1.5H15V14z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RsvpSharp)]
pub fn rsvp_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RsvpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16,9h1.5l-1.75,6h-1.5L12.5,9H14l1,3.43L16,9z M5.14,13L6,15H4.5l-0.85-2H2.5v2H1V9h5v4L5.14,13z M4.5,10.5h-2v1h2V10.5z M23,13h-3.5v2H18V9h5V13z M21.5,10.5h-2v1h2V10.5z M11.5,9v1.5h-3v0.75h3V15H7v-1.5h3v-0.75H7V9H11.5z"/>
        </SvgIcon>
    }
}

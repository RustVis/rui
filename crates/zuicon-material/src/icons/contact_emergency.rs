// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ContactEmergency)]
pub fn contact_emergency(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ContactEmergency"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,3H2C0.9,3,0,3.9,0,5v14c0,1.1,0.9,2,2,2h20c1.1,0,1.99-0.9,1.99-2L24,5C24,3.9,23.1,3,22,3z M9,8c1.65,0,3,1.35,3,3 s-1.35,3-3,3s-3-1.35-3-3S7.35,8,9,8z M2.08,19c1.38-2.39,3.96-4,6.92-4s5.54,1.61,6.92,4H2.08z M20.97,9.85l-0.75,1.3l-1.47-0.85 V12h-1.5v-1.7l-1.47,0.85l-0.75-1.3L16.5,9l-1.47-0.85l0.75-1.3l1.47,0.85V6h1.5v1.7l1.47-0.85l0.75,1.3L19.5,9L20.97,9.85z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextIncreaseSharp)]
pub fn text_increase_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextIncreaseSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0.99,19h2.42l1.27-3.58h5.65L11.59,19h2.42L8.75,5h-2.5L0.99,19z M5.41,13.39L7.44,7.6h0.12l2.03,5.79H5.41z M20,11h3v2h-3 v3h-2v-3h-3v-2h3V8h2V11z"/>
        </SvgIcon>
    }
}

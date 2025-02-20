// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ExtensionOffSharp)]
pub fn extension_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ExtensionOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.78,22.61l-1.63-1.63C18.1,20.98,18.05,21,18,21h-3.8c0-2.71-2.16-3-2.7-3s-2.7,0.29-2.7,3H3v-5.8c2.71,0,3-2.16,3-2.7 c0-0.54-0.3-2.7-2.99-2.7V6c0-0.05,0.02-0.09,0.02-0.14L1.39,4.22l1.41-1.41l18.38,18.38L19.78,22.61z M20,17.17V15 c1.38,0,2.5-1.12,2.5-2.5S21.38,10,20,10V4h-6c0-1.38-1.12-2.5-2.5-2.5S9,2.62,9,4H6.83L20,17.17z"/>
        </SvgIcon>
    }
}

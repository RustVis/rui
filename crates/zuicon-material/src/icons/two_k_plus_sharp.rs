// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TwoKPlusSharp)]
pub fn two_k_plus_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TwoKPlusSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M10,12.5H7.5v1H10V15H6v-3.5h2.5v-1H6V9h4V12.5z M14.25,15l-1.75-2.25V15H11V9h1.5v2.25L14.25,9H16 l-2.25,3L16,15H14.25z M19,12.5h-1.5V14h-1v-1.5H15v-1h1.5V10h1v1.5H19V12.5z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GiteRounded)]
pub fn gite_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GiteRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.41,9.41l-2.83-2.83C18.21,6.21,17.7,6,17.17,6H9V5c0-0.55-0.45-1-1-1S7,4.45,7,5v1H6.83C6.3,6,5.79,6.21,5.41,6.59 L2.59,9.41C2.21,9.79,2,10.3,2,10.83V17c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-6.17C22,10.3,21.79,9.79,21.41,9.41z M14,17H4v-5h10 V17z M20,17h-4v-6.17l2-2l2,2V17z"/>
        </SvgIcon>
    }
}

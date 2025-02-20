// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoTransfer)]
pub fn no_transfer(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoTransfer"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.19,21.19L2.81,2.81L1.39,4.22L4,6.83V16c0,0.88,0.39,1.67,1,2.22V20c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1v-1h8v1 c0,0.55,0.45,1,1,1h1c0.05,0,0.09-0.02,0.14-0.03l1.64,1.64L21.19,21.19z M7.5,17C6.67,17,6,16.33,6,15.5C6,14.67,6.67,14,7.5,14 S9,14.67,9,15.5C9,16.33,8.33,17,7.5,17z M6,11V8.83L8.17,11H6z M8.83,6L5.78,2.95C7.24,2.16,9.48,2,12,2c4.42,0,8,0.5,8,4v10 c0,0.35-0.08,0.67-0.19,0.98L13.83,11H18V6H8.83z"/>
        </SvgIcon>
    }
}

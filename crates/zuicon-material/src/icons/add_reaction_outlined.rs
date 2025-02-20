// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddReactionOutlined)]
pub fn add_reaction_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddReactionOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7,9.5C7,8.67,7.67,8,8.5,8S10,8.67,10,9.5c0,0.83-0.67,1.5-1.5,1.5S7,10.33,7,9.5z M12,17.5c2.33,0,4.31-1.46,5.11-3.5 H6.89C7.69,16.04,9.67,17.5,12,17.5z M15.5,11c0.83,0,1.5-0.67,1.5-1.5C17,8.67,16.33,8,15.5,8S14,8.67,14,9.5 C14,10.33,14.67,11,15.5,11z M22,1h-2v2h-2v2h2v2h2V5h2V3h-2V1z M20,12c0,4.42-3.58,8-8,8s-8-3.58-8-8c0-4.42,3.58-8,8-8 c1.46,0,2.82,0.4,4,1.08V2.84C14.77,2.3,13.42,2,11.99,2C6.47,2,2,6.48,2,12c0,5.52,4.47,10,9.99,10C17.52,22,22,17.52,22,12 c0-1.05-0.17-2.05-0.47-3h-2.13C19.78,9.93,20,10.94,20,12z"/>
        </SvgIcon>
    }
}

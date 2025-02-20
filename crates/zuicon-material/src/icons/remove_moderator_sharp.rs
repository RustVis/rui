// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RemoveModeratorSharp)]
pub fn remove_moderator_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RemoveModeratorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,11.09V5l-8-3L6.78,3.96l12.09,12.09C19.59,14.52,20,12.83,20,11.09z M2.81,2.81L1.39,4.22L4,6.83v4.26 c0,5.05,3.41,9.76,8,10.91c1.72-0.43,3.28-1.36,4.55-2.62l3.23,3.23l1.41-1.41L2.81,2.81z"/>
        </SvgIcon>
    }
}

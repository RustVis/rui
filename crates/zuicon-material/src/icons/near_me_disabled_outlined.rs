// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NearMeDisabledOutlined)]
pub fn near_me_disabled_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NearMeDisabledOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,6.34L21,3l-3.34,9l-1.56-1.56l1.5-4.05l-4.05,1.5L12,6.34z M21.19,21.19l-5.07-5.07L14.31,21H12.9l-2.83-7.07L3,11.1 V9.69l4.88-1.81L2.81,2.81l1.41-1.41l18.38,18.38L21.19,21.19z M14.57,14.57L9.43,9.43l-2.71,1.01l4.89,1.95l1.95,4.89L14.57,14.57z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutboundTwoTone)]
pub fn outbound_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutboundTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4c-4.41,0-8,3.59-8,8c0,4.41,3.59,8,8,8s8-3.59,8-8C20,7.59,16.41,4,12,4z M13.88,11.54l-4.96,4.96 l-1.41-1.41l4.96-4.96L10.34,8l5.65,0.01L16,13.66L13.88,11.54z" opacity=".3"/><path d="M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8s-8-3.59-8-8S7.59,4,12,4 M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10 C22,6.48,17.52,2,12,2L12,2z M13.88,11.54l-4.96,4.96l-1.41-1.41l4.96-4.96L10.34,8l5.65,0.01L16,13.66L13.88,11.54z"/>
        </SvgIcon>
    }
}

// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RuleRounded)]
pub fn rule_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RuleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.83,10.29l-2.12-2.12c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0l1.41,1.41l3.54-3.54 c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41l-4.24,4.24C16.85,10.68,16.22,10.68,15.83,10.29z M10,7H3 C2.45,7,2,7.45,2,8v0c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1v0C11,7.45,10.55,7,10,7z M20.29,12.71L20.29,12.71 c-0.39-0.39-1.02-0.39-1.41,0L17,14.59l-1.88-1.88c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41L15.59,16 l-1.88,1.88c-0.39,0.39-0.39,1.02,0,1.41l0,0c0.39,0.39,1.02,0.39,1.41,0L17,17.41l1.88,1.88c0.39,0.39,1.02,0.39,1.41,0l0,0 c0.39-0.39,0.39-1.02,0-1.41L18.41,16l1.88-1.88C20.68,13.73,20.68,13.1,20.29,12.71z M10,15H3c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h7c0.55,0,1-0.45,1-1v0C11,15.45,10.55,15,10,15z"/>
        </SvgIcon>
    }
}

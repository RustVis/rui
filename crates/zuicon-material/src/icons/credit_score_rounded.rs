// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CreditScoreRounded)]
pub fn credit_score_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CreditScoreRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,4H4C2.89,4,2.01,4.89,2.01,6L2,18c0,1.11,0.89,2,2,2h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H4v-6h18V6 C22,4.9,21.1,4,20,4z M20,8H4V6h16V8z M14.93,19.17l-2.12-2.12c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41 l2.83,2.83c0.39,0.39,1.02,0.39,1.41,0l5.66-5.66c0.39-0.39,0.39-1.02,0-1.41v0c-0.39-0.39-1.02-0.39-1.41,0L14.93,19.17z"/>
        </SvgIcon>
    }
}

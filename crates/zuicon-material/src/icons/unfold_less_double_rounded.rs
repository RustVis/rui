// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UnfoldLessDoubleRounded)]
pub fn unfold_less_double_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UnfoldLessDoubleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.46,5.7l-2.47,2.46L9.53,5.7c-0.39-0.39-1.02-0.39-1.41,0s-0.39,1.02,0,1.41l3.17,3.18c0.39,0.39,1.02,0.39,1.41,0 l3.17-3.18c0.39-0.39,0.39-1.02,0-1.41S14.85,5.31,14.46,5.7z"/><path d="M14.46,0.7l-2.47,2.46L9.53,0.7c-0.39-0.39-1.02-0.39-1.41,0s-0.39,1.02,0,1.41l3.17,3.18c0.39,0.39,1.02,0.39,1.41,0 l3.17-3.18c0.39-0.39,0.39-1.02,0-1.41S14.85,0.31,14.46,0.7z"/><path d="M9.54,23.3l2.47-2.46l2.46,2.46c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-3.17-3.18 c-0.39-0.39-1.02-0.39-1.41,0l-3.17,3.18c-0.39,0.39-0.39,1.02,0,1.41C8.52,23.69,9.15,23.69,9.54,23.3z"/><path d="M9.54,18.29l2.47-2.45l2.46,2.46c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41l-3.17-3.18 c-0.39-0.39-1.02-0.39-1.41,0l-3.17,3.17c-0.39,0.39-0.39,1.02,0,1.41S9.15,18.68,9.54,18.29z"/>
        </SvgIcon>
    }
}

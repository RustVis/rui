// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LandslideRounded)]
pub fn landslide_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LandslideRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10.57,11.42L8.6,8.8C8.22,8.3,7.63,8,7,8H4c-1.1,0-2,0.9-2,2v1.61l4,1.33L10.57,11.42z"/><path d="M6,19.05l-4-1.33V20c0,1.1,0.9,2,2,2h14c1.65,0,2.59-1.88,1.6-3.2l-2.57-3.42L6,19.05z"/><path d="M17,4.65V2.64c0-0.95-0.67-1.77-1.61-1.96l-2.58-0.52c-0.52-0.1-1.06,0-1.5,0.3L9.89,1.41C9.33,1.78,9,2.4,9,3.07v1.86 C9,5.6,9.33,6.22,9.89,6.59l1.23,0.82c0.55,0.37,1.24,0.44,1.85,0.19l2.77-1.11C16.5,6.2,17,5.46,17,4.65z"/><path d="M17.75,7.6l-1,0.8C16.28,8.78,16,9.35,16,9.96v1.08c0,0.61,0.28,1.18,0.75,1.56l0.8,0.64c0.58,0.47,1.38,0.57,2.06,0.27 l2.2-0.98C22.53,12.21,23,11.49,23,10.7V9.6c0-0.94-0.65-1.75-1.57-1.95l-2-0.44C18.84,7.08,18.22,7.22,17.75,7.6z"/>
        </SvgIcon>
    }
}

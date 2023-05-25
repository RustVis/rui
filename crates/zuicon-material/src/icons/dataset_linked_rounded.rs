// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DatasetLinkedRounded)]
pub fn dataset_linked_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DatasetLinkedRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.09,17H7v-4h3.69c0.95-0.63,2.09-1,3.31-1h6c0.34,0,0.67,0.04,1,0.09V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v14 c0,1.1,0.9,2,2,2h3.81C8.3,20.12,8,19.09,8,18C8,17.66,8.04,17.33,8.09,17z M13,7h4v4h-4V7z M7,7h4v4H7V7z"/><path d="M12.03,17.66c0.16-0.98,1.09-1.66,2.08-1.66L15,16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1l-0.83,0 c-2.09,0-3.95,1.53-4.15,3.61C9.79,19.99,11.66,22,14,22h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1 C12.79,20,11.82,18.91,12.03,17.66z"/><path d="M19.83,14L19,14c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1l0.89,0c1,0,1.92,0.68,2.08,1.66C22.18,18.91,21.21,20,20,20h-1 c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1c2.34,0,4.21-2.01,3.98-4.39C23.78,15.53,21.92,14,19.83,14z"/><path d="M15,19h4c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-4c-0.55,0-1,0.45-1,1v0C14,18.55,14.45,19,15,19z"/>
        </SvgIcon>
    }
}

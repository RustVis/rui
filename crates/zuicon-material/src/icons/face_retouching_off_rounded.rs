// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaceRetouchingOffRounded)]
pub fn face_retouching_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaceRetouchingOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.5,10c0.75,0,1.47-0.09,2.17-0.24C19.88,10.47,20,11.22,20,12c0,1.22-0.28,2.37-0.77,3.4l1.49,1.49 C21.53,15.44,22,13.78,22,12c0-5.52-4.48-10-10-10c-1.78,0-3.44,0.47-4.89,1.28l5.33,5.33C13.93,9.49,15.65,10,17.5,10z"/><path d="M2.6,4.43l1.48,1.48C2.51,7.95,1.7,10.6,2.1,13.46c0.62,4.33,4.11,7.82,8.44,8.44c2.85,0.41,5.51-0.41,7.55-1.98 l1.48,1.48c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41L4.01,3.01c-0.39-0.39-1.02-0.39-1.41,0l0,0 C2.21,3.41,2.21,4.04,2.6,4.43z M16.66,18.49C15.35,19.44,13.74,20,12,20c-4.41,0-8-3.59-8-8c0-0.05,0.01-0.1,0-0.14 c1.39-0.52,2.63-1.35,3.64-2.39L16.66,18.49z"/>
        </SvgIcon>
    }
}

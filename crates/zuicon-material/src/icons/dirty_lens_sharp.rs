// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirtyLensSharp)]
pub fn dirty_lens_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirtyLensSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,5h-5.17L15,3H9L7.17,5H2v16h20V5z M16.66,16.58c-1.14,1.86-3-1.03-3.81,0.09c-0.39,0.57-0.09,1.49,0.13,2.33 c0,0.47-0.38,0.85-0.85,0.85c-0.47,0-0.86-0.38-0.86-0.85c0.14-0.98,0.42-2.05-0.16-2.43c-0.89-0.59-1.27,2.06-2.8,1.35 c-1.39-1.12,1.05-1.29,0.5-3.27c-0.22-0.79-2.28,0.36-2.4-1.24c-0.08-1,1.49-0.74,1.51-1.49c0.03-0.75-1.03-1.05-0.25-1.91 c0.22-0.24,0.71-0.26,0.91-0.19c0.79,0.27,1.55,1.82,2.51,1.19c1.03-0.66-1.88-2.35,0-2.86c1.64-0.44,1.31,2.08,2.65,2.44 c1.94,0.52,2.65-4.55,4.41-2.33c1.85,2.33-3.43,2.27-2.85,4.01c0.34,1.01,2.15-1.2,2.76,0.53c0.64,1.83-3.09,0.82-3.04,1.66 C15.08,15.29,17.43,15.01,16.66,16.58z M18.14,18.01c-0.47,0-0.86-0.38-0.86-0.86s0.38-0.86,0.86-0.86c0.47,0,0.86,0.38,0.86,0.86 S18.62,18.01,18.14,18.01z"/>
        </SvgIcon>
    }
}

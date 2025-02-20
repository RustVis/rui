// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DirtyLensRounded)]
pub fn dirty_lens_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DirtyLensRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,5h-3.17l-1.24-1.35C15.22,3.24,14.68,3,14.12,3H9.88c-0.56,0-1.1,0.24-1.48,0.65L7.17,5H4C2.9,5,2,5.9,2,7v12 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V7C22,5.9,21.1,5,20,5z M16.66,16.58c-1.14,1.86-3-1.03-3.81,0.09 c-0.39,0.57-0.09,1.49,0.13,2.33c0,0.47-0.38,0.85-0.85,0.85c-0.47,0-0.86-0.38-0.86-0.85c0.14-0.98,0.42-2.05-0.16-2.43 c-0.89-0.59-1.27,2.06-2.8,1.35c-1.39-1.12,1.05-1.29,0.5-3.27c-0.22-0.79-2.28,0.36-2.4-1.24c-0.08-1,1.49-0.74,1.51-1.49 c0.03-0.75-1.03-1.05-0.25-1.91c0.22-0.24,0.71-0.26,0.91-0.19c0.79,0.27,1.55,1.82,2.51,1.19c1.03-0.66-1.88-2.35,0-2.86 c1.64-0.44,1.31,2.08,2.65,2.44c1.94,0.52,2.65-4.55,4.41-2.33c1.85,2.33-3.43,2.27-2.85,4.01c0.34,1.01,2.15-1.2,2.76,0.53 c0.64,1.83-3.09,0.82-3.04,1.66C15.08,15.29,17.43,15.01,16.66,16.58z M18.14,18.01c-0.47,0-0.86-0.38-0.86-0.86 s0.38-0.86,0.86-0.86c0.47,0,0.86,0.38,0.86,0.86S18.62,18.01,18.14,18.01z"/>
        </SvgIcon>
    }
}

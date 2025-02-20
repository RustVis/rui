// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NoFlashRounded)]
pub fn no_flash_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NoFlashRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.16,3.16c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l4.6,4.6L6.14,9.4H3.6C2.72,9.4,2,10.12,2,11v9.4 C2,21.28,2.72,22,3.6,22h12.8c0.75,0,1.38-0.52,1.55-1.22l1.47,1.47c0.39,0.39,1.02,0.39,1.41,0l0,0c0.39-0.39,0.39-1.02,0-1.41 L3.16,3.16z M10,20c-2.21,0-4-1.79-4-4c0-1.95,1.4-3.57,3.25-3.92l1.57,1.57c-0.26-0.09-0.53-0.15-0.82-0.15 c-1.38,0-2.5,1.12-2.5,2.5c0,1.38,1.12,2.5,2.5,2.5c1.38,0,2.5-1.12,2.5-2.5c0-0.29-0.06-0.56-0.15-0.82l1.57,1.57 C13.57,18.6,11.95,20,10,20z M18,15.17L10.83,8h0.87c0.56,0,1.1,0.24,1.48,0.65l0.69,0.75h2.54c0.88,0,1.6,0.72,1.6,1.6V15.17z M20.4,5.6h0.75c0.38,0,0.62,0.41,0.44,0.74L19,11V7h-0.5C18.22,7,18,6.78,18,6.5v-4C18,2.22,18.22,2,18.5,2h2.73 c0.36,0,0.6,0.37,0.46,0.7L20.4,5.6z"/>
        </SvgIcon>
    }
}

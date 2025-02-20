// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowseGalleryRounded)]
pub fn browse_gallery_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrowseGalleryRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9S13.97,3,9,3z M11.09,15.5L8.59,13C8.21,12.62,8,12.12,8,11.59V8 c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3.59l2.5,2.5c0.39,0.39,0.39,1.02,0,1.41l0,0C12.11,15.89,11.48,15.89,11.09,15.5z"/><path d="M17.99,5.08L17.99,5.08c0,0.37,0.21,0.69,0.53,0.88C20.6,7.17,22,9.42,22,12c0,2.58-1.4,4.83-3.48,6.04 c-0.32,0.19-0.53,0.51-0.53,0.88v0c0,0.77,0.84,1.25,1.51,0.86C22.18,18.22,24,15.32,24,12c0-3.32-1.82-6.22-4.5-7.78 C18.83,3.83,17.99,4.31,17.99,5.08z"/>
        </SvgIcon>
    }
}

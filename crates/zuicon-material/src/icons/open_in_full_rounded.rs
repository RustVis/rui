// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OpenInFullRounded)]
pub fn open_in_full_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OpenInFullRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,8.59V4c0-0.55-0.45-1-1-1h-4.59c-0.89,0-1.34,1.08-0.71,1.71l1.59,1.59l-10,10l-1.59-1.59C4.08,14.08,3,14.52,3,15.41 V20c0,0.55,0.45,1,1,1h4.59c0.89,0,1.34-1.08,0.71-1.71l-1.59-1.59l10-10l1.59,1.59C19.92,9.92,21,9.48,21,8.59z"/>
        </SvgIcon>
    }
}

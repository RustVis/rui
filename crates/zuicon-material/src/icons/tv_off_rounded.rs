// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TvOffRounded)]
pub fn tv_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TvOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21 8v9.88l1.85 1.85c.1-.22.15-.47.15-.73V7c0-1.11-.9-2-2-2h-7.59l2.94-2.94c.2-.2.2-.51 0-.71s-.51-.2-.71 0L12 4.99 8.36 1.35c-.2-.2-.51-.2-.71 0s-.2.51 0 .71L10.59 5H8.12l2 2H20c.55 0 1 .45 1 1zM3.12 2.83c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41l.82.82C1.65 5.28 1 6.06 1 7v12c0 1.1.9 2 2 2h15.46l1.29 1.29c.39.39 1.02.39 1.41 0 .36-.36.37-.92.07-1.31h.03L3.12 2.83zM3 18V8c0-.55.45-1 1-1h.46l12 12H4c-.55 0-1-.45-1-1z"/>
        </SvgIcon>
    }
}

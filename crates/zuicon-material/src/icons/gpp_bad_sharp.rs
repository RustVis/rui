// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GppBadSharp)]
pub fn gpp_bad_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GppBadSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,2L4,5v6.09c0,5.05,3.41,9.76,8,10.91c4.59-1.15,8-5.86,8-10.91V5L12,2z M15.5,14.09l-1.41,1.41L12,13.42L9.91,15.5 L8.5,14.09L10.59,12L8.5,9.91L9.91,8.5L12,10.59l2.09-2.09l1.41,1.41L13.42,12L15.5,14.09z"/>
        </SvgIcon>
    }
}

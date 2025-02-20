// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TenMpSharp)]
pub fn ten_mp_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TenMpSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.5,7H15v3h-1.5V7z M21,3H3v18h18V3z M12.5,18.5H11V14h-1v3H8.5v-3h-1v4.5H6v-6h6.5V18.5z M15,18.5h-1.5v-6H18V17h-3 V18.5z M10,5.5v6H8.5V7H7V5.5H10z M16.5,5.5v6H12v-6H16.5z M15,14h1.5v1.5H15V14z"/>
        </SvgIcon>
    }
}

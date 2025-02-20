// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoAwesomeMosaicSharp)]
pub fn auto_awesome_mosaic_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoAwesomeMosaicSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,21h8V3L3,3V21z M21,3h-8v8h8V3z M13,21h8v-8h-8V21z"/>
        </SvgIcon>
    }
}

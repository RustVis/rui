// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShopTwoRounded)]
pub fn shop_two_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ShopTwoRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M2 9c-.55 0-1 .45-1 1v10c0 1.1.9 2 2 2h14c1.11 0 2-.89 2-2H4c-.55 0-1-.45-1-1v-9c0-.55-.45-1-1-1zm16-4V3c0-1.1-.9-2-2-2h-4c-1.1 0-2 .9-2 2v2H7c-1.1 0-2 .9-2 2v9c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2h-3zm-6-2h4v2h-4V3zm0 11.02V8.84c0-.38.41-.62.74-.44l4.07 2.22c.32.18.35.63.05.84l-4.07 2.96c-.33.24-.79.01-.79-.4z"/>
        </SvgIcon>
    }
}

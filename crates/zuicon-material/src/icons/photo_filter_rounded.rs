// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhotoFilterRounded)]
pub fn photo_filter_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhotoFilterRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19.02 10.99V18c0 .55-.45 1-1 1H6c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h7c.55 0 1-.45 1-1s-.45-1-1-1H5.02c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2H19c1.1 0 2-.89 2-2v-8.01c0-.55-.44-.99-.99-.99s-.99.44-.99.99zm-5.77-.24L12.46 9c-.18-.39-.73-.39-.91 0l-.79 1.75-1.76.79c-.39.18-.39.73 0 .91l1.75.79.79 1.76c.18.39.73.39.91 0l.79-1.75 1.76-.79c.39-.18.39-.73 0-.91l-1.75-.8zm4.69-4.69l-.6-1.32c-.13-.29-.55-.29-.69 0l-.6 1.32-1.32.6c-.29.13-.29.55 0 .69l1.32.6.6 1.32c.13.29.55.29.69 0l.6-1.32 1.32-.6c.29-.13.29-.55 0-.69l-1.32-.6z"/>
        </SvgIcon>
    }
}

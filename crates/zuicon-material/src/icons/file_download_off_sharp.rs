// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FileDownloadOffSharp)]
pub fn file_download_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FileDownloadOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,6.17V3h6v6h4l-3.59,3.59L9,6.17z M21.19,21.19L2.81,2.81L1.39,4.22L6.17,9H5l7,7l0.59-0.59L15.17,18H5v2h12.17l2.61,2.61 L21.19,21.19z"/>
        </SvgIcon>
    }
}

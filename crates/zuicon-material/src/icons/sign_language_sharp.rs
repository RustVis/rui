// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignLanguageSharp)]
pub fn sign_language_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignLanguageSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.49,13l-1.39-2.7L12.49,9L19,15.2V24H4.5v-2H10v-1H3v-2h7v-1H2v-2h8v-1H3.5v-2H12.49z M11.78,7.12 c-0.84,0.4-1.17,0.62-1.63,1.19L6.76,4.74l1.45-1.38L11.78,7.12z M9.64,9.21C9.41,9.76,9.35,10.45,9.44,11H8.58L5.62,7.89 l1.45-1.38L9.64,9.21z M21.98,12.34L22,3.35l-1.9-0.1l-1,2.86L13.3,0l-1.45,1.38l4.09,4.3l-0.73,0.69L9.74,0.64L8.3,2l3.36,3.53 l1.06,1.11l2.65,2.33l5.08,4.83L21.98,12.34z"/>
        </SvgIcon>
    }
}

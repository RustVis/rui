// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::button_variant::ButtonVariant;

pub const fn root_class(variant: ButtonVariant) -> &'static str {
    match variant {
        ButtonVariant::Contained => "ZuButtonGroup-contained",
        ButtonVariant::Outlined => "ZuButtonGroup-outlined",
        ButtonVariant::Text => "ZuButtonGroup-text",
    }
}

// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::CssClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Buffer,
    Determinate,
    Indeterminate,
    Query,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Indeterminate
    }
}

impl CssClass for Variant {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Buffer => "ZuLinearProgress-buffer",
            Self::Determinate => "ZuLinearProgress-determinate",
            Self::Indeterminate => "ZuLinearProgress-indeterminate",
            Self::Query => "ZuLinearProgress-query",
        }
    }
}

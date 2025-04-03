/*
 * SPDX-FileCopyrightText: 2022 Valentin Brandl <valentin.brandl@ebsnet.de>
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

macro_rules! render {
    ($template:path) => {{
        use $crate::axum_ructe::Render;
        Render(|o| $template(o))
    }};
    ($template:path, $($arg:expr),* $(,)*) => {{
        use $crate::axum_ructe::Render;
        Render(move |o| $template(o, $($arg),*))
    }}
}

pub struct Render<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>>(pub T);

impl<T: FnOnce(&mut Vec<u8>) -> std::io::Result<()>> IntoResponse for Render<T> {
    fn into_response(self) -> axum::response::Response {
        let mut buf = Vec::new();
        match self.0(&mut buf) {
            Ok(()) => Html(buf).into_response(),
            Err(_e) => {
                // TODO: logging
                (StatusCode::INTERNAL_SERVER_ERROR, "Render failed").into_response()
            }
        }
    }
}

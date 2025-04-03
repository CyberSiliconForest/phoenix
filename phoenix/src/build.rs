/*
 * SPDX-FileCopyrightText: 2022 Valentin Brandl <valentin.brandl@ebsnet.de>
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use ructe::{Ructe, RucteError};

fn main() -> Result<(), RucteError> {
    println!("OUT_DIR: {:?}", std::env::var("OUT_DIR"));
    let mut ructe = Ructe::from_env()?;
    ructe
        .statics()?
        .add_files("statics")?
        .add_sass_file("scss/index.scss")?;
    ructe.compile_templates("templates")
}

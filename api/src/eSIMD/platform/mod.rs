/*
 * ╔═════════════════════════════════════════════════════════════════════════════════════╗
 * ║                                 ANTI-VIRUS LICENSE                                  ║
 * ║                                                                                     ║
 * ║                          Code Shielded from Viral Threats                           ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  Copyright Notice                                                                   ║
 * ║                                                                                     ║
 * ║  Copyright (c) 2025 Stanislav Mikhailov (xavetar)                                   ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  License Terms                                                                      ║
 * ║                                                                                     ║
 * ║  Licensed under the Anti-Virus License Agreement.                                   ║
 * ║  This file may not be used except in compliance with the License.                   ║
 * ║                                                                                     ║
 * ║  The License is included within the project distribution.                           ║
 * ║  If this file is obtained without the accompanying License, it must be deleted.     ║
 * ╟─────────────────────────────────────────────────────────────────────────────────────╢
 * ║  Warranty Disclaimer                                                                ║
 * ║                                                                                     ║
 * ║  Unless required by applicable law or agreed to in writing, software                ║
 * ║  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT          ║
 * ║  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.                   ║
 * ║                                                                                     ║
 * ║  See the License for the specific language governing permissions and limitations    ║
 * ║  under the License.                                                                 ║
 * ╚═════════════════════════════════════════════════════════════════════════════════════╝
 */

#[cfg(target_arch = "aarch64")]
mod aarch64;

#[cfg(target_arch = "arm")]
mod arm;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86")]
mod x86;

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "x86_64",
    target_arch = "x86"
))]
macro_rules! import {
    ($platform:ident) => {
        pub use self::$platform::{*};
    };
}

#[cfg(target_arch = "aarch64")]
import!(aarch64);

#[cfg(target_arch = "arm")]
import!(arm);

#[cfg(target_arch = "x86_64")]
import!(x86_64);

#[cfg(target_arch = "x86")]
import!(x86);
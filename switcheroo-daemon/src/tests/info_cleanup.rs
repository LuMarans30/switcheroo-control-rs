// SPDX-License-Identifier: GPL-3.0-or-later

use crate::info_cleanup::info_cleanup;

#[test]
fn empty_input_returns_empty() {
    assert_eq!(info_cleanup(""), "");
}

#[test]
fn strips_mesa_prefix_and_marks_trademark() {
    assert_eq!(
        info_cleanup("Mesa DRI Intel(R) UHD Graphics 620 (KBL GT2)"),
        "Intel® UHD Graphics 620 (KBL GT2)"
    );
}

#[test]
fn removes_drm_llvm_parenthetical() {
    assert_eq!(
        info_cleanup("Mesa Intel(R) HD Graphics 530 (Skylake GT2) (DRM 2.49.0, LLVM 9.0.0)"),
        "Intel® HD Graphics 530 (Skylake GT2)"
    );
}

#[test]
fn maps_llvmpipe_to_software_rendering() {
    assert_eq!(
        info_cleanup("llvmpipe (LLVM 15.0.7, 256 bits)"),
        "Software Rendering"
    );
}

#[test]
fn gallium_prefix_keeps_only_renderer() {
    assert_eq!(
        info_cleanup("Gallium 0.4 on AMD TONGA (DRM 3.1.0, LLVM 3.9.0)"),
        "AMD TONGA"
    );
}

#[test]
fn adds_trademark_to_known_brands() {
    assert_eq!(
        info_cleanup("AMD Radeon RX 6800 XT"),
        "AMD Radeon™ RX 6800 XT"
    );
    assert_eq!(
        info_cleanup("NVIDIA GeForce RTX 3080"),
        "NVIDIA GeForce RTX™ 3080"
    );
    assert_eq!(info_cleanup("AMD Ryzen 7 5800H"), "AMD Ryzen™ 7 5800H");
}

#[test]
fn shortens_graphics_controller() {
    assert_eq!(
        info_cleanup("Unknown Graphics Controller"),
        "Unknown Graphics"
    );
}

#[test]
fn escapes_pango_markup() {
    assert_eq!(
        info_cleanup("AMD Radeon <RX> & \"Co\""),
        "AMD Radeon™ &lt;RX&gt; &amp; &quot;Co&quot;"
    );
}

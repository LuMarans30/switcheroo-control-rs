// SPDX-License-Identifier: GPL-3.0-or-later

use regex::Regex;
use std::{borrow::Cow, sync::LazyLock};

struct ReplaceString {
    re: &'static str,
    replacement: &'static str,
}

const REPLACEMENTS: &[ReplaceString] = &[
    ReplaceString {
        re: r"Mesa DRI ",
        replacement: "",
    },
    ReplaceString {
        re: r"Mesa Intel",
        replacement: "Intel",
    },
    ReplaceString {
        re: r"\(R\)",
        replacement: "®",
    },
    ReplaceString {
        re: r"\((tm|TM)\)",
        replacement: "™",
    },
    ReplaceString {
        re: r"(ATI|EPYC|AMD FX|Radeon|Ryzen|Threadripper|GeForce (?:GTX|RTX)) ",
        replacement: "${1}™ ",
    },
    ReplaceString {
        re: r"Gallium \d+\.\d+ on (.*)",
        replacement: "$1",
    },
    ReplaceString {
        re: r" CPU| Processor| \S+-Core| @ \d+\.\d+GHz",
        replacement: "",
    },
    ReplaceString {
        re: r" x86|/MMX|/SSE2|/PCIe",
        replacement: "",
    },
    ReplaceString {
        re: r" \([^)]*(DRM|MESA|LLVM)[^)]*\)?",
        replacement: "",
    },
    ReplaceString {
        re: r"Graphics Controller",
        replacement: "Graphics",
    },
    ReplaceString {
        re: r".*llvmpipe.*",
        replacement: "Software Rendering",
    },
];

static REGEXES: LazyLock<Vec<(Regex, &'static str)>> = LazyLock::new(|| {
    REPLACEMENTS
        .iter()
        .map(|r| (Regex::new(r.re).expect("Invalid regex"), r.replacement))
        .collect()
});

static WS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[ \t\n\r]+").expect("Invalid regex"));

/// Mimics `g_markup_escape_text` to prevent Pango markup injection in GNOME
fn escape_markup(text: &str) -> Cow<'_, str> {
    if !text.contains(['&', '<', '>', '\'', '"']) {
        return Cow::Borrowed(text);
    }

    let mut result = String::with_capacity(text.len() + 16);
    for ch in text.chars() {
        match ch {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '\'' => result.push_str("&apos;"),
            '"' => result.push_str("&quot;"),
            _ => result.push(ch),
        }
    }
    Cow::Owned(result)
}

pub fn info_cleanup(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut pretty = escape_markup(input.trim());

    for (re, replacement) in &*REGEXES {
        if let Cow::Owned(modified) = re.replace_all(pretty.as_ref(), *replacement) {
            pretty = Cow::Owned(modified);
        }
    }

    WS_RE.replace_all(pretty.as_ref(), " ").into_owned()
}

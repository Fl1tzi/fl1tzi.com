use proc_macro_markdown::md_to_html;

pub struct Post {
    pub number: usize,
    pub visible: bool,
    pub name: &'static str,
    pub prev: &'static str,
    pub desc: &'static str,
}

// the posts which are shown
const LEN: usize = 3;
pub const POSTS: [Post; LEN] = 
    [
    Post {
        number: 1,
        visible: true,
        name: "WASM on this site",
        prev: "This site now runs WebAssembly!",
        desc: md_to_html!(
r"
This site is rendered in [WebAssembly](https://webassembly.org/). The full source is available at [Fl1tzi/fl1tzi.com](https://github.com/Fl1tzi/fl1tzi.com/).
"
        )
    },
    Post {
        number: 404,
        visible: false,
        name: "Error",
        prev: "",
        // Will be created at runtime
        desc: ""
    },
    Post {
        number: 0,
        visible: false,
        name: "Datenschutzerklärung",
        prev: "",
        desc: md_to_html!(r"
**TL;DR**

```conf
access_log off;
error_log off;
```

## Kontakt

kontakt@tgerber.net

## Daten beim Aufruf der Website

Wenn Sie diese Seite aufrufen, werden keine Daten von Ihnen gespeichert.
")
    }
    ];


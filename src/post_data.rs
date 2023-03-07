use proc_macro_markdown::md_to_html;

pub struct Post {
    pub number: usize,
    pub name: &'static str,
    pub prev: &'static str,
    pub desc: &'static str,
}

// the posts which are shown
// do NOT use 404
const LEN: usize = 1;
pub const POSTS: [Post; LEN] = 
    [
    Post {
        number: 1,
        name: "WASM on this site",
        prev: "This site now runs WebAssembly!",
        desc: md_to_html!(
r"
This site is rendered in [WebAssembly](https://webassembly.org/). The full source is available at [Fl1tzi/fl1tzi.com](https://github.com/Fl1tzi/fl1tzi.com/).
"
        )
    }
    ];


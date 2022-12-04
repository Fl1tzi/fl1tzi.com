use proc_macro_markdown::md_to_html;

pub struct Post {
    pub number: usize,
    pub name: &'static str,
    pub prev: &'static str,
    pub desc: &'static str,
}

// the posts which are shown
// do NOT use 404
const LEN: usize = 2;
pub const POSTS: [Post; LEN] = 
    [
    Post {
        number: 3,
        name: "[DE] Chatkontrolle stoppen!",
        prev: "Eine EU Verordnung gegen die Privatsphäre",
        desc: "
            <p>
            Die EU möchte mithilfe einer digitalen Kommunikationskontrolle die Chats von Benutzern umfassend überwachen.
            </p>
            <p>
            Sämtliche Nachrichten in sozialen Medien sollen hiermit kontrolliert werden und die Ende-zu-Ende Verschlüsselung umgangen werden, sodass Daten gelesen und ausgewertet werden können.
            </p>
            <p>
            Eine gute Übersicht gibts im Artikel vom CCC: <a href=\"https://www.ccc.de/de/updates/2022/eu-kommission-will-alle-chatnachrichten-durchleuchten\">https://www.ccc.de/de/updates/2022/eu-kommission-will-alle-chatnachrichten-durchleuchten</a>
            "
        },
    Post{
        number: 1,
        name: "WASM on this site",
        prev: "This site now runs WebAssembly!",
        desc: md_to_html!(
r"
Every post is rendered in [WebAssembly](https://webassembly.org/). The full source is available at [Fl1tzi/fl1tzi.com](https://github.com/Fl1tzi/fl1tzi.com/).
"
            )
    },
];

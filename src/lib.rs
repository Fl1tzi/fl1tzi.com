use yew::prelude::*;
use yew::html::Scope;
use web_sys;
use wasm_bindgen::prelude::*;

pub struct Post {
    name: &'static str,
    prev: &'static str,
    desc: &'static str
}

// the posts which are shown
// TODO: make it dynamic
const LEN: usize = 2;
const POSTS: [Post; LEN] = 
    [
    Post{
        name: "WASM on this site",
        prev: "This site now runs WebAssembly!",
        desc: "
            <p style=\"font-weight: 900\">Yes, this site is now running some WebAssembly.</p>
            <p>All posts are rendered from Rust using WASM. You can see the whole source here: <a href=\"https://github.com/Fl1tzi/tgerber.net\">Fl1tzi/tgerber.net</a></p>
            "
    },
    Post {
        name: "Nushell is AWESOME!",
        prev: "My new shell",
        desc: "
        <p>
        I recently discovered <a href=\"https://nushell.sh\">Nushell</a> as a shell. And I was suprised what it can do. </p>
        <p>I really like the data oriented functionality of Nushell, which provides me with tables as an output for many things and the easy piping syntax it has.</p>
        <p>Not only that but it already implements many data types like JSON, YAML, SQLite, Excel, csv and many similar data types. And they allow you to do many things with them. If I like a value of a key out of a JSON file I would usually would open NVIM and search for the key but now I just do </p>

        <code>open config.json | get a.\"1\"</code>

        <p>And it would print this readable table</p>

<pre><code>
╭────────────┬───────╮
│ name       │       │
│ vol        │ 100   │
│ mute       │ false │
│ eq_control │       │
│ eq_name    │       │
│ use_eq     │ false │
│ channels   │ 1     │
╰────────────┴───────╯
</code></pre>


        <p>For example listing only directories in Bash would look like this: </p>

        <code>ls -d */</code>

        <p>If you would see this syntax for the first time you would need to look up what the -d flag does and remember that.<p>
        <p>This on the other hand in Nushell would make it much easier to remember and to use on other commands: </p>
        <code>ls | where type == dir</code>
        <p>And the major thing you see is that a command does not pipe only simple text anymore but also big data structures. It opens up new ways to interact with other commands.</p>
        <p>Nushell did not reach version 1.0 yet and still has some bugs, but I like the concept it is currently following.</p>"
    },
];

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    type Buffer;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

// console log
#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Properties, PartialEq, Default)]
pub struct PostProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct App {
    pub container: NodeRef,
    pub post_prompt_text: &'static str,
    pub post_prompt_title: &'static str,
}

pub enum Msg {
    OpenBox(usize),
    CloseBox
}

impl App {
    // TODO: make it smaller because it almost the same
    fn open_box(&self, _sink: &Scope<Self>) {
        console_log!("Opening post overlay");
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let box_element = document
            .get_element_by_id("post-popup-box")
            .unwrap();
        let box_inner = document
            .get_element_by_id("post-popup-inner")
            .unwrap();
        match box_element.set_attribute("style", "display: block;") {
            Ok(_v) => console_log!("Showing outer box"),
            Err(e) => console_log!("Could not show box: {:?}", e)
        };
        match box_inner.set_attribute("style", "display: block;") {
            Ok(_v) => console_log!("Showing inner box"),
            Err(e) => console_log!("Could not show outer box: {:?}", e)
        }
    }
    fn close_box(&self, _sink: &Scope<Self>) {
        console_log!("Closing post overlay");
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let box_element = document
            .get_element_by_id("post-popup-box")
            .unwrap();
        let box_inner = document
            .get_element_by_id("post-popup-inner")
            .unwrap();
        match box_element.set_attribute("style", "display: none;") {
            Ok(_v) => console_log!("Hiding outer box"),
            Err(e) => console_log!("Could not hide box: {:?}", e)
        };
        match box_inner.set_attribute("style", "display: none;") {
            Ok(_v) => console_log!("Hiding inner box"),
            Err(e) => console_log!("Could not hide outer box: {:?}", e)
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = PostProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container: NodeRef::default(),
            post_prompt_text: "",
            post_prompt_title: "",
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        //let container = self
        //    .container;
        // let container = Html::VRef(container.into());

        match msg {
            Msg::OpenBox(n) => {
                let post = &POSTS[n];
                self.post_prompt_text = post.desc;
                self.post_prompt_title = post.name;
                self.open_box(_ctx.link());
                true
            },
            Msg::CloseBox => {
                self.close_box(_ctx.link());
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let get_rendered_html = |html| -> yew::virtual_dom::VNode {
            let document = gloo_utils::document();
            let element = document.create_element("div").unwrap();
            element.set_inner_html(html);
            Html::VRef(element.into())
        };
        html! {
        <>
        <hr/>
        // verticall FOSS title
        <div class="vertical-left">
            <span>{ " FOSS <3 // "}</span>
        </div>
        // popup
        <div class="post-popup-box" id="post-popup-box">
            <div class="post-popup-inner" id="post-popup-inner">
                <button class="no-btn-style" style="float: right;" onclick={ _ctx.link().callback(|_| Msg::CloseBox) }>
                <span class="close-popup">{ "[X]" }</span>
                </button>
                <h3>{ &self.post_prompt_title }</h3>
                <br/>
                { get_rendered_html(&self.post_prompt_text) }
            </div>
        </div>
        // socials
        <div class="container grid">
            <div class="logo">
                <h1 style="margin-top: 0; margin-left: -25px;">{ "Fl1tzi" }</h1>
                <p style="margin-top: -20px; margin-left: -20px;">{ "kontakt@tgerber.net" }</p>
            </div>
            <div class="card">
                <img src="assets/GitHub.png" alt="GitHub avatar" class="card-logo"/>
                // <img src="http://ghchart.rshah.org/Fl1tzi" alt="Github chart" style="margin-top: 20px; margin-bottom: 10px; width: 90%;"/>
                <p>{ "Profile:" }</p>
                <a class="no-underline" href="https://github.com/Fl1tzi">
                <button class="btn">{ "Tobias (Fl1tzi)" }</button>
                </a>
            </div>
            <div class="card">
                <img src="assets/Discord.png" alt="Discord avatar" class="card-logo"/>
                <p>{ "Profile:" }</p>
                <button id=1 class="btn" >{ "Fl1tzi#0001" }</button>
            </div>
            <div class="card">
                <p style="font-size: 25px;">{ "[ Matrix ]" }</p>
                <p>{ "Find me in the Matrix:" }</p>
                <a class="no-underline" href="https://matrix.to/#/@fl1tzi:server.tgerber.net">
                <button class="btn green-button">{ "@Fl1tzi:server.tgerber.net" }</button>
                </a>
            </div>
         </div>

         <div class="container" id="post-container">
            <div class="logo">
                <h1 style="margin-top: 0; margin-left: -25px;">{ "Posts" }</h1>
            </div>

         <template id="post-template">
            <div class="card">
                <details open=true>
                <summary style="margin: 5px;" class="insert-title"></summary>
                <div class="insert-div"></div>
                </details>
            </div>
          </template>

            { for POSTS.iter().enumerate().map(|(index, post)| {
                                          html! {
                                              <div class="card">
                                                  <h3 class="post-title">
                                                    { post.name }
                                                  </h3>
                                                <p class="post-prev">
                                                    { post.prev }
                                                  </p>

                                                <button onclick={_ctx
                                                    .link()
                                                    .callback(move |_| Msg::OpenBox(index))} class="btn">{ "Open post" }</button>
                                                  </div>

                                          }
                                      }) }

          </div>

        <div class="blur">
        <footer>
            <details style="margin-top: 10px; font-size: 13px; word-break: break-word;">
                // I can't get it to center properly
                <summary style="border: 1px solid black; border-radius: 5px; padding: 10px;">
                <p>{ "Datenschutzerklärung" }</p>
                </summary>
                <p>{"
                Kontakt: kontakt@tgerber.net
                "}</p>
                <p>{"
                Ich verwende für diese Webseite einen Webhosting-Dienst von GitHub Inc. (GitHub Pages).Unser Hoster erhebt in sog. Logfiles folgende Daten, die Ihr Browser übermittelt:

IP-Adresse, die Adresse der vorher besuchten Website (Referer Anfrage-Header), Datum und Uhrzeit der Anfrage, Zeitzonendifferenz zur Greenwich Mean Time, Inhalt der Anforderung, HTTP-Statuscode, übertragene Datenmenge, Website, von der die Anforderung kommt und Informationen zu Browser und Betriebssystem.
                "}</p>

                <p>{"Wir setzen für die Zurverfügungstellung unserer Website folgenden Hoster ein:"}</p>

                <em><p>{"
                GitHub Inc.:
                88 Colin P. Kelly Jr. St.
                San Francisco
                CA 94107 USA
                "}</p></em>

                <p>{ "Dieser ist Empfänger Ihrer personenbezogenen Daten. Dies entspricht unserem berechtigten Interesse im Sinne des Art. 6 Abs. 1 S. 1 lit. f DSGVO, selbst keinen Server in unseren Räumlichkeiten vorhalten zu müssen. Serverstandort ist USA." }</p>
                <p>{"
                Es erfolgt kein Tracking und wir haben auf diese Daten keinen direkten Zugriff, sondern erhalten lediglich eine anonymisierte, statistische Zusammenfassung. Diese beinhaltet die Adresse der vorher besuchten Seite, die Häufigkeit der jeweils aufgerufenen Seiten und die Anzahl eindeutiger Besucher. Diese Daten führen wir nicht mit anderen Daten zusammen.
                "}</p>

                <p>{"
                Weitere Informationen zu Widerspruchs- und Beseitigungsmöglichkeiten gegenüber GitHub finden Sie unter: https://docs.github.com/en/free-pro-team@latest/github/site-policy/github-privacy-statement#github-pages
                    "}</p>

                    <p>{"
                    GitHub hat Compliance-Maßnahmen für internationale Datenübermittlungen umgesetzt. Diese gelten für alle weltweiten Aktivitäten, bei denen GitHub personenbezogene Daten von natürlichen Personen in der EU verarbeitet. Diese Maßnahmen basieren auf den EU-Standardvertragsklauseln (SCCs). Weitere Informationen finden Sie unter: https://docs.github.com/en/free-pro-team@latest/github/site-policy/github-data-protection-addendum#attachment-1–the-standard-contractual-clauses-processors
                    "}</p>

                    <p>{ "Rechtliche Hinweise" }</p>

                    <p>{"
Grundsätzlich ist ein Auftragsverarbeitungsvertrag mit dem Hoster abzuschließen. Das bayerische Landesamt für Datenschutzaufsicht hat für das Hosting rein statischer Websites eine Ausnahme gemacht. Für den Fall, dass die Webseite der Selbstdarstellung dient, z.B. von Vereinen oder Kleinunternehmen, keine personenbezogenen Daten an den Betreiber fließen und kein Tracking stattfindet, liegt keine Auftragsverarbeitung vor. Weiter heißt es: „Die Tatsache, dass auch beim Hosting von statischen Webseiten zwangsläufig IP-Adressen, d.h. personenbezogene Daten, verarbeitet werden müssen, führt nicht zur Annahme einer Auftragsverarbeitung. Das wäre nicht sachgerecht. Die (kurzfristige) IP-Adressenspeicherung ist vielmehr noch der TK-Zugangsvermittlung des Website-Hosters nach dem TKG zuzurechnen und dient in erster Linie Sicherheitszwecken des Hosters.“ (https://www.lda.bayern.de/media/veroeffentlichungen/FAQ_Hosting_keine_Auftragsverarbeitung.pdf) Wir gehen davon aus, dass diese Ausnahme auf GitHub Pages anzuwenden ist.

                    "}</p>

                <p>{"
                GitHub Datenverarbeitungsbedingungen: https://docs.github.com/en/site-policy/privacy-policies/github-data-protection-agreement"}</p>
                <p>{"
                GitHub Datenschutzerklärung: https://docs.github.com/en/site-policy/privacy-policies/github-privacy-statement?tid=134222759
                "}</p>
                    <p>{"
                    Vielen Dank an opr.vs für ihre Arbeit.
                        "}</p>
            </details>
            <p>{ "//" }</p>
        <p>{ "short version: This site does not use any data from you but GitHub (the hoster) could save some data." }</p>
        </footer>
        </div>
        </>
        }
    }

}

/* #[function_component]
fn App() -> Html {
    let _err_text = |e: io::Error| -> Vec<Html> {
        vec![html_nested! { <p style="color: red;"> {
            format!("Error listing posts: {}", e) } </p>
        }]
    };

    let _window = web_sys::window();
    html!{}

}*/

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}

use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Post {
    name: &'static str,
    desc: &'static str
}

// the posts which are shown
// TODO: make it dynamic
const LEN: usize = 2;
const POSTS: [Post; LEN] = 
    [Post{
        name: "Wasm on this site",
        desc: "
            <p style=\"font-weight: 900\">Yes, this site is now running some WebAssembly.</p>
            "
    },
    Post {
        name: "Nushell is AWESOME!",
        desc: "
        <p>
        I recently discovered <a href=\"https://nushell.com\">Nushell</a> as a shell. And I was suprised what it can do. </p>
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
    }];

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
    pub container: NodeRef
}

impl Component for App {
    type Message = Post;
    type Properties = PostProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            container: NodeRef::default(),
        }

    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        //let container = self
        //    .container;
        // let container = Html::VRef(container.into());

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let posts = || -> yew::virtual_dom::VNode {
            let div = gloo_utils::document().create_element("div").unwrap();
            for post in POSTS {
                let post_html = gloo_utils::document().create_element("div").unwrap();
                post_html.set_class_name("card");

                // title
                let title = gloo_utils::document().create_element("h2").unwrap();
                title.set_class_name("post-title");
                title.set_inner_html(post.name);

                let line = gloo_utils::document().create_element("hr").unwrap();
                line.set_class_name("post-line");

                // content
                let content = gloo_utils::document().create_element("div").unwrap();
                content.set_class_name("post-content");
                content.set_inner_html(post.desc);

                post_html.append_child(&title).unwrap();
                post_html.append_child(&line).unwrap();
                post_html.append_child(&content).unwrap();
                div.append_child(&post_html).unwrap();
            }
            // div.set_inner_html(POSTS[0].desc);
            Html::VRef(div.into())
        };
        html! {
        <>
        <div class="container">
            <div class="logo">
                <h1 style="margin-top: 0; margin-left: -25px;">{ "Fl1tzi" }</h1>
                <p style="margin-top: -20px; margin-left: -20px;">{ "kontakt@tgerber.net" }</p>
            </div>
            <div class="card">
                <img src="assets/GitHub.png" alt="GitHub avatar" class="card-logo"/>
                <img src="http://ghchart.rshah.org/Fl1tzi" alt="Github chart" style="margin-top: 20px; margin-bottom: 10px; width: 90%;"/>
                <a class="no-underline" href="https://github.com/Fl1tzi">
                <button class="btn">{ "Tobias (Fl1tzi)" }</button>
                </a>
            </div>
            <div class="card">
                <img src="assets/Discord.png" alt="Discord avatar" class="card-logo"/>
                <p>{ "Profile:" }</p>
                <button id=1 class="btn" >{ "Fl1tzi#0001" }</button>

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

        { posts() }

          </div>

        <div class="blur">
        <footer>
            <p style="margin-left: 10px">{ "This site does not save data from you but does some requests to other servers (:" }</p>
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

fn main() {
    yew::Renderer::<App>::new().render();
}

use epub::book::Book;
use epub::nav::NavItem;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use url::Url;
use wasm_bindgen::prelude::*;
use web_sys::{Event, FileList, HtmlIFrameElement, HtmlInputElement, HtmlLinkElement};
use yew::html::Scope;
use yew::prelude::*;

use log::info;

mod resources;
use resources::Resources;

#[derive(Default)]
#[allow(dead_code)]
pub struct ReadingBook {
    book: Option<Book>,
    file_name: String,
    file_type: String,
    resources: Resources,
}

impl ReadingBook {
    pub fn new(file_name: String, file_type: String, buffer: Vec<u8>) -> Self {
        let book = Book::open_from_memory(buffer).unwrap();
        ReadingBook {
            book: Some(book),
            file_name,
            file_type,
            resources: Resources::default(),
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.book.is_some()
    }

    pub fn title(&self) -> &str {
        if self.book.is_some() {
            self.book.as_ref().unwrap().title()
        } else {
            ""
        }
    }

    pub fn toc(&self) -> &[NavItem] {
        self.book.as_ref().unwrap().nav().unwrap().toc()
    }

    pub fn read_content(&mut self, name: &str) -> String {
        self.resources.clear();
        let mut content = self.book.as_mut().unwrap().content(name).unwrap();

        let res = content.ralative_sources().unwrap();
        for url in res {
            let image_path = url;

            let data = self
                .book
                .as_mut()
                .unwrap()
                .read_binary_file(image_path.as_str())
                .unwrap();

            let dest_url = self
                .resources
                .add_resource(image_path.as_str(), data.as_slice());
            info!("{:?},{:?}",image_path, dest_url);
            content.update_image_url(image_path.as_str(), dest_url.as_str());
        }
        content.to_string().unwrap()
    }

    pub fn create_resources(&mut self) {}
}

pub struct App {
    book: ReadingBook,
    reader: Option<FileReader>,
    page: NodeRef,
    current_path: String,
}

pub enum Msg {
    Open(File),
    CreateBook(String, String, Vec<u8>),
    Content(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            book: ReadingBook::default(),
            reader: None,
            page: NodeRef::default(),
            current_path: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open(file) => {
                let file_name = file.name();
                let file_type = file.raw_mime_type();

                let link = ctx.link().clone();
                let task = {
                    gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        link.send_message(Msg::CreateBook(
                            file_name.clone(),
                            file_type,
                            res.expect("failed to read file"),
                        ))
                    })
                };
                self.reader = Some(task);
                true
            }
            Msg::CreateBook(file_name, file_type, buffer) => {
                self.book = ReadingBook::new(file_name, file_type, buffer);
                if let Some(page) = self.page.cast::<HtmlIFrameElement>() {
                    page.set_srcdoc("");
                }
                // create object for all resources;
                true
            }
            Msg::Content(src) => {
                let page: HtmlIFrameElement = self.page.cast::<HtmlIFrameElement>().unwrap();
                page.set_srcdoc("");
                self.current_path = src.clone();

                let url = Url::parse(src.as_str()).unwrap().path().to_owned();
                let content = self.book.read_content(url.strip_prefix('/').unwrap());
                self.view_page(content, ctx.link());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Epub Reader" }</h1>
                <input type="file"
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::open_files(input.files())
                    })}
                />
                <div>
                    { self.view_book(ctx.link()) }
                </div>
           </div>
        }
    }
}
impl App {
    pub fn view_book(&self, link: &Scope<Self>) -> Html {
        html! {
            <div>
                <h1>{format!("{}",self.book.title())}</h1>
                if self.book.is_loaded(){
                  <div id="book-area">
                     <div id="book-nav">
                     {self.nav_view(link)}
                     </div>
                           <iframe ref={&self.page} title="Iframe Example" id="book-page">
                           </iframe>
                  </div>
                }
            </div>
        }
    }

    pub fn open_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }

        Msg::Open(result.first().unwrap().to_owned())
    }

    pub fn view_page(&self, content: String, link: &Scope<Self>) {
        let page: HtmlIFrameElement = self.page.cast::<HtmlIFrameElement>().unwrap();
        page.set_srcdoc(&content);

        let click_handler = link.callback(move |e: MouseEvent| {
            e.prevent_default();
            let t: HtmlLinkElement = e.target_unchecked_into();
            Msg::Content(t.href())
        });

        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            event.prevent_default();
            click_handler.emit(event)
        });
        let current_path = self.current_path.clone();

        let call = Closure::<dyn FnMut(_)>::new(move |e: Event| {
            let iframe: HtmlIFrameElement = e.target().unwrap().dyn_into().unwrap();
            let document = iframe.content_document().unwrap();
            let base = document.create_element("base").unwrap();
            base.set_attribute("href", current_path.as_str()).unwrap();
            document.body().unwrap().append_child(&base).unwrap();

            let links = document.query_selector_all("a").unwrap();
            let mut i = 0;
            while i < links.length() {
                let a = links.get(i).unwrap();
                i += 1;
                a.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
            }
        });
        page.add_event_listener_with_callback("load", call.as_ref().unchecked_ref())
            .unwrap();
        call.forget();
    }

    pub fn nav_view(&self, link: &Scope<Self>) -> Html {
        let toc = self.book.toc();
        html! {
            <div>
            {for toc.iter().map(|tc|
                                html!{
                <li>
                  <a href={tc.href().to_string()}
                  onclick=
                  {link.callback(|e:MouseEvent|{
                        e.prevent_default();
                        let a: HtmlLinkElement= e.target_unchecked_into();
                        Msg::Content(a.href())
                  }
                                         )}
                  >
                 {tc.text()}
                 </a>
                 if !tc.children().is_empty(){
                    <ul>
                     {for tc.children().iter().map(|child|html!{
                        <li>
                               <a href={child.href().to_string()}
                               onclick={link.callback(|e:MouseEvent|{
                                     e.prevent_default();
                                     let a: HtmlLinkElement= e.target_unchecked_into();
                                     Msg::Content(a.href())
                               }
                                                      )}>
                                {child.text()}
                                </a>
                        </li>
                     })
                     }
                   </ul>
                 }
                </li>}
                )}
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<App>::new().render();
}

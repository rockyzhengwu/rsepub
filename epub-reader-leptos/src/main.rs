use gloo::file::{Blob, File};
use leptos::*;
use log::info;
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};

mod book;
mod resources;

async fn read_file(file: Blob) -> Vec<u8> {
    let buffer = gloo::file::futures::read_as_bytes(&file).await.unwrap();
    let book = book::ReadingBook::new(buffer.clone());
    info!("readedbook:{:?}", book.title());

    buffer
}

#[derive(Clone, Default)]
struct GlobalState {}

#[component]
fn App() -> impl IntoView {
    provide_context(GlobalState::default());

    let upload = create_action(|blob: &Blob| {
        // `task` is given as `&String` because its value is available in `input`
        info!("upload");
        let blob = blob.to_owned();
        async move { read_file(blob).await }
    });

    view! {
      <input type="file" on:change=move |e| {
          let target:EventTarget = e.target().unwrap();
          let input:HtmlInputElement = target.dyn_into::<HtmlInputElement>().unwrap();

         // TODO: Can this be simple ?
        let mut files = Vec::new();
        if let Some(fs) = input.files() {
            let f = js_sys::try_iter(&fs)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            files.extend(f);
        }
        info!("{:?},{:?}",files[0].name(),files[0].raw_mime_type());
        let file:Blob= files.first().unwrap().to_owned().into();
        upload.dispatch(file);


      }/>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}

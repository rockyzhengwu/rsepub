use std::collections::HashMap;
use web_sys::{Blob, BlobPropertyBag, Url};

#[derive(Default)]
pub struct Resources {
    urlmap: HashMap<String, String>,
    blobs: Vec<Blob>,
}

impl Resources {
    fn get_datatype(&self, url: &str) -> &str {
        let sts = url.split('.');
        let ext = sts.last().unwrap();
        match ext {
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "css" => "text/css",
            _ => "",
        }
    }

    pub fn add_resource(&mut self, url: &str, content: &[u8]) -> String {
        let uint8arr =
            js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(content) }.into());
        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());
        let blob = Blob::new_with_u8_array_sequence_and_options(
            &array,
            BlobPropertyBag::new().type_(self.get_datatype(url)),
        )
        .unwrap();

        let dest_url = Url::create_object_url_with_blob(&blob).unwrap();
        self.urlmap.insert(url.to_string(), dest_url.clone());
        self.blobs.push(blob);
        dest_url
    }

    pub fn clear(&mut self) {
        self.urlmap.clear();
        self.blobs.clear();
    }
}

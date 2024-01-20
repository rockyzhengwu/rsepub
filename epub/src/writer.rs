pub struct ZipWriter {}
pub struct DirWriter {}

pub enum Writer {
    Zip(ZipWriter),
    Dir(DirWriter),
}

pub trait Host {
    fn get_file(&self, url: &str);
}

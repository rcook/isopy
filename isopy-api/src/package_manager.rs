use crate::host::Host;

pub trait PackageManager {
    fn test(&self, host: &Box<dyn Host>);
}

use crate::result::Result;
use reqwest::{IntoUrl, Url};

#[allow(unused)]
pub fn file_url<U>(url: U) -> Result<Url>
where
    U: IntoUrl,
{
    helper(url, false)
}

#[allow(unused)]
pub fn dir_url<U>(url: U) -> Result<Url>
where
    U: IntoUrl,
{
    helper(url, true)
}

fn helper<U>(url: U, add_slash: bool) -> Result<Url>
where
    U: IntoUrl,
{
    let mut temp = url.into_url()?;
    let mut p = String::from(temp.path());
    while p.ends_with('/') {
        _ = p.pop().expect("must succeed")
    }
    if add_slash {
        p.push('/')
    }
    temp.set_path(&p);
    Ok(temp)
}

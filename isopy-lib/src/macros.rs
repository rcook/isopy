// Copyright (c) 2023 Richard Cook
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub trait TryToString: Sized {
    fn to_string_lossy(&self) -> String;
    fn try_to_string(&self) -> Option<String>;
}

#[macro_export]
macro_rules! serializable_newtype {
    ($name: ident, $inner_type: ty) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name($inner_type);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(
                    f,
                    "{}",
                    <Self as $crate::TryToString>::to_string_lossy(&self)
                )
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_str(
                    &<Self as $crate::TryToString>::try_to_string(&self).ok_or_else(|| {
                        ::serde::ser::Error::custom("failed to serialize as string")
                    })?,
                )
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <Self as ::std::str::FromStr>::from_str(&String::deserialize(deserializer)?)
                    .map_err(::serde::de::Error::custom)
            }
        }
    };
}

macro_rules! dyn_trait_struct {
    ($name : ident, $trait: ident) => {
        #[derive(derive_more::Deref)]
        pub struct $name(Box<dyn $trait>);

        impl $name {
            pub fn new<T: $trait + 'static>(inner: T) -> Self {
                Self(Box::new(inner))
            }
        }
    };
}

pub(crate) use dyn_trait_struct;

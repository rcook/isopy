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
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;
use uuid::Uuid;

use crate::RepoError;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MetaId(Uuid);

impl MetaId {
    #[must_use]
    pub fn random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for MetaId {
    type Err = RepoError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        s.parse::<Uuid>()
            .map_err(|_e| RepoError::invalid_meta_id(s))
            .map(Self)
    }
}

impl Display for MetaId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.as_simple())
    }
}

impl Serialize for MetaId {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0.as_simple()))
    }
}

impl<'de> Deserialize<'de> for MetaId {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(Uuid::deserialize(deserializer)?))
    }
}

#[cfg(test)]
mod tests {
    use super::MetaId;
    use anyhow::Result;
    use rstest::rstest;
    use uuid::Uuid;

    #[rstest]
    #[case(
        "41941524f6da41dfa06fdb5c55f32a3d",
        "41941524-F6DA-41DF-A06F-DB5C55F32A3D"
    )]
    #[case(
        "41941524f6da41dfa06fdb5c55f32a3d",
        "41941524-f6da-41df-a06f-db5c55f32a3d"
    )]
    #[case("41941524f6da41dfa06fdb5c55f32a3d", "41941524F6DA41DFA06FDB5C55F32A3D")]
    #[case("41941524f6da41dfa06fdb5c55f32a3d", "41941524f6da41dfa06fdb5c55f32a3d")]
    #[case(
        "41941524f6da41dfa06fdb5c55f32a3d",
        "{41941524-F6DA-41DF-A06F-DB5C55F32A3D}"
    )]
    #[case(
        "41941524f6da41dfa06fdb5c55f32a3d",
        "{41941524-f6da-41df-a06f-db5c55f32a3d}"
    )]
    fn parse_basics(#[case] expected_str: &str, #[case] input: &str) -> Result<()> {
        assert_eq!(expected_str, input.parse::<MetaId>()?.to_string());
        Ok(())
    }

    #[rstest]
    #[case("")]
    #[case("garbage")]
    fn parse_error(#[case] input: &str) {
        assert!(input.parse::<MetaId>().is_err());
    }

    #[test]
    fn random_basics() {
        assert!(MetaId::random().to_string().parse::<Uuid>().is_ok());
    }
}

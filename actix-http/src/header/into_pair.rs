//! [`TryIntoHeaderPair`] trait and implementations.

use super::{
    Header, HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue, TryIntoHeaderValue,
};
use crate::error::HttpError;

/// An interface for types that can be converted into a [`HeaderName`] + [`HeaderValue`] pair for
/// insertion into a [`HeaderMap`].
///
/// [`HeaderMap`]: super::HeaderMap
pub trait TryIntoHeaderPair: Sized {
    type Error: Into<HttpError>;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error>;

    #[doc(hidden)]
    fn try_into_pair_with_name(
        self,
    ) -> Result<(HeaderName, HeaderValue, Option<String>), Self::Error> {
        let (name, value) = self.try_into_pair()?;
        Ok((name, value, None))
    }
}

#[derive(Debug)]
pub enum InvalidHeaderPart {
    Name(InvalidHeaderName),
    Value(InvalidHeaderValue),
}

impl From<InvalidHeaderPart> for HttpError {
    fn from(part_err: InvalidHeaderPart) -> Self {
        match part_err {
            InvalidHeaderPart::Name(err) => err.into(),
            InvalidHeaderPart::Value(err) => err.into(),
        }
    }
}

impl<V> TryIntoHeaderPair for (HeaderName, V)
where
    V: TryIntoHeaderValue,
    V::Error: Into<InvalidHeaderValue>,
{
    type Error = InvalidHeaderPart;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let (name, value) = self;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name, value))
    }
}

impl<V> TryIntoHeaderPair for (&HeaderName, V)
where
    V: TryIntoHeaderValue,
    V::Error: Into<InvalidHeaderValue>,
{
    type Error = InvalidHeaderPart;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let (name, value) = self;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name.clone(), value))
    }
}

impl<V> TryIntoHeaderPair for (&[u8], V)
where
    V: TryIntoHeaderValue,
    V::Error: Into<InvalidHeaderValue>,
{
    type Error = InvalidHeaderPart;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let (name, value) = self;
        let name = HeaderName::try_from(name).map_err(InvalidHeaderPart::Name)?;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name, value))
    }

    fn try_into_pair_with_name(
        self,
    ) -> Result<(HeaderName, HeaderValue, Option<String>), Self::Error> {
        let (name, value) = self;
        let header_name = HeaderName::try_from(name).map_err(InvalidHeaderPart::Name)?;
        let original_name = std::str::from_utf8(name)
            .expect("valid header names are ASCII")
            .to_owned();
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((header_name, value, Some(original_name)))
    }
}

impl<V> TryIntoHeaderPair for (&str, V)
where
    V: TryIntoHeaderValue,
    V::Error: Into<InvalidHeaderValue>,
{
    type Error = InvalidHeaderPart;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let (name, value) = self;
        let name = HeaderName::try_from(name).map_err(InvalidHeaderPart::Name)?;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name, value))
    }

    fn try_into_pair_with_name(
        self,
    ) -> Result<(HeaderName, HeaderValue, Option<String>), Self::Error> {
        let (name, value) = self;
        let original_name = name.to_owned();
        let name = HeaderName::try_from(name).map_err(InvalidHeaderPart::Name)?;
        let value = value
            .try_into_value()
            .map_err(|err| InvalidHeaderPart::Value(err.into()))?;
        Ok((name, value, Some(original_name)))
    }
}

impl<V> TryIntoHeaderPair for (String, V)
where
    V: TryIntoHeaderValue,
    V::Error: Into<InvalidHeaderValue>,
{
    type Error = InvalidHeaderPart;

    #[inline]
    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        let (name, value) = self;
        (name.as_str(), value).try_into_pair()
    }

    fn try_into_pair_with_name(
        self,
    ) -> Result<(HeaderName, HeaderValue, Option<String>), Self::Error> {
        let (name, value) = self;
        (name.as_str(), value).try_into_pair_with_name()
    }
}

impl<T: Header> TryIntoHeaderPair for T {
    type Error = <T as TryIntoHeaderValue>::Error;

    #[inline]
    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        Ok((T::name(), self.try_into_value()?))
    }
}

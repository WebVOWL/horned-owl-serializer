use std::{error::Error, panic::Location};

use horned_owl::error::HornedError;
use oxigraph::{model::IriParseError, store::{LoaderError, StorageError}};

#[derive(Debug)]
pub enum HornedOxiErrorKind {
    InvalidInput(String),
    HornedError(HornedError),
    OxigraphError(StorageError),
    IOError(std::io::Error),
    LoaderError(LoaderError),
    IriParseError(IriParseError),
}

#[derive(Debug)]
pub struct HornedOxiError {
    inner: HornedOxiErrorKind,
    location: &'static Location<'static>
}

impl From<HornedError> for HornedOxiError {
    #[track_caller]
    fn from(error: HornedError) -> Self {
        HornedOxiError {
            inner: HornedOxiErrorKind::HornedError(error),
            location: &Location::caller()
        }
    }
}

impl From<StorageError> for HornedOxiError {
    #[track_caller]
    fn from(error: StorageError) -> Self {
        HornedOxiError {
            inner: HornedOxiErrorKind::OxigraphError(error),
            location: &Location::caller()
        }
    }
}

impl From<LoaderError> for HornedOxiError {
    #[track_caller]
    fn from(error: LoaderError) -> Self {
        HornedOxiError {
            inner: HornedOxiErrorKind::LoaderError(error),
            location: &Location::caller()
        }
    }
}

impl From<IriParseError> for HornedOxiError {   
    #[track_caller]
    fn from(error: IriParseError) -> Self {
        HornedOxiError {
            inner: HornedOxiErrorKind::IriParseError(error),
            location: &Location::caller()
        }
    }
}

impl From<HornedOxiErrorKind> for HornedOxiError {
    #[track_caller]
    fn from(error: HornedOxiErrorKind) -> Self {
        HornedOxiError {
            inner: error,
            location: &Location::caller()
        }
    }
}

impl From<std::io::Error> for HornedOxiError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        HornedOxiError {
            inner: HornedOxiErrorKind::IOError(error),
            location: &Location::caller()
        }
    }
}

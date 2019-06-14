use std::error::Error;
use std::fmt;
use toml;
use toml_query;

use crate::blockchain;

#[derive(Debug)]
pub enum DeploymentError {
  MissingConfig,
  InvalidParamType(ethabi::Error),
  TokenizeParam(ethabi::Error, String),
  NothingToDeploy,
  MissingArtifact(String, String),
  TooManyConstructorArgs(String),
  Connection(blockchain::error::ConnectionError),
  DeployContract(web3::contract::deploy::Error, String),
  InvalidConstructorArgs(ethabi::Error, String),
  TrackingError(DeploymentTrackingError),
  Other(String),
}

impl Error for DeploymentError {
  fn cause(&self) -> Option<&Error> {
    match self {
      DeploymentError::MissingConfig => None,
      DeploymentError::InvalidParamType(error) => Some(error),
      DeploymentError::TokenizeParam(error, _value) => Some(error),
      DeploymentError::NothingToDeploy => None,
      DeploymentError::MissingArtifact(_kind, _name) => None,
      DeploymentError::TooManyConstructorArgs(_name) => None,
      DeploymentError::Connection(error) => Some(error),
      DeploymentError::DeployContract(error, _name) => Some(error),
      DeploymentError::InvalidConstructorArgs(error, _name) => Some(error),
      DeploymentError::TrackingError(error) => Some(error),
      DeploymentError::Other(_message) => None,
    }
  } 
}

impl fmt::Display for DeploymentError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DeploymentError::MissingConfig => write!(f, "Missing deployment configuration."),
      DeploymentError::InvalidParamType(error) => write!(f, "Couldn't read Smart Contract constructor parameter: {}", error),
      DeploymentError::TokenizeParam(error, value) => write!(f, "Couldn't tokenize Smart Contract constructor parameter: {} with value {:?}", error, value),
      DeploymentError::NothingToDeploy => write!(f, "Couldn't find artifacts to deploy. Please compile first."),
      DeploymentError::MissingArtifact(kind, name) => write!(f, "Couldn't find {} file for artifact '{}'", kind, name),
      DeploymentError::TooManyConstructorArgs(name) => write!(f, "Couldn't deploy Smart Contract '{}' due to too many constructor arguments (max. 10)", name),
      DeploymentError::Connection(error) => write!(f, "{}", error),
      DeploymentError::DeployContract(error, name) => write!(f, "Couldn't deploy Smart Contract '{}' due to {}", name, error),
      DeploymentError::InvalidConstructorArgs(_error, name) => write!(f, "Couldn't deploy Smart Contract '{}' due to mismatching types in constructor arguments.", name),
      DeploymentError::TrackingError(error) => write!(f, "Couldn't track deployed Smart Contracts: {}", error),
      DeploymentError::Other(message) => write!(f, "{}", message),
    }
  }
}


#[derive(Debug)]
pub enum DeploymentTrackingError {
  Other(String),
  Deserialization(toml::de::Error),
  Serialization(toml::ser::Error),
  Insertion(toml_query::error::Error),
  Read(toml_query::error::Error),
  Set(toml_query::error::Error),
  Delete(toml_query::error::Error),
}

impl Error for DeploymentTrackingError {
  fn cause(&self) -> Option<&Error> {
    match self {
      DeploymentTrackingError::Other(_) => None,
      DeploymentTrackingError::Deserialization(error) => Some(error),
      DeploymentTrackingError::Serialization(error) => Some(error),
      DeploymentTrackingError::Insertion(_error) => None,
      DeploymentTrackingError::Read(_error) => None,
      DeploymentTrackingError::Set(_error) => None,
      DeploymentTrackingError::Delete(_error) => None,
    }
  }
}

impl fmt::Display for DeploymentTrackingError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DeploymentTrackingError::Other(message) => write!(f, "{}", message),
      DeploymentTrackingError::Deserialization(error) => write!(f, "Couldn't deserialize tracking data: {}", error),
      DeploymentTrackingError::Serialization(error) => write!(f, "Couldn't serialize tracking data: {}", error),
      DeploymentTrackingError::Insertion(error) => write!(f, "Couldn't insert tracking data before writing to disc: {}", error),
      DeploymentTrackingError::Read(error) => write!(f, "Couldn't read tracking data: {}", error),
      DeploymentTrackingError::Set(error) => write!(f, "Couldn't set tracking data: {}", error),
      DeploymentTrackingError::Delete(error) => write!(f, "Couldn't delete tracking data: {}", error),
    }
  }
}

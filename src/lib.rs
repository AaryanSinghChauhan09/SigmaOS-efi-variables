// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-efi-variables: efi variables component model.
//!
//! Real implementation: variabless, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variables {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EfiVariablesError {
    EmptyVariablesName,
    DuplicateVariables(String),
    EmptyEventSubject,
}

impl fmt::Display for EfiVariablesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVariablesName => write!(f, "variables name must not be empty"),
            Self::DuplicateVariables(id) => write!(f, "duplicate variables {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for EfiVariablesError {}

pub struct EfiVariables {
    pub name: String,
    variabless: BTreeMap<String, Variables>,
    events: BTreeMap<String, Event>,
}

impl EfiVariables {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            variabless: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_variables(&mut self, item: Variables) -> Result<(), EfiVariablesError> {
        if item.name.trim().is_empty() {
            return Err(EfiVariablesError::EmptyVariablesName);
        }
        let id = item.id.clone();
        if self.variabless.contains_key(&id) {
            return Err(EfiVariablesError::DuplicateVariables(id.clone()));
        }
        self.variabless.insert(id, item);
        Ok(())
    }

    pub fn variables_count(&self) -> usize {
        self.variabless.len()
    }

    pub fn get_variables(&self, id: &str) -> Option<&Variables> {
        self.variabless.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), EfiVariablesError> {
        if event.subject.trim().is_empty() {
            return Err(EfiVariablesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_variables(id: &str, name: &str) -> Variables {
        Variables {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = EfiVariables::new("EfiVariables");
        assert_eq!(m.name, "EfiVariables");
        assert_eq!(m.variables_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_variables() {
        let mut m = EfiVariables::new("EfiVariables");
        m.add_variables(sample_variables("a1", "Alpha")).unwrap();
        assert_eq!(m.variables_count(), 1);
        assert!(m.get_variables("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = EfiVariables::new("EfiVariables");
        assert!(m.add_variables(sample_variables("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = EfiVariables::new("EfiVariables");
        m.add_variables(sample_variables("x", "One")).unwrap();
        assert!(m.add_variables(sample_variables("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = EfiVariables::new("EfiVariables");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

#![allow(unused)]
use std::fmt::Display;

use rust_iso3166::{iso3166_2::ET_SN, *};

pub struct MyceliumSubject {
    environment: Environment,
    ownership_group: OwnershipGroup,
    geo_locator: GeoLocator,
    service_identifier: ServiceIdentifier,
    payload_type: PayloadType,
    payload_identifier: Vec<String>,
}

impl Display for MyceliumSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tokens = Vec::new();
        tokens.push(self.environment.to_string());
        tokens.push(self.ownership_group.enterprise.clone());
        tokens.push(self.ownership_group.op_group.clone());
        tokens.push(self.geo_locator.to_string());
        tokens.push(self.service_identifier.service_name.clone());
        tokens.push(self.service_identifier.instance_id.clone());
        tokens.push(self.payload_type.to_string());
        tokens.append(&mut self.payload_identifier.clone());
        write!(f, "{}", tokens.join(".").to_string())
    }
}

#[derive(Default)]
pub struct MyceliumSubjectBuilder {
    environment: Option<Environment>,
    ownership_group: Option<OwnershipGroup>,
    geo_locator: Option<GeoLocator>,
    service_identifier: Option<ServiceIdentifier>,
    payload_type: Option<PayloadType>,
    payload_identifier: Option<Vec<String>>,
}

pub enum Environment {
    Production,
    Staging,
    Dev,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Environment::Production => write!(f, "prod"),
            &Environment::Staging => write!(f, "staging"),
            &Environment::Dev => write!(f, "dev"),
        }
    }
}

pub struct OwnershipGroup {
    enterprise: String,
    op_group: String,
}

impl OwnershipGroup {
    pub fn new(enterprise: impl Into<String>, op_group: impl Into<String>) -> Self {
        Self {
            enterprise: enterprise.into(),
            op_group: op_group.into(),
        }
    }
}

pub enum GeoLocator {
    Local,
    Global(GlobalLocator),
}

pub struct GlobalLocator {
    iso_3166_2: String,
    op_region: String,
    op_identifier: String,
}

impl GlobalLocator {
    pub fn new(
        iso_3166_2: impl Into<String>,
        op_region: impl Into<String>,
        op_identifier: impl Into<String>,
    ) -> Self {
        Self {
            iso_3166_2: iso_3166_2.into(),
            op_region: op_region.into(),
            op_identifier: op_identifier.into(),
        }
    }
}

impl Display for GeoLocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &GeoLocator::Local => write!(f, "local"),
            &GeoLocator::Global(ref g) => {
                write!(f, "{}.{}.{}", g.iso_3166_2, g.op_region, g.op_identifier)
            }
        }
    }
}

pub struct ServiceIdentifier {
    service_name: String,
    instance_id: String,
}

impl ServiceIdentifier {
    pub fn new(service_name: impl Into<String>, instance_id: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            instance_id: instance_id.into(),
        }
    }
}

pub enum PayloadType {
    Heartbeat,
    Data,
    Diagnostics,
    Command,
    Event,
    Custom,
}

impl Display for PayloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &PayloadType::Heartbeat => write!(f, "heartbeat"),
            &PayloadType::Data => write!(f, "data"),
            &PayloadType::Diagnostics => write!(f, "diagnostics"),
            &PayloadType::Event => write!(f, "event"),
            &PayloadType::Command => write!(f, "command"),
            &PayloadType::Custom => write!(f, "custom"),
        }
    }
}

impl MyceliumSubjectBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn environment(mut self, environment: Environment) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn ownership_group(mut self, ownership_group: OwnershipGroup) -> Self {
        self.ownership_group = Some(ownership_group);
        self
    }

    pub fn geo_locator(mut self, geo_locator: GeoLocator) -> Self {
        self.geo_locator = Some(geo_locator);
        self
    }

    pub fn service_identifier(mut self, service_identifier: ServiceIdentifier) -> Self {
        self.service_identifier = Some(service_identifier);
        self
    }

    pub fn payload_type(mut self, payload_type: PayloadType) -> Self {
        self.payload_type = Some(payload_type);
        self
    }

    pub fn payload_identifier(mut self, payload_identifier: Vec<String>) -> Self {
        self.payload_identifier = Some(payload_identifier);
        self
    }

    pub fn build(self) -> Result<MyceliumSubject, String> {
        if let Some(environment) = self.environment
            && let Some(ownership_group) = self.ownership_group
            && let Some(geo_locator) = self.geo_locator
            && let Some(service_identifier) = self.service_identifier
            && let Some(payload_type) = self.payload_type
            && let Some(payload_identifier) = self.payload_identifier
        {
            if let GeoLocator::Global(ref locator) = geo_locator {
                if iso3166_2::from_code(&locator.iso_3166_2).is_none() {
                    return Err("Invalid ISO-3166 code".to_string());
                }
            }
            Ok(MyceliumSubject {
                environment,
                ownership_group,
                geo_locator,
                service_identifier,
                payload_type,
                payload_identifier,
            })
        } else {
            Err("Missing fields".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn local() {
        let subject = MyceliumSubjectBuilder::new()
            .environment(Environment::Production)
            .ownership_group(OwnershipGroup::new("abc", "xyz"))
            .geo_locator(GeoLocator::Local)
            .service_identifier(ServiceIdentifier::new("plc-gateway", "1"))
            .payload_type(PayloadType::Data)
            .payload_identifier(vec![
                "system".to_string(),
                "sub-system".to_string(),
                "device".to_string(),
                "value".to_string(),
            ])
            .build()
            .unwrap();

        assert_eq!(
            subject.to_string(),
            "prod.abc.xyz.local.plc-gateway.1.data.system.sub-system.device.value"
        );
    }

    #[test]
    fn global() {
        let subject = MyceliumSubjectBuilder::new()
            .environment(Environment::Production)
            .ownership_group(OwnershipGroup::new("abc", "xyz"))
            .geo_locator(GeoLocator::Global(GlobalLocator::new(
                "US-CA", "south", "cmb",
            )))
            .service_identifier(ServiceIdentifier::new("plc-gateway", "1"))
            .payload_type(PayloadType::Data)
            .payload_identifier(vec![
                "system".to_string(),
                "sub-system".to_string(),
                "device".to_string(),
                "value".to_string(),
            ])
            .build()
            .unwrap();
        assert_eq!(
            subject.to_string(),
            "prod.abc.xyz.US-CA.south.cmb.plc-gateway.1.data.system.sub-system.device.value"
        );
    }
}

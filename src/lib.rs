#![allow(unused)]
use rust_iso3166::iso3166_2;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Environment {
    Production,
    Staging,
    Dev,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Production => write!(f, "prod"),
            Environment::Staging => write!(f, "staging"),
            Environment::Dev => write!(f, "dev"),
        }
    }
}

impl FromStr for Environment {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "prod" => Ok(Environment::Production),
            "staging" => Ok(Environment::Staging),
            "dev" => Ok(Environment::Dev),
            _ => Err("Invalid Environment string"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OwnershipGroup {
    enterprise: String,
    op_group: String,
}

impl Display for OwnershipGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.enterprise, self.op_group)
    }
}

impl FromStr for OwnershipGroup {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 2 {
            Ok(OwnershipGroup {
                enterprise: parts[0].to_string(),
                op_group: parts[1].to_string(),
            })
        } else {
            Err("Invalid OwnershipGroup string format. Expected: enterprise.op_group")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Locator {
    iso_3166_2: String,
    op_region: String,
    op_identifier: String,
}

impl Display for Locator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.iso_3166_2, self.op_region, self.op_identifier
        )
    }
}

impl FromStr for Locator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 3 {
            let iso3166_2_code = parts[0];
            if iso3166_2::from_code(iso3166_2_code).is_none() {
                Err("Invalid ISO 3166-2 code")
            } else {
                Ok(Locator {
                    iso_3166_2: iso3166_2_code.to_string(),
                    op_region: parts[1].to_string(),
                    op_identifier: parts[2].to_string(),
                })
            }
        } else {
            Err("Invalid GlobalLocator string format. Expected: iso.region.id")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GeoLocator {
    Local,
    Global,
    Locator(Locator),
}

impl Display for GeoLocator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoLocator::Local => write!(f, "local"),
            GeoLocator::Global => write!(f, "global"),
            GeoLocator::Locator(g) => write!(f, "{g}"),
        }
    }
}

impl FromStr for GeoLocator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 1 {
            Ok(Self::Local)
        } else {
            Ok(Self::Locator(
                Locator::from_str(s).map_err(|_| "Invalid GlobalLocator string format.")?,
            ))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceIdentifier {
    service_name: String,
    instance_id: String,
}

impl Display for ServiceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.service_name, self.instance_id)
    }
}

impl FromStr for ServiceIdentifier {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 2 {
            Ok(ServiceIdentifier {
                service_name: parts[0].to_string(),
                instance_id: parts[1].to_string(),
            })
        } else {
            Err("Invalid ServiceIdentifier string format. Expected: name.id")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
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
            PayloadType::Heartbeat => write!(f, "heartbeat"),
            PayloadType::Data => write!(f, "data"),
            PayloadType::Diagnostics => write!(f, "diagnostics"),
            PayloadType::Command => write!(f, "command"),
            PayloadType::Event => write!(f, "event"),
            PayloadType::Custom => write!(f, "custom"),
        }
    }
}

impl FromStr for PayloadType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heartbeat" => Ok(PayloadType::Heartbeat),
            "data" => Ok(PayloadType::Data),
            "diagnostics" => Ok(PayloadType::Diagnostics),
            "command" => Ok(PayloadType::Command),
            "event" => Ok(PayloadType::Event),
            "custom" => Ok(PayloadType::Custom),
            _ => Err("Invalid PayloadType string"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MyceliumSubject {
    pub environment: Environment,
    pub ownership_group: OwnershipGroup,
    pub geo_locator: GeoLocator,
    pub service_identifier: ServiceIdentifier,
    pub payload_type: PayloadType,
    pub payload_identifier: Vec<String>,
}

impl Display for MyceliumSubject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}",
            self.environment,
            self.ownership_group,
            self.geo_locator,
            self.service_identifier,
            self.payload_type
        )?;

        for part in &self.payload_identifier {
            write!(f, ".{}", part)?;
        }
        Ok(())
    }
}

impl FromStr for MyceliumSubject {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() < 7 {
            return Err("String too short to represent a local or global MyceliumSubject");
        }
        let environment = Environment::from_str(parts[0])?;
        let ownership_group = OwnershipGroup::from_str(&format!("{}.{}", parts[1], parts[2]))?;

        let geo_locator;
        let mut global_offset = 0;
        if parts[3] == "local" {
            geo_locator = GeoLocator::Local;
        } else if parts[3] == "global" {
            geo_locator = GeoLocator::Global;
        } else {
            global_offset = 2;
            if parts.len() < 9 {
                return Err("String too short to represent a global MyceliumSubject");
            }
            let global_locator_str = format!("{}.{}.{}", parts[3], parts[4], parts[5]);
            geo_locator = GeoLocator::Locator(Locator::from_str(&global_locator_str)?);
        }
        let service_identifier = ServiceIdentifier::from_str(&format!(
            "{}.{}",
            parts[4 + global_offset],
            parts[5 + global_offset]
        ))?;

        let payload_type = PayloadType::from_str(parts[6 + global_offset])?;

        let payload_identifier: Vec<String> = parts[(7 + global_offset)..]
            .iter()
            .map(|s| s.to_string())
            .collect();

        Ok(MyceliumSubject {
            environment,
            ownership_group,
            geo_locator,
            service_identifier,
            payload_type,
            payload_identifier,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn locator_from_string_success() {
        let subject_string =
            "prod.abc.xyz.US-CA.south.abc.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn locator_from_string_fail_bad_environment() {
        let subject_string =
            "production.abc.xyz.US-CA.south.abc.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn locator_from_string_no_payload_id() {
        let subject_string = "prod.abc.xyz.US-CA.south.abc.plc-gateway.1.data";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn locator_from_string_fail_no_payload_type() {
        let subject_string = "prod.abc.xyz.US-CA.south.abc.plc-gateway.1";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn locator_from_string_fail_bad_iso_code() {
        let subject_string = "prod.abc.xyz.US-AA.south.abc.plc-gateway.1";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn local_from_string_success() {
        let subject_string = "prod.abc.xyz.local.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn local_from_string_fail_bad_environment() {
        let subject_string =
            "production.abc.xyz.local.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn local_from_string_no_payload_id() {
        let subject_string = "prod.abc.xyz.local.plc-gateway.1.data";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn local_from_string_fail_no_payload_type() {
        let subject_string = "prod.abc.xyz.local.plc-gateway.1";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn local_from_string_fail_bad_payload_id() {
        let subject_string = "prod.abc.xyz.local.plc-gateway.1.datas";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn global_from_string_success() {
        let subject_string =
            "prod.abc.xyz.global.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn global_from_string_fail_bad_environment() {
        let subject_string =
            "production.abc.xyz.global.plc-gateway.1.data.system.sub-system.sensor.value";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn global_from_string_no_payload_id() {
        let subject_string = "prod.abc.xyz.global.plc-gateway.1.data";
        let res = MyceliumSubject::from_str(subject_string).unwrap();
        assert_eq!(subject_string, res.to_string());
    }

    #[test]
    fn global_from_string_fail_no_payload_type() {
        let subject_string = "prod.abc.xyz.global.plc-gateway.1";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }

    #[test]
    fn global_from_string_fail_bad_payload_id() {
        let subject_string = "prod.abc.xyz.global.plc-gateway.1.datas";
        let res = MyceliumSubject::from_str(subject_string);
        assert!(res.is_err());
    }
}

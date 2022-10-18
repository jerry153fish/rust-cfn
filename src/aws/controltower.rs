//! Types for the `ControlTower` service.

/// The [`AWS::ControlTower::EnabledControl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html) resource type.
#[derive(Debug, Default)]
pub struct EnabledControl {
    properties: EnabledControlProperties
}

/// Properties for the `EnabledControl` resource.
#[derive(Debug, Default)]
pub struct EnabledControlProperties {
    /// Property [`ControlIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html#cfn-controltower-enabledcontrol-controlidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub control_identifier: ::Value<String>,
    /// Property [`TargetIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-controltower-enabledcontrol.html#cfn-controltower-enabledcontrol-targetidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub target_identifier: ::Value<String>,
}

impl ::serde::Serialize for EnabledControlProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ControlIdentifier", &self.control_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetIdentifier", &self.target_identifier)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EnabledControlProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EnabledControlProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EnabledControlProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EnabledControlProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut control_identifier: Option<::Value<String>> = None;
                let mut target_identifier: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ControlIdentifier" => {
                            control_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetIdentifier" => {
                            target_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EnabledControlProperties {
                    control_identifier: control_identifier.ok_or(::serde::de::Error::missing_field("ControlIdentifier"))?,
                    target_identifier: target_identifier.ok_or(::serde::de::Error::missing_field("TargetIdentifier"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EnabledControl {
    type Properties = EnabledControlProperties;
    const TYPE: &'static str = "AWS::ControlTower::EnabledControl";
    fn properties(&self) -> &EnabledControlProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EnabledControlProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EnabledControl {}

impl From<EnabledControlProperties> for EnabledControl {
    fn from(properties: EnabledControlProperties) -> EnabledControl {
        EnabledControl { properties }
    }
}

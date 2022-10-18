//! Types for the `Connect` service.

/// The [`AWS::Connect::ContactFlow`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlow {
    properties: ContactFlowProperties
}

/// Properties for the `ContactFlow` resource.
#[derive(Debug, Default)]
pub struct ContactFlowProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html#cfn-connect-contactflow-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ContactFlowProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactFlowProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlow {
    type Properties = ContactFlowProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlow";
    fn properties(&self) -> &ContactFlowProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlow {}

impl From<ContactFlowProperties> for ContactFlow {
    fn from(properties: ContactFlowProperties) -> ContactFlow {
        ContactFlow { properties }
    }
}

/// The [`AWS::Connect::ContactFlowModule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html) resource type.
#[derive(Debug, Default)]
pub struct ContactFlowModule {
    properties: ContactFlowModuleProperties
}

/// Properties for the `ContactFlowModule` resource.
#[derive(Debug, Default)]
pub struct ContactFlowModuleProperties {
    /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-content).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub content: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`State`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-state).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub state: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html#cfn-connect-contactflowmodule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ContactFlowModuleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref state) = self.state {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "State", state)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ContactFlowModuleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ContactFlowModuleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ContactFlowModuleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ContactFlowModuleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut state: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "State" => {
                            state = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ContactFlowModuleProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    state: state,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ContactFlowModule {
    type Properties = ContactFlowModuleProperties;
    const TYPE: &'static str = "AWS::Connect::ContactFlowModule";
    fn properties(&self) -> &ContactFlowModuleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ContactFlowModuleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ContactFlowModule {}

impl From<ContactFlowModuleProperties> for ContactFlowModule {
    fn from(properties: ContactFlowModuleProperties) -> ContactFlowModule {
        ContactFlowModule { properties }
    }
}

/// The [`AWS::Connect::HoursOfOperation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html) resource type.
#[derive(Debug, Default)]
pub struct HoursOfOperation {
    properties: HoursOfOperationProperties
}

/// Properties for the `HoursOfOperation` resource.
#[derive(Debug, Default)]
pub struct HoursOfOperationProperties {
    /// Property [`Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config: ::ValueList<self::hours_of_operation::HoursOfOperationConfig>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TimeZone`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html#cfn-connect-hoursofoperation-timezone).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub time_zone: ::Value<String>,
}

impl ::serde::Serialize for HoursOfOperationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Config", &self.config)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeZone", &self.time_zone)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for HoursOfOperationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = HoursOfOperationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type HoursOfOperationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut config: Option<::ValueList<self::hours_of_operation::HoursOfOperationConfig>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut time_zone: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Config" => {
                            config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TimeZone" => {
                            time_zone = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(HoursOfOperationProperties {
                    config: config.ok_or(::serde::de::Error::missing_field("Config"))?,
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    tags: tags,
                    time_zone: time_zone.ok_or(::serde::de::Error::missing_field("TimeZone"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for HoursOfOperation {
    type Properties = HoursOfOperationProperties;
    const TYPE: &'static str = "AWS::Connect::HoursOfOperation";
    fn properties(&self) -> &HoursOfOperationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut HoursOfOperationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for HoursOfOperation {}

impl From<HoursOfOperationProperties> for HoursOfOperation {
    fn from(properties: HoursOfOperationProperties) -> HoursOfOperation {
        HoursOfOperation { properties }
    }
}

/// The [`AWS::Connect::Instance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html) resource type.
#[derive(Debug, Default)]
pub struct Instance {
    properties: InstanceProperties
}

/// Properties for the `Instance` resource.
#[derive(Debug, Default)]
pub struct InstanceProperties {
    /// Property [`Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-attributes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub attributes: ::Value<self::instance::Attributes>,
    /// Property [`DirectoryId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-directoryid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub directory_id: Option<::Value<String>>,
    /// Property [`IdentityManagementType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-identitymanagementtype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub identity_management_type: ::Value<String>,
    /// Property [`InstanceAlias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html#cfn-connect-instance-instancealias).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_alias: Option<::Value<String>>,
}

impl ::serde::Serialize for InstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Attributes", &self.attributes)?;
        if let Some(ref directory_id) = self.directory_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryId", directory_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityManagementType", &self.identity_management_type)?;
        if let Some(ref instance_alias) = self.instance_alias {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceAlias", instance_alias)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut attributes: Option<::Value<self::instance::Attributes>> = None;
                let mut directory_id: Option<::Value<String>> = None;
                let mut identity_management_type: Option<::Value<String>> = None;
                let mut instance_alias: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Attributes" => {
                            attributes = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectoryId" => {
                            directory_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityManagementType" => {
                            identity_management_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceAlias" => {
                            instance_alias = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceProperties {
                    attributes: attributes.ok_or(::serde::de::Error::missing_field("Attributes"))?,
                    directory_id: directory_id,
                    identity_management_type: identity_management_type.ok_or(::serde::de::Error::missing_field("IdentityManagementType"))?,
                    instance_alias: instance_alias,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Instance {
    type Properties = InstanceProperties;
    const TYPE: &'static str = "AWS::Connect::Instance";
    fn properties(&self) -> &InstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Instance {}

impl From<InstanceProperties> for Instance {
    fn from(properties: InstanceProperties) -> Instance {
        Instance { properties }
    }
}

/// The [`AWS::Connect::InstanceStorageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html) resource type.
#[derive(Debug, Default)]
pub struct InstanceStorageConfig {
    properties: InstanceStorageConfigProperties
}

/// Properties for the `InstanceStorageConfig` resource.
#[derive(Debug, Default)]
pub struct InstanceStorageConfigProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-instancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`KinesisFirehoseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisfirehoseconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_firehose_config: Option<::Value<self::instance_storage_config::KinesisFirehoseConfig>>,
    /// Property [`KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisstreamconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_stream_config: Option<::Value<self::instance_storage_config::KinesisStreamConfig>>,
    /// Property [`KinesisVideoStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kinesis_video_stream_config: Option<::Value<self::instance_storage_config::KinesisVideoStreamConfig>>,
    /// Property [`ResourceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-resourcetype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub resource_type: ::Value<String>,
    /// Property [`S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-s3config).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub s3_config: Option<::Value<self::instance_storage_config::S3Config>>,
    /// Property [`StorageType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html#cfn-connect-instancestorageconfig-storagetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub storage_type: ::Value<String>,
}

impl ::serde::Serialize for InstanceStorageConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref kinesis_firehose_config) = self.kinesis_firehose_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisFirehoseConfig", kinesis_firehose_config)?;
        }
        if let Some(ref kinesis_stream_config) = self.kinesis_stream_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisStreamConfig", kinesis_stream_config)?;
        }
        if let Some(ref kinesis_video_stream_config) = self.kinesis_video_stream_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KinesisVideoStreamConfig", kinesis_video_stream_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceType", &self.resource_type)?;
        if let Some(ref s3_config) = self.s3_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Config", s3_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", &self.storage_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for InstanceStorageConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceStorageConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = InstanceStorageConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type InstanceStorageConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut kinesis_firehose_config: Option<::Value<self::instance_storage_config::KinesisFirehoseConfig>> = None;
                let mut kinesis_stream_config: Option<::Value<self::instance_storage_config::KinesisStreamConfig>> = None;
                let mut kinesis_video_stream_config: Option<::Value<self::instance_storage_config::KinesisVideoStreamConfig>> = None;
                let mut resource_type: Option<::Value<String>> = None;
                let mut s3_config: Option<::Value<self::instance_storage_config::S3Config>> = None;
                let mut storage_type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisFirehoseConfig" => {
                            kinesis_firehose_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisStreamConfig" => {
                            kinesis_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KinesisVideoStreamConfig" => {
                            kinesis_video_stream_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceType" => {
                            resource_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "S3Config" => {
                            s3_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StorageType" => {
                            storage_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(InstanceStorageConfigProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    kinesis_firehose_config: kinesis_firehose_config,
                    kinesis_stream_config: kinesis_stream_config,
                    kinesis_video_stream_config: kinesis_video_stream_config,
                    resource_type: resource_type.ok_or(::serde::de::Error::missing_field("ResourceType"))?,
                    s3_config: s3_config,
                    storage_type: storage_type.ok_or(::serde::de::Error::missing_field("StorageType"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for InstanceStorageConfig {
    type Properties = InstanceStorageConfigProperties;
    const TYPE: &'static str = "AWS::Connect::InstanceStorageConfig";
    fn properties(&self) -> &InstanceStorageConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut InstanceStorageConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for InstanceStorageConfig {}

impl From<InstanceStorageConfigProperties> for InstanceStorageConfig {
    fn from(properties: InstanceStorageConfigProperties) -> InstanceStorageConfig {
        InstanceStorageConfig { properties }
    }
}

/// The [`AWS::Connect::PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html) resource type.
#[derive(Debug, Default)]
pub struct PhoneNumber {
    properties: PhoneNumberProperties
}

/// Properties for the `PhoneNumber` resource.
#[derive(Debug, Default)]
pub struct PhoneNumberProperties {
    /// Property [`CountryCode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-countrycode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub country_code: ::Value<String>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-prefix).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub prefix: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`TargetArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-targetarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub target_arn: ::Value<String>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html#cfn-connect-phonenumber-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for PhoneNumberProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CountryCode", &self.country_code)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref prefix) = self.prefix {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", prefix)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TargetArn", &self.target_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PhoneNumberProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PhoneNumberProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PhoneNumberProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PhoneNumberProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut country_code: Option<::Value<String>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut prefix: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut target_arn: Option<::Value<String>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CountryCode" => {
                            country_code = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Prefix" => {
                            prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "TargetArn" => {
                            target_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Type" => {
                            r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PhoneNumberProperties {
                    country_code: country_code.ok_or(::serde::de::Error::missing_field("CountryCode"))?,
                    description: description,
                    prefix: prefix,
                    tags: tags,
                    target_arn: target_arn.ok_or(::serde::de::Error::missing_field("TargetArn"))?,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for PhoneNumber {
    type Properties = PhoneNumberProperties;
    const TYPE: &'static str = "AWS::Connect::PhoneNumber";
    fn properties(&self) -> &PhoneNumberProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PhoneNumberProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PhoneNumber {}

impl From<PhoneNumberProperties> for PhoneNumber {
    fn from(properties: PhoneNumberProperties) -> PhoneNumber {
        PhoneNumber { properties }
    }
}

/// The [`AWS::Connect::QuickConnect`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html) resource type.
#[derive(Debug, Default)]
pub struct QuickConnect {
    properties: QuickConnectProperties
}

/// Properties for the `QuickConnect` resource.
#[derive(Debug, Default)]
pub struct QuickConnectProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-quickconnectconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub quick_connect_config: ::Value<self::quick_connect::QuickConnectConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html#cfn-connect-quickconnect-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for QuickConnectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectConfig", &self.quick_connect_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for QuickConnectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = QuickConnectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type QuickConnectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut quick_connect_config: Option<::Value<self::quick_connect::QuickConnectConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "QuickConnectConfig" => {
                            quick_connect_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(QuickConnectProperties {
                    description: description,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    quick_connect_config: quick_connect_config.ok_or(::serde::de::Error::missing_field("QuickConnectConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for QuickConnect {
    type Properties = QuickConnectProperties;
    const TYPE: &'static str = "AWS::Connect::QuickConnect";
    fn properties(&self) -> &QuickConnectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut QuickConnectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for QuickConnect {}

impl From<QuickConnectProperties> for QuickConnect {
    fn from(properties: QuickConnectProperties) -> QuickConnect {
        QuickConnect { properties }
    }
}

/// The [`AWS::Connect::TaskTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html) resource type.
#[derive(Debug, Default)]
pub struct TaskTemplate {
    properties: TaskTemplateProperties
}

/// Properties for the `TaskTemplate` resource.
#[derive(Debug, Default)]
pub struct TaskTemplateProperties {
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-clienttoken).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-constraints).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub constraints: Option<::Value<::json::Value>>,
    /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-contactflowarn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub contact_flow_arn: Option<::Value<String>>,
    /// Property [`Defaults`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-defaults).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub defaults: Option<::ValueList<self::task_template::DefaultFieldValue>>,
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`Fields`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-fields).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub fields: Option<::ValueList<self::task_template::Field>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-status).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html#cfn-connect-tasktemplate-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for TaskTemplateProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        if let Some(ref constraints) = self.constraints {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
        }
        if let Some(ref contact_flow_arn) = self.contact_flow_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", contact_flow_arn)?;
        }
        if let Some(ref defaults) = self.defaults {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Defaults", defaults)?;
        }
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref fields) = self.fields {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Fields", fields)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref status) = self.status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for TaskTemplateProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskTemplateProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = TaskTemplateProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type TaskTemplateProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut client_token: Option<::Value<String>> = None;
                let mut constraints: Option<::Value<::json::Value>> = None;
                let mut contact_flow_arn: Option<::Value<String>> = None;
                let mut defaults: Option<::ValueList<self::task_template::DefaultFieldValue>> = None;
                let mut description: Option<::Value<String>> = None;
                let mut fields: Option<::ValueList<self::task_template::Field>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Constraints" => {
                            constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ContactFlowArn" => {
                            contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Defaults" => {
                            defaults = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Fields" => {
                            fields = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Status" => {
                            status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(TaskTemplateProperties {
                    client_token: client_token,
                    constraints: constraints,
                    contact_flow_arn: contact_flow_arn,
                    defaults: defaults,
                    description: description,
                    fields: fields,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name,
                    status: status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for TaskTemplate {
    type Properties = TaskTemplateProperties;
    const TYPE: &'static str = "AWS::Connect::TaskTemplate";
    fn properties(&self) -> &TaskTemplateProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut TaskTemplateProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for TaskTemplate {}

impl From<TaskTemplateProperties> for TaskTemplate {
    fn from(properties: TaskTemplateProperties) -> TaskTemplate {
        TaskTemplate { properties }
    }
}

/// The [`AWS::Connect::User`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html) resource type.
#[derive(Debug, Default)]
pub struct User {
    properties: UserProperties
}

/// Properties for the `User` resource.
#[derive(Debug, Default)]
pub struct UserProperties {
    /// Property [`DirectoryUserId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-directoryuserid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub directory_user_id: Option<::Value<String>>,
    /// Property [`HierarchyGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-hierarchygrouparn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub hierarchy_group_arn: Option<::Value<String>>,
    /// Property [`IdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-identityinfo).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub identity_info: Option<::Value<self::user::UserIdentityInfo>>,
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Password`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-password).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub password: Option<::Value<String>>,
    /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-phoneconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub phone_config: ::Value<self::user::UserPhoneConfig>,
    /// Property [`RoutingProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-routingprofilearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub routing_profile_arn: ::Value<String>,
    /// Property [`SecurityProfileArns`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-securityprofilearns).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_profile_arns: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Username`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html#cfn-connect-user-username).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub username: ::Value<String>,
}

impl ::serde::Serialize for UserProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref directory_user_id) = self.directory_user_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectoryUserId", directory_user_id)?;
        }
        if let Some(ref hierarchy_group_arn) = self.hierarchy_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "HierarchyGroupArn", hierarchy_group_arn)?;
        }
        if let Some(ref identity_info) = self.identity_info {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdentityInfo", identity_info)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        if let Some(ref password) = self.password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Password", password)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", &self.phone_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoutingProfileArn", &self.routing_profile_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityProfileArns", &self.security_profile_arns)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Username", &self.username)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut directory_user_id: Option<::Value<String>> = None;
                let mut hierarchy_group_arn: Option<::Value<String>> = None;
                let mut identity_info: Option<::Value<self::user::UserIdentityInfo>> = None;
                let mut instance_arn: Option<::Value<String>> = None;
                let mut password: Option<::Value<String>> = None;
                let mut phone_config: Option<::Value<self::user::UserPhoneConfig>> = None;
                let mut routing_profile_arn: Option<::Value<String>> = None;
                let mut security_profile_arns: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut username: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DirectoryUserId" => {
                            directory_user_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "HierarchyGroupArn" => {
                            hierarchy_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IdentityInfo" => {
                            identity_info = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Password" => {
                            password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PhoneConfig" => {
                            phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoutingProfileArn" => {
                            routing_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityProfileArns" => {
                            security_profile_arns = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Username" => {
                            username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProperties {
                    directory_user_id: directory_user_id,
                    hierarchy_group_arn: hierarchy_group_arn,
                    identity_info: identity_info,
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    password: password,
                    phone_config: phone_config.ok_or(::serde::de::Error::missing_field("PhoneConfig"))?,
                    routing_profile_arn: routing_profile_arn.ok_or(::serde::de::Error::missing_field("RoutingProfileArn"))?,
                    security_profile_arns: security_profile_arns.ok_or(::serde::de::Error::missing_field("SecurityProfileArns"))?,
                    tags: tags,
                    username: username.ok_or(::serde::de::Error::missing_field("Username"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for User {
    type Properties = UserProperties;
    const TYPE: &'static str = "AWS::Connect::User";
    fn properties(&self) -> &UserProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for User {}

impl From<UserProperties> for User {
    fn from(properties: UserProperties) -> User {
        User { properties }
    }
}

/// The [`AWS::Connect::UserHierarchyGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html) resource type.
#[derive(Debug, Default)]
pub struct UserHierarchyGroup {
    properties: UserHierarchyGroupProperties
}

/// Properties for the `UserHierarchyGroup` resource.
#[derive(Debug, Default)]
pub struct UserHierarchyGroupProperties {
    /// Property [`InstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-instancearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_arn: ::Value<String>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`ParentGroupArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html#cfn-connect-userhierarchygroup-parentgrouparn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub parent_group_arn: Option<::Value<String>>,
}

impl ::serde::Serialize for UserHierarchyGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceArn", &self.instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        if let Some(ref parent_group_arn) = self.parent_group_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParentGroupArn", parent_group_arn)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserHierarchyGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserHierarchyGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserHierarchyGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserHierarchyGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut instance_arn: Option<::Value<String>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut parent_group_arn: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "InstanceArn" => {
                            instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ParentGroupArn" => {
                            parent_group_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserHierarchyGroupProperties {
                    instance_arn: instance_arn.ok_or(::serde::de::Error::missing_field("InstanceArn"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parent_group_arn: parent_group_arn,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserHierarchyGroup {
    type Properties = UserHierarchyGroupProperties;
    const TYPE: &'static str = "AWS::Connect::UserHierarchyGroup";
    fn properties(&self) -> &UserHierarchyGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserHierarchyGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserHierarchyGroup {}

impl From<UserHierarchyGroupProperties> for UserHierarchyGroup {
    fn from(properties: UserHierarchyGroupProperties) -> UserHierarchyGroup {
        UserHierarchyGroup { properties }
    }
}

pub mod hours_of_operation {
    //! Property types for the `HoursOfOperation` resource.

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationConfig {
        /// Property [`Day`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-day).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub day: ::Value<String>,
        /// Property [`EndTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-endtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub end_time: ::Value<HoursOfOperationTimeSlice>,
        /// Property [`StartTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html#cfn-connect-hoursofoperation-hoursofoperationconfig-starttime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub start_time: ::Value<HoursOfOperationTimeSlice>,
    }

    impl ::codec::SerializeValue for HoursOfOperationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Day", &self.day)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTime", &self.end_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTime", &self.start_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut day: Option<::Value<String>> = None;
                    let mut end_time: Option<::Value<HoursOfOperationTimeSlice>> = None;
                    let mut start_time: Option<::Value<HoursOfOperationTimeSlice>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Day" => {
                                day = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndTime" => {
                                end_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTime" => {
                                start_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationConfig {
                        day: day.ok_or(::serde::de::Error::missing_field("Day"))?,
                        end_time: end_time.ok_or(::serde::de::Error::missing_field("EndTime"))?,
                        start_time: start_time.ok_or(::serde::de::Error::missing_field("StartTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::HoursOfOperation.HoursOfOperationTimeSlice`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html) property type.
    #[derive(Debug, Default)]
    pub struct HoursOfOperationTimeSlice {
        /// Property [`Hours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-hours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub hours: ::Value<u32>,
        /// Property [`Minutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html#cfn-connect-hoursofoperation-hoursofoperationtimeslice-minutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minutes: ::Value<u32>,
    }

    impl ::codec::SerializeValue for HoursOfOperationTimeSlice {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Hours", &self.hours)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Minutes", &self.minutes)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for HoursOfOperationTimeSlice {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<HoursOfOperationTimeSlice, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = HoursOfOperationTimeSlice;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type HoursOfOperationTimeSlice")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut hours: Option<::Value<u32>> = None;
                    let mut minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Hours" => {
                                hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Minutes" => {
                                minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(HoursOfOperationTimeSlice {
                        hours: hours.ok_or(::serde::de::Error::missing_field("Hours"))?,
                        minutes: minutes.ok_or(::serde::de::Error::missing_field("Minutes"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance {
    //! Property types for the `Instance` resource.

    /// The [`AWS::Connect::Instance.Attributes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html) property type.
    #[derive(Debug, Default)]
    pub struct Attributes {
        /// Property [`AutoResolveBestVoices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-autoresolvebestvoices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_resolve_best_voices: Option<::Value<bool>>,
        /// Property [`ContactLens`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-contactlens).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_lens: Option<::Value<bool>>,
        /// Property [`ContactflowLogs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-contactflowlogs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contactflow_logs: Option<::Value<bool>>,
        /// Property [`EarlyMedia`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-earlymedia).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub early_media: Option<::Value<bool>>,
        /// Property [`InboundCalls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-inboundcalls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub inbound_calls: ::Value<bool>,
        /// Property [`OutboundCalls`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-outboundcalls).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub outbound_calls: ::Value<bool>,
        /// Property [`UseCustomTTSVoices`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html#cfn-connect-instance-attributes-usecustomttsvoices).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub use_custom_tts_voices: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for Attributes {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_resolve_best_voices) = self.auto_resolve_best_voices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoResolveBestVoices", auto_resolve_best_voices)?;
            }
            if let Some(ref contact_lens) = self.contact_lens {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactLens", contact_lens)?;
            }
            if let Some(ref contactflow_logs) = self.contactflow_logs {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactflowLogs", contactflow_logs)?;
            }
            if let Some(ref early_media) = self.early_media {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EarlyMedia", early_media)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InboundCalls", &self.inbound_calls)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCalls", &self.outbound_calls)?;
            if let Some(ref use_custom_tts_voices) = self.use_custom_tts_voices {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseCustomTTSVoices", use_custom_tts_voices)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Attributes {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Attributes, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Attributes;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Attributes")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_resolve_best_voices: Option<::Value<bool>> = None;
                    let mut contact_lens: Option<::Value<bool>> = None;
                    let mut contactflow_logs: Option<::Value<bool>> = None;
                    let mut early_media: Option<::Value<bool>> = None;
                    let mut inbound_calls: Option<::Value<bool>> = None;
                    let mut outbound_calls: Option<::Value<bool>> = None;
                    let mut use_custom_tts_voices: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoResolveBestVoices" => {
                                auto_resolve_best_voices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContactLens" => {
                                contact_lens = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContactflowLogs" => {
                                contactflow_logs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EarlyMedia" => {
                                early_media = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InboundCalls" => {
                                inbound_calls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutboundCalls" => {
                                outbound_calls = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseCustomTTSVoices" => {
                                use_custom_tts_voices = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Attributes {
                        auto_resolve_best_voices: auto_resolve_best_voices,
                        contact_lens: contact_lens,
                        contactflow_logs: contactflow_logs,
                        early_media: early_media,
                        inbound_calls: inbound_calls.ok_or(::serde::de::Error::missing_field("InboundCalls"))?,
                        outbound_calls: outbound_calls.ok_or(::serde::de::Error::missing_field("OutboundCalls"))?,
                        use_custom_tts_voices: use_custom_tts_voices,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod instance_storage_config {
    //! Property types for the `InstanceStorageConfig` resource.

    /// The [`AWS::Connect::InstanceStorageConfig.EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EncryptionConfig {
        /// Property [`EncryptionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html#cfn-connect-instancestorageconfig-encryptionconfig-encryptiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_type: ::Value<String>,
        /// Property [`KeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html#cfn-connect-instancestorageconfig-encryptionconfig-keyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key_id: ::Value<String>,
    }

    impl ::codec::SerializeValue for EncryptionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionType", &self.encryption_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KeyId", &self.key_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EncryptionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EncryptionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EncryptionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EncryptionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_type: Option<::Value<String>> = None;
                    let mut key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionType" => {
                                encryption_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KeyId" => {
                                key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EncryptionConfig {
                        encryption_type: encryption_type.ok_or(::serde::de::Error::missing_field("EncryptionType"))?,
                        key_id: key_id.ok_or(::serde::de::Error::missing_field("KeyId"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisFirehoseConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisfirehoseconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisFirehoseConfig {
        /// Property [`FirehoseArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisfirehoseconfig.html#cfn-connect-instancestorageconfig-kinesisfirehoseconfig-firehosearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub firehose_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisFirehoseConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirehoseArn", &self.firehose_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisFirehoseConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisFirehoseConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisFirehoseConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisFirehoseConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut firehose_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FirehoseArn" => {
                                firehose_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisFirehoseConfig {
                        firehose_arn: firehose_arn.ok_or(::serde::de::Error::missing_field("FirehoseArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisstreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisStreamConfig {
        /// Property [`StreamArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisstreamconfig.html#cfn-connect-instancestorageconfig-kinesisstreamconfig-streamarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stream_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for KinesisStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StreamArn", &self.stream_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut stream_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "StreamArn" => {
                                stream_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisStreamConfig {
                        stream_arn: stream_arn.ok_or(::serde::de::Error::missing_field("StreamArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.KinesisVideoStreamConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KinesisVideoStreamConfig {
        /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-encryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_config: Option<::Value<EncryptionConfig>>,
        /// Property [`Prefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-prefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub prefix: ::Value<String>,
        /// Property [`RetentionPeriodHours`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html#cfn-connect-instancestorageconfig-kinesisvideostreamconfig-retentionperiodhours).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub retention_period_hours: ::Value<f64>,
    }

    impl ::codec::SerializeValue for KinesisVideoStreamConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref encryption_config) = self.encryption_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", encryption_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Prefix", &self.prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetentionPeriodHours", &self.retention_period_hours)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KinesisVideoStreamConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KinesisVideoStreamConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KinesisVideoStreamConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KinesisVideoStreamConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut encryption_config: Option<::Value<EncryptionConfig>> = None;
                    let mut prefix: Option<::Value<String>> = None;
                    let mut retention_period_hours: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EncryptionConfig" => {
                                encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Prefix" => {
                                prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RetentionPeriodHours" => {
                                retention_period_hours = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KinesisVideoStreamConfig {
                        encryption_config: encryption_config,
                        prefix: prefix.ok_or(::serde::de::Error::missing_field("Prefix"))?,
                        retention_period_hours: retention_period_hours.ok_or(::serde::de::Error::missing_field("RetentionPeriodHours"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::InstanceStorageConfig.S3Config`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Config {
        /// Property [`BucketName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-bucketname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_name: ::Value<String>,
        /// Property [`BucketPrefix`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-bucketprefix).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bucket_prefix: ::Value<String>,
        /// Property [`EncryptionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html#cfn-connect-instancestorageconfig-s3config-encryptionconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub encryption_config: Option<::Value<EncryptionConfig>>,
    }

    impl ::codec::SerializeValue for S3Config {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketName", &self.bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BucketPrefix", &self.bucket_prefix)?;
            if let Some(ref encryption_config) = self.encryption_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EncryptionConfig", encryption_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Config {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Config, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Config;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Config")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bucket_name: Option<::Value<String>> = None;
                    let mut bucket_prefix: Option<::Value<String>> = None;
                    let mut encryption_config: Option<::Value<EncryptionConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BucketName" => {
                                bucket_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BucketPrefix" => {
                                bucket_prefix = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EncryptionConfig" => {
                                encryption_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Config {
                        bucket_name: bucket_name.ok_or(::serde::de::Error::missing_field("BucketName"))?,
                        bucket_prefix: bucket_prefix.ok_or(::serde::de::Error::missing_field("BucketPrefix"))?,
                        encryption_config: encryption_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod quick_connect {
    //! Property types for the `QuickConnect` resource.

    /// The [`AWS::Connect::QuickConnect.PhoneNumberQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PhoneNumberQuickConnectConfig {
        /// Property [`PhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html#cfn-connect-quickconnect-phonenumberquickconnectconfig-phonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_number: ::Value<String>,
    }

    impl ::codec::SerializeValue for PhoneNumberQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneNumber", &self.phone_number)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PhoneNumberQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PhoneNumberQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PhoneNumberQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PhoneNumberQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneNumber" => {
                                phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PhoneNumberQuickConnectConfig {
                        phone_number: phone_number.ok_or(::serde::de::Error::missing_field("PhoneNumber"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QueueQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QueueQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`QueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html#cfn-connect-quickconnect-queuequickconnectconfig-queuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for QueueQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueArn", &self.queue_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QueueQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QueueQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QueueQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QueueQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut queue_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueArn" => {
                                queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QueueQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        queue_arn: queue_arn.ok_or(::serde::de::Error::missing_field("QueueArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.QuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct QuickConnectConfig {
        /// Property [`PhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-phoneconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_config: Option<::Value<PhoneNumberQuickConnectConfig>>,
        /// Property [`QueueConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-queueconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub queue_config: Option<::Value<QueueQuickConnectConfig>>,
        /// Property [`QuickConnectType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-quickconnecttype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub quick_connect_type: ::Value<String>,
        /// Property [`UserConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html#cfn-connect-quickconnect-quickconnectconfig-userconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_config: Option<::Value<UserQuickConnectConfig>>,
    }

    impl ::codec::SerializeValue for QuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref phone_config) = self.phone_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneConfig", phone_config)?;
            }
            if let Some(ref queue_config) = self.queue_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "QueueConfig", queue_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "QuickConnectType", &self.quick_connect_type)?;
            if let Some(ref user_config) = self.user_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserConfig", user_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for QuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<QuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = QuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type QuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut phone_config: Option<::Value<PhoneNumberQuickConnectConfig>> = None;
                    let mut queue_config: Option<::Value<QueueQuickConnectConfig>> = None;
                    let mut quick_connect_type: Option<::Value<String>> = None;
                    let mut user_config: Option<::Value<UserQuickConnectConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PhoneConfig" => {
                                phone_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QueueConfig" => {
                                queue_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "QuickConnectType" => {
                                quick_connect_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserConfig" => {
                                user_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(QuickConnectConfig {
                        phone_config: phone_config,
                        queue_config: queue_config,
                        quick_connect_type: quick_connect_type.ok_or(::serde::de::Error::missing_field("QuickConnectType"))?,
                        user_config: user_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::QuickConnect.UserQuickConnectConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserQuickConnectConfig {
        /// Property [`ContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-contactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub contact_flow_arn: ::Value<String>,
        /// Property [`UserArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html#cfn-connect-quickconnect-userquickconnectconfig-userarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserQuickConnectConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContactFlowArn", &self.contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserArn", &self.user_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserQuickConnectConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserQuickConnectConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserQuickConnectConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserQuickConnectConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut contact_flow_arn: Option<::Value<String>> = None;
                    let mut user_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContactFlowArn" => {
                                contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserArn" => {
                                user_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserQuickConnectConfig {
                        contact_flow_arn: contact_flow_arn.ok_or(::serde::de::Error::missing_field("ContactFlowArn"))?,
                        user_arn: user_arn.ok_or(::serde::de::Error::missing_field("UserArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod task_template {
    //! Property types for the `TaskTemplate` resource.

    /// The [`AWS::Connect::TaskTemplate.DefaultFieldValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html) property type.
    #[derive(Debug, Default)]
    pub struct DefaultFieldValue {
        /// Property [`DefaultValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html#cfn-connect-tasktemplate-defaultfieldvalue-defaultvalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_value: ::Value<String>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html#cfn-connect-tasktemplate-defaultfieldvalue-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
    }

    impl ::codec::SerializeValue for DefaultFieldValue {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultValue", &self.default_value)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DefaultFieldValue {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DefaultFieldValue, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DefaultFieldValue;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DefaultFieldValue")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_value: Option<::Value<String>> = None;
                    let mut id: Option<::Value<FieldIdentifier>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultValue" => {
                                default_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DefaultFieldValue {
                        default_value: default_value.ok_or(::serde::de::Error::missing_field("DefaultValue"))?,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.Field`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html) property type.
    #[derive(Debug, Default)]
    pub struct Field {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Id`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-id).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub id: ::Value<FieldIdentifier>,
        /// Property [`SingleSelectOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-singleselectoptions).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub single_select_options: Option<::ValueList<String>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html#cfn-connect-tasktemplate-field-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
    }

    impl ::codec::SerializeValue for Field {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Id", &self.id)?;
            if let Some(ref single_select_options) = self.single_select_options {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSelectOptions", single_select_options)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Field {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Field, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Field;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Field")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut id: Option<::Value<FieldIdentifier>> = None;
                    let mut single_select_options: Option<::ValueList<String>> = None;
                    let mut r#type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Id" => {
                                id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SingleSelectOptions" => {
                                single_select_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Field {
                        description: description,
                        id: id.ok_or(::serde::de::Error::missing_field("Id"))?,
                        single_select_options: single_select_options,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::TaskTemplate.FieldIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-fieldidentifier.html) property type.
    #[derive(Debug, Default)]
    pub struct FieldIdentifier {
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-fieldidentifier.html#cfn-connect-tasktemplate-fieldidentifier-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for FieldIdentifier {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FieldIdentifier {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FieldIdentifier, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FieldIdentifier;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FieldIdentifier")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FieldIdentifier {
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user {
    //! Property types for the `User` resource.

    /// The [`AWS::Connect::User.UserIdentityInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html) property type.
    #[derive(Debug, Default)]
    pub struct UserIdentityInfo {
        /// Property [`Email`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-email).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub email: Option<::Value<String>>,
        /// Property [`FirstName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-firstname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub first_name: Option<::Value<String>>,
        /// Property [`LastName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html#cfn-connect-user-useridentityinfo-lastname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserIdentityInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref email) = self.email {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Email", email)?;
            }
            if let Some(ref first_name) = self.first_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FirstName", first_name)?;
            }
            if let Some(ref last_name) = self.last_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastName", last_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserIdentityInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserIdentityInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserIdentityInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserIdentityInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut email: Option<::Value<String>> = None;
                    let mut first_name: Option<::Value<String>> = None;
                    let mut last_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Email" => {
                                email = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FirstName" => {
                                first_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastName" => {
                                last_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserIdentityInfo {
                        email: email,
                        first_name: first_name,
                        last_name: last_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::Connect::User.UserPhoneConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct UserPhoneConfig {
        /// Property [`AfterContactWorkTimeLimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-aftercontactworktimelimit).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub after_contact_work_time_limit: Option<::Value<u32>>,
        /// Property [`AutoAccept`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-autoaccept).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_accept: Option<::Value<bool>>,
        /// Property [`DeskPhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-deskphonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub desk_phone_number: Option<::Value<String>>,
        /// Property [`PhoneType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html#cfn-connect-user-userphoneconfig-phonetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub phone_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for UserPhoneConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref after_contact_work_time_limit) = self.after_contact_work_time_limit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AfterContactWorkTimeLimit", after_contact_work_time_limit)?;
            }
            if let Some(ref auto_accept) = self.auto_accept {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoAccept", auto_accept)?;
            }
            if let Some(ref desk_phone_number) = self.desk_phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeskPhoneNumber", desk_phone_number)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PhoneType", &self.phone_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserPhoneConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserPhoneConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserPhoneConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserPhoneConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut after_contact_work_time_limit: Option<::Value<u32>> = None;
                    let mut auto_accept: Option<::Value<bool>> = None;
                    let mut desk_phone_number: Option<::Value<String>> = None;
                    let mut phone_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AfterContactWorkTimeLimit" => {
                                after_contact_work_time_limit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AutoAccept" => {
                                auto_accept = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeskPhoneNumber" => {
                                desk_phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PhoneType" => {
                                phone_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserPhoneConfig {
                        after_contact_work_time_limit: after_contact_work_time_limit,
                        auto_accept: auto_accept,
                        desk_phone_number: desk_phone_number,
                        phone_type: phone_type.ok_or(::serde::de::Error::missing_field("PhoneType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

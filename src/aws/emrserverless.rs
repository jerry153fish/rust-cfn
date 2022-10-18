//! Types for the `EMRServerless` service.

/// The [`AWS::EMRServerless::Application`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html) resource type.
#[derive(Debug, Default)]
pub struct Application {
    properties: ApplicationProperties
}

/// Properties for the `Application` resource.
#[derive(Debug, Default)]
pub struct ApplicationProperties {
    /// Property [`AutoStartConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-autostartconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_start_configuration: Option<::Value<self::application::AutoStartConfiguration>>,
    /// Property [`AutoStopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-autostopconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub auto_stop_configuration: Option<::Value<self::application::AutoStopConfiguration>>,
    /// Property [`InitialCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-initialcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub initial_capacity: Option<::ValueList<self::application::InitialCapacityConfigKeyValuePair>>,
    /// Property [`MaximumCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-maximumcapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub maximum_capacity: Option<::Value<self::application::MaximumAllowedResources>>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-name).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub name: Option<::Value<String>>,
    /// Property [`NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-networkconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub network_configuration: Option<::Value<self::application::NetworkConfiguration>>,
    /// Property [`ReleaseLabel`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-releaselabel).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub release_label: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html#cfn-emrserverless-application-type).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub r#type: ::Value<String>,
}

impl ::serde::Serialize for ApplicationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref auto_start_configuration) = self.auto_start_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoStartConfiguration", auto_start_configuration)?;
        }
        if let Some(ref auto_stop_configuration) = self.auto_stop_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoStopConfiguration", auto_stop_configuration)?;
        }
        if let Some(ref initial_capacity) = self.initial_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialCapacity", initial_capacity)?;
        }
        if let Some(ref maximum_capacity) = self.maximum_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumCapacity", maximum_capacity)?;
        }
        if let Some(ref name) = self.name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", name)?;
        }
        if let Some(ref network_configuration) = self.network_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfiguration", network_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReleaseLabel", &self.release_label)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ApplicationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ApplicationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ApplicationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ApplicationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut auto_start_configuration: Option<::Value<self::application::AutoStartConfiguration>> = None;
                let mut auto_stop_configuration: Option<::Value<self::application::AutoStopConfiguration>> = None;
                let mut initial_capacity: Option<::ValueList<self::application::InitialCapacityConfigKeyValuePair>> = None;
                let mut maximum_capacity: Option<::Value<self::application::MaximumAllowedResources>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut network_configuration: Option<::Value<self::application::NetworkConfiguration>> = None;
                let mut release_label: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut r#type: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AutoStartConfiguration" => {
                            auto_start_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AutoStopConfiguration" => {
                            auto_stop_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InitialCapacity" => {
                            initial_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MaximumCapacity" => {
                            maximum_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfiguration" => {
                            network_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ReleaseLabel" => {
                            release_label = ::serde::de::MapAccess::next_value(&mut map)?;
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

                Ok(ApplicationProperties {
                    auto_start_configuration: auto_start_configuration,
                    auto_stop_configuration: auto_stop_configuration,
                    initial_capacity: initial_capacity,
                    maximum_capacity: maximum_capacity,
                    name: name,
                    network_configuration: network_configuration,
                    release_label: release_label.ok_or(::serde::de::Error::missing_field("ReleaseLabel"))?,
                    tags: tags,
                    r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Application {
    type Properties = ApplicationProperties;
    const TYPE: &'static str = "AWS::EMRServerless::Application";
    fn properties(&self) -> &ApplicationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ApplicationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Application {}

impl From<ApplicationProperties> for Application {
    fn from(properties: ApplicationProperties) -> Application {
        Application { properties }
    }
}

pub mod application {
    //! Property types for the `Application` resource.

    /// The [`AWS::EMRServerless::Application.AutoStartConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostartconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoStartConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostartconfiguration.html#cfn-emrserverless-application-autostartconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for AutoStartConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoStartConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoStartConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoStartConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoStartConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoStartConfiguration {
                        enabled: enabled,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.AutoStopConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoStopConfiguration {
        /// Property [`Enabled`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html#cfn-emrserverless-application-autostopconfiguration-enabled).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enabled: Option<::Value<bool>>,
        /// Property [`IdleTimeoutMinutes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html#cfn-emrserverless-application-autostopconfiguration-idletimeoutminutes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub idle_timeout_minutes: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AutoStopConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enabled) = self.enabled {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", enabled)?;
            }
            if let Some(ref idle_timeout_minutes) = self.idle_timeout_minutes {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IdleTimeoutMinutes", idle_timeout_minutes)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoStopConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoStopConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoStopConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoStopConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enabled: Option<::Value<bool>> = None;
                    let mut idle_timeout_minutes: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Enabled" => {
                                enabled = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IdleTimeoutMinutes" => {
                                idle_timeout_minutes = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoStopConfiguration {
                        enabled: enabled,
                        idle_timeout_minutes: idle_timeout_minutes,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.InitialCapacityConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct InitialCapacityConfig {
        /// Property [`WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html#cfn-emrserverless-application-initialcapacityconfig-workerconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub worker_configuration: ::Value<WorkerConfiguration>,
        /// Property [`WorkerCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html#cfn-emrserverless-application-initialcapacityconfig-workercount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub worker_count: ::Value<u32>,
    }

    impl ::codec::SerializeValue for InitialCapacityConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerConfiguration", &self.worker_configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkerCount", &self.worker_count)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InitialCapacityConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InitialCapacityConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InitialCapacityConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InitialCapacityConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut worker_configuration: Option<::Value<WorkerConfiguration>> = None;
                    let mut worker_count: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "WorkerConfiguration" => {
                                worker_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WorkerCount" => {
                                worker_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InitialCapacityConfig {
                        worker_configuration: worker_configuration.ok_or(::serde::de::Error::missing_field("WorkerConfiguration"))?,
                        worker_count: worker_count.ok_or(::serde::de::Error::missing_field("WorkerCount"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.InitialCapacityConfigKeyValuePair`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html) property type.
    #[derive(Debug, Default)]
    pub struct InitialCapacityConfigKeyValuePair {
        /// Property [`Key`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html#cfn-emrserverless-application-initialcapacityconfigkeyvaluepair-key).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub key: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html#cfn-emrserverless-application-initialcapacityconfigkeyvaluepair-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<InitialCapacityConfig>,
    }

    impl ::codec::SerializeValue for InitialCapacityConfigKeyValuePair {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InitialCapacityConfigKeyValuePair {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InitialCapacityConfigKeyValuePair, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InitialCapacityConfigKeyValuePair;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InitialCapacityConfigKeyValuePair")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key: Option<::Value<String>> = None;
                    let mut value: Option<::Value<InitialCapacityConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InitialCapacityConfigKeyValuePair {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.MaximumAllowedResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MaximumAllowedResources {
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-cpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu: ::Value<String>,
        /// Property [`Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-disk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disk: Option<::Value<String>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html#cfn-emrserverless-application-maximumallowedresources-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: ::Value<String>,
    }

    impl ::codec::SerializeValue for MaximumAllowedResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
            if let Some(ref disk) = self.disk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Disk", disk)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaximumAllowedResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaximumAllowedResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaximumAllowedResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaximumAllowedResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu: Option<::Value<String>> = None;
                    let mut disk: Option<::Value<String>> = None;
                    let mut memory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Disk" => {
                                disk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MaximumAllowedResources {
                        cpu: cpu.ok_or(::serde::de::Error::missing_field("Cpu"))?,
                        disk: disk,
                        memory: memory.ok_or(::serde::de::Error::missing_field("Memory"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.NetworkConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfiguration {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html#cfn-emrserverless-application-networkconfiguration-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
        /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html#cfn-emrserverless-application-networkconfiguration-subnetids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnet_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for NetworkConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            if let Some(ref subnet_ids) = self.subnet_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnet_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SubnetIds" => {
                                subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfiguration {
                        security_group_ids: security_group_ids,
                        subnet_ids: subnet_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::EMRServerless::Application.WorkerConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct WorkerConfiguration {
        /// Property [`Cpu`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-cpu).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cpu: ::Value<String>,
        /// Property [`Disk`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-disk).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub disk: Option<::Value<String>>,
        /// Property [`Memory`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html#cfn-emrserverless-application-workerconfiguration-memory).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub memory: ::Value<String>,
    }

    impl ::codec::SerializeValue for WorkerConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Cpu", &self.cpu)?;
            if let Some(ref disk) = self.disk {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Disk", disk)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Memory", &self.memory)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for WorkerConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkerConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = WorkerConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type WorkerConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cpu: Option<::Value<String>> = None;
                    let mut disk: Option<::Value<String>> = None;
                    let mut memory: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Cpu" => {
                                cpu = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Disk" => {
                                disk = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Memory" => {
                                memory = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(WorkerConfiguration {
                        cpu: cpu.ok_or(::serde::de::Error::missing_field("Cpu"))?,
                        disk: disk,
                        memory: memory.ok_or(::serde::de::Error::missing_field("Memory"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

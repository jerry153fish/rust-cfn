//! Types for the `RedshiftServerless` service.

/// The [`AWS::RedshiftServerless::Namespace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html) resource type.
#[derive(Debug, Default)]
pub struct Namespace {
    properties: NamespaceProperties
}

/// Properties for the `Namespace` resource.
#[derive(Debug, Default)]
pub struct NamespaceProperties {
    /// Property [`AdminUserPassword`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-adminuserpassword).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_user_password: Option<::Value<String>>,
    /// Property [`AdminUsername`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-adminusername).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub admin_username: Option<::Value<String>>,
    /// Property [`DbName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-dbname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub db_name: Option<::Value<String>>,
    /// Property [`DefaultIamRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-defaultiamrolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_iam_role_arn: Option<::Value<String>>,
    /// Property [`FinalSnapshotName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-finalsnapshotname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub final_snapshot_name: Option<::Value<String>>,
    /// Property [`FinalSnapshotRetentionPeriod`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-finalsnapshotretentionperiod).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub final_snapshot_retention_period: Option<::Value<u32>>,
    /// Property [`IamRoles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-iamroles).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub iam_roles: Option<::ValueList<String>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-kmskeyid).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LogExports`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-logexports).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub log_exports: Option<::ValueList<String>>,
    /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-namespacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub namespace_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html#cfn-redshiftserverless-namespace-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for NamespaceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref admin_user_password) = self.admin_user_password {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUserPassword", admin_user_password)?;
        }
        if let Some(ref admin_username) = self.admin_username {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdminUsername", admin_username)?;
        }
        if let Some(ref db_name) = self.db_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DbName", db_name)?;
        }
        if let Some(ref default_iam_role_arn) = self.default_iam_role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultIamRoleArn", default_iam_role_arn)?;
        }
        if let Some(ref final_snapshot_name) = self.final_snapshot_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FinalSnapshotName", final_snapshot_name)?;
        }
        if let Some(ref final_snapshot_retention_period) = self.final_snapshot_retention_period {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FinalSnapshotRetentionPeriod", final_snapshot_retention_period)?;
        }
        if let Some(ref iam_roles) = self.iam_roles {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "IamRoles", iam_roles)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref log_exports) = self.log_exports {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LogExports", log_exports)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", &self.namespace_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NamespaceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NamespaceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NamespaceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NamespaceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut admin_user_password: Option<::Value<String>> = None;
                let mut admin_username: Option<::Value<String>> = None;
                let mut db_name: Option<::Value<String>> = None;
                let mut default_iam_role_arn: Option<::Value<String>> = None;
                let mut final_snapshot_name: Option<::Value<String>> = None;
                let mut final_snapshot_retention_period: Option<::Value<u32>> = None;
                let mut iam_roles: Option<::ValueList<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut log_exports: Option<::ValueList<String>> = None;
                let mut namespace_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdminUserPassword" => {
                            admin_user_password = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdminUsername" => {
                            admin_username = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DbName" => {
                            db_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultIamRoleArn" => {
                            default_iam_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FinalSnapshotName" => {
                            final_snapshot_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FinalSnapshotRetentionPeriod" => {
                            final_snapshot_retention_period = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "IamRoles" => {
                            iam_roles = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LogExports" => {
                            log_exports = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NamespaceName" => {
                            namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NamespaceProperties {
                    admin_user_password: admin_user_password,
                    admin_username: admin_username,
                    db_name: db_name,
                    default_iam_role_arn: default_iam_role_arn,
                    final_snapshot_name: final_snapshot_name,
                    final_snapshot_retention_period: final_snapshot_retention_period,
                    iam_roles: iam_roles,
                    kms_key_id: kms_key_id,
                    log_exports: log_exports,
                    namespace_name: namespace_name.ok_or(::serde::de::Error::missing_field("NamespaceName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Namespace {
    type Properties = NamespaceProperties;
    const TYPE: &'static str = "AWS::RedshiftServerless::Namespace";
    fn properties(&self) -> &NamespaceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NamespaceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Namespace {}

impl From<NamespaceProperties> for Namespace {
    fn from(properties: NamespaceProperties) -> Namespace {
        Namespace { properties }
    }
}

/// The [`AWS::RedshiftServerless::Workgroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html) resource type.
#[derive(Debug, Default)]
pub struct Workgroup {
    properties: WorkgroupProperties
}

/// Properties for the `Workgroup` resource.
#[derive(Debug, Default)]
pub struct WorkgroupProperties {
    /// Property [`BaseCapacity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-basecapacity).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub base_capacity: Option<::Value<u32>>,
    /// Property [`ConfigParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-configparameters).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub config_parameters: Option<::ValueList<self::workgroup::ConfigParameter>>,
    /// Property [`EnhancedVpcRouting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-enhancedvpcrouting).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub enhanced_vpc_routing: Option<::Value<bool>>,
    /// Property [`NamespaceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-namespacename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub namespace_name: Option<::Value<String>>,
    /// Property [`PubliclyAccessible`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-publiclyaccessible).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-securitygroupids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-subnetids).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub subnet_ids: Option<::ValueList<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkgroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html#cfn-redshiftserverless-workgroup-workgroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workgroup_name: ::Value<String>,
}

impl ::serde::Serialize for WorkgroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref base_capacity) = self.base_capacity {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseCapacity", base_capacity)?;
        }
        if let Some(ref config_parameters) = self.config_parameters {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigParameters", config_parameters)?;
        }
        if let Some(ref enhanced_vpc_routing) = self.enhanced_vpc_routing {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnhancedVpcRouting", enhanced_vpc_routing)?;
        }
        if let Some(ref namespace_name) = self.namespace_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NamespaceName", namespace_name)?;
        }
        if let Some(ref publicly_accessible) = self.publicly_accessible {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", publicly_accessible)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_ids) = self.subnet_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", subnet_ids)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkgroupName", &self.workgroup_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkgroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkgroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkgroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkgroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut base_capacity: Option<::Value<u32>> = None;
                let mut config_parameters: Option<::ValueList<self::workgroup::ConfigParameter>> = None;
                let mut enhanced_vpc_routing: Option<::Value<bool>> = None;
                let mut namespace_name: Option<::Value<String>> = None;
                let mut publicly_accessible: Option<::Value<bool>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workgroup_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BaseCapacity" => {
                            base_capacity = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ConfigParameters" => {
                            config_parameters = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnhancedVpcRouting" => {
                            enhanced_vpc_routing = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NamespaceName" => {
                            namespace_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkgroupName" => {
                            workgroup_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkgroupProperties {
                    base_capacity: base_capacity,
                    config_parameters: config_parameters,
                    enhanced_vpc_routing: enhanced_vpc_routing,
                    namespace_name: namespace_name,
                    publicly_accessible: publicly_accessible,
                    security_group_ids: security_group_ids,
                    subnet_ids: subnet_ids,
                    tags: tags,
                    workgroup_name: workgroup_name.ok_or(::serde::de::Error::missing_field("WorkgroupName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workgroup {
    type Properties = WorkgroupProperties;
    const TYPE: &'static str = "AWS::RedshiftServerless::Workgroup";
    fn properties(&self) -> &WorkgroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkgroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workgroup {}

impl From<WorkgroupProperties> for Workgroup {
    fn from(properties: WorkgroupProperties) -> Workgroup {
        Workgroup { properties }
    }
}

pub mod workgroup {
    //! Property types for the `Workgroup` resource.

    /// The [`AWS::RedshiftServerless::Workgroup.ConfigParameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html) property type.
    #[derive(Debug, Default)]
    pub struct ConfigParameter {
        /// Property [`ParameterKey`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html#cfn-redshiftserverless-workgroup-configparameter-parameterkey).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_key: Option<::Value<String>>,
        /// Property [`ParameterValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html#cfn-redshiftserverless-workgroup-configparameter-parametervalue).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub parameter_value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConfigParameter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref parameter_key) = self.parameter_key {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterKey", parameter_key)?;
            }
            if let Some(ref parameter_value) = self.parameter_value {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValue", parameter_value)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConfigParameter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConfigParameter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConfigParameter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConfigParameter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_key: Option<::Value<String>> = None;
                    let mut parameter_value: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterKey" => {
                                parameter_key = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ParameterValue" => {
                                parameter_value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConfigParameter {
                        parameter_key: parameter_key,
                        parameter_value: parameter_value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

//! Types for the `RDS` service.

/// The [`AWS::RDS::DBCluster`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbcluster.html) resource type.
#[derive(Debug)]
pub struct DBCluster {
    properties: DBClusterProperties
}

/// Properties for the `DBCluster` resource.
#[derive(Debug)]
pub struct DBClusterProperties {
    /// Property `AvailabilityZones`.
    pub availability_zones: Option<::ValueList<String>>,
    /// Property `BackupRetentionPeriod`.
    pub backup_retention_period: Option<::Value<u32>>,
    /// Property `DBClusterIdentifier`.
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property `DBClusterParameterGroupName`.
    pub db_cluster_parameter_group_name: Option<::Value<String>>,
    /// Property `DBSubnetGroupName`.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `DatabaseName`.
    pub database_name: Option<::Value<String>>,
    /// Property `Engine`.
    pub engine: ::Value<String>,
    /// Property `EngineVersion`.
    pub engine_version: Option<::Value<String>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `MasterUserPassword`.
    pub master_user_password: Option<::Value<String>>,
    /// Property `MasterUsername`.
    pub master_username: Option<::Value<String>>,
    /// Property `Port`.
    pub port: Option<::Value<u32>>,
    /// Property `PreferredBackupWindow`.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `ReplicationSourceIdentifier`.
    pub replication_source_identifier: Option<::Value<String>>,
    /// Property `SnapshotIdentifier`.
    pub snapshot_identifier: Option<::Value<String>>,
    /// Property `StorageEncrypted`.
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `VpcSecurityGroupIds`.
    pub vpc_security_group_ids: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DBClusterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZones", &self.availability_zones)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", &self.backup_retention_period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", &self.db_cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterParameterGroupName", &self.db_cluster_parameter_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", &self.db_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DatabaseName", &self.database_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", &self.master_user_password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", &self.preferred_backup_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ReplicationSourceIdentifier", &self.replication_source_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnapshotIdentifier", &self.snapshot_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", &self.storage_encrypted)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupIds", &self.vpc_security_group_ids)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut availability_zones = None;
                let mut backup_retention_period = None;
                let mut db_cluster_identifier = None;
                let mut db_cluster_parameter_group_name = None;
                let mut db_subnet_group_name = None;
                let mut database_name = None;
                let mut engine = None;
                let mut engine_version = None;
                let mut kms_key_id = None;
                let mut master_user_password = None;
                let mut master_username = None;
                let mut port = None;
                let mut preferred_backup_window = None;
                let mut preferred_maintenance_window = None;
                let mut replication_source_identifier = None;
                let mut snapshot_identifier = None;
                let mut storage_encrypted = None;
                let mut tags = None;
                let mut vpc_security_group_ids = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AvailabilityZones" => {
                            availability_zones = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBClusterParameterGroupName" => {
                            db_cluster_parameter_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DatabaseName" => {
                            database_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Engine" => {
                            engine = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineVersion" => {
                            engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUserPassword" => {
                            master_user_password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUsername" => {
                            master_username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ReplicationSourceIdentifier" => {
                            replication_source_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnapshotIdentifier" => {
                            snapshot_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VpcSecurityGroupIds" => {
                            vpc_security_group_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterProperties {
                    availability_zones: availability_zones,
                    backup_retention_period: backup_retention_period,
                    db_cluster_identifier: db_cluster_identifier,
                    db_cluster_parameter_group_name: db_cluster_parameter_group_name,
                    db_subnet_group_name: db_subnet_group_name,
                    database_name: database_name,
                    engine: engine.ok_or(::serde::de::Error::missing_field("Engine"))?,
                    engine_version: engine_version,
                    kms_key_id: kms_key_id,
                    master_user_password: master_user_password,
                    master_username: master_username,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    replication_source_identifier: replication_source_identifier,
                    snapshot_identifier: snapshot_identifier,
                    storage_encrypted: storage_encrypted,
                    tags: tags,
                    vpc_security_group_ids: vpc_security_group_ids,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBCluster {
    type Properties = DBClusterProperties;
    const TYPE: &'static str = "AWS::RDS::DBCluster";
    fn properties(&self) -> &DBClusterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBCluster {}

impl From<DBClusterProperties> for DBCluster {
    fn from(properties: DBClusterProperties) -> DBCluster {
        DBCluster { properties }
    }
}

/// The [`AWS::RDS::DBClusterParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbclusterparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBClusterParameterGroup {
    properties: DBClusterParameterGroupProperties
}

/// Properties for the `DBClusterParameterGroup` resource.
#[derive(Debug)]
pub struct DBClusterParameterGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `Family`.
    pub family: ::Value<String>,
    /// Property `Parameters`.
    pub parameters: ::Value<::json::Value>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBClusterParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBClusterParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBClusterParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBClusterParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBClusterParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut family = None;
                let mut parameters = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Family" => {
                            family = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBClusterParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    parameters: parameters.ok_or(::serde::de::Error::missing_field("Parameters"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBClusterParameterGroup {
    type Properties = DBClusterParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBClusterParameterGroup";
    fn properties(&self) -> &DBClusterParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBClusterParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBClusterParameterGroup {}

impl From<DBClusterParameterGroupProperties> for DBClusterParameterGroup {
    fn from(properties: DBClusterParameterGroupProperties) -> DBClusterParameterGroup {
        DBClusterParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-database-instance.html) resource type.
#[derive(Debug)]
pub struct DBInstance {
    properties: DBInstanceProperties
}

/// Properties for the `DBInstance` resource.
#[derive(Debug)]
pub struct DBInstanceProperties {
    /// Property `AllocatedStorage`.
    pub allocated_storage: Option<::Value<String>>,
    /// Property `AllowMajorVersionUpgrade`.
    pub allow_major_version_upgrade: Option<::Value<bool>>,
    /// Property `AutoMinorVersionUpgrade`.
    pub auto_minor_version_upgrade: Option<::Value<bool>>,
    /// Property `AvailabilityZone`.
    pub availability_zone: Option<::Value<String>>,
    /// Property `BackupRetentionPeriod`.
    pub backup_retention_period: Option<::Value<String>>,
    /// Property `CharacterSetName`.
    pub character_set_name: Option<::Value<String>>,
    /// Property `CopyTagsToSnapshot`.
    pub copy_tags_to_snapshot: Option<::Value<bool>>,
    /// Property `DBClusterIdentifier`.
    pub db_cluster_identifier: Option<::Value<String>>,
    /// Property `DBInstanceClass`.
    pub db_instance_class: ::Value<String>,
    /// Property `DBInstanceIdentifier`.
    pub db_instance_identifier: Option<::Value<String>>,
    /// Property `DBName`.
    pub db_name: Option<::Value<String>>,
    /// Property `DBParameterGroupName`.
    pub db_parameter_group_name: Option<::Value<String>>,
    /// Property `DBSecurityGroups`.
    pub db_security_groups: Option<::ValueList<String>>,
    /// Property `DBSnapshotIdentifier`.
    pub db_snapshot_identifier: Option<::Value<String>>,
    /// Property `DBSubnetGroupName`.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `Domain`.
    pub domain: Option<::Value<String>>,
    /// Property `DomainIAMRoleName`.
    pub domain_iam_role_name: Option<::Value<String>>,
    /// Property `Engine`.
    pub engine: Option<::Value<String>>,
    /// Property `EngineVersion`.
    pub engine_version: Option<::Value<String>>,
    /// Property `Iops`.
    pub iops: Option<::Value<u32>>,
    /// Property `KmsKeyId`.
    pub kms_key_id: Option<::Value<String>>,
    /// Property `LicenseModel`.
    pub license_model: Option<::Value<String>>,
    /// Property `MasterUserPassword`.
    pub master_user_password: Option<::Value<String>>,
    /// Property `MasterUsername`.
    pub master_username: Option<::Value<String>>,
    /// Property `MonitoringInterval`.
    pub monitoring_interval: Option<::Value<u32>>,
    /// Property `MonitoringRoleArn`.
    pub monitoring_role_arn: Option<::Value<String>>,
    /// Property `MultiAZ`.
    pub multi_az: Option<::Value<bool>>,
    /// Property `OptionGroupName`.
    pub option_group_name: Option<::Value<String>>,
    /// Property `Port`.
    pub port: Option<::Value<String>>,
    /// Property `PreferredBackupWindow`.
    pub preferred_backup_window: Option<::Value<String>>,
    /// Property `PreferredMaintenanceWindow`.
    pub preferred_maintenance_window: Option<::Value<String>>,
    /// Property `PubliclyAccessible`.
    pub publicly_accessible: Option<::Value<bool>>,
    /// Property `SourceDBInstanceIdentifier`.
    pub source_db_instance_identifier: Option<::Value<String>>,
    /// Property `SourceRegion`.
    pub source_region: Option<::Value<String>>,
    /// Property `StorageEncrypted`.
    pub storage_encrypted: Option<::Value<bool>>,
    /// Property `StorageType`.
    pub storage_type: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property `Timezone`.
    pub timezone: Option<::Value<String>>,
    /// Property `VPCSecurityGroups`.
    pub vpc_security_groups: Option<::ValueList<String>>,
}

impl ::serde::Serialize for DBInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllocatedStorage", &self.allocated_storage)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowMajorVersionUpgrade", &self.allow_major_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoMinorVersionUpgrade", &self.auto_minor_version_upgrade)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AvailabilityZone", &self.availability_zone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BackupRetentionPeriod", &self.backup_retention_period)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CharacterSetName", &self.character_set_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CopyTagsToSnapshot", &self.copy_tags_to_snapshot)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBClusterIdentifier", &self.db_cluster_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceClass", &self.db_instance_class)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBInstanceIdentifier", &self.db_instance_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBName", &self.db_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBParameterGroupName", &self.db_parameter_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroups", &self.db_security_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSnapshotIdentifier", &self.db_snapshot_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", &self.db_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", &self.domain)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainIAMRoleName", &self.domain_iam_role_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Engine", &self.engine)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineVersion", &self.engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Iops", &self.iops)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", &self.kms_key_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LicenseModel", &self.license_model)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUserPassword", &self.master_user_password)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MasterUsername", &self.master_username)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringInterval", &self.monitoring_interval)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringRoleArn", &self.monitoring_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiAZ", &self.multi_az)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupName", &self.option_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredBackupWindow", &self.preferred_backup_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreferredMaintenanceWindow", &self.preferred_maintenance_window)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PubliclyAccessible", &self.publicly_accessible)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceDBInstanceIdentifier", &self.source_db_instance_identifier)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceRegion", &self.source_region)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageEncrypted", &self.storage_encrypted)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "StorageType", &self.storage_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Timezone", &self.timezone)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VPCSecurityGroups", &self.vpc_security_groups)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allocated_storage = None;
                let mut allow_major_version_upgrade = None;
                let mut auto_minor_version_upgrade = None;
                let mut availability_zone = None;
                let mut backup_retention_period = None;
                let mut character_set_name = None;
                let mut copy_tags_to_snapshot = None;
                let mut db_cluster_identifier = None;
                let mut db_instance_class = None;
                let mut db_instance_identifier = None;
                let mut db_name = None;
                let mut db_parameter_group_name = None;
                let mut db_security_groups = None;
                let mut db_snapshot_identifier = None;
                let mut db_subnet_group_name = None;
                let mut domain = None;
                let mut domain_iam_role_name = None;
                let mut engine = None;
                let mut engine_version = None;
                let mut iops = None;
                let mut kms_key_id = None;
                let mut license_model = None;
                let mut master_user_password = None;
                let mut master_username = None;
                let mut monitoring_interval = None;
                let mut monitoring_role_arn = None;
                let mut multi_az = None;
                let mut option_group_name = None;
                let mut port = None;
                let mut preferred_backup_window = None;
                let mut preferred_maintenance_window = None;
                let mut publicly_accessible = None;
                let mut source_db_instance_identifier = None;
                let mut source_region = None;
                let mut storage_encrypted = None;
                let mut storage_type = None;
                let mut tags = None;
                let mut timezone = None;
                let mut vpc_security_groups = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllocatedStorage" => {
                            allocated_storage = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AllowMajorVersionUpgrade" => {
                            allow_major_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AutoMinorVersionUpgrade" => {
                            auto_minor_version_upgrade = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "AvailabilityZone" => {
                            availability_zone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "BackupRetentionPeriod" => {
                            backup_retention_period = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CharacterSetName" => {
                            character_set_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "CopyTagsToSnapshot" => {
                            copy_tags_to_snapshot = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBClusterIdentifier" => {
                            db_cluster_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBInstanceClass" => {
                            db_instance_class = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBInstanceIdentifier" => {
                            db_instance_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBName" => {
                            db_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBParameterGroupName" => {
                            db_parameter_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSecurityGroups" => {
                            db_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSnapshotIdentifier" => {
                            db_snapshot_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Domain" => {
                            domain = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DomainIAMRoleName" => {
                            domain_iam_role_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Engine" => {
                            engine = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EngineVersion" => {
                            engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Iops" => {
                            iops = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "KmsKeyId" => {
                            kms_key_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LicenseModel" => {
                            license_model = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUserPassword" => {
                            master_user_password = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MasterUsername" => {
                            master_username = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MonitoringInterval" => {
                            monitoring_interval = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MonitoringRoleArn" => {
                            monitoring_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MultiAZ" => {
                            multi_az = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OptionGroupName" => {
                            option_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Port" => {
                            port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredBackupWindow" => {
                            preferred_backup_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PreferredMaintenanceWindow" => {
                            preferred_maintenance_window = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PubliclyAccessible" => {
                            publicly_accessible = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceDBInstanceIdentifier" => {
                            source_db_instance_identifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceRegion" => {
                            source_region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StorageEncrypted" => {
                            storage_encrypted = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "StorageType" => {
                            storage_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Timezone" => {
                            timezone = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "VPCSecurityGroups" => {
                            vpc_security_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBInstanceProperties {
                    allocated_storage: allocated_storage,
                    allow_major_version_upgrade: allow_major_version_upgrade,
                    auto_minor_version_upgrade: auto_minor_version_upgrade,
                    availability_zone: availability_zone,
                    backup_retention_period: backup_retention_period,
                    character_set_name: character_set_name,
                    copy_tags_to_snapshot: copy_tags_to_snapshot,
                    db_cluster_identifier: db_cluster_identifier,
                    db_instance_class: db_instance_class.ok_or(::serde::de::Error::missing_field("DBInstanceClass"))?,
                    db_instance_identifier: db_instance_identifier,
                    db_name: db_name,
                    db_parameter_group_name: db_parameter_group_name,
                    db_security_groups: db_security_groups,
                    db_snapshot_identifier: db_snapshot_identifier,
                    db_subnet_group_name: db_subnet_group_name,
                    domain: domain,
                    domain_iam_role_name: domain_iam_role_name,
                    engine: engine,
                    engine_version: engine_version,
                    iops: iops,
                    kms_key_id: kms_key_id,
                    license_model: license_model,
                    master_user_password: master_user_password,
                    master_username: master_username,
                    monitoring_interval: monitoring_interval,
                    monitoring_role_arn: monitoring_role_arn,
                    multi_az: multi_az,
                    option_group_name: option_group_name,
                    port: port,
                    preferred_backup_window: preferred_backup_window,
                    preferred_maintenance_window: preferred_maintenance_window,
                    publicly_accessible: publicly_accessible,
                    source_db_instance_identifier: source_db_instance_identifier,
                    source_region: source_region,
                    storage_encrypted: storage_encrypted,
                    storage_type: storage_type,
                    tags: tags,
                    timezone: timezone,
                    vpc_security_groups: vpc_security_groups,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBInstance {
    type Properties = DBInstanceProperties;
    const TYPE: &'static str = "AWS::RDS::DBInstance";
    fn properties(&self) -> &DBInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBInstance {}

impl From<DBInstanceProperties> for DBInstance {
    fn from(properties: DBInstanceProperties) -> DBInstance {
        DBInstance { properties }
    }
}

/// The [`AWS::RDS::DBParameterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-dbparametergroup.html) resource type.
#[derive(Debug)]
pub struct DBParameterGroup {
    properties: DBParameterGroupProperties
}

/// Properties for the `DBParameterGroup` resource.
#[derive(Debug)]
pub struct DBParameterGroupProperties {
    /// Property `Description`.
    pub description: ::Value<String>,
    /// Property `Family`.
    pub family: ::Value<String>,
    /// Property `Parameters`.
    pub parameters: Option<::ValueMap<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBParameterGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Family", &self.family)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBParameterGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBParameterGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBParameterGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBParameterGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut family = None;
                let mut parameters = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Family" => {
                            family = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBParameterGroupProperties {
                    description: description.ok_or(::serde::de::Error::missing_field("Description"))?,
                    family: family.ok_or(::serde::de::Error::missing_field("Family"))?,
                    parameters: parameters,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBParameterGroup {
    type Properties = DBParameterGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBParameterGroup";
    fn properties(&self) -> &DBParameterGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBParameterGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBParameterGroup {}

impl From<DBParameterGroupProperties> for DBParameterGroup {
    fn from(properties: DBParameterGroupProperties) -> DBParameterGroup {
        DBParameterGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroup {
    properties: DBSecurityGroupProperties
}

/// Properties for the `DBSecurityGroup` resource.
#[derive(Debug)]
pub struct DBSecurityGroupProperties {
    /// Property `DBSecurityGroupIngress`.
    pub db_security_group_ingress: ::ValueList<self::db_security_group::Ingress>,
    /// Property `EC2VpcId`.
    pub ec2_vpc_id: Option<::Value<String>>,
    /// Property `GroupDescription`.
    pub group_description: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBSecurityGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupIngress", &self.db_security_group_ingress)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2VpcId", &self.ec2_vpc_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroupDescription", &self.group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_security_group_ingress = None;
                let mut ec2_vpc_id = None;
                let mut group_description = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSecurityGroupIngress" => {
                            db_security_group_ingress = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2VpcId" => {
                            ec2_vpc_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GroupDescription" => {
                            group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupProperties {
                    db_security_group_ingress: db_security_group_ingress.ok_or(::serde::de::Error::missing_field("DBSecurityGroupIngress"))?,
                    ec2_vpc_id: ec2_vpc_id,
                    group_description: group_description.ok_or(::serde::de::Error::missing_field("GroupDescription"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBSecurityGroup {
    type Properties = DBSecurityGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroup";
    fn properties(&self) -> &DBSecurityGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroup {}

impl From<DBSecurityGroupProperties> for DBSecurityGroup {
    fn from(properties: DBSecurityGroupProperties) -> DBSecurityGroup {
        DBSecurityGroup { properties }
    }
}

/// The [`AWS::RDS::DBSecurityGroupIngress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-security-group-ingress.html) resource type.
#[derive(Debug)]
pub struct DBSecurityGroupIngress {
    properties: DBSecurityGroupIngressProperties
}

/// Properties for the `DBSecurityGroupIngress` resource.
#[derive(Debug)]
pub struct DBSecurityGroupIngressProperties {
    /// Property `CIDRIP`.
    pub cidrip: Option<::Value<String>>,
    /// Property `DBSecurityGroupName`.
    pub db_security_group_name: ::Value<String>,
    /// Property `EC2SecurityGroupId`.
    pub ec2_security_group_id: Option<::Value<String>>,
    /// Property `EC2SecurityGroupName`.
    pub ec2_security_group_name: Option<::Value<String>>,
    /// Property `EC2SecurityGroupOwnerId`.
    pub ec2_security_group_owner_id: Option<::Value<String>>,
}

impl ::serde::Serialize for DBSecurityGroupIngressProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", &self.cidrip)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupName", &self.db_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", &self.ec2_security_group_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", &self.ec2_security_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", &self.ec2_security_group_owner_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSecurityGroupIngressProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSecurityGroupIngressProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSecurityGroupIngressProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSecurityGroupIngressProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut cidrip = None;
                let mut db_security_group_name = None;
                let mut ec2_security_group_id = None;
                let mut ec2_security_group_name = None;
                let mut ec2_security_group_owner_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CIDRIP" => {
                            cidrip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSecurityGroupName" => {
                            db_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2SecurityGroupId" => {
                            ec2_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2SecurityGroupName" => {
                            ec2_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EC2SecurityGroupOwnerId" => {
                            ec2_security_group_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBSecurityGroupIngressProperties {
                    cidrip: cidrip,
                    db_security_group_name: db_security_group_name.ok_or(::serde::de::Error::missing_field("DBSecurityGroupName"))?,
                    ec2_security_group_id: ec2_security_group_id,
                    ec2_security_group_name: ec2_security_group_name,
                    ec2_security_group_owner_id: ec2_security_group_owner_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBSecurityGroupIngress {
    type Properties = DBSecurityGroupIngressProperties;
    const TYPE: &'static str = "AWS::RDS::DBSecurityGroupIngress";
    fn properties(&self) -> &DBSecurityGroupIngressProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSecurityGroupIngressProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSecurityGroupIngress {}

impl From<DBSecurityGroupIngressProperties> for DBSecurityGroupIngress {
    fn from(properties: DBSecurityGroupIngressProperties) -> DBSecurityGroupIngress {
        DBSecurityGroupIngress { properties }
    }
}

/// The [`AWS::RDS::DBSubnetGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-dbsubnet-group.html) resource type.
#[derive(Debug)]
pub struct DBSubnetGroup {
    properties: DBSubnetGroupProperties
}

/// Properties for the `DBSubnetGroup` resource.
#[derive(Debug)]
pub struct DBSubnetGroupProperties {
    /// Property `DBSubnetGroupDescription`.
    pub db_subnet_group_description: ::Value<String>,
    /// Property `DBSubnetGroupName`.
    pub db_subnet_group_name: Option<::Value<String>>,
    /// Property `SubnetIds`.
    pub subnet_ids: ::ValueList<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DBSubnetGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupDescription", &self.db_subnet_group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSubnetGroupName", &self.db_subnet_group_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DBSubnetGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DBSubnetGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DBSubnetGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DBSubnetGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut db_subnet_group_description = None;
                let mut db_subnet_group_name = None;
                let mut subnet_ids = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DBSubnetGroupDescription" => {
                            db_subnet_group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DBSubnetGroupName" => {
                            db_subnet_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SubnetIds" => {
                            subnet_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DBSubnetGroupProperties {
                    db_subnet_group_description: db_subnet_group_description.ok_or(::serde::de::Error::missing_field("DBSubnetGroupDescription"))?,
                    db_subnet_group_name: db_subnet_group_name,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for DBSubnetGroup {
    type Properties = DBSubnetGroupProperties;
    const TYPE: &'static str = "AWS::RDS::DBSubnetGroup";
    fn properties(&self) -> &DBSubnetGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DBSubnetGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DBSubnetGroup {}

impl From<DBSubnetGroupProperties> for DBSubnetGroup {
    fn from(properties: DBSubnetGroupProperties) -> DBSubnetGroup {
        DBSubnetGroup { properties }
    }
}

/// The [`AWS::RDS::EventSubscription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-eventsubscription.html) resource type.
#[derive(Debug)]
pub struct EventSubscription {
    properties: EventSubscriptionProperties
}

/// Properties for the `EventSubscription` resource.
#[derive(Debug)]
pub struct EventSubscriptionProperties {
    /// Property `Enabled`.
    pub enabled: Option<::Value<bool>>,
    /// Property `EventCategories`.
    pub event_categories: Option<::ValueList<String>>,
    /// Property `SnsTopicArn`.
    pub sns_topic_arn: ::Value<String>,
    /// Property `SourceIds`.
    pub source_ids: Option<::ValueList<String>>,
    /// Property `SourceType`.
    pub source_type: Option<::Value<String>>,
}

impl ::serde::Serialize for EventSubscriptionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Enabled", &self.enabled)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventCategories", &self.event_categories)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SnsTopicArn", &self.sns_topic_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceIds", &self.source_ids)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceType", &self.source_type)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EventSubscriptionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EventSubscriptionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EventSubscriptionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EventSubscriptionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut enabled = None;
                let mut event_categories = None;
                let mut sns_topic_arn = None;
                let mut source_ids = None;
                let mut source_type = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Enabled" => {
                            enabled = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "EventCategories" => {
                            event_categories = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SnsTopicArn" => {
                            sns_topic_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceIds" => {
                            source_ids = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "SourceType" => {
                            source_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(EventSubscriptionProperties {
                    enabled: enabled,
                    event_categories: event_categories,
                    sns_topic_arn: sns_topic_arn.ok_or(::serde::de::Error::missing_field("SnsTopicArn"))?,
                    source_ids: source_ids,
                    source_type: source_type,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for EventSubscription {
    type Properties = EventSubscriptionProperties;
    const TYPE: &'static str = "AWS::RDS::EventSubscription";
    fn properties(&self) -> &EventSubscriptionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EventSubscriptionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EventSubscription {}

impl From<EventSubscriptionProperties> for EventSubscription {
    fn from(properties: EventSubscriptionProperties) -> EventSubscription {
        EventSubscription { properties }
    }
}

/// The [`AWS::RDS::OptionGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rds-optiongroup.html) resource type.
#[derive(Debug)]
pub struct OptionGroup {
    properties: OptionGroupProperties
}

/// Properties for the `OptionGroup` resource.
#[derive(Debug)]
pub struct OptionGroupProperties {
    /// Property `EngineName`.
    pub engine_name: ::Value<String>,
    /// Property `MajorEngineVersion`.
    pub major_engine_version: ::Value<String>,
    /// Property `OptionConfigurations`.
    pub option_configurations: ::ValueList<self::option_group::OptionConfiguration>,
    /// Property `OptionGroupDescription`.
    pub option_group_description: ::Value<String>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for OptionGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EngineName", &self.engine_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MajorEngineVersion", &self.major_engine_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionConfigurations", &self.option_configurations)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionGroupDescription", &self.option_group_description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for OptionGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = OptionGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type OptionGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut engine_name = None;
                let mut major_engine_version = None;
                let mut option_configurations = None;
                let mut option_group_description = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EngineName" => {
                            engine_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MajorEngineVersion" => {
                            major_engine_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OptionConfigurations" => {
                            option_configurations = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OptionGroupDescription" => {
                            option_group_description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(OptionGroupProperties {
                    engine_name: engine_name.ok_or(::serde::de::Error::missing_field("EngineName"))?,
                    major_engine_version: major_engine_version.ok_or(::serde::de::Error::missing_field("MajorEngineVersion"))?,
                    option_configurations: option_configurations.ok_or(::serde::de::Error::missing_field("OptionConfigurations"))?,
                    option_group_description: option_group_description.ok_or(::serde::de::Error::missing_field("OptionGroupDescription"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for OptionGroup {
    type Properties = OptionGroupProperties;
    const TYPE: &'static str = "AWS::RDS::OptionGroup";
    fn properties(&self) -> &OptionGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut OptionGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for OptionGroup {}

impl From<OptionGroupProperties> for OptionGroup {
    fn from(properties: OptionGroupProperties) -> OptionGroup {
        OptionGroup { properties }
    }
}

pub mod db_security_group {
    //! Property types for the `DBSecurityGroup` resource.

    /// The [`AWS::RDS::DBSecurityGroup.Ingress`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-security-group-rule.html) property type.
    #[derive(Debug)]
    pub struct Ingress {
        /// Property `CIDRIP`.
        pub cidrip: Option<::Value<String>>,
        /// Property `EC2SecurityGroupId`.
        pub ec2_security_group_id: Option<::Value<String>>,
        /// Property `EC2SecurityGroupName`.
        pub ec2_security_group_name: Option<::Value<String>>,
        /// Property `EC2SecurityGroupOwnerId`.
        pub ec2_security_group_owner_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Ingress {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CIDRIP", &self.cidrip)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupId", &self.ec2_security_group_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupName", &self.ec2_security_group_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EC2SecurityGroupOwnerId", &self.ec2_security_group_owner_id)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Ingress {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Ingress, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Ingress;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Ingress")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cidrip = None;
                    let mut ec2_security_group_id = None;
                    let mut ec2_security_group_name = None;
                    let mut ec2_security_group_owner_id = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CIDRIP" => {
                                cidrip = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EC2SecurityGroupId" => {
                                ec2_security_group_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EC2SecurityGroupName" => {
                                ec2_security_group_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EC2SecurityGroupOwnerId" => {
                                ec2_security_group_owner_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Ingress {
                        cidrip: cidrip,
                        ec2_security_group_id: ec2_security_group_id,
                        ec2_security_group_name: ec2_security_group_name,
                        ec2_security_group_owner_id: ec2_security_group_owner_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod option_group {
    //! Property types for the `OptionGroup` resource.

    /// The [`AWS::RDS::OptionGroup.OptionConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations.html) property type.
    #[derive(Debug)]
    pub struct OptionConfiguration {
        /// Property `DBSecurityGroupMemberships`.
        pub db_security_group_memberships: Option<::ValueList<String>>,
        /// Property `OptionName`.
        pub option_name: ::Value<String>,
        /// Property `OptionSettings`.
        pub option_settings: Option<::Value<OptionSetting>>,
        /// Property `OptionVersion`.
        pub option_version: Option<::Value<String>>,
        /// Property `Port`.
        pub port: Option<::Value<u32>>,
        /// Property `VpcSecurityGroupMemberships`.
        pub vpc_security_group_memberships: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for OptionConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DBSecurityGroupMemberships", &self.db_security_group_memberships)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionName", &self.option_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionSettings", &self.option_settings)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OptionVersion", &self.option_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Port", &self.port)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcSecurityGroupMemberships", &self.vpc_security_group_memberships)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OptionConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut db_security_group_memberships = None;
                    let mut option_name = None;
                    let mut option_settings = None;
                    let mut option_version = None;
                    let mut port = None;
                    let mut vpc_security_group_memberships = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DBSecurityGroupMemberships" => {
                                db_security_group_memberships = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OptionName" => {
                                option_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OptionSettings" => {
                                option_settings = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OptionVersion" => {
                                option_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Port" => {
                                port = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "VpcSecurityGroupMemberships" => {
                                vpc_security_group_memberships = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionConfiguration {
                        db_security_group_memberships: db_security_group_memberships,
                        option_name: option_name.ok_or(::serde::de::Error::missing_field("OptionName"))?,
                        option_settings: option_settings,
                        option_version: option_version,
                        port: port,
                        vpc_security_group_memberships: vpc_security_group_memberships,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::RDS::OptionGroup.OptionSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rds-optiongroup-optionconfigurations-optionsettings.html) property type.
    #[derive(Debug)]
    pub struct OptionSetting {
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Value`.
        pub value: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OptionSetting {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OptionSetting {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OptionSetting, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OptionSetting;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OptionSetting")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut name = None;
                    let mut value = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Value" => {
                                value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(OptionSetting {
                        name: name,
                        value: value,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

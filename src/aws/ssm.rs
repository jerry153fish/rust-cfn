//! Types for the `SSM` service.

/// The [`AWS::SSM::Association`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html) resource type.
#[derive(Debug)]
pub struct Association {
    properties: AssociationProperties
}

/// Properties for the `Association` resource.
#[derive(Debug)]
pub struct AssociationProperties {
    /// Property `AssociationName`.
    pub association_name: Option<::Value<String>>,
    /// Property `DocumentVersion`.
    pub document_version: Option<::Value<String>>,
    /// Property `InstanceId`.
    pub instance_id: Option<::Value<String>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `Parameters`.
    pub parameters: Option<::ValueMap<self::association::ParameterValues>>,
    /// Property `ScheduleExpression`.
    pub schedule_expression: Option<::Value<String>>,
    /// Property `Targets`.
    pub targets: Option<::ValueList<self::association::Target>>,
}

impl ::serde::Serialize for AssociationProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssociationName", &self.association_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentVersion", &self.document_version)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceId", &self.instance_id)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AssociationProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AssociationProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AssociationProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AssociationProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut association_name = None;
                let mut document_version = None;
                let mut instance_id = None;
                let mut name = None;
                let mut parameters = None;
                let mut schedule_expression = None;
                let mut targets = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AssociationName" => {
                            association_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DocumentVersion" => {
                            document_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "InstanceId" => {
                            instance_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Parameters" => {
                            parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ScheduleExpression" => {
                            schedule_expression = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Targets" => {
                            targets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(AssociationProperties {
                    association_name: association_name,
                    document_version: document_version,
                    instance_id: instance_id,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    parameters: parameters,
                    schedule_expression: schedule_expression,
                    targets: targets,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Association {
    type Properties = AssociationProperties;
    const TYPE: &'static str = "AWS::SSM::Association";
    fn properties(&self) -> &AssociationProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AssociationProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Association {}

impl From<AssociationProperties> for Association {
    fn from(properties: AssociationProperties) -> Association {
        Association { properties }
    }
}

/// The [`AWS::SSM::Document`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html) resource type.
#[derive(Debug)]
pub struct Document {
    properties: DocumentProperties
}

/// Properties for the `Document` resource.
#[derive(Debug)]
pub struct DocumentProperties {
    /// Property `Content`.
    pub content: ::Value<::json::Value>,
    /// Property `DocumentType`.
    pub document_type: Option<::Value<String>>,
    /// Property `Tags`.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DocumentProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", &self.content)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentType", &self.document_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", &self.tags)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DocumentProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DocumentProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DocumentProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DocumentProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut content = None;
                let mut document_type = None;
                let mut tags = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Content" => {
                            content = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "DocumentType" => {
                            document_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Tags" => {
                            tags = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(DocumentProperties {
                    content: content.ok_or(::serde::de::Error::missing_field("Content"))?,
                    document_type: document_type,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Document {
    type Properties = DocumentProperties;
    const TYPE: &'static str = "AWS::SSM::Document";
    fn properties(&self) -> &DocumentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DocumentProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Document {}

impl From<DocumentProperties> for Document {
    fn from(properties: DocumentProperties) -> Document {
        Document { properties }
    }
}

/// The [`AWS::SSM::MaintenanceWindowTask`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html) resource type.
#[derive(Debug)]
pub struct MaintenanceWindowTask {
    properties: MaintenanceWindowTaskProperties
}

/// Properties for the `MaintenanceWindowTask` resource.
#[derive(Debug)]
pub struct MaintenanceWindowTaskProperties {
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `LoggingInfo`.
    pub logging_info: Option<::Value<self::maintenance_window_task::LoggingInfo>>,
    /// Property `MaxConcurrency`.
    pub max_concurrency: ::Value<String>,
    /// Property `MaxErrors`.
    pub max_errors: ::Value<String>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `Priority`.
    pub priority: ::Value<u32>,
    /// Property `ServiceRoleArn`.
    pub service_role_arn: ::Value<String>,
    /// Property `Targets`.
    pub targets: ::ValueList<self::maintenance_window_task::Target>,
    /// Property `TaskArn`.
    pub task_arn: ::Value<String>,
    /// Property `TaskInvocationParameters`.
    pub task_invocation_parameters: Option<::Value<self::maintenance_window_task::TaskInvocationParameters>>,
    /// Property `TaskParameters`.
    pub task_parameters: Option<::Value<::json::Value>>,
    /// Property `TaskType`.
    pub task_type: ::Value<String>,
    /// Property `WindowId`.
    pub window_id: Option<::Value<String>>,
}

impl ::serde::Serialize for MaintenanceWindowTaskProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "LoggingInfo", &self.logging_info)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrency", &self.max_concurrency)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxErrors", &self.max_errors)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Priority", &self.priority)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Targets", &self.targets)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskArn", &self.task_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskInvocationParameters", &self.task_invocation_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskParameters", &self.task_parameters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "TaskType", &self.task_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "WindowId", &self.window_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MaintenanceWindowTaskProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowTaskProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MaintenanceWindowTaskProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MaintenanceWindowTaskProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description = None;
                let mut logging_info = None;
                let mut max_concurrency = None;
                let mut max_errors = None;
                let mut name = None;
                let mut priority = None;
                let mut service_role_arn = None;
                let mut targets = None;
                let mut task_arn = None;
                let mut task_invocation_parameters = None;
                let mut task_parameters = None;
                let mut task_type = None;
                let mut window_id = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "LoggingInfo" => {
                            logging_info = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaxConcurrency" => {
                            max_concurrency = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "MaxErrors" => {
                            max_errors = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Priority" => {
                            priority = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ServiceRoleArn" => {
                            service_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Targets" => {
                            targets = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskArn" => {
                            task_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskInvocationParameters" => {
                            task_invocation_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskParameters" => {
                            task_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "TaskType" => {
                            task_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "WindowId" => {
                            window_id = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(MaintenanceWindowTaskProperties {
                    description: description,
                    logging_info: logging_info,
                    max_concurrency: max_concurrency.ok_or(::serde::de::Error::missing_field("MaxConcurrency"))?,
                    max_errors: max_errors.ok_or(::serde::de::Error::missing_field("MaxErrors"))?,
                    name: name,
                    priority: priority.ok_or(::serde::de::Error::missing_field("Priority"))?,
                    service_role_arn: service_role_arn.ok_or(::serde::de::Error::missing_field("ServiceRoleArn"))?,
                    targets: targets.ok_or(::serde::de::Error::missing_field("Targets"))?,
                    task_arn: task_arn.ok_or(::serde::de::Error::missing_field("TaskArn"))?,
                    task_invocation_parameters: task_invocation_parameters,
                    task_parameters: task_parameters,
                    task_type: task_type.ok_or(::serde::de::Error::missing_field("TaskType"))?,
                    window_id: window_id,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for MaintenanceWindowTask {
    type Properties = MaintenanceWindowTaskProperties;
    const TYPE: &'static str = "AWS::SSM::MaintenanceWindowTask";
    fn properties(&self) -> &MaintenanceWindowTaskProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MaintenanceWindowTaskProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MaintenanceWindowTask {}

impl From<MaintenanceWindowTaskProperties> for MaintenanceWindowTask {
    fn from(properties: MaintenanceWindowTaskProperties) -> MaintenanceWindowTask {
        MaintenanceWindowTask { properties }
    }
}

/// The [`AWS::SSM::Parameter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html) resource type.
#[derive(Debug)]
pub struct Parameter {
    properties: ParameterProperties
}

/// Properties for the `Parameter` resource.
#[derive(Debug)]
pub struct ParameterProperties {
    /// Property `AllowedPattern`.
    pub allowed_pattern: Option<::Value<String>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `Name`.
    pub name: Option<::Value<String>>,
    /// Property `Type`.
    pub type_: ::Value<String>,
    /// Property `Value`.
    pub value: ::Value<String>,
}

impl ::serde::Serialize for ParameterProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AllowedPattern", &self.allowed_pattern)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.type_)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ParameterProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ParameterProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ParameterProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut allowed_pattern = None;
                let mut description = None;
                let mut name = None;
                let mut type_ = None;
                let mut value = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AllowedPattern" => {
                            allowed_pattern = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Type" => {
                            type_ = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Value" => {
                            value = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(ParameterProperties {
                    allowed_pattern: allowed_pattern,
                    description: description,
                    name: name,
                    type_: type_.ok_or(::serde::de::Error::missing_field("Type"))?,
                    value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for Parameter {
    type Properties = ParameterProperties;
    const TYPE: &'static str = "AWS::SSM::Parameter";
    fn properties(&self) -> &ParameterProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ParameterProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Parameter {}

impl From<ParameterProperties> for Parameter {
    fn from(properties: ParameterProperties) -> Parameter {
        Parameter { properties }
    }
}

/// The [`AWS::SSM::PatchBaseline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html) resource type.
#[derive(Debug)]
pub struct PatchBaseline {
    properties: PatchBaselineProperties
}

/// Properties for the `PatchBaseline` resource.
#[derive(Debug)]
pub struct PatchBaselineProperties {
    /// Property `ApprovalRules`.
    pub approval_rules: Option<::Value<self::patch_baseline::RuleGroup>>,
    /// Property `ApprovedPatches`.
    pub approved_patches: Option<::ValueList<String>>,
    /// Property `ApprovedPatchesComplianceLevel`.
    pub approved_patches_compliance_level: Option<::Value<String>>,
    /// Property `ApprovedPatchesEnableNonSecurity`.
    pub approved_patches_enable_non_security: Option<::Value<bool>>,
    /// Property `Description`.
    pub description: Option<::Value<String>>,
    /// Property `GlobalFilters`.
    pub global_filters: Option<::Value<self::patch_baseline::PatchFilterGroup>>,
    /// Property `Name`.
    pub name: ::Value<String>,
    /// Property `OperatingSystem`.
    pub operating_system: Option<::Value<String>>,
    /// Property `PatchGroups`.
    pub patch_groups: Option<::ValueList<String>>,
    /// Property `RejectedPatches`.
    pub rejected_patches: Option<::ValueList<String>>,
    /// Property `Sources`.
    pub sources: Option<::ValueList<self::patch_baseline::PatchSource>>,
}

impl ::serde::Serialize for PatchBaselineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovalRules", &self.approval_rules)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatches", &self.approved_patches)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatchesComplianceLevel", &self.approved_patches_compliance_level)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovedPatchesEnableNonSecurity", &self.approved_patches_enable_non_security)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", &self.description)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GlobalFilters", &self.global_filters)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OperatingSystem", &self.operating_system)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchGroups", &self.patch_groups)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RejectedPatches", &self.rejected_patches)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Sources", &self.sources)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PatchBaselineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchBaselineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PatchBaselineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PatchBaselineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut approval_rules = None;
                let mut approved_patches = None;
                let mut approved_patches_compliance_level = None;
                let mut approved_patches_enable_non_security = None;
                let mut description = None;
                let mut global_filters = None;
                let mut name = None;
                let mut operating_system = None;
                let mut patch_groups = None;
                let mut rejected_patches = None;
                let mut sources = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ApprovalRules" => {
                            approval_rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApprovedPatches" => {
                            approved_patches = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApprovedPatchesComplianceLevel" => {
                            approved_patches_compliance_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "ApprovedPatchesEnableNonSecurity" => {
                            approved_patches_enable_non_security = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Description" => {
                            description = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "GlobalFilters" => {
                            global_filters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Name" => {
                            name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "OperatingSystem" => {
                            operating_system = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "PatchGroups" => {
                            patch_groups = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "RejectedPatches" => {
                            rejected_patches = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        "Sources" => {
                            sources = Some(::serde::de::MapAccess::next_value(&mut map)?);
                        }
                        _ => {}
                    }
                }

                Ok(PatchBaselineProperties {
                    approval_rules: approval_rules,
                    approved_patches: approved_patches,
                    approved_patches_compliance_level: approved_patches_compliance_level,
                    approved_patches_enable_non_security: approved_patches_enable_non_security,
                    description: description,
                    global_filters: global_filters,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    operating_system: operating_system,
                    patch_groups: patch_groups,
                    rejected_patches: rejected_patches,
                    sources: sources,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl<'a> ::Resource<'a> for PatchBaseline {
    type Properties = PatchBaselineProperties;
    const TYPE: &'static str = "AWS::SSM::PatchBaseline";
    fn properties(&self) -> &PatchBaselineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PatchBaselineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for PatchBaseline {}

impl From<PatchBaselineProperties> for PatchBaseline {
    fn from(properties: PatchBaselineProperties) -> PatchBaseline {
        PatchBaseline { properties }
    }
}

pub mod association {
    //! Property types for the `Association` resource.

    /// The [`AWS::SSM::Association.ParameterValues`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-parametervalues.html) property type.
    #[derive(Debug)]
    pub struct ParameterValues {
        /// Property `ParameterValues`.
        pub parameter_values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for ParameterValues {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParameterValues", &self.parameter_values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ParameterValues {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ParameterValues, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ParameterValues;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ParameterValues")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut parameter_values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ParameterValues" => {
                                parameter_values = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(ParameterValues {
                        parameter_values: parameter_values.ok_or(::serde::de::Error::missing_field("ParameterValues"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::Association.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html) property type.
    #[derive(Debug)]
    pub struct Target {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Values`.
        pub values: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Values" => {
                                values = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values.ok_or(::serde::de::Error::missing_field("Values"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod maintenance_window_task {
    //! Property types for the `MaintenanceWindowTask` resource.

    /// The [`AWS::SSM::MaintenanceWindowTask.LoggingInfo`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html) property type.
    #[derive(Debug)]
    pub struct LoggingInfo {
        /// Property `Region`.
        pub region: ::Value<String>,
        /// Property `S3Bucket`.
        pub s3_bucket: ::Value<String>,
        /// Property `S3Prefix`.
        pub s3_prefix: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for LoggingInfo {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Region", &self.region)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Bucket", &self.s3_bucket)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Prefix", &self.s3_prefix)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for LoggingInfo {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<LoggingInfo, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = LoggingInfo;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type LoggingInfo")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut region = None;
                    let mut s3_bucket = None;
                    let mut s3_prefix = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Region" => {
                                region = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Bucket" => {
                                s3_bucket = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "S3Prefix" => {
                                s3_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(LoggingInfo {
                        region: region.ok_or(::serde::de::Error::missing_field("Region"))?,
                        s3_bucket: s3_bucket.ok_or(::serde::de::Error::missing_field("S3Bucket"))?,
                        s3_prefix: s3_prefix,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html) property type.
    #[derive(Debug)]
    pub struct MaintenanceWindowAutomationParameters {
        /// Property `DocumentVersion`.
        pub document_version: Option<::Value<String>>,
        /// Property `Parameters`.
        pub parameters: Option<::Value<::json::Value>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowAutomationParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentVersion", &self.document_version)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowAutomationParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowAutomationParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowAutomationParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowAutomationParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut document_version = None;
                    let mut parameters = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DocumentVersion" => {
                                document_version = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Parameters" => {
                                parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowAutomationParameters {
                        document_version: document_version,
                        parameters: parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html) property type.
    #[derive(Debug)]
    pub struct MaintenanceWindowLambdaParameters {
        /// Property `ClientContext`.
        pub client_context: Option<::Value<String>>,
        /// Property `Payload`.
        pub payload: Option<::Value<String>>,
        /// Property `Qualifier`.
        pub qualifier: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowLambdaParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientContext", &self.client_context)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Payload", &self.payload)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Qualifier", &self.qualifier)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowLambdaParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowLambdaParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowLambdaParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowLambdaParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_context = None;
                    let mut payload = None;
                    let mut qualifier = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientContext" => {
                                client_context = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Payload" => {
                                payload = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Qualifier" => {
                                qualifier = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowLambdaParameters {
                        client_context: client_context,
                        payload: payload,
                        qualifier: qualifier,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html) property type.
    #[derive(Debug)]
    pub struct MaintenanceWindowRunCommandParameters {
        /// Property `Comment`.
        pub comment: Option<::Value<String>>,
        /// Property `DocumentHash`.
        pub document_hash: Option<::Value<String>>,
        /// Property `DocumentHashType`.
        pub document_hash_type: Option<::Value<String>>,
        /// Property `NotificationConfig`.
        pub notification_config: Option<::Value<NotificationConfig>>,
        /// Property `OutputS3BucketName`.
        pub output_s3_bucket_name: Option<::Value<String>>,
        /// Property `OutputS3KeyPrefix`.
        pub output_s3_key_prefix: Option<::Value<String>>,
        /// Property `Parameters`.
        pub parameters: Option<::Value<::json::Value>>,
        /// Property `ServiceRoleArn`.
        pub service_role_arn: Option<::Value<String>>,
        /// Property `TimeoutSeconds`.
        pub timeout_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowRunCommandParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Comment", &self.comment)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentHash", &self.document_hash)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DocumentHashType", &self.document_hash_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfig", &self.notification_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3BucketName", &self.output_s3_bucket_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputS3KeyPrefix", &self.output_s3_key_prefix)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Parameters", &self.parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceRoleArn", &self.service_role_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TimeoutSeconds", &self.timeout_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowRunCommandParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowRunCommandParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowRunCommandParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowRunCommandParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut comment = None;
                    let mut document_hash = None;
                    let mut document_hash_type = None;
                    let mut notification_config = None;
                    let mut output_s3_bucket_name = None;
                    let mut output_s3_key_prefix = None;
                    let mut parameters = None;
                    let mut service_role_arn = None;
                    let mut timeout_seconds = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Comment" => {
                                comment = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DocumentHash" => {
                                document_hash = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "DocumentHashType" => {
                                document_hash_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NotificationConfig" => {
                                notification_config = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OutputS3BucketName" => {
                                output_s3_bucket_name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "OutputS3KeyPrefix" => {
                                output_s3_key_prefix = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Parameters" => {
                                parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ServiceRoleArn" => {
                                service_role_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "TimeoutSeconds" => {
                                timeout_seconds = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowRunCommandParameters {
                        comment: comment,
                        document_hash: document_hash,
                        document_hash_type: document_hash_type,
                        notification_config: notification_config,
                        output_s3_bucket_name: output_s3_bucket_name,
                        output_s3_key_prefix: output_s3_key_prefix,
                        parameters: parameters,
                        service_role_arn: service_role_arn,
                        timeout_seconds: timeout_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html) property type.
    #[derive(Debug)]
    pub struct MaintenanceWindowStepFunctionsParameters {
        /// Property `Input`.
        pub input: Option<::Value<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MaintenanceWindowStepFunctionsParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Input", &self.input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MaintenanceWindowStepFunctionsParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MaintenanceWindowStepFunctionsParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MaintenanceWindowStepFunctionsParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MaintenanceWindowStepFunctionsParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut input = None;
                    let mut name = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Input" => {
                                input = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(MaintenanceWindowStepFunctionsParameters {
                        input: input,
                        name: name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html) property type.
    #[derive(Debug)]
    pub struct NotificationConfig {
        /// Property `NotificationArn`.
        pub notification_arn: ::Value<String>,
        /// Property `NotificationEvents`.
        pub notification_events: Option<::ValueList<String>>,
        /// Property `NotificationType`.
        pub notification_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NotificationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationArn", &self.notification_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationEvents", &self.notification_events)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationType", &self.notification_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification_arn = None;
                    let mut notification_events = None;
                    let mut notification_type = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotificationArn" => {
                                notification_arn = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NotificationEvents" => {
                                notification_events = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "NotificationType" => {
                                notification_type = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfig {
                        notification_arn: notification_arn.ok_or(::serde::de::Error::missing_field("NotificationArn"))?,
                        notification_events: notification_events,
                        notification_type: notification_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.Target`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html) property type.
    #[derive(Debug)]
    pub struct Target {
        /// Property `Key`.
        pub key: ::Value<String>,
        /// Property `Values`.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for Target {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Target {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Target, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Target;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Target")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Values" => {
                                values = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Target {
                        key: key.ok_or(::serde::de::Error::missing_field("Key"))?,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html) property type.
    #[derive(Debug)]
    pub struct TaskInvocationParameters {
        /// Property `MaintenanceWindowAutomationParameters`.
        pub maintenance_window_automation_parameters: Option<::Value<MaintenanceWindowAutomationParameters>>,
        /// Property `MaintenanceWindowLambdaParameters`.
        pub maintenance_window_lambda_parameters: Option<::Value<MaintenanceWindowLambdaParameters>>,
        /// Property `MaintenanceWindowRunCommandParameters`.
        pub maintenance_window_run_command_parameters: Option<::Value<MaintenanceWindowRunCommandParameters>>,
        /// Property `MaintenanceWindowStepFunctionsParameters`.
        pub maintenance_window_step_functions_parameters: Option<::Value<MaintenanceWindowStepFunctionsParameters>>,
    }

    impl ::codec::SerializeValue for TaskInvocationParameters {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowAutomationParameters", &self.maintenance_window_automation_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowLambdaParameters", &self.maintenance_window_lambda_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowRunCommandParameters", &self.maintenance_window_run_command_parameters)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaintenanceWindowStepFunctionsParameters", &self.maintenance_window_step_functions_parameters)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TaskInvocationParameters {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TaskInvocationParameters, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TaskInvocationParameters;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TaskInvocationParameters")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maintenance_window_automation_parameters = None;
                    let mut maintenance_window_lambda_parameters = None;
                    let mut maintenance_window_run_command_parameters = None;
                    let mut maintenance_window_step_functions_parameters = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaintenanceWindowAutomationParameters" => {
                                maintenance_window_automation_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaintenanceWindowLambdaParameters" => {
                                maintenance_window_lambda_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaintenanceWindowRunCommandParameters" => {
                                maintenance_window_run_command_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "MaintenanceWindowStepFunctionsParameters" => {
                                maintenance_window_step_functions_parameters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(TaskInvocationParameters {
                        maintenance_window_automation_parameters: maintenance_window_automation_parameters,
                        maintenance_window_lambda_parameters: maintenance_window_lambda_parameters,
                        maintenance_window_run_command_parameters: maintenance_window_run_command_parameters,
                        maintenance_window_step_functions_parameters: maintenance_window_step_functions_parameters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod patch_baseline {
    //! Property types for the `PatchBaseline` resource.

    /// The [`AWS::SSM::PatchBaseline.PatchFilter`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html) property type.
    #[derive(Debug)]
    pub struct PatchFilter {
        /// Property `Key`.
        pub key: Option<::Value<String>>,
        /// Property `Values`.
        pub values: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PatchFilter {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Key", &self.key)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Values", &self.values)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchFilter {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchFilter, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchFilter;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchFilter")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut key = None;
                    let mut values = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Key" => {
                                key = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Values" => {
                                values = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchFilter {
                        key: key,
                        values: values,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.PatchFilterGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html) property type.
    #[derive(Debug)]
    pub struct PatchFilterGroup {
        /// Property `PatchFilters`.
        pub patch_filters: Option<::ValueList<PatchFilter>>,
    }

    impl ::codec::SerializeValue for PatchFilterGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchFilters", &self.patch_filters)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchFilterGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchFilterGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchFilterGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchFilterGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut patch_filters = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PatchFilters" => {
                                patch_filters = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchFilterGroup {
                        patch_filters: patch_filters,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.PatchSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html) property type.
    #[derive(Debug)]
    pub struct PatchSource {
        /// Property `Configuration`.
        pub configuration: Option<::Value<String>>,
        /// Property `Name`.
        pub name: Option<::Value<String>>,
        /// Property `Products`.
        pub products: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for PatchSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Configuration", &self.configuration)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Products", &self.products)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PatchSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PatchSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PatchSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PatchSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut configuration = None;
                    let mut name = None;
                    let mut products = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Configuration" => {
                                configuration = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Name" => {
                                name = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "Products" => {
                                products = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(PatchSource {
                        configuration: configuration,
                        name: name,
                        products: products,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.Rule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html) property type.
    #[derive(Debug)]
    pub struct Rule {
        /// Property `ApproveAfterDays`.
        pub approve_after_days: Option<::Value<u32>>,
        /// Property `ComplianceLevel`.
        pub compliance_level: Option<::Value<String>>,
        /// Property `EnableNonSecurity`.
        pub enable_non_security: Option<::Value<bool>>,
        /// Property `PatchFilterGroup`.
        pub patch_filter_group: Option<::Value<PatchFilterGroup>>,
    }

    impl ::codec::SerializeValue for Rule {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApproveAfterDays", &self.approve_after_days)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ComplianceLevel", &self.compliance_level)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNonSecurity", &self.enable_non_security)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchFilterGroup", &self.patch_filter_group)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Rule {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Rule, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Rule;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Rule")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut approve_after_days = None;
                    let mut compliance_level = None;
                    let mut enable_non_security = None;
                    let mut patch_filter_group = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ApproveAfterDays" => {
                                approve_after_days = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "ComplianceLevel" => {
                                compliance_level = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "EnableNonSecurity" => {
                                enable_non_security = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            "PatchFilterGroup" => {
                                patch_filter_group = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(Rule {
                        approve_after_days: approve_after_days,
                        compliance_level: compliance_level,
                        enable_non_security: enable_non_security,
                        patch_filter_group: patch_filter_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SSM::PatchBaseline.RuleGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html) property type.
    #[derive(Debug)]
    pub struct RuleGroup {
        /// Property `PatchRules`.
        pub patch_rules: Option<::ValueList<Rule>>,
    }

    impl ::codec::SerializeValue for RuleGroup {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PatchRules", &self.patch_rules)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RuleGroup {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RuleGroup, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RuleGroup;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RuleGroup")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut patch_rules = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PatchRules" => {
                                patch_rules = Some(::serde::de::MapAccess::next_value(&mut map)?);
                            }
                            _ => {}
                        }
                    }

                    Ok(RuleGroup {
                        patch_rules: patch_rules,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

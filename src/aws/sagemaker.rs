//! Types for the `SageMaker` service.

/// The [`AWS::SageMaker::App`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html) resource type.
#[derive(Debug, Default)]
pub struct App {
    properties: AppProperties
}

/// Properties for the `App` resource.
#[derive(Debug, Default)]
pub struct AppProperties {
    /// Property [`AppName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-appname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_name: ::Value<String>,
    /// Property [`AppType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-apptype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_type: ::Value<String>,
    /// Property [`DomainId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-domainid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_id: ::Value<String>,
    /// Property [`ResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-resourcespec).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub resource_spec: Option<::Value<self::app::ResourceSpec>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-app.html#cfn-sagemaker-app-userprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_profile_name: ::Value<String>,
}

impl ::serde::Serialize for AppProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppName", &self.app_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppType", &self.app_type)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainId", &self.domain_id)?;
        if let Some(ref resource_spec) = self.resource_spec {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ResourceSpec", resource_spec)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProfileName", &self.user_profile_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_name: Option<::Value<String>> = None;
                let mut app_type: Option<::Value<String>> = None;
                let mut domain_id: Option<::Value<String>> = None;
                let mut resource_spec: Option<::Value<self::app::ResourceSpec>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_profile_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppName" => {
                            app_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppType" => {
                            app_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainId" => {
                            domain_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ResourceSpec" => {
                            resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserProfileName" => {
                            user_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppProperties {
                    app_name: app_name.ok_or(::serde::de::Error::missing_field("AppName"))?,
                    app_type: app_type.ok_or(::serde::de::Error::missing_field("AppType"))?,
                    domain_id: domain_id.ok_or(::serde::de::Error::missing_field("DomainId"))?,
                    resource_spec: resource_spec,
                    tags: tags,
                    user_profile_name: user_profile_name.ok_or(::serde::de::Error::missing_field("UserProfileName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for App {
    type Properties = AppProperties;
    const TYPE: &'static str = "AWS::SageMaker::App";
    fn properties(&self) -> &AppProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for App {}

impl From<AppProperties> for App {
    fn from(properties: AppProperties) -> App {
        App { properties }
    }
}

/// The [`AWS::SageMaker::AppImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-appimageconfig.html) resource type.
#[derive(Debug, Default)]
pub struct AppImageConfig {
    properties: AppImageConfigProperties
}

/// Properties for the `AppImageConfig` resource.
#[derive(Debug, Default)]
pub struct AppImageConfigProperties {
    /// Property [`AppImageConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-appimageconfig.html#cfn-sagemaker-appimageconfig-appimageconfigname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_image_config_name: ::Value<String>,
    /// Property [`KernelGatewayImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-appimageconfig.html#cfn-sagemaker-appimageconfig-kernelgatewayimageconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub kernel_gateway_image_config: Option<::Value<self::app_image_config::KernelGatewayImageConfig>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-appimageconfig.html#cfn-sagemaker-appimageconfig-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for AppImageConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppImageConfigName", &self.app_image_config_name)?;
        if let Some(ref kernel_gateway_image_config) = self.kernel_gateway_image_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelGatewayImageConfig", kernel_gateway_image_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for AppImageConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<AppImageConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = AppImageConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type AppImageConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_image_config_name: Option<::Value<String>> = None;
                let mut kernel_gateway_image_config: Option<::Value<self::app_image_config::KernelGatewayImageConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppImageConfigName" => {
                            app_image_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KernelGatewayImageConfig" => {
                            kernel_gateway_image_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(AppImageConfigProperties {
                    app_image_config_name: app_image_config_name.ok_or(::serde::de::Error::missing_field("AppImageConfigName"))?,
                    kernel_gateway_image_config: kernel_gateway_image_config,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for AppImageConfig {
    type Properties = AppImageConfigProperties;
    const TYPE: &'static str = "AWS::SageMaker::AppImageConfig";
    fn properties(&self) -> &AppImageConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut AppImageConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for AppImageConfig {}

impl From<AppImageConfigProperties> for AppImageConfig {
    fn from(properties: AppImageConfigProperties) -> AppImageConfig {
        AppImageConfig { properties }
    }
}

/// The [`AWS::SageMaker::CodeRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-coderepository.html) resource type.
#[derive(Debug, Default)]
pub struct CodeRepository {
    properties: CodeRepositoryProperties
}

/// Properties for the `CodeRepository` resource.
#[derive(Debug, Default)]
pub struct CodeRepositoryProperties {
    /// Property [`CodeRepositoryName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-coderepository.html#cfn-sagemaker-coderepository-coderepositoryname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub code_repository_name: Option<::Value<String>>,
    /// Property [`GitConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-coderepository.html#cfn-sagemaker-coderepository-gitconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub git_config: ::Value<self::code_repository::GitConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-coderepository.html#cfn-sagemaker-coderepository-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CodeRepositoryProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref code_repository_name) = self.code_repository_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CodeRepositoryName", code_repository_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "GitConfig", &self.git_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CodeRepositoryProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CodeRepositoryProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CodeRepositoryProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CodeRepositoryProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut code_repository_name: Option<::Value<String>> = None;
                let mut git_config: Option<::Value<self::code_repository::GitConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "CodeRepositoryName" => {
                            code_repository_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "GitConfig" => {
                            git_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CodeRepositoryProperties {
                    code_repository_name: code_repository_name,
                    git_config: git_config.ok_or(::serde::de::Error::missing_field("GitConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for CodeRepository {
    type Properties = CodeRepositoryProperties;
    const TYPE: &'static str = "AWS::SageMaker::CodeRepository";
    fn properties(&self) -> &CodeRepositoryProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CodeRepositoryProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for CodeRepository {}

impl From<CodeRepositoryProperties> for CodeRepository {
    fn from(properties: CodeRepositoryProperties) -> CodeRepository {
        CodeRepository { properties }
    }
}

/// The [`AWS::SageMaker::DataQualityJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct DataQualityJobDefinition {
    properties: DataQualityJobDefinitionProperties
}

/// Properties for the `DataQualityJobDefinition` resource.
#[derive(Debug, Default)]
pub struct DataQualityJobDefinitionProperties {
    /// Property [`DataQualityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_quality_app_specification: ::Value<self::data_quality_job_definition::DataQualityAppSpecification>,
    /// Property [`DataQualityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_quality_baseline_config: Option<::Value<self::data_quality_job_definition::DataQualityBaselineConfig>>,
    /// Property [`DataQualityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityjobinput).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_quality_job_input: ::Value<self::data_quality_job_definition::DataQualityJobInput>,
    /// Property [`DataQualityJobOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityjoboutputconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_quality_job_output_config: ::Value<self::data_quality_job_definition::MonitoringOutputConfig>,
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-endpointname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`JobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-jobdefinitionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_definition_name: Option<::Value<String>>,
    /// Property [`JobResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-jobresources).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_resources: ::Value<self::data_quality_job_definition::MonitoringResources>,
    /// Property [`NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-networkconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_config: Option<::Value<self::data_quality_job_definition::NetworkConfig>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-stoppingcondition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stopping_condition: Option<::Value<self::data_quality_job_definition::StoppingCondition>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-dataqualityjobdefinition.html#cfn-sagemaker-dataqualityjobdefinition-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DataQualityJobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataQualityAppSpecification", &self.data_quality_app_specification)?;
        if let Some(ref data_quality_baseline_config) = self.data_quality_baseline_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataQualityBaselineConfig", data_quality_baseline_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataQualityJobInput", &self.data_quality_job_input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataQualityJobOutputConfig", &self.data_quality_job_output_config)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobResources", &self.job_resources)?;
        if let Some(ref network_config) = self.network_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfig", network_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref stopping_condition) = self.stopping_condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoppingCondition", stopping_condition)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DataQualityJobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityJobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DataQualityJobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DataQualityJobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut data_quality_app_specification: Option<::Value<self::data_quality_job_definition::DataQualityAppSpecification>> = None;
                let mut data_quality_baseline_config: Option<::Value<self::data_quality_job_definition::DataQualityBaselineConfig>> = None;
                let mut data_quality_job_input: Option<::Value<self::data_quality_job_definition::DataQualityJobInput>> = None;
                let mut data_quality_job_output_config: Option<::Value<self::data_quality_job_definition::MonitoringOutputConfig>> = None;
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut job_definition_name: Option<::Value<String>> = None;
                let mut job_resources: Option<::Value<self::data_quality_job_definition::MonitoringResources>> = None;
                let mut network_config: Option<::Value<self::data_quality_job_definition::NetworkConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stopping_condition: Option<::Value<self::data_quality_job_definition::StoppingCondition>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DataQualityAppSpecification" => {
                            data_quality_app_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataQualityBaselineConfig" => {
                            data_quality_baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataQualityJobInput" => {
                            data_quality_job_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataQualityJobOutputConfig" => {
                            data_quality_job_output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobResources" => {
                            job_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfig" => {
                            network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoppingCondition" => {
                            stopping_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DataQualityJobDefinitionProperties {
                    data_quality_app_specification: data_quality_app_specification.ok_or(::serde::de::Error::missing_field("DataQualityAppSpecification"))?,
                    data_quality_baseline_config: data_quality_baseline_config,
                    data_quality_job_input: data_quality_job_input.ok_or(::serde::de::Error::missing_field("DataQualityJobInput"))?,
                    data_quality_job_output_config: data_quality_job_output_config.ok_or(::serde::de::Error::missing_field("DataQualityJobOutputConfig"))?,
                    endpoint_name: endpoint_name,
                    job_definition_name: job_definition_name,
                    job_resources: job_resources.ok_or(::serde::de::Error::missing_field("JobResources"))?,
                    network_config: network_config,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stopping_condition: stopping_condition,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DataQualityJobDefinition {
    type Properties = DataQualityJobDefinitionProperties;
    const TYPE: &'static str = "AWS::SageMaker::DataQualityJobDefinition";
    fn properties(&self) -> &DataQualityJobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DataQualityJobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DataQualityJobDefinition {}

impl From<DataQualityJobDefinitionProperties> for DataQualityJobDefinition {
    fn from(properties: DataQualityJobDefinitionProperties) -> DataQualityJobDefinition {
        DataQualityJobDefinition { properties }
    }
}

/// The [`AWS::SageMaker::Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-device.html) resource type.
#[derive(Debug, Default)]
pub struct Device {
    properties: DeviceProperties
}

/// Properties for the `Device` resource.
#[derive(Debug, Default)]
pub struct DeviceProperties {
    /// Property [`Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-device.html#cfn-sagemaker-device-device).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub device: Option<::Value<self::device::Device>>,
    /// Property [`DeviceFleetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-device.html#cfn-sagemaker-device-devicefleetname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub device_fleet_name: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-device.html#cfn-sagemaker-device-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DeviceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref device) = self.device {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Device", device)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceFleetName", &self.device_fleet_name)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut device: Option<::Value<self::device::Device>> = None;
                let mut device_fleet_name: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Device" => {
                            device = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceFleetName" => {
                            device_fleet_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceProperties {
                    device: device,
                    device_fleet_name: device_fleet_name.ok_or(::serde::de::Error::missing_field("DeviceFleetName"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Device {
    type Properties = DeviceProperties;
    const TYPE: &'static str = "AWS::SageMaker::Device";
    fn properties(&self) -> &DeviceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Device {}

impl From<DeviceProperties> for Device {
    fn from(properties: DeviceProperties) -> Device {
        Device { properties }
    }
}

/// The [`AWS::SageMaker::DeviceFleet`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html) resource type.
#[derive(Debug, Default)]
pub struct DeviceFleet {
    properties: DeviceFleetProperties
}

/// Properties for the `DeviceFleet` resource.
#[derive(Debug, Default)]
pub struct DeviceFleetProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html#cfn-sagemaker-devicefleet-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`DeviceFleetName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html#cfn-sagemaker-devicefleet-devicefleetname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub device_fleet_name: ::Value<String>,
    /// Property [`OutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html#cfn-sagemaker-devicefleet-outputconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub output_config: ::Value<self::device_fleet::EdgeOutputConfig>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html#cfn-sagemaker-devicefleet-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-devicefleet.html#cfn-sagemaker-devicefleet-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for DeviceFleetProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceFleetName", &self.device_fleet_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputConfig", &self.output_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DeviceFleetProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DeviceFleetProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DeviceFleetProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DeviceFleetProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut device_fleet_name: Option<::Value<String>> = None;
                let mut output_config: Option<::Value<self::device_fleet::EdgeOutputConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DeviceFleetName" => {
                            device_fleet_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutputConfig" => {
                            output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DeviceFleetProperties {
                    description: description,
                    device_fleet_name: device_fleet_name.ok_or(::serde::de::Error::missing_field("DeviceFleetName"))?,
                    output_config: output_config.ok_or(::serde::de::Error::missing_field("OutputConfig"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for DeviceFleet {
    type Properties = DeviceFleetProperties;
    const TYPE: &'static str = "AWS::SageMaker::DeviceFleet";
    fn properties(&self) -> &DeviceFleetProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DeviceFleetProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for DeviceFleet {}

impl From<DeviceFleetProperties> for DeviceFleet {
    fn from(properties: DeviceFleetProperties) -> DeviceFleet {
        DeviceFleet { properties }
    }
}

/// The [`AWS::SageMaker::Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html) resource type.
#[derive(Debug, Default)]
pub struct Domain {
    properties: DomainProperties
}

/// Properties for the `Domain` resource.
#[derive(Debug, Default)]
pub struct DomainProperties {
    /// Property [`AppNetworkAccessType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-appnetworkaccesstype).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_network_access_type: Option<::Value<String>>,
    /// Property [`AppSecurityGroupManagement`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-appsecuritygroupmanagement).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub app_security_group_management: Option<::Value<String>>,
    /// Property [`AuthMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-authmode).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub auth_mode: ::Value<String>,
    /// Property [`DefaultUserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-defaultusersettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_user_settings: ::Value<self::domain::UserSettings>,
    /// Property [`DomainName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-domainname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_name: ::Value<String>,
    /// Property [`DomainSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-domainsettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub domain_settings: Option<::Value<self::domain::DomainSettings>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`SubnetIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-subnetids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_ids: ::ValueList<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-domain.html#cfn-sagemaker-domain-vpcid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_id: ::Value<String>,
}

impl ::serde::Serialize for DomainProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref app_network_access_type) = self.app_network_access_type {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppNetworkAccessType", app_network_access_type)?;
        }
        if let Some(ref app_security_group_management) = self.app_security_group_management {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppSecurityGroupManagement", app_security_group_management)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "AuthMode", &self.auth_mode)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultUserSettings", &self.default_user_settings)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainName", &self.domain_name)?;
        if let Some(ref domain_settings) = self.domain_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainSettings", domain_settings)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetIds", &self.subnet_ids)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcId", &self.vpc_id)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for DomainProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = DomainProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type DomainProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut app_network_access_type: Option<::Value<String>> = None;
                let mut app_security_group_management: Option<::Value<String>> = None;
                let mut auth_mode: Option<::Value<String>> = None;
                let mut default_user_settings: Option<::Value<self::domain::UserSettings>> = None;
                let mut domain_name: Option<::Value<String>> = None;
                let mut domain_settings: Option<::Value<self::domain::DomainSettings>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut subnet_ids: Option<::ValueList<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_id: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AppNetworkAccessType" => {
                            app_network_access_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AppSecurityGroupManagement" => {
                            app_security_group_management = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AuthMode" => {
                            auth_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultUserSettings" => {
                            default_user_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainName" => {
                            domain_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DomainSettings" => {
                            domain_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetIds" => {
                            subnet_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcId" => {
                            vpc_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(DomainProperties {
                    app_network_access_type: app_network_access_type,
                    app_security_group_management: app_security_group_management,
                    auth_mode: auth_mode.ok_or(::serde::de::Error::missing_field("AuthMode"))?,
                    default_user_settings: default_user_settings.ok_or(::serde::de::Error::missing_field("DefaultUserSettings"))?,
                    domain_name: domain_name.ok_or(::serde::de::Error::missing_field("DomainName"))?,
                    domain_settings: domain_settings,
                    kms_key_id: kms_key_id,
                    subnet_ids: subnet_ids.ok_or(::serde::de::Error::missing_field("SubnetIds"))?,
                    tags: tags,
                    vpc_id: vpc_id.ok_or(::serde::de::Error::missing_field("VpcId"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Domain {
    type Properties = DomainProperties;
    const TYPE: &'static str = "AWS::SageMaker::Domain";
    fn properties(&self) -> &DomainProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut DomainProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Domain {}

impl From<DomainProperties> for Domain {
    fn from(properties: DomainProperties) -> Domain {
        Domain { properties }
    }
}

/// The [`AWS::SageMaker::Endpoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html) resource type.
#[derive(Debug, Default)]
pub struct Endpoint {
    properties: EndpointProperties
}

/// Properties for the `Endpoint` resource.
#[derive(Debug, Default)]
pub struct EndpointProperties {
    /// Property [`DeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-deploymentconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub deployment_config: Option<::Value<self::endpoint::DeploymentConfig>>,
    /// Property [`EndpointConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-endpointconfigname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_config_name: ::Value<String>,
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-endpointname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`ExcludeRetainedVariantProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-excluderetainedvariantproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub exclude_retained_variant_properties: Option<::ValueList<self::endpoint::VariantProperty>>,
    /// Property [`RetainAllVariantProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-retainallvariantproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retain_all_variant_properties: Option<::Value<bool>>,
    /// Property [`RetainDeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-retaindeploymentconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub retain_deployment_config: Option<::Value<bool>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpoint.html#cfn-sagemaker-endpoint-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EndpointProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref deployment_config) = self.deployment_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeploymentConfig", deployment_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfigName", &self.endpoint_config_name)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref exclude_retained_variant_properties) = self.exclude_retained_variant_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExcludeRetainedVariantProperties", exclude_retained_variant_properties)?;
        }
        if let Some(ref retain_all_variant_properties) = self.retain_all_variant_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainAllVariantProperties", retain_all_variant_properties)?;
        }
        if let Some(ref retain_deployment_config) = self.retain_deployment_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RetainDeploymentConfig", retain_deployment_config)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut deployment_config: Option<::Value<self::endpoint::DeploymentConfig>> = None;
                let mut endpoint_config_name: Option<::Value<String>> = None;
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut exclude_retained_variant_properties: Option<::ValueList<self::endpoint::VariantProperty>> = None;
                let mut retain_all_variant_properties: Option<::Value<bool>> = None;
                let mut retain_deployment_config: Option<::Value<bool>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DeploymentConfig" => {
                            deployment_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointConfigName" => {
                            endpoint_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExcludeRetainedVariantProperties" => {
                            exclude_retained_variant_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetainAllVariantProperties" => {
                            retain_all_variant_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RetainDeploymentConfig" => {
                            retain_deployment_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointProperties {
                    deployment_config: deployment_config,
                    endpoint_config_name: endpoint_config_name.ok_or(::serde::de::Error::missing_field("EndpointConfigName"))?,
                    endpoint_name: endpoint_name,
                    exclude_retained_variant_properties: exclude_retained_variant_properties,
                    retain_all_variant_properties: retain_all_variant_properties,
                    retain_deployment_config: retain_deployment_config,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Endpoint {
    type Properties = EndpointProperties;
    const TYPE: &'static str = "AWS::SageMaker::Endpoint";
    fn properties(&self) -> &EndpointProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Endpoint {}

impl From<EndpointProperties> for Endpoint {
    fn from(properties: EndpointProperties) -> Endpoint {
        Endpoint { properties }
    }
}

/// The [`AWS::SageMaker::EndpointConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html) resource type.
#[derive(Debug, Default)]
pub struct EndpointConfig {
    properties: EndpointConfigProperties
}

/// Properties for the `EndpointConfig` resource.
#[derive(Debug, Default)]
pub struct EndpointConfigProperties {
    /// Property [`AsyncInferenceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub async_inference_config: Option<::Value<self::endpoint_config::AsyncInferenceConfig>>,
    /// Property [`DataCaptureConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub data_capture_config: Option<::Value<self::endpoint_config::DataCaptureConfig>>,
    /// Property [`EndpointConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-endpointconfigname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub endpoint_config_name: Option<::Value<String>>,
    /// Property [`ExplainerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-explainerconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub explainer_config: Option<::Value<self::endpoint_config::ExplainerConfig>>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`ProductionVariants`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-productionvariants).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub production_variants: ::ValueList<self::endpoint_config::ProductionVariant>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-endpointconfig.html#cfn-sagemaker-endpointconfig-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for EndpointConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref async_inference_config) = self.async_inference_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AsyncInferenceConfig", async_inference_config)?;
        }
        if let Some(ref data_capture_config) = self.data_capture_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataCaptureConfig", data_capture_config)?;
        }
        if let Some(ref endpoint_config_name) = self.endpoint_config_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointConfigName", endpoint_config_name)?;
        }
        if let Some(ref explainer_config) = self.explainer_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExplainerConfig", explainer_config)?;
        }
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductionVariants", &self.production_variants)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for EndpointConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = EndpointConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type EndpointConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut async_inference_config: Option<::Value<self::endpoint_config::AsyncInferenceConfig>> = None;
                let mut data_capture_config: Option<::Value<self::endpoint_config::DataCaptureConfig>> = None;
                let mut endpoint_config_name: Option<::Value<String>> = None;
                let mut explainer_config: Option<::Value<self::endpoint_config::ExplainerConfig>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut production_variants: Option<::ValueList<self::endpoint_config::ProductionVariant>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AsyncInferenceConfig" => {
                            async_inference_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DataCaptureConfig" => {
                            data_capture_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EndpointConfigName" => {
                            endpoint_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExplainerConfig" => {
                            explainer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProductionVariants" => {
                            production_variants = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(EndpointConfigProperties {
                    async_inference_config: async_inference_config,
                    data_capture_config: data_capture_config,
                    endpoint_config_name: endpoint_config_name,
                    explainer_config: explainer_config,
                    kms_key_id: kms_key_id,
                    production_variants: production_variants.ok_or(::serde::de::Error::missing_field("ProductionVariants"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for EndpointConfig {
    type Properties = EndpointConfigProperties;
    const TYPE: &'static str = "AWS::SageMaker::EndpointConfig";
    fn properties(&self) -> &EndpointConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut EndpointConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for EndpointConfig {}

impl From<EndpointConfigProperties> for EndpointConfig {
    fn from(properties: EndpointConfigProperties) -> EndpointConfig {
        EndpointConfig { properties }
    }
}

/// The [`AWS::SageMaker::FeatureGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html) resource type.
#[derive(Debug, Default)]
pub struct FeatureGroup {
    properties: FeatureGroupProperties
}

/// Properties for the `FeatureGroup` resource.
#[derive(Debug, Default)]
pub struct FeatureGroupProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-description).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`EventTimeFeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-eventtimefeaturename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub event_time_feature_name: ::Value<String>,
    /// Property [`FeatureDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-featuredefinitions).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub feature_definitions: ::ValueList<self::feature_group::FeatureDefinition>,
    /// Property [`FeatureGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-featuregroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub feature_group_name: ::Value<String>,
    /// Property [`OfflineStoreConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-offlinestoreconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub offline_store_config: Option<::Value<::json::Value>>,
    /// Property [`OnlineStoreConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-onlinestoreconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub online_store_config: Option<::Value<::json::Value>>,
    /// Property [`RecordIdentifierFeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-recordidentifierfeaturename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub record_identifier_feature_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-featuregroup.html#cfn-sagemaker-featuregroup-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for FeatureGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "EventTimeFeatureName", &self.event_time_feature_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureDefinitions", &self.feature_definitions)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureGroupName", &self.feature_group_name)?;
        if let Some(ref offline_store_config) = self.offline_store_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OfflineStoreConfig", offline_store_config)?;
        }
        if let Some(ref online_store_config) = self.online_store_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnlineStoreConfig", online_store_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordIdentifierFeatureName", &self.record_identifier_feature_name)?;
        if let Some(ref role_arn) = self.role_arn {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", role_arn)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for FeatureGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<FeatureGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = FeatureGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type FeatureGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut event_time_feature_name: Option<::Value<String>> = None;
                let mut feature_definitions: Option<::ValueList<self::feature_group::FeatureDefinition>> = None;
                let mut feature_group_name: Option<::Value<String>> = None;
                let mut offline_store_config: Option<::Value<::json::Value>> = None;
                let mut online_store_config: Option<::Value<::json::Value>> = None;
                let mut record_identifier_feature_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EventTimeFeatureName" => {
                            event_time_feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FeatureDefinitions" => {
                            feature_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FeatureGroupName" => {
                            feature_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OfflineStoreConfig" => {
                            offline_store_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnlineStoreConfig" => {
                            online_store_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RecordIdentifierFeatureName" => {
                            record_identifier_feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(FeatureGroupProperties {
                    description: description,
                    event_time_feature_name: event_time_feature_name.ok_or(::serde::de::Error::missing_field("EventTimeFeatureName"))?,
                    feature_definitions: feature_definitions.ok_or(::serde::de::Error::missing_field("FeatureDefinitions"))?,
                    feature_group_name: feature_group_name.ok_or(::serde::de::Error::missing_field("FeatureGroupName"))?,
                    offline_store_config: offline_store_config,
                    online_store_config: online_store_config,
                    record_identifier_feature_name: record_identifier_feature_name.ok_or(::serde::de::Error::missing_field("RecordIdentifierFeatureName"))?,
                    role_arn: role_arn,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for FeatureGroup {
    type Properties = FeatureGroupProperties;
    const TYPE: &'static str = "AWS::SageMaker::FeatureGroup";
    fn properties(&self) -> &FeatureGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut FeatureGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for FeatureGroup {}

impl From<FeatureGroupProperties> for FeatureGroup {
    fn from(properties: FeatureGroupProperties) -> FeatureGroup {
        FeatureGroup { properties }
    }
}

/// The [`AWS::SageMaker::Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html) resource type.
#[derive(Debug, Default)]
pub struct Image {
    properties: ImageProperties
}

/// Properties for the `Image` resource.
#[derive(Debug, Default)]
pub struct ImageProperties {
    /// Property [`ImageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html#cfn-sagemaker-image-imagedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_description: Option<::Value<String>>,
    /// Property [`ImageDisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html#cfn-sagemaker-image-imagedisplayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_display_name: Option<::Value<String>>,
    /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html#cfn-sagemaker-image-imagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_name: ::Value<String>,
    /// Property [`ImageRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html#cfn-sagemaker-image-imagerolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub image_role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-image.html#cfn-sagemaker-image-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ImageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref image_description) = self.image_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageDescription", image_description)?;
        }
        if let Some(ref image_display_name) = self.image_display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageDisplayName", image_display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", &self.image_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageRoleArn", &self.image_role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut image_description: Option<::Value<String>> = None;
                let mut image_display_name: Option<::Value<String>> = None;
                let mut image_name: Option<::Value<String>> = None;
                let mut image_role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ImageDescription" => {
                            image_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageDisplayName" => {
                            image_display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageName" => {
                            image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageRoleArn" => {
                            image_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImageProperties {
                    image_description: image_description,
                    image_display_name: image_display_name,
                    image_name: image_name.ok_or(::serde::de::Error::missing_field("ImageName"))?,
                    image_role_arn: image_role_arn.ok_or(::serde::de::Error::missing_field("ImageRoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Image {
    type Properties = ImageProperties;
    const TYPE: &'static str = "AWS::SageMaker::Image";
    fn properties(&self) -> &ImageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Image {}

impl From<ImageProperties> for Image {
    fn from(properties: ImageProperties) -> Image {
        Image { properties }
    }
}

/// The [`AWS::SageMaker::ImageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-imageversion.html) resource type.
#[derive(Debug, Default)]
pub struct ImageVersion {
    properties: ImageVersionProperties
}

/// Properties for the `ImageVersion` resource.
#[derive(Debug, Default)]
pub struct ImageVersionProperties {
    /// Property [`BaseImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-imageversion.html#cfn-sagemaker-imageversion-baseimage).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub base_image: ::Value<String>,
    /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-imageversion.html#cfn-sagemaker-imageversion-imagename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub image_name: ::Value<String>,
}

impl ::serde::Serialize for ImageVersionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseImage", &self.base_image)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", &self.image_name)?;
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ImageVersionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageVersionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ImageVersionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ImageVersionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut base_image: Option<::Value<String>> = None;
                let mut image_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "BaseImage" => {
                            base_image = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ImageName" => {
                            image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ImageVersionProperties {
                    base_image: base_image.ok_or(::serde::de::Error::missing_field("BaseImage"))?,
                    image_name: image_name.ok_or(::serde::de::Error::missing_field("ImageName"))?,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ImageVersion {
    type Properties = ImageVersionProperties;
    const TYPE: &'static str = "AWS::SageMaker::ImageVersion";
    fn properties(&self) -> &ImageVersionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ImageVersionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ImageVersion {}

impl From<ImageVersionProperties> for ImageVersion {
    fn from(properties: ImageVersionProperties) -> ImageVersion {
        ImageVersion { properties }
    }
}

/// The [`AWS::SageMaker::Model`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html) resource type.
#[derive(Debug, Default)]
pub struct Model {
    properties: ModelProperties
}

/// Properties for the `Model` resource.
#[derive(Debug, Default)]
pub struct ModelProperties {
    /// Property [`Containers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-containers).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub containers: Option<::ValueList<self::model::ContainerDefinition>>,
    /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-enablenetworkisolation).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub enable_network_isolation: Option<::Value<bool>>,
    /// Property [`ExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-executionrolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub execution_role_arn: ::Value<String>,
    /// Property [`InferenceExecutionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-inferenceexecutionconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inference_execution_config: Option<::Value<self::model::InferenceExecutionConfig>>,
    /// Property [`ModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-modelname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_name: Option<::Value<String>>,
    /// Property [`PrimaryContainer`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-primarycontainer).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub primary_container: Option<::Value<self::model::ContainerDefinition>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-model.html#cfn-sagemaker-model-vpcconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub vpc_config: Option<::Value<self::model::VpcConfig>>,
}

impl ::serde::Serialize for ModelProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref containers) = self.containers {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Containers", containers)?;
        }
        if let Some(ref enable_network_isolation) = self.enable_network_isolation {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRoleArn", &self.execution_role_arn)?;
        if let Some(ref inference_execution_config) = self.inference_execution_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceExecutionConfig", inference_execution_config)?;
        }
        if let Some(ref model_name) = self.model_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelName", model_name)?;
        }
        if let Some(ref primary_container) = self.primary_container {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PrimaryContainer", primary_container)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref vpc_config) = self.vpc_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut containers: Option<::ValueList<self::model::ContainerDefinition>> = None;
                let mut enable_network_isolation: Option<::Value<bool>> = None;
                let mut execution_role_arn: Option<::Value<String>> = None;
                let mut inference_execution_config: Option<::Value<self::model::InferenceExecutionConfig>> = None;
                let mut model_name: Option<::Value<String>> = None;
                let mut primary_container: Option<::Value<self::model::ContainerDefinition>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut vpc_config: Option<::Value<self::model::VpcConfig>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Containers" => {
                            containers = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "EnableNetworkIsolation" => {
                            enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ExecutionRoleArn" => {
                            execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InferenceExecutionConfig" => {
                            inference_execution_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelName" => {
                            model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PrimaryContainer" => {
                            primary_container = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VpcConfig" => {
                            vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelProperties {
                    containers: containers,
                    enable_network_isolation: enable_network_isolation,
                    execution_role_arn: execution_role_arn.ok_or(::serde::de::Error::missing_field("ExecutionRoleArn"))?,
                    inference_execution_config: inference_execution_config,
                    model_name: model_name,
                    primary_container: primary_container,
                    tags: tags,
                    vpc_config: vpc_config,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Model {
    type Properties = ModelProperties;
    const TYPE: &'static str = "AWS::SageMaker::Model";
    fn properties(&self) -> &ModelProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Model {}

impl From<ModelProperties> for Model {
    fn from(properties: ModelProperties) -> Model {
        Model { properties }
    }
}

/// The [`AWS::SageMaker::ModelBiasJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ModelBiasJobDefinition {
    properties: ModelBiasJobDefinitionProperties
}

/// Properties for the `ModelBiasJobDefinition` resource.
#[derive(Debug, Default)]
pub struct ModelBiasJobDefinitionProperties {
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-endpointname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`JobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-jobdefinitionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_definition_name: Option<::Value<String>>,
    /// Property [`JobResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-jobresources).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_resources: ::Value<self::model_bias_job_definition::MonitoringResources>,
    /// Property [`ModelBiasAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasappspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_bias_app_specification: ::Value<self::model_bias_job_definition::ModelBiasAppSpecification>,
    /// Property [`ModelBiasBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_bias_baseline_config: Option<::Value<self::model_bias_job_definition::ModelBiasBaselineConfig>>,
    /// Property [`ModelBiasJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasjobinput).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_bias_job_input: ::Value<self::model_bias_job_definition::ModelBiasJobInput>,
    /// Property [`ModelBiasJobOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasjoboutputconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_bias_job_output_config: ::Value<self::model_bias_job_definition::MonitoringOutputConfig>,
    /// Property [`NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-networkconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_config: Option<::Value<self::model_bias_job_definition::NetworkConfig>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-stoppingcondition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stopping_condition: Option<::Value<self::model_bias_job_definition::StoppingCondition>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelbiasjobdefinition.html#cfn-sagemaker-modelbiasjobdefinition-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ModelBiasJobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobResources", &self.job_resources)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelBiasAppSpecification", &self.model_bias_app_specification)?;
        if let Some(ref model_bias_baseline_config) = self.model_bias_baseline_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelBiasBaselineConfig", model_bias_baseline_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelBiasJobInput", &self.model_bias_job_input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelBiasJobOutputConfig", &self.model_bias_job_output_config)?;
        if let Some(ref network_config) = self.network_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfig", network_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref stopping_condition) = self.stopping_condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoppingCondition", stopping_condition)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelBiasJobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelBiasJobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelBiasJobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelBiasJobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut job_definition_name: Option<::Value<String>> = None;
                let mut job_resources: Option<::Value<self::model_bias_job_definition::MonitoringResources>> = None;
                let mut model_bias_app_specification: Option<::Value<self::model_bias_job_definition::ModelBiasAppSpecification>> = None;
                let mut model_bias_baseline_config: Option<::Value<self::model_bias_job_definition::ModelBiasBaselineConfig>> = None;
                let mut model_bias_job_input: Option<::Value<self::model_bias_job_definition::ModelBiasJobInput>> = None;
                let mut model_bias_job_output_config: Option<::Value<self::model_bias_job_definition::MonitoringOutputConfig>> = None;
                let mut network_config: Option<::Value<self::model_bias_job_definition::NetworkConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stopping_condition: Option<::Value<self::model_bias_job_definition::StoppingCondition>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobResources" => {
                            job_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelBiasAppSpecification" => {
                            model_bias_app_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelBiasBaselineConfig" => {
                            model_bias_baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelBiasJobInput" => {
                            model_bias_job_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelBiasJobOutputConfig" => {
                            model_bias_job_output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfig" => {
                            network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoppingCondition" => {
                            stopping_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelBiasJobDefinitionProperties {
                    endpoint_name: endpoint_name,
                    job_definition_name: job_definition_name,
                    job_resources: job_resources.ok_or(::serde::de::Error::missing_field("JobResources"))?,
                    model_bias_app_specification: model_bias_app_specification.ok_or(::serde::de::Error::missing_field("ModelBiasAppSpecification"))?,
                    model_bias_baseline_config: model_bias_baseline_config,
                    model_bias_job_input: model_bias_job_input.ok_or(::serde::de::Error::missing_field("ModelBiasJobInput"))?,
                    model_bias_job_output_config: model_bias_job_output_config.ok_or(::serde::de::Error::missing_field("ModelBiasJobOutputConfig"))?,
                    network_config: network_config,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stopping_condition: stopping_condition,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelBiasJobDefinition {
    type Properties = ModelBiasJobDefinitionProperties;
    const TYPE: &'static str = "AWS::SageMaker::ModelBiasJobDefinition";
    fn properties(&self) -> &ModelBiasJobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelBiasJobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelBiasJobDefinition {}

impl From<ModelBiasJobDefinitionProperties> for ModelBiasJobDefinition {
    fn from(properties: ModelBiasJobDefinitionProperties) -> ModelBiasJobDefinition {
        ModelBiasJobDefinition { properties }
    }
}

/// The [`AWS::SageMaker::ModelExplainabilityJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ModelExplainabilityJobDefinition {
    properties: ModelExplainabilityJobDefinitionProperties
}

/// Properties for the `ModelExplainabilityJobDefinition` resource.
#[derive(Debug, Default)]
pub struct ModelExplainabilityJobDefinitionProperties {
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`JobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-jobdefinitionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_definition_name: Option<::Value<String>>,
    /// Property [`JobResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-jobresources).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_resources: ::Value<self::model_explainability_job_definition::MonitoringResources>,
    /// Property [`ModelExplainabilityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_explainability_app_specification: ::Value<self::model_explainability_job_definition::ModelExplainabilityAppSpecification>,
    /// Property [`ModelExplainabilityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_explainability_baseline_config: Option<::Value<self::model_explainability_job_definition::ModelExplainabilityBaselineConfig>>,
    /// Property [`ModelExplainabilityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjobinput).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_explainability_job_input: ::Value<self::model_explainability_job_definition::ModelExplainabilityJobInput>,
    /// Property [`ModelExplainabilityJobOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjoboutputconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_explainability_job_output_config: ::Value<self::model_explainability_job_definition::MonitoringOutputConfig>,
    /// Property [`NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-networkconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_config: Option<::Value<self::model_explainability_job_definition::NetworkConfig>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-stoppingcondition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stopping_condition: Option<::Value<self::model_explainability_job_definition::StoppingCondition>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelexplainabilityjobdefinition.html#cfn-sagemaker-modelexplainabilityjobdefinition-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ModelExplainabilityJobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobResources", &self.job_resources)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelExplainabilityAppSpecification", &self.model_explainability_app_specification)?;
        if let Some(ref model_explainability_baseline_config) = self.model_explainability_baseline_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelExplainabilityBaselineConfig", model_explainability_baseline_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelExplainabilityJobInput", &self.model_explainability_job_input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelExplainabilityJobOutputConfig", &self.model_explainability_job_output_config)?;
        if let Some(ref network_config) = self.network_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfig", network_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref stopping_condition) = self.stopping_condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoppingCondition", stopping_condition)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelExplainabilityJobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelExplainabilityJobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelExplainabilityJobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelExplainabilityJobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut job_definition_name: Option<::Value<String>> = None;
                let mut job_resources: Option<::Value<self::model_explainability_job_definition::MonitoringResources>> = None;
                let mut model_explainability_app_specification: Option<::Value<self::model_explainability_job_definition::ModelExplainabilityAppSpecification>> = None;
                let mut model_explainability_baseline_config: Option<::Value<self::model_explainability_job_definition::ModelExplainabilityBaselineConfig>> = None;
                let mut model_explainability_job_input: Option<::Value<self::model_explainability_job_definition::ModelExplainabilityJobInput>> = None;
                let mut model_explainability_job_output_config: Option<::Value<self::model_explainability_job_definition::MonitoringOutputConfig>> = None;
                let mut network_config: Option<::Value<self::model_explainability_job_definition::NetworkConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stopping_condition: Option<::Value<self::model_explainability_job_definition::StoppingCondition>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobResources" => {
                            job_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelExplainabilityAppSpecification" => {
                            model_explainability_app_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelExplainabilityBaselineConfig" => {
                            model_explainability_baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelExplainabilityJobInput" => {
                            model_explainability_job_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelExplainabilityJobOutputConfig" => {
                            model_explainability_job_output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfig" => {
                            network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoppingCondition" => {
                            stopping_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelExplainabilityJobDefinitionProperties {
                    endpoint_name: endpoint_name,
                    job_definition_name: job_definition_name,
                    job_resources: job_resources.ok_or(::serde::de::Error::missing_field("JobResources"))?,
                    model_explainability_app_specification: model_explainability_app_specification.ok_or(::serde::de::Error::missing_field("ModelExplainabilityAppSpecification"))?,
                    model_explainability_baseline_config: model_explainability_baseline_config,
                    model_explainability_job_input: model_explainability_job_input.ok_or(::serde::de::Error::missing_field("ModelExplainabilityJobInput"))?,
                    model_explainability_job_output_config: model_explainability_job_output_config.ok_or(::serde::de::Error::missing_field("ModelExplainabilityJobOutputConfig"))?,
                    network_config: network_config,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stopping_condition: stopping_condition,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelExplainabilityJobDefinition {
    type Properties = ModelExplainabilityJobDefinitionProperties;
    const TYPE: &'static str = "AWS::SageMaker::ModelExplainabilityJobDefinition";
    fn properties(&self) -> &ModelExplainabilityJobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelExplainabilityJobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelExplainabilityJobDefinition {}

impl From<ModelExplainabilityJobDefinitionProperties> for ModelExplainabilityJobDefinition {
    fn from(properties: ModelExplainabilityJobDefinitionProperties) -> ModelExplainabilityJobDefinition {
        ModelExplainabilityJobDefinition { properties }
    }
}

/// The [`AWS::SageMaker::ModelPackage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html) resource type.
#[derive(Debug, Default)]
pub struct ModelPackage {
    properties: ModelPackageProperties
}

/// Properties for the `ModelPackage` resource.
#[derive(Debug, Default)]
pub struct ModelPackageProperties {
    /// Property [`AdditionalInferenceSpecificationDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_inference_specification_definition: Option<::Value<self::model_package::AdditionalInferenceSpecificationDefinition>>,
    /// Property [`AdditionalInferenceSpecifications`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-additionalinferencespecifications).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_inference_specifications: Option<::ValueList<self::model_package::AdditionalInferenceSpecificationDefinition>>,
    /// Property [`AdditionalInferenceSpecificationsToAdd`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-additionalinferencespecificationstoadd).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_inference_specifications_to_add: Option<::ValueList<self::model_package::AdditionalInferenceSpecificationDefinition>>,
    /// Property [`ApprovalDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-approvaldescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub approval_description: Option<::Value<String>>,
    /// Property [`CertifyForMarketplace`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-certifyformarketplace).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub certify_for_marketplace: Option<::Value<bool>>,
    /// Property [`ClientToken`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-clienttoken).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub client_token: Option<::Value<String>>,
    /// Property [`CreatedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-createdby).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub created_by: Option<::Value<self::model_package::UserContext>>,
    /// Property [`CustomerMetadataProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-customermetadataproperties).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub customer_metadata_properties: Option<::ValueMap<String>>,
    /// Property [`Domain`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-domain).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain: Option<::Value<String>>,
    /// Property [`DriftCheckBaselines`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-driftcheckbaselines).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub drift_check_baselines: Option<::Value<self::model_package::DriftCheckBaselines>>,
    /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-environment).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub environment: Option<::ValueMap<String>>,
    /// Property [`InferenceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-inferencespecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub inference_specification: Option<::Value<self::model_package::InferenceSpecification>>,
    /// Property [`LastModifiedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-lastmodifiedby).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub last_modified_by: Option<::Value<self::model_package::UserContext>>,
    /// Property [`LastModifiedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-lastmodifiedtime).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub last_modified_time: Option<::Value<String>>,
    /// Property [`MetadataProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-metadataproperties).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub metadata_properties: Option<::Value<self::model_package::MetadataProperties>>,
    /// Property [`ModelApprovalStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelapprovalstatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_approval_status: Option<::Value<String>>,
    /// Property [`ModelMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelmetrics).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_metrics: Option<::Value<self::model_package::ModelMetrics>>,
    /// Property [`ModelPackageDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackagedescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_package_description: Option<::Value<String>>,
    /// Property [`ModelPackageGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackagegroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_package_group_name: Option<::Value<String>>,
    /// Property [`ModelPackageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackagename).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_package_name: Option<::Value<String>>,
    /// Property [`ModelPackageStatusDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackagestatusdetails).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_package_status_details: Option<::Value<self::model_package::ModelPackageStatusDetails>>,
    /// Property [`ModelPackageStatusItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackagestatusitem).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_package_status_item: Option<::Value<self::model_package::ModelPackageStatusItem>>,
    /// Property [`ModelPackageVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-modelpackageversion).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_package_version: Option<::Value<u32>>,
    /// Property [`SamplePayloadUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-samplepayloadurl).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub sample_payload_url: Option<::Value<String>>,
    /// Property [`SourceAlgorithmSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-sourcealgorithmspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub source_algorithm_specification: Option<::Value<self::model_package::SourceAlgorithmSpecification>>,
    /// Property [`Tag`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-tag).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tag: Option<::Value<::Tag>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`Task`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-task).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub task: Option<::Value<String>>,
    /// Property [`ValidationSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackage.html#cfn-sagemaker-modelpackage-validationspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub validation_specification: Option<::Value<self::model_package::ValidationSpecification>>,
}

impl ::serde::Serialize for ModelPackageProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref additional_inference_specification_definition) = self.additional_inference_specification_definition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInferenceSpecificationDefinition", additional_inference_specification_definition)?;
        }
        if let Some(ref additional_inference_specifications) = self.additional_inference_specifications {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInferenceSpecifications", additional_inference_specifications)?;
        }
        if let Some(ref additional_inference_specifications_to_add) = self.additional_inference_specifications_to_add {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalInferenceSpecificationsToAdd", additional_inference_specifications_to_add)?;
        }
        if let Some(ref approval_description) = self.approval_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ApprovalDescription", approval_description)?;
        }
        if let Some(ref certify_for_marketplace) = self.certify_for_marketplace {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CertifyForMarketplace", certify_for_marketplace)?;
        }
        if let Some(ref client_token) = self.client_token {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientToken", client_token)?;
        }
        if let Some(ref created_by) = self.created_by {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreatedBy", created_by)?;
        }
        if let Some(ref customer_metadata_properties) = self.customer_metadata_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomerMetadataProperties", customer_metadata_properties)?;
        }
        if let Some(ref domain) = self.domain {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Domain", domain)?;
        }
        if let Some(ref drift_check_baselines) = self.drift_check_baselines {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DriftCheckBaselines", drift_check_baselines)?;
        }
        if let Some(ref environment) = self.environment {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
        }
        if let Some(ref inference_specification) = self.inference_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceSpecification", inference_specification)?;
        }
        if let Some(ref last_modified_by) = self.last_modified_by {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastModifiedBy", last_modified_by)?;
        }
        if let Some(ref last_modified_time) = self.last_modified_time {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastModifiedTime", last_modified_time)?;
        }
        if let Some(ref metadata_properties) = self.metadata_properties {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MetadataProperties", metadata_properties)?;
        }
        if let Some(ref model_approval_status) = self.model_approval_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelApprovalStatus", model_approval_status)?;
        }
        if let Some(ref model_metrics) = self.model_metrics {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelMetrics", model_metrics)?;
        }
        if let Some(ref model_package_description) = self.model_package_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageDescription", model_package_description)?;
        }
        if let Some(ref model_package_group_name) = self.model_package_group_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageGroupName", model_package_group_name)?;
        }
        if let Some(ref model_package_name) = self.model_package_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageName", model_package_name)?;
        }
        if let Some(ref model_package_status_details) = self.model_package_status_details {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageStatusDetails", model_package_status_details)?;
        }
        if let Some(ref model_package_status_item) = self.model_package_status_item {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageStatusItem", model_package_status_item)?;
        }
        if let Some(ref model_package_version) = self.model_package_version {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageVersion", model_package_version)?;
        }
        if let Some(ref sample_payload_url) = self.sample_payload_url {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SamplePayloadUrl", sample_payload_url)?;
        }
        if let Some(ref source_algorithm_specification) = self.source_algorithm_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAlgorithmSpecification", source_algorithm_specification)?;
        }
        if let Some(ref tag) = self.tag {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tag", tag)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref task) = self.task {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Task", task)?;
        }
        if let Some(ref validation_specification) = self.validation_specification {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationSpecification", validation_specification)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelPackageProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelPackageProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelPackageProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelPackageProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut additional_inference_specification_definition: Option<::Value<self::model_package::AdditionalInferenceSpecificationDefinition>> = None;
                let mut additional_inference_specifications: Option<::ValueList<self::model_package::AdditionalInferenceSpecificationDefinition>> = None;
                let mut additional_inference_specifications_to_add: Option<::ValueList<self::model_package::AdditionalInferenceSpecificationDefinition>> = None;
                let mut approval_description: Option<::Value<String>> = None;
                let mut certify_for_marketplace: Option<::Value<bool>> = None;
                let mut client_token: Option<::Value<String>> = None;
                let mut created_by: Option<::Value<self::model_package::UserContext>> = None;
                let mut customer_metadata_properties: Option<::ValueMap<String>> = None;
                let mut domain: Option<::Value<String>> = None;
                let mut drift_check_baselines: Option<::Value<self::model_package::DriftCheckBaselines>> = None;
                let mut environment: Option<::ValueMap<String>> = None;
                let mut inference_specification: Option<::Value<self::model_package::InferenceSpecification>> = None;
                let mut last_modified_by: Option<::Value<self::model_package::UserContext>> = None;
                let mut last_modified_time: Option<::Value<String>> = None;
                let mut metadata_properties: Option<::Value<self::model_package::MetadataProperties>> = None;
                let mut model_approval_status: Option<::Value<String>> = None;
                let mut model_metrics: Option<::Value<self::model_package::ModelMetrics>> = None;
                let mut model_package_description: Option<::Value<String>> = None;
                let mut model_package_group_name: Option<::Value<String>> = None;
                let mut model_package_name: Option<::Value<String>> = None;
                let mut model_package_status_details: Option<::Value<self::model_package::ModelPackageStatusDetails>> = None;
                let mut model_package_status_item: Option<::Value<self::model_package::ModelPackageStatusItem>> = None;
                let mut model_package_version: Option<::Value<u32>> = None;
                let mut sample_payload_url: Option<::Value<String>> = None;
                let mut source_algorithm_specification: Option<::Value<self::model_package::SourceAlgorithmSpecification>> = None;
                let mut tag: Option<::Value<::Tag>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut task: Option<::Value<String>> = None;
                let mut validation_specification: Option<::Value<self::model_package::ValidationSpecification>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AdditionalInferenceSpecificationDefinition" => {
                            additional_inference_specification_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdditionalInferenceSpecifications" => {
                            additional_inference_specifications = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdditionalInferenceSpecificationsToAdd" => {
                            additional_inference_specifications_to_add = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ApprovalDescription" => {
                            approval_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CertifyForMarketplace" => {
                            certify_for_marketplace = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ClientToken" => {
                            client_token = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CreatedBy" => {
                            created_by = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "CustomerMetadataProperties" => {
                            customer_metadata_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Domain" => {
                            domain = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DriftCheckBaselines" => {
                            drift_check_baselines = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Environment" => {
                            environment = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InferenceSpecification" => {
                            inference_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastModifiedBy" => {
                            last_modified_by = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastModifiedTime" => {
                            last_modified_time = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MetadataProperties" => {
                            metadata_properties = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelApprovalStatus" => {
                            model_approval_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelMetrics" => {
                            model_metrics = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageDescription" => {
                            model_package_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageGroupName" => {
                            model_package_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageName" => {
                            model_package_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageStatusDetails" => {
                            model_package_status_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageStatusItem" => {
                            model_package_status_item = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageVersion" => {
                            model_package_version = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SamplePayloadUrl" => {
                            sample_payload_url = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SourceAlgorithmSpecification" => {
                            source_algorithm_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tag" => {
                            tag = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Task" => {
                            task = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ValidationSpecification" => {
                            validation_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelPackageProperties {
                    additional_inference_specification_definition: additional_inference_specification_definition,
                    additional_inference_specifications: additional_inference_specifications,
                    additional_inference_specifications_to_add: additional_inference_specifications_to_add,
                    approval_description: approval_description,
                    certify_for_marketplace: certify_for_marketplace,
                    client_token: client_token,
                    created_by: created_by,
                    customer_metadata_properties: customer_metadata_properties,
                    domain: domain,
                    drift_check_baselines: drift_check_baselines,
                    environment: environment,
                    inference_specification: inference_specification,
                    last_modified_by: last_modified_by,
                    last_modified_time: last_modified_time,
                    metadata_properties: metadata_properties,
                    model_approval_status: model_approval_status,
                    model_metrics: model_metrics,
                    model_package_description: model_package_description,
                    model_package_group_name: model_package_group_name,
                    model_package_name: model_package_name,
                    model_package_status_details: model_package_status_details,
                    model_package_status_item: model_package_status_item,
                    model_package_version: model_package_version,
                    sample_payload_url: sample_payload_url,
                    source_algorithm_specification: source_algorithm_specification,
                    tag: tag,
                    tags: tags,
                    task: task,
                    validation_specification: validation_specification,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelPackage {
    type Properties = ModelPackageProperties;
    const TYPE: &'static str = "AWS::SageMaker::ModelPackage";
    fn properties(&self) -> &ModelPackageProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelPackageProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelPackage {}

impl From<ModelPackageProperties> for ModelPackage {
    fn from(properties: ModelPackageProperties) -> ModelPackage {
        ModelPackage { properties }
    }
}

/// The [`AWS::SageMaker::ModelPackageGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html) resource type.
#[derive(Debug, Default)]
pub struct ModelPackageGroup {
    properties: ModelPackageGroupProperties
}

/// Properties for the `ModelPackageGroup` resource.
#[derive(Debug, Default)]
pub struct ModelPackageGroupProperties {
    /// Property [`ModelPackageGroupDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html#cfn-sagemaker-modelpackagegroup-modelpackagegroupdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_package_group_description: Option<::Value<String>>,
    /// Property [`ModelPackageGroupName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html#cfn-sagemaker-modelpackagegroup-modelpackagegroupname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_package_group_name: ::Value<String>,
    /// Property [`ModelPackageGroupPolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html#cfn-sagemaker-modelpackagegroup-modelpackagegrouppolicy).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub model_package_group_policy: Option<::Value<::json::Value>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelpackagegroup.html#cfn-sagemaker-modelpackagegroup-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ModelPackageGroupProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref model_package_group_description) = self.model_package_group_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageGroupDescription", model_package_group_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageGroupName", &self.model_package_group_name)?;
        if let Some(ref model_package_group_policy) = self.model_package_group_policy {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageGroupPolicy", model_package_group_policy)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelPackageGroupProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelPackageGroupProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelPackageGroupProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelPackageGroupProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut model_package_group_description: Option<::Value<String>> = None;
                let mut model_package_group_name: Option<::Value<String>> = None;
                let mut model_package_group_policy: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ModelPackageGroupDescription" => {
                            model_package_group_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageGroupName" => {
                            model_package_group_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelPackageGroupPolicy" => {
                            model_package_group_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelPackageGroupProperties {
                    model_package_group_description: model_package_group_description,
                    model_package_group_name: model_package_group_name.ok_or(::serde::de::Error::missing_field("ModelPackageGroupName"))?,
                    model_package_group_policy: model_package_group_policy,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelPackageGroup {
    type Properties = ModelPackageGroupProperties;
    const TYPE: &'static str = "AWS::SageMaker::ModelPackageGroup";
    fn properties(&self) -> &ModelPackageGroupProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelPackageGroupProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelPackageGroup {}

impl From<ModelPackageGroupProperties> for ModelPackageGroup {
    fn from(properties: ModelPackageGroupProperties) -> ModelPackageGroup {
        ModelPackageGroup { properties }
    }
}

/// The [`AWS::SageMaker::ModelQualityJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html) resource type.
#[derive(Debug, Default)]
pub struct ModelQualityJobDefinition {
    properties: ModelQualityJobDefinitionProperties
}

/// Properties for the `ModelQualityJobDefinition` resource.
#[derive(Debug, Default)]
pub struct ModelQualityJobDefinitionProperties {
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-endpointname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`JobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-jobdefinitionname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_definition_name: Option<::Value<String>>,
    /// Property [`JobResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-jobresources).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub job_resources: ::Value<self::model_quality_job_definition::MonitoringResources>,
    /// Property [`ModelQualityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_quality_app_specification: ::Value<self::model_quality_job_definition::ModelQualityAppSpecification>,
    /// Property [`ModelQualityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_quality_baseline_config: Option<::Value<self::model_quality_job_definition::ModelQualityBaselineConfig>>,
    /// Property [`ModelQualityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityjobinput).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_quality_job_input: ::Value<self::model_quality_job_definition::ModelQualityJobInput>,
    /// Property [`ModelQualityJobOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityjoboutputconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub model_quality_job_output_config: ::Value<self::model_quality_job_definition::MonitoringOutputConfig>,
    /// Property [`NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-networkconfig).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub network_config: Option<::Value<self::model_quality_job_definition::NetworkConfig>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-rolearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-stoppingcondition).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub stopping_condition: Option<::Value<self::model_quality_job_definition::StoppingCondition>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-modelqualityjobdefinition.html#cfn-sagemaker-modelqualityjobdefinition-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ModelQualityJobDefinitionProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref job_definition_name) = self.job_definition_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobDefinitionName", job_definition_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "JobResources", &self.job_resources)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQualityAppSpecification", &self.model_quality_app_specification)?;
        if let Some(ref model_quality_baseline_config) = self.model_quality_baseline_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQualityBaselineConfig", model_quality_baseline_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQualityJobInput", &self.model_quality_job_input)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQualityJobOutputConfig", &self.model_quality_job_output_config)?;
        if let Some(ref network_config) = self.network_config {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfig", network_config)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref stopping_condition) = self.stopping_condition {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoppingCondition", stopping_condition)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ModelQualityJobDefinitionProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelQualityJobDefinitionProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ModelQualityJobDefinitionProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ModelQualityJobDefinitionProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut job_definition_name: Option<::Value<String>> = None;
                let mut job_resources: Option<::Value<self::model_quality_job_definition::MonitoringResources>> = None;
                let mut model_quality_app_specification: Option<::Value<self::model_quality_job_definition::ModelQualityAppSpecification>> = None;
                let mut model_quality_baseline_config: Option<::Value<self::model_quality_job_definition::ModelQualityBaselineConfig>> = None;
                let mut model_quality_job_input: Option<::Value<self::model_quality_job_definition::ModelQualityJobInput>> = None;
                let mut model_quality_job_output_config: Option<::Value<self::model_quality_job_definition::MonitoringOutputConfig>> = None;
                let mut network_config: Option<::Value<self::model_quality_job_definition::NetworkConfig>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut stopping_condition: Option<::Value<self::model_quality_job_definition::StoppingCondition>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobDefinitionName" => {
                            job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "JobResources" => {
                            job_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelQualityAppSpecification" => {
                            model_quality_app_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelQualityBaselineConfig" => {
                            model_quality_baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelQualityJobInput" => {
                            model_quality_job_input = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ModelQualityJobOutputConfig" => {
                            model_quality_job_output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NetworkConfig" => {
                            network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "StoppingCondition" => {
                            stopping_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ModelQualityJobDefinitionProperties {
                    endpoint_name: endpoint_name,
                    job_definition_name: job_definition_name,
                    job_resources: job_resources.ok_or(::serde::de::Error::missing_field("JobResources"))?,
                    model_quality_app_specification: model_quality_app_specification.ok_or(::serde::de::Error::missing_field("ModelQualityAppSpecification"))?,
                    model_quality_baseline_config: model_quality_baseline_config,
                    model_quality_job_input: model_quality_job_input.ok_or(::serde::de::Error::missing_field("ModelQualityJobInput"))?,
                    model_quality_job_output_config: model_quality_job_output_config.ok_or(::serde::de::Error::missing_field("ModelQualityJobOutputConfig"))?,
                    network_config: network_config,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    stopping_condition: stopping_condition,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for ModelQualityJobDefinition {
    type Properties = ModelQualityJobDefinitionProperties;
    const TYPE: &'static str = "AWS::SageMaker::ModelQualityJobDefinition";
    fn properties(&self) -> &ModelQualityJobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ModelQualityJobDefinitionProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for ModelQualityJobDefinition {}

impl From<ModelQualityJobDefinitionProperties> for ModelQualityJobDefinition {
    fn from(properties: ModelQualityJobDefinitionProperties) -> ModelQualityJobDefinition {
        ModelQualityJobDefinition { properties }
    }
}

/// The [`AWS::SageMaker::MonitoringSchedule`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html) resource type.
#[derive(Debug, Default)]
pub struct MonitoringSchedule {
    properties: MonitoringScheduleProperties
}

/// Properties for the `MonitoringSchedule` resource.
#[derive(Debug, Default)]
pub struct MonitoringScheduleProperties {
    /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-endpointname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub endpoint_name: Option<::Value<String>>,
    /// Property [`FailureReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-failurereason).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub failure_reason: Option<::Value<String>>,
    /// Property [`LastMonitoringExecutionSummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-lastmonitoringexecutionsummary).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub last_monitoring_execution_summary: Option<::Value<self::monitoring_schedule::MonitoringExecutionSummary>>,
    /// Property [`MonitoringScheduleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-monitoringscheduleconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_schedule_config: ::Value<self::monitoring_schedule::MonitoringScheduleConfig>,
    /// Property [`MonitoringScheduleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-monitoringschedulename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub monitoring_schedule_name: ::Value<String>,
    /// Property [`MonitoringScheduleStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-monitoringschedulestatus).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub monitoring_schedule_status: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-monitoringschedule.html#cfn-sagemaker-monitoringschedule-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for MonitoringScheduleProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref endpoint_name) = self.endpoint_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
        }
        if let Some(ref failure_reason) = self.failure_reason {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureReason", failure_reason)?;
        }
        if let Some(ref last_monitoring_execution_summary) = self.last_monitoring_execution_summary {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastMonitoringExecutionSummary", last_monitoring_execution_summary)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringScheduleConfig", &self.monitoring_schedule_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringScheduleName", &self.monitoring_schedule_name)?;
        if let Some(ref monitoring_schedule_status) = self.monitoring_schedule_status {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringScheduleStatus", monitoring_schedule_status)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for MonitoringScheduleProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringScheduleProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = MonitoringScheduleProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type MonitoringScheduleProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut endpoint_name: Option<::Value<String>> = None;
                let mut failure_reason: Option<::Value<String>> = None;
                let mut last_monitoring_execution_summary: Option<::Value<self::monitoring_schedule::MonitoringExecutionSummary>> = None;
                let mut monitoring_schedule_config: Option<::Value<self::monitoring_schedule::MonitoringScheduleConfig>> = None;
                let mut monitoring_schedule_name: Option<::Value<String>> = None;
                let mut monitoring_schedule_status: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "EndpointName" => {
                            endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "FailureReason" => {
                            failure_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LastMonitoringExecutionSummary" => {
                            last_monitoring_execution_summary = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringScheduleConfig" => {
                            monitoring_schedule_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringScheduleName" => {
                            monitoring_schedule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MonitoringScheduleStatus" => {
                            monitoring_schedule_status = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(MonitoringScheduleProperties {
                    endpoint_name: endpoint_name,
                    failure_reason: failure_reason,
                    last_monitoring_execution_summary: last_monitoring_execution_summary,
                    monitoring_schedule_config: monitoring_schedule_config.ok_or(::serde::de::Error::missing_field("MonitoringScheduleConfig"))?,
                    monitoring_schedule_name: monitoring_schedule_name.ok_or(::serde::de::Error::missing_field("MonitoringScheduleName"))?,
                    monitoring_schedule_status: monitoring_schedule_status,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for MonitoringSchedule {
    type Properties = MonitoringScheduleProperties;
    const TYPE: &'static str = "AWS::SageMaker::MonitoringSchedule";
    fn properties(&self) -> &MonitoringScheduleProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut MonitoringScheduleProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for MonitoringSchedule {}

impl From<MonitoringScheduleProperties> for MonitoringSchedule {
    fn from(properties: MonitoringScheduleProperties) -> MonitoringSchedule {
        MonitoringSchedule { properties }
    }
}

/// The [`AWS::SageMaker::NotebookInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html) resource type.
#[derive(Debug, Default)]
pub struct NotebookInstance {
    properties: NotebookInstanceProperties
}

/// Properties for the `NotebookInstance` resource.
#[derive(Debug, Default)]
pub struct NotebookInstanceProperties {
    /// Property [`AcceleratorTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-acceleratortypes).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub accelerator_types: Option<::ValueList<String>>,
    /// Property [`AdditionalCodeRepositories`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-additionalcoderepositories).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub additional_code_repositories: Option<::ValueList<String>>,
    /// Property [`DefaultCodeRepository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-defaultcoderepository).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub default_code_repository: Option<::Value<String>>,
    /// Property [`DirectInternetAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-directinternetaccess).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub direct_internet_access: Option<::Value<String>>,
    /// Property [`InstanceMetadataServiceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-instancemetadataserviceconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_metadata_service_configuration: Option<::Value<self::notebook_instance::InstanceMetadataServiceConfiguration>>,
    /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-instancetype).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub instance_type: ::Value<String>,
    /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-kmskeyid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub kms_key_id: Option<::Value<String>>,
    /// Property [`LifecycleConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-lifecycleconfigname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub lifecycle_config_name: Option<::Value<String>>,
    /// Property [`NotebookInstanceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-notebookinstancename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub notebook_instance_name: Option<::Value<String>>,
    /// Property [`PlatformIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-platformidentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub platform_identifier: Option<::Value<String>>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`RootAccess`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-rootaccess).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub root_access: Option<::Value<String>>,
    /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-securitygroupids).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub security_group_ids: Option<::ValueList<String>>,
    /// Property [`SubnetId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-subnetid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub subnet_id: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstance.html#cfn-sagemaker-notebookinstance-volumesizeingb).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub volume_size_in_gb: Option<::Value<u32>>,
}

impl ::serde::Serialize for NotebookInstanceProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref accelerator_types) = self.accelerator_types {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceleratorTypes", accelerator_types)?;
        }
        if let Some(ref additional_code_repositories) = self.additional_code_repositories {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AdditionalCodeRepositories", additional_code_repositories)?;
        }
        if let Some(ref default_code_repository) = self.default_code_repository {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultCodeRepository", default_code_repository)?;
        }
        if let Some(ref direct_internet_access) = self.direct_internet_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DirectInternetAccess", direct_internet_access)?;
        }
        if let Some(ref instance_metadata_service_configuration) = self.instance_metadata_service_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceMetadataServiceConfiguration", instance_metadata_service_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
        if let Some(ref kms_key_id) = self.kms_key_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
        }
        if let Some(ref lifecycle_config_name) = self.lifecycle_config_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleConfigName", lifecycle_config_name)?;
        }
        if let Some(ref notebook_instance_name) = self.notebook_instance_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotebookInstanceName", notebook_instance_name)?;
        }
        if let Some(ref platform_identifier) = self.platform_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PlatformIdentifier", platform_identifier)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref root_access) = self.root_access {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RootAccess", root_access)?;
        }
        if let Some(ref security_group_ids) = self.security_group_ids {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
        }
        if let Some(ref subnet_id) = self.subnet_id {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SubnetId", subnet_id)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref volume_size_in_gb) = self.volume_size_in_gb {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", volume_size_in_gb)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotebookInstanceProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NotebookInstanceProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotebookInstanceProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotebookInstanceProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut accelerator_types: Option<::ValueList<String>> = None;
                let mut additional_code_repositories: Option<::ValueList<String>> = None;
                let mut default_code_repository: Option<::Value<String>> = None;
                let mut direct_internet_access: Option<::Value<String>> = None;
                let mut instance_metadata_service_configuration: Option<::Value<self::notebook_instance::InstanceMetadataServiceConfiguration>> = None;
                let mut instance_type: Option<::Value<String>> = None;
                let mut kms_key_id: Option<::Value<String>> = None;
                let mut lifecycle_config_name: Option<::Value<String>> = None;
                let mut notebook_instance_name: Option<::Value<String>> = None;
                let mut platform_identifier: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut root_access: Option<::Value<String>> = None;
                let mut security_group_ids: Option<::ValueList<String>> = None;
                let mut subnet_id: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut volume_size_in_gb: Option<::Value<u32>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "AcceleratorTypes" => {
                            accelerator_types = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "AdditionalCodeRepositories" => {
                            additional_code_repositories = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DefaultCodeRepository" => {
                            default_code_repository = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DirectInternetAccess" => {
                            direct_internet_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceMetadataServiceConfiguration" => {
                            instance_metadata_service_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "InstanceType" => {
                            instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "KmsKeyId" => {
                            kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "LifecycleConfigName" => {
                            lifecycle_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotebookInstanceName" => {
                            notebook_instance_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PlatformIdentifier" => {
                            platform_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RootAccess" => {
                            root_access = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SecurityGroupIds" => {
                            security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SubnetId" => {
                            subnet_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "VolumeSizeInGB" => {
                            volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotebookInstanceProperties {
                    accelerator_types: accelerator_types,
                    additional_code_repositories: additional_code_repositories,
                    default_code_repository: default_code_repository,
                    direct_internet_access: direct_internet_access,
                    instance_metadata_service_configuration: instance_metadata_service_configuration,
                    instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                    kms_key_id: kms_key_id,
                    lifecycle_config_name: lifecycle_config_name,
                    notebook_instance_name: notebook_instance_name,
                    platform_identifier: platform_identifier,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    root_access: root_access,
                    security_group_ids: security_group_ids,
                    subnet_id: subnet_id,
                    tags: tags,
                    volume_size_in_gb: volume_size_in_gb,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NotebookInstance {
    type Properties = NotebookInstanceProperties;
    const TYPE: &'static str = "AWS::SageMaker::NotebookInstance";
    fn properties(&self) -> &NotebookInstanceProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotebookInstanceProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NotebookInstance {}

impl From<NotebookInstanceProperties> for NotebookInstance {
    fn from(properties: NotebookInstanceProperties) -> NotebookInstance {
        NotebookInstance { properties }
    }
}

/// The [`AWS::SageMaker::NotebookInstanceLifecycleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstancelifecycleconfig.html) resource type.
#[derive(Debug, Default)]
pub struct NotebookInstanceLifecycleConfig {
    properties: NotebookInstanceLifecycleConfigProperties
}

/// Properties for the `NotebookInstanceLifecycleConfig` resource.
#[derive(Debug, Default)]
pub struct NotebookInstanceLifecycleConfigProperties {
    /// Property [`NotebookInstanceLifecycleConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstancelifecycleconfig.html#cfn-sagemaker-notebookinstancelifecycleconfig-notebookinstancelifecycleconfigname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub notebook_instance_lifecycle_config_name: Option<::Value<String>>,
    /// Property [`OnCreate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstancelifecycleconfig.html#cfn-sagemaker-notebookinstancelifecycleconfig-oncreate).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub on_create: Option<::ValueList<self::notebook_instance_lifecycle_config::NotebookInstanceLifecycleHook>>,
    /// Property [`OnStart`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-notebookinstancelifecycleconfig.html#cfn-sagemaker-notebookinstancelifecycleconfig-onstart).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub on_start: Option<::ValueList<self::notebook_instance_lifecycle_config::NotebookInstanceLifecycleHook>>,
}

impl ::serde::Serialize for NotebookInstanceLifecycleConfigProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref notebook_instance_lifecycle_config_name) = self.notebook_instance_lifecycle_config_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotebookInstanceLifecycleConfigName", notebook_instance_lifecycle_config_name)?;
        }
        if let Some(ref on_create) = self.on_create {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnCreate", on_create)?;
        }
        if let Some(ref on_start) = self.on_start {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OnStart", on_start)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for NotebookInstanceLifecycleConfigProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<NotebookInstanceLifecycleConfigProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = NotebookInstanceLifecycleConfigProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type NotebookInstanceLifecycleConfigProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut notebook_instance_lifecycle_config_name: Option<::Value<String>> = None;
                let mut on_create: Option<::ValueList<self::notebook_instance_lifecycle_config::NotebookInstanceLifecycleHook>> = None;
                let mut on_start: Option<::ValueList<self::notebook_instance_lifecycle_config::NotebookInstanceLifecycleHook>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "NotebookInstanceLifecycleConfigName" => {
                            notebook_instance_lifecycle_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnCreate" => {
                            on_create = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OnStart" => {
                            on_start = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(NotebookInstanceLifecycleConfigProperties {
                    notebook_instance_lifecycle_config_name: notebook_instance_lifecycle_config_name,
                    on_create: on_create,
                    on_start: on_start,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for NotebookInstanceLifecycleConfig {
    type Properties = NotebookInstanceLifecycleConfigProperties;
    const TYPE: &'static str = "AWS::SageMaker::NotebookInstanceLifecycleConfig";
    fn properties(&self) -> &NotebookInstanceLifecycleConfigProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut NotebookInstanceLifecycleConfigProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for NotebookInstanceLifecycleConfig {}

impl From<NotebookInstanceLifecycleConfigProperties> for NotebookInstanceLifecycleConfig {
    fn from(properties: NotebookInstanceLifecycleConfigProperties) -> NotebookInstanceLifecycleConfig {
        NotebookInstanceLifecycleConfig { properties }
    }
}

/// The [`AWS::SageMaker::Pipeline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html) resource type.
#[derive(Debug, Default)]
pub struct Pipeline {
    properties: PipelineProperties
}

/// Properties for the `Pipeline` resource.
#[derive(Debug, Default)]
pub struct PipelineProperties {
    /// Property [`ParallelismConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-parallelismconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub parallelism_configuration: Option<::Value<::json::Value>>,
    /// Property [`PipelineDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-pipelinedefinition).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_definition: ::Value<::json::Value>,
    /// Property [`PipelineDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-pipelinedescription).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_description: Option<::Value<String>>,
    /// Property [`PipelineDisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-pipelinedisplayname).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub pipeline_display_name: Option<::Value<String>>,
    /// Property [`PipelineName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-pipelinename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub pipeline_name: ::Value<String>,
    /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-rolearn).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub role_arn: ::Value<String>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-pipeline.html#cfn-sagemaker-pipeline-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for PipelineProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref parallelism_configuration) = self.parallelism_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ParallelismConfiguration", parallelism_configuration)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineDefinition", &self.pipeline_definition)?;
        if let Some(ref pipeline_description) = self.pipeline_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineDescription", pipeline_description)?;
        }
        if let Some(ref pipeline_display_name) = self.pipeline_display_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineDisplayName", pipeline_display_name)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "PipelineName", &self.pipeline_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for PipelineProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<PipelineProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = PipelineProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type PipelineProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut parallelism_configuration: Option<::Value<::json::Value>> = None;
                let mut pipeline_definition: Option<::Value<::json::Value>> = None;
                let mut pipeline_description: Option<::Value<String>> = None;
                let mut pipeline_display_name: Option<::Value<String>> = None;
                let mut pipeline_name: Option<::Value<String>> = None;
                let mut role_arn: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ParallelismConfiguration" => {
                            parallelism_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineDefinition" => {
                            pipeline_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineDescription" => {
                            pipeline_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineDisplayName" => {
                            pipeline_display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "PipelineName" => {
                            pipeline_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "RoleArn" => {
                            role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(PipelineProperties {
                    parallelism_configuration: parallelism_configuration,
                    pipeline_definition: pipeline_definition.ok_or(::serde::de::Error::missing_field("PipelineDefinition"))?,
                    pipeline_description: pipeline_description,
                    pipeline_display_name: pipeline_display_name,
                    pipeline_name: pipeline_name.ok_or(::serde::de::Error::missing_field("PipelineName"))?,
                    role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Pipeline {
    type Properties = PipelineProperties;
    const TYPE: &'static str = "AWS::SageMaker::Pipeline";
    fn properties(&self) -> &PipelineProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut PipelineProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Pipeline {}

impl From<PipelineProperties> for Pipeline {
    fn from(properties: PipelineProperties) -> Pipeline {
        Pipeline { properties }
    }
}

/// The [`AWS::SageMaker::Project`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html) resource type.
#[derive(Debug, Default)]
pub struct Project {
    properties: ProjectProperties
}

/// Properties for the `Project` resource.
#[derive(Debug, Default)]
pub struct ProjectProperties {
    /// Property [`ProjectDescription`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html#cfn-sagemaker-project-projectdescription).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_description: Option<::Value<String>>,
    /// Property [`ProjectName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html#cfn-sagemaker-project-projectname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub project_name: ::Value<String>,
    /// Property [`ServiceCatalogProvisioningDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html#cfn-sagemaker-project-servicecatalogprovisioningdetails).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub service_catalog_provisioning_details: ::Value<::json::Value>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-project.html#cfn-sagemaker-project-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for ProjectProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref project_description) = self.project_description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectDescription", project_description)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectName", &self.project_name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServiceCatalogProvisioningDetails", &self.service_catalog_provisioning_details)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for ProjectProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<ProjectProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ProjectProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type ProjectProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut project_description: Option<::Value<String>> = None;
                let mut project_name: Option<::Value<String>> = None;
                let mut service_catalog_provisioning_details: Option<::Value<::json::Value>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ProjectDescription" => {
                            project_description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ProjectName" => {
                            project_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "ServiceCatalogProvisioningDetails" => {
                            service_catalog_provisioning_details = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(ProjectProperties {
                    project_description: project_description,
                    project_name: project_name.ok_or(::serde::de::Error::missing_field("ProjectName"))?,
                    service_catalog_provisioning_details: service_catalog_provisioning_details.ok_or(::serde::de::Error::missing_field("ServiceCatalogProvisioningDetails"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Project {
    type Properties = ProjectProperties;
    const TYPE: &'static str = "AWS::SageMaker::Project";
    fn properties(&self) -> &ProjectProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ProjectProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Project {}

impl From<ProjectProperties> for Project {
    fn from(properties: ProjectProperties) -> Project {
        Project { properties }
    }
}

/// The [`AWS::SageMaker::UserProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html) resource type.
#[derive(Debug, Default)]
pub struct UserProfile {
    properties: UserProfileProperties
}

/// Properties for the `UserProfile` resource.
#[derive(Debug, Default)]
pub struct UserProfileProperties {
    /// Property [`DomainId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-domainid).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub domain_id: ::Value<String>,
    /// Property [`SingleSignOnUserIdentifier`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-singlesignonuseridentifier).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub single_sign_on_user_identifier: Option<::Value<String>>,
    /// Property [`SingleSignOnUserValue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-singlesignonuservalue).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub single_sign_on_user_value: Option<::Value<String>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-tags).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`UserProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-userprofilename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub user_profile_name: ::Value<String>,
    /// Property [`UserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-userprofile.html#cfn-sagemaker-userprofile-usersettings).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub user_settings: Option<::Value<self::user_profile::UserSettings>>,
}

impl ::serde::Serialize for UserProfileProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainId", &self.domain_id)?;
        if let Some(ref single_sign_on_user_identifier) = self.single_sign_on_user_identifier {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSignOnUserIdentifier", single_sign_on_user_identifier)?;
        }
        if let Some(ref single_sign_on_user_value) = self.single_sign_on_user_value {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SingleSignOnUserValue", single_sign_on_user_value)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProfileName", &self.user_profile_name)?;
        if let Some(ref user_settings) = self.user_settings {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserSettings", user_settings)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for UserProfileProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<UserProfileProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = UserProfileProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type UserProfileProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut domain_id: Option<::Value<String>> = None;
                let mut single_sign_on_user_identifier: Option<::Value<String>> = None;
                let mut single_sign_on_user_value: Option<::Value<String>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut user_profile_name: Option<::Value<String>> = None;
                let mut user_settings: Option<::Value<self::user_profile::UserSettings>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "DomainId" => {
                            domain_id = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SingleSignOnUserIdentifier" => {
                            single_sign_on_user_identifier = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "SingleSignOnUserValue" => {
                            single_sign_on_user_value = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserProfileName" => {
                            user_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "UserSettings" => {
                            user_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(UserProfileProperties {
                    domain_id: domain_id.ok_or(::serde::de::Error::missing_field("DomainId"))?,
                    single_sign_on_user_identifier: single_sign_on_user_identifier,
                    single_sign_on_user_value: single_sign_on_user_value,
                    tags: tags,
                    user_profile_name: user_profile_name.ok_or(::serde::de::Error::missing_field("UserProfileName"))?,
                    user_settings: user_settings,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for UserProfile {
    type Properties = UserProfileProperties;
    const TYPE: &'static str = "AWS::SageMaker::UserProfile";
    fn properties(&self) -> &UserProfileProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut UserProfileProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for UserProfile {}

impl From<UserProfileProperties> for UserProfile {
    fn from(properties: UserProfileProperties) -> UserProfile {
        UserProfile { properties }
    }
}

/// The [`AWS::SageMaker::Workteam`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html) resource type.
#[derive(Debug, Default)]
pub struct Workteam {
    properties: WorkteamProperties
}

/// Properties for the `Workteam` resource.
#[derive(Debug, Default)]
pub struct WorkteamProperties {
    /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-description).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub description: Option<::Value<String>>,
    /// Property [`MemberDefinitions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-memberdefinitions).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub member_definitions: Option<::ValueList<self::workteam::MemberDefinition>>,
    /// Property [`NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-notificationconfiguration).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub notification_configuration: Option<::Value<self::workteam::NotificationConfiguration>>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
    /// Property [`WorkforceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-workforcename).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workforce_name: Option<::Value<String>>,
    /// Property [`WorkteamName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sagemaker-workteam.html#cfn-sagemaker-workteam-workteamname).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub workteam_name: Option<::Value<String>>,
}

impl ::serde::Serialize for WorkteamProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        if let Some(ref description) = self.description {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
        }
        if let Some(ref member_definitions) = self.member_definitions {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemberDefinitions", member_definitions)?;
        }
        if let Some(ref notification_configuration) = self.notification_configuration {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfiguration", notification_configuration)?;
        }
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        if let Some(ref workforce_name) = self.workforce_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkforceName", workforce_name)?;
        }
        if let Some(ref workteam_name) = self.workteam_name {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "WorkteamName", workteam_name)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for WorkteamProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<WorkteamProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = WorkteamProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type WorkteamProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut description: Option<::Value<String>> = None;
                let mut member_definitions: Option<::ValueList<self::workteam::MemberDefinition>> = None;
                let mut notification_configuration: Option<::Value<self::workteam::NotificationConfiguration>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;
                let mut workforce_name: Option<::Value<String>> = None;
                let mut workteam_name: Option<::Value<String>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "Description" => {
                            description = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "MemberDefinitions" => {
                            member_definitions = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "NotificationConfiguration" => {
                            notification_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkforceName" => {
                            workforce_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "WorkteamName" => {
                            workteam_name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(WorkteamProperties {
                    description: description,
                    member_definitions: member_definitions,
                    notification_configuration: notification_configuration,
                    tags: tags,
                    workforce_name: workforce_name,
                    workteam_name: workteam_name,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Workteam {
    type Properties = WorkteamProperties;
    const TYPE: &'static str = "AWS::SageMaker::Workteam";
    fn properties(&self) -> &WorkteamProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut WorkteamProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Workteam {}

impl From<WorkteamProperties> for Workteam {
    fn from(properties: WorkteamProperties) -> Workteam {
        Workteam { properties }
    }
}

pub mod app {
    //! Property types for the `App` resource.

    /// The [`AWS::SageMaker::App.ResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-app-resourcespec.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceSpec {
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-app-resourcespec.html#cfn-sagemaker-app-resourcespec-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`SageMakerImageArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-app-resourcespec.html#cfn-sagemaker-app-resourcespec-sagemakerimagearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_arn: Option<::Value<String>>,
        /// Property [`SageMakerImageVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-app-resourcespec.html#cfn-sagemaker-app-resourcespec-sagemakerimageversionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_version_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref sage_maker_image_arn) = self.sage_maker_image_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageArn", sage_maker_image_arn)?;
            }
            if let Some(ref sage_maker_image_version_arn) = self.sage_maker_image_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageVersionArn", sage_maker_image_version_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut sage_maker_image_arn: Option<::Value<String>> = None;
                    let mut sage_maker_image_version_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageArn" => {
                                sage_maker_image_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageVersionArn" => {
                                sage_maker_image_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceSpec {
                        instance_type: instance_type,
                        sage_maker_image_arn: sage_maker_image_arn,
                        sage_maker_image_version_arn: sage_maker_image_version_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod app_image_config {
    //! Property types for the `AppImageConfig` resource.

    /// The [`AWS::SageMaker::AppImageConfig.FileSystemConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-filesystemconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct FileSystemConfig {
        /// Property [`DefaultGid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-filesystemconfig.html#cfn-sagemaker-appimageconfig-filesystemconfig-defaultgid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_gid: Option<::Value<u32>>,
        /// Property [`DefaultUid`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-filesystemconfig.html#cfn-sagemaker-appimageconfig-filesystemconfig-defaultuid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_uid: Option<::Value<u32>>,
        /// Property [`MountPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-filesystemconfig.html#cfn-sagemaker-appimageconfig-filesystemconfig-mountpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub mount_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for FileSystemConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_gid) = self.default_gid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultGid", default_gid)?;
            }
            if let Some(ref default_uid) = self.default_uid {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultUid", default_uid)?;
            }
            if let Some(ref mount_path) = self.mount_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MountPath", mount_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileSystemConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSystemConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileSystemConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileSystemConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_gid: Option<::Value<u32>> = None;
                    let mut default_uid: Option<::Value<u32>> = None;
                    let mut mount_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultGid" => {
                                default_gid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultUid" => {
                                default_uid = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MountPath" => {
                                mount_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileSystemConfig {
                        default_gid: default_gid,
                        default_uid: default_uid,
                        mount_path: mount_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::AppImageConfig.KernelGatewayImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelgatewayimageconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct KernelGatewayImageConfig {
        /// Property [`FileSystemConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelgatewayimageconfig.html#cfn-sagemaker-appimageconfig-kernelgatewayimageconfig-filesystemconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub file_system_config: Option<::Value<FileSystemConfig>>,
        /// Property [`KernelSpecs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelgatewayimageconfig.html#cfn-sagemaker-appimageconfig-kernelgatewayimageconfig-kernelspecs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kernel_specs: ::ValueList<KernelSpec>,
    }

    impl ::codec::SerializeValue for KernelGatewayImageConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref file_system_config) = self.file_system_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FileSystemConfig", file_system_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelSpecs", &self.kernel_specs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KernelGatewayImageConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KernelGatewayImageConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KernelGatewayImageConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KernelGatewayImageConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut file_system_config: Option<::Value<FileSystemConfig>> = None;
                    let mut kernel_specs: Option<::ValueList<KernelSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FileSystemConfig" => {
                                file_system_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KernelSpecs" => {
                                kernel_specs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KernelGatewayImageConfig {
                        file_system_config: file_system_config,
                        kernel_specs: kernel_specs.ok_or(::serde::de::Error::missing_field("KernelSpecs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::AppImageConfig.KernelSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelspec.html) property type.
    #[derive(Debug, Default)]
    pub struct KernelSpec {
        /// Property [`DisplayName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelspec.html#cfn-sagemaker-appimageconfig-kernelspec-displayname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub display_name: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-appimageconfig-kernelspec.html#cfn-sagemaker-appimageconfig-kernelspec-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
    }

    impl ::codec::SerializeValue for KernelSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref display_name) = self.display_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DisplayName", display_name)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KernelSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KernelSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KernelSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KernelSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut display_name: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DisplayName" => {
                                display_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KernelSpec {
                        display_name: display_name,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod code_repository {
    //! Property types for the `CodeRepository` resource.

    /// The [`AWS::SageMaker::CodeRepository.GitConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-coderepository-gitconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct GitConfig {
        /// Property [`Branch`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-coderepository-gitconfig.html#cfn-sagemaker-coderepository-gitconfig-branch).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub branch: Option<::Value<String>>,
        /// Property [`RepositoryUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-coderepository-gitconfig.html#cfn-sagemaker-coderepository-gitconfig-repositoryurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_url: ::Value<String>,
        /// Property [`SecretArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-coderepository-gitconfig.html#cfn-sagemaker-coderepository-gitconfig-secretarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub secret_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for GitConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref branch) = self.branch {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Branch", branch)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryUrl", &self.repository_url)?;
            if let Some(ref secret_arn) = self.secret_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecretArn", secret_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for GitConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<GitConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = GitConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type GitConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut branch: Option<::Value<String>> = None;
                    let mut repository_url: Option<::Value<String>> = None;
                    let mut secret_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Branch" => {
                                branch = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepositoryUrl" => {
                                repository_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecretArn" => {
                                secret_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(GitConfig {
                        branch: branch,
                        repository_url: repository_url.ok_or(::serde::de::Error::missing_field("RepositoryUrl"))?,
                        secret_arn: secret_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod data_quality_job_definition {
    //! Property types for the `DataQualityJobDefinition` resource.

    /// The [`AWS::SageMaker::DataQualityJobDefinition.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html#cfn-sagemaker-dataqualityjobdefinition-clusterconfig-instancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html#cfn-sagemaker-dataqualityjobdefinition-clusterconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html#cfn-sagemaker-dataqualityjobdefinition-clusterconfig-volumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-clusterconfig.html#cfn-sagemaker-dataqualityjobdefinition-clusterconfig-volumesizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-constraintsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct ConstraintsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-constraintsresource.html#cfn-sagemaker-dataqualityjobdefinition-constraintsresource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConstraintsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConstraintsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConstraintsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConstraintsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConstraintsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConstraintsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.DataQualityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct DataQualityAppSpecification {
        /// Property [`ContainerArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-containerarguments).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_arguments: Option<::ValueList<String>>,
        /// Property [`ContainerEntrypoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-containerentrypoint).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_entrypoint: Option<::ValueList<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::ValueMap<String>>,
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-imageuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_uri: ::Value<String>,
        /// Property [`PostAnalyticsProcessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-postanalyticsprocessorsourceuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub post_analytics_processor_source_uri: Option<::Value<String>>,
        /// Property [`RecordPreprocessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityappspecification.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityappspecification-recordpreprocessorsourceuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub record_preprocessor_source_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataQualityAppSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_arguments) = self.container_arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerArguments", container_arguments)?;
            }
            if let Some(ref container_entrypoint) = self.container_entrypoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerEntrypoint", container_entrypoint)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", &self.image_uri)?;
            if let Some(ref post_analytics_processor_source_uri) = self.post_analytics_processor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAnalyticsProcessorSourceUri", post_analytics_processor_source_uri)?;
            }
            if let Some(ref record_preprocessor_source_uri) = self.record_preprocessor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordPreprocessorSourceUri", record_preprocessor_source_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataQualityAppSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityAppSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataQualityAppSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataQualityAppSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_arguments: Option<::ValueList<String>> = None;
                    let mut container_entrypoint: Option<::ValueList<String>> = None;
                    let mut environment: Option<::ValueMap<String>> = None;
                    let mut image_uri: Option<::Value<String>> = None;
                    let mut post_analytics_processor_source_uri: Option<::Value<String>> = None;
                    let mut record_preprocessor_source_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerArguments" => {
                                container_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerEntrypoint" => {
                                container_entrypoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostAnalyticsProcessorSourceUri" => {
                                post_analytics_processor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordPreprocessorSourceUri" => {
                                record_preprocessor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataQualityAppSpecification {
                        container_arguments: container_arguments,
                        container_entrypoint: container_entrypoint,
                        environment: environment,
                        image_uri: image_uri.ok_or(::serde::de::Error::missing_field("ImageUri"))?,
                        post_analytics_processor_source_uri: post_analytics_processor_source_uri,
                        record_preprocessor_source_uri: record_preprocessor_source_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.DataQualityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DataQualityBaselineConfig {
        /// Property [`BaseliningJobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig.html#cfn-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig-baseliningjobname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub baselining_job_name: Option<::Value<String>>,
        /// Property [`ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig.html#cfn-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig-constraintsresource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints_resource: Option<::Value<ConstraintsResource>>,
        /// Property [`StatisticsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig.html#cfn-sagemaker-dataqualityjobdefinition-dataqualitybaselineconfig-statisticsresource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub statistics_resource: Option<::Value<StatisticsResource>>,
    }

    impl ::codec::SerializeValue for DataQualityBaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref baselining_job_name) = self.baselining_job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseliningJobName", baselining_job_name)?;
            }
            if let Some(ref constraints_resource) = self.constraints_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstraintsResource", constraints_resource)?;
            }
            if let Some(ref statistics_resource) = self.statistics_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatisticsResource", statistics_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataQualityBaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityBaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataQualityBaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataQualityBaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut baselining_job_name: Option<::Value<String>> = None;
                    let mut constraints_resource: Option<::Value<ConstraintsResource>> = None;
                    let mut statistics_resource: Option<::Value<StatisticsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseliningJobName" => {
                                baselining_job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstraintsResource" => {
                                constraints_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatisticsResource" => {
                                statistics_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataQualityBaselineConfig {
                        baselining_job_name: baselining_job_name,
                        constraints_resource: constraints_resource,
                        statistics_resource: statistics_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.DataQualityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityjobinput.html) property type.
    #[derive(Debug, Default)]
    pub struct DataQualityJobInput {
        /// Property [`EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-dataqualityjobinput.html#cfn-sagemaker-dataqualityjobdefinition-dataqualityjobinput-endpointinput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_input: ::Value<EndpointInput>,
    }

    impl ::codec::SerializeValue for DataQualityJobInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointInput", &self.endpoint_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataQualityJobInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataQualityJobInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataQualityJobInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataQualityJobInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_input: Option<::Value<EndpointInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointInput" => {
                                endpoint_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataQualityJobInput {
                        endpoint_input: endpoint_input.ok_or(::serde::de::Error::missing_field("EndpointInput"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointInput {
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html#cfn-sagemaker-dataqualityjobdefinition-endpointinput-endpointname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_name: ::Value<String>,
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html#cfn-sagemaker-dataqualityjobdefinition-endpointinput-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3DataDistributionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html#cfn-sagemaker-dataqualityjobdefinition-endpointinput-s3datadistributiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_distribution_type: Option<::Value<String>>,
        /// Property [`S3InputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-endpointinput.html#cfn-sagemaker-dataqualityjobdefinition-endpointinput-s3inputmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_input_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_data_distribution_type) = self.s3_data_distribution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataDistributionType", s3_data_distribution_type)?;
            }
            if let Some(ref s3_input_mode) = self.s3_input_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputMode", s3_input_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_data_distribution_type: Option<::Value<String>> = None;
                    let mut s3_input_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DataDistributionType" => {
                                s3_data_distribution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputMode" => {
                                s3_input_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointInput {
                        endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_data_distribution_type: s3_data_distribution_type,
                        s3_input_mode: s3_input_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.MonitoringOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutput {
        /// Property [`S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutput.html#cfn-sagemaker-dataqualityjobdefinition-monitoringoutput-s3output).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output: ::Value<S3Output>,
    }

    impl ::codec::SerializeValue for MonitoringOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Output", &self.s3_output)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_output: Option<::Value<S3Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Output" => {
                                s3_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutput {
                        s3_output: s3_output.ok_or(::serde::de::Error::missing_field("S3Output"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-dataqualityjobdefinition-monitoringoutputconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MonitoringOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-dataqualityjobdefinition-monitoringoutputconfig-monitoringoutputs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub monitoring_outputs: ::ValueList<MonitoringOutput>,
    }

    impl ::codec::SerializeValue for MonitoringOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputs", &self.monitoring_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut monitoring_outputs: Option<::ValueList<MonitoringOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputs" => {
                                monitoring_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutputConfig {
                        kms_key_id: kms_key_id,
                        monitoring_outputs: monitoring_outputs.ok_or(::serde::de::Error::missing_field("MonitoringOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringResources {
        /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-monitoringresources.html#cfn-sagemaker-dataqualityjobdefinition-monitoringresources-clusterconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cluster_config: ::Value<ClusterConfig>,
    }

    impl ::codec::SerializeValue for MonitoringResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", &self.cluster_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_config: Option<::Value<ClusterConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterConfig" => {
                                cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringResources {
                        cluster_config: cluster_config.ok_or(::serde::de::Error::missing_field("ClusterConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-networkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfig {
        /// Property [`EnableInterContainerTrafficEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-networkconfig.html#cfn-sagemaker-dataqualityjobdefinition-networkconfig-enableintercontainertrafficencryption).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_inter_container_traffic_encryption: Option<::Value<bool>>,
        /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-networkconfig.html#cfn-sagemaker-dataqualityjobdefinition-networkconfig-enablenetworkisolation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_network_isolation: Option<::Value<bool>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-networkconfig.html#cfn-sagemaker-dataqualityjobdefinition-networkconfig-vpcconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for NetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_inter_container_traffic_encryption) = self.enable_inter_container_traffic_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableInterContainerTrafficEncryption", enable_inter_container_traffic_encryption)?;
            }
            if let Some(ref enable_network_isolation) = self.enable_network_isolation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_inter_container_traffic_encryption: Option<::Value<bool>> = None;
                    let mut enable_network_isolation: Option<::Value<bool>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                enable_inter_container_traffic_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNetworkIsolation" => {
                                enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfig {
                        enable_inter_container_traffic_encryption: enable_inter_container_traffic_encryption,
                        enable_network_isolation: enable_network_isolation,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-s3output.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Output {
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-s3output.html#cfn-sagemaker-dataqualityjobdefinition-s3output-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3UploadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-s3output.html#cfn-sagemaker-dataqualityjobdefinition-s3output-s3uploadmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_upload_mode: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-s3output.html#cfn-sagemaker-dataqualityjobdefinition-s3output-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_upload_mode) = self.s3_upload_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3UploadMode", s3_upload_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_upload_mode: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3UploadMode" => {
                                s3_upload_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Output {
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_upload_mode: s3_upload_mode,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.StatisticsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-statisticsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct StatisticsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-statisticsresource.html#cfn-sagemaker-dataqualityjobdefinition-statisticsresource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatisticsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatisticsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatisticsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatisticsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatisticsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatisticsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-stoppingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct StoppingCondition {
        /// Property [`MaxRuntimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-stoppingcondition.html#cfn-sagemaker-dataqualityjobdefinition-stoppingcondition-maxruntimeinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_runtime_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StoppingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuntimeInSeconds", &self.max_runtime_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoppingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoppingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoppingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoppingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_runtime_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                max_runtime_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoppingCondition {
                        max_runtime_in_seconds: max_runtime_in_seconds.ok_or(::serde::de::Error::missing_field("MaxRuntimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::DataQualityJobDefinition.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-vpcconfig.html#cfn-sagemaker-dataqualityjobdefinition-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-dataqualityjobdefinition-vpcconfig.html#cfn-sagemaker-dataqualityjobdefinition-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod device {
    //! Property types for the `Device` resource.

    /// The [`AWS::SageMaker::Device.Device`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-device-device.html) property type.
    #[derive(Debug, Default)]
    pub struct Device {
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-device-device.html#cfn-sagemaker-device-device-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`DeviceName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-device-device.html#cfn-sagemaker-device-device-devicename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub device_name: ::Value<String>,
        /// Property [`IotThingName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-device-device.html#cfn-sagemaker-device-device-iotthingname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub iot_thing_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for Device {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DeviceName", &self.device_name)?;
            if let Some(ref iot_thing_name) = self.iot_thing_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "IotThingName", iot_thing_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Device {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Device, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Device;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Device")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut description: Option<::Value<String>> = None;
                    let mut device_name: Option<::Value<String>> = None;
                    let mut iot_thing_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DeviceName" => {
                                device_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "IotThingName" => {
                                iot_thing_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Device {
                        description: description,
                        device_name: device_name.ok_or(::serde::de::Error::missing_field("DeviceName"))?,
                        iot_thing_name: iot_thing_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod device_fleet {
    //! Property types for the `DeviceFleet` resource.

    /// The [`AWS::SageMaker::DeviceFleet.EdgeOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-devicefleet-edgeoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct EdgeOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-devicefleet-edgeoutputconfig.html#cfn-sagemaker-devicefleet-edgeoutputconfig-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`S3OutputLocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-devicefleet-edgeoutputconfig.html#cfn-sagemaker-devicefleet-edgeoutputconfig-s3outputlocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_location: ::Value<String>,
    }

    impl ::codec::SerializeValue for EdgeOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputLocation", &self.s3_output_location)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EdgeOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EdgeOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EdgeOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EdgeOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut s3_output_location: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputLocation" => {
                                s3_output_location = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EdgeOutputConfig {
                        kms_key_id: kms_key_id,
                        s3_output_location: s3_output_location.ok_or(::serde::de::Error::missing_field("S3OutputLocation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod domain {
    //! Property types for the `Domain` resource.

    /// The [`AWS::SageMaker::Domain.CustomImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customimage.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomImage {
        /// Property [`AppImageConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customimage.html#cfn-sagemaker-domain-customimage-appimageconfigname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_image_config_name: ::Value<String>,
        /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customimage.html#cfn-sagemaker-domain-customimage-imagename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_name: ::Value<String>,
        /// Property [`ImageVersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-customimage.html#cfn-sagemaker-domain-customimage-imageversionnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_version_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CustomImage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppImageConfigName", &self.app_image_config_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", &self.image_name)?;
            if let Some(ref image_version_number) = self.image_version_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageVersionNumber", image_version_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomImage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomImage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomImage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomImage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_image_config_name: Option<::Value<String>> = None;
                    let mut image_name: Option<::Value<String>> = None;
                    let mut image_version_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppImageConfigName" => {
                                app_image_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageName" => {
                                image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageVersionNumber" => {
                                image_version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomImage {
                        app_image_config_name: app_image_config_name.ok_or(::serde::de::Error::missing_field("AppImageConfigName"))?,
                        image_name: image_name.ok_or(::serde::de::Error::missing_field("ImageName"))?,
                        image_version_number: image_version_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.DomainSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-domainsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct DomainSettings {
        /// Property [`RStudioServerProDomainSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-domainsettings.html#cfn-sagemaker-domain-domainsettings-rstudioserverprodomainsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_studio_server_pro_domain_settings: Option<::Value<RStudioServerProDomainSettings>>,
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-domainsettings.html#cfn-sagemaker-domain-domainsettings-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for DomainSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref r_studio_server_pro_domain_settings) = self.r_studio_server_pro_domain_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RStudioServerProDomainSettings", r_studio_server_pro_domain_settings)?;
            }
            if let Some(ref security_group_ids) = self.security_group_ids {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", security_group_ids)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DomainSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DomainSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DomainSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DomainSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r_studio_server_pro_domain_settings: Option<::Value<RStudioServerProDomainSettings>> = None;
                    let mut security_group_ids: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RStudioServerProDomainSettings" => {
                                r_studio_server_pro_domain_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DomainSettings {
                        r_studio_server_pro_domain_settings: r_studio_server_pro_domain_settings,
                        security_group_ids: security_group_ids,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.JupyterServerAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-jupyterserverappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct JupyterServerAppSettings {
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-jupyterserverappsettings.html#cfn-sagemaker-domain-jupyterserverappsettings-defaultresourcespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
    }

    impl ::codec::SerializeValue for JupyterServerAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JupyterServerAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JupyterServerAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JupyterServerAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JupyterServerAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JupyterServerAppSettings {
                        default_resource_spec: default_resource_spec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.KernelGatewayAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-kernelgatewayappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct KernelGatewayAppSettings {
        /// Property [`CustomImages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-kernelgatewayappsettings.html#cfn-sagemaker-domain-kernelgatewayappsettings-customimages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_images: Option<::ValueList<CustomImage>>,
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-kernelgatewayappsettings.html#cfn-sagemaker-domain-kernelgatewayappsettings-defaultresourcespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
    }

    impl ::codec::SerializeValue for KernelGatewayAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_images) = self.custom_images {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomImages", custom_images)?;
            }
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KernelGatewayAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KernelGatewayAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KernelGatewayAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KernelGatewayAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_images: Option<::ValueList<CustomImage>> = None;
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomImages" => {
                                custom_images = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KernelGatewayAppSettings {
                        custom_images: custom_images,
                        default_resource_spec: default_resource_spec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.RSessionAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rsessionappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RSessionAppSettings {
        /// Property [`CustomImages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rsessionappsettings.html#cfn-sagemaker-domain-rsessionappsettings-customimages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_images: Option<::ValueList<CustomImage>>,
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rsessionappsettings.html#cfn-sagemaker-domain-rsessionappsettings-defaultresourcespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
    }

    impl ::codec::SerializeValue for RSessionAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_images) = self.custom_images {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomImages", custom_images)?;
            }
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RSessionAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RSessionAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RSessionAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RSessionAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_images: Option<::ValueList<CustomImage>> = None;
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomImages" => {
                                custom_images = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RSessionAppSettings {
                        custom_images: custom_images,
                        default_resource_spec: default_resource_spec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.RStudioServerProAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverproappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RStudioServerProAppSettings {
        /// Property [`AccessStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverproappsettings.html#cfn-sagemaker-domain-rstudioserverproappsettings-accessstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub access_status: Option<::Value<String>>,
        /// Property [`UserGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverproappsettings.html#cfn-sagemaker-domain-rstudioserverproappsettings-usergroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RStudioServerProAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_status) = self.access_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessStatus", access_status)?;
            }
            if let Some(ref user_group) = self.user_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserGroup", user_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RStudioServerProAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RStudioServerProAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RStudioServerProAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RStudioServerProAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_status: Option<::Value<String>> = None;
                    let mut user_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessStatus" => {
                                access_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserGroup" => {
                                user_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RStudioServerProAppSettings {
                        access_status: access_status,
                        user_group: user_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.RStudioServerProDomainSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RStudioServerProDomainSettings {
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html#cfn-sagemaker-domain-rstudioserverprodomainsettings-defaultresourcespec).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
        /// Property [`DomainExecutionRoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html#cfn-sagemaker-domain-rstudioserverprodomainsettings-domainexecutionrolearn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub domain_execution_role_arn: ::Value<String>,
        /// Property [`RStudioConnectUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html#cfn-sagemaker-domain-rstudioserverprodomainsettings-rstudioconnecturl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_studio_connect_url: Option<::Value<String>>,
        /// Property [`RStudioPackageManagerUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-rstudioserverprodomainsettings.html#cfn-sagemaker-domain-rstudioserverprodomainsettings-rstudiopackagemanagerurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_studio_package_manager_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RStudioServerProDomainSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainExecutionRoleArn", &self.domain_execution_role_arn)?;
            if let Some(ref r_studio_connect_url) = self.r_studio_connect_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RStudioConnectUrl", r_studio_connect_url)?;
            }
            if let Some(ref r_studio_package_manager_url) = self.r_studio_package_manager_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RStudioPackageManagerUrl", r_studio_package_manager_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RStudioServerProDomainSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RStudioServerProDomainSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RStudioServerProDomainSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RStudioServerProDomainSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;
                    let mut domain_execution_role_arn: Option<::Value<String>> = None;
                    let mut r_studio_connect_url: Option<::Value<String>> = None;
                    let mut r_studio_package_manager_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DomainExecutionRoleArn" => {
                                domain_execution_role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RStudioConnectUrl" => {
                                r_studio_connect_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RStudioPackageManagerUrl" => {
                                r_studio_package_manager_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RStudioServerProDomainSettings {
                        default_resource_spec: default_resource_spec,
                        domain_execution_role_arn: domain_execution_role_arn.ok_or(::serde::de::Error::missing_field("DomainExecutionRoleArn"))?,
                        r_studio_connect_url: r_studio_connect_url,
                        r_studio_package_manager_url: r_studio_package_manager_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.ResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceSpec {
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html#cfn-sagemaker-domain-resourcespec-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`LifecycleConfigArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html#cfn-sagemaker-domain-resourcespec-lifecycleconfigarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub lifecycle_config_arn: Option<::Value<String>>,
        /// Property [`SageMakerImageArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html#cfn-sagemaker-domain-resourcespec-sagemakerimagearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_arn: Option<::Value<String>>,
        /// Property [`SageMakerImageVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-resourcespec.html#cfn-sagemaker-domain-resourcespec-sagemakerimageversionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_version_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref lifecycle_config_arn) = self.lifecycle_config_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LifecycleConfigArn", lifecycle_config_arn)?;
            }
            if let Some(ref sage_maker_image_arn) = self.sage_maker_image_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageArn", sage_maker_image_arn)?;
            }
            if let Some(ref sage_maker_image_version_arn) = self.sage_maker_image_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageVersionArn", sage_maker_image_version_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut lifecycle_config_arn: Option<::Value<String>> = None;
                    let mut sage_maker_image_arn: Option<::Value<String>> = None;
                    let mut sage_maker_image_version_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LifecycleConfigArn" => {
                                lifecycle_config_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageArn" => {
                                sage_maker_image_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageVersionArn" => {
                                sage_maker_image_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceSpec {
                        instance_type: instance_type,
                        lifecycle_config_arn: lifecycle_config_arn,
                        sage_maker_image_arn: sage_maker_image_arn,
                        sage_maker_image_version_arn: sage_maker_image_version_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.SharingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-sharingsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SharingSettings {
        /// Property [`NotebookOutputOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-sharingsettings.html#cfn-sagemaker-domain-sharingsettings-notebookoutputoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notebook_output_option: Option<::Value<String>>,
        /// Property [`S3KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-sharingsettings.html#cfn-sagemaker-domain-sharingsettings-s3kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_kms_key_id: Option<::Value<String>>,
        /// Property [`S3OutputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-sharingsettings.html#cfn-sagemaker-domain-sharingsettings-s3outputpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SharingSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref notebook_output_option) = self.notebook_output_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotebookOutputOption", notebook_output_option)?;
            }
            if let Some(ref s3_kms_key_id) = self.s3_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KmsKeyId", s3_kms_key_id)?;
            }
            if let Some(ref s3_output_path) = self.s3_output_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputPath", s3_output_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SharingSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SharingSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SharingSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SharingSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notebook_output_option: Option<::Value<String>> = None;
                    let mut s3_kms_key_id: Option<::Value<String>> = None;
                    let mut s3_output_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotebookOutputOption" => {
                                notebook_output_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KmsKeyId" => {
                                s3_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputPath" => {
                                s3_output_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SharingSettings {
                        notebook_output_option: notebook_output_option,
                        s3_kms_key_id: s3_kms_key_id,
                        s3_output_path: s3_output_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Domain.UserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UserSettings {
        /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-executionrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role: Option<::Value<String>>,
        /// Property [`JupyterServerAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-jupyterserverappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jupyter_server_app_settings: Option<::Value<JupyterServerAppSettings>>,
        /// Property [`KernelGatewayAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-kernelgatewayappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kernel_gateway_app_settings: Option<::Value<KernelGatewayAppSettings>>,
        /// Property [`RSessionAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-rsessionappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_session_app_settings: Option<::Value<RSessionAppSettings>>,
        /// Property [`RStudioServerProAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-rstudioserverproappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_studio_server_pro_app_settings: Option<::Value<RStudioServerProAppSettings>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`SharingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-domain-usersettings.html#cfn-sagemaker-domain-usersettings-sharingsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sharing_settings: Option<::Value<SharingSettings>>,
    }

    impl ::codec::SerializeValue for UserSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref execution_role) = self.execution_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", execution_role)?;
            }
            if let Some(ref jupyter_server_app_settings) = self.jupyter_server_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JupyterServerAppSettings", jupyter_server_app_settings)?;
            }
            if let Some(ref kernel_gateway_app_settings) = self.kernel_gateway_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelGatewayAppSettings", kernel_gateway_app_settings)?;
            }
            if let Some(ref r_session_app_settings) = self.r_session_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RSessionAppSettings", r_session_app_settings)?;
            }
            if let Some(ref r_studio_server_pro_app_settings) = self.r_studio_server_pro_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RStudioServerProAppSettings", r_studio_server_pro_app_settings)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            if let Some(ref sharing_settings) = self.sharing_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharingSettings", sharing_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role: Option<::Value<String>> = None;
                    let mut jupyter_server_app_settings: Option<::Value<JupyterServerAppSettings>> = None;
                    let mut kernel_gateway_app_settings: Option<::Value<KernelGatewayAppSettings>> = None;
                    let mut r_session_app_settings: Option<::Value<RSessionAppSettings>> = None;
                    let mut r_studio_server_pro_app_settings: Option<::Value<RStudioServerProAppSettings>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut sharing_settings: Option<::Value<SharingSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRole" => {
                                execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JupyterServerAppSettings" => {
                                jupyter_server_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KernelGatewayAppSettings" => {
                                kernel_gateway_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RSessionAppSettings" => {
                                r_session_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RStudioServerProAppSettings" => {
                                r_studio_server_pro_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharingSettings" => {
                                sharing_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserSettings {
                        execution_role: execution_role,
                        jupyter_server_app_settings: jupyter_server_app_settings,
                        kernel_gateway_app_settings: kernel_gateway_app_settings,
                        r_session_app_settings: r_session_app_settings,
                        r_studio_server_pro_app_settings: r_studio_server_pro_app_settings,
                        security_groups: security_groups,
                        sharing_settings: sharing_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod endpoint {
    //! Property types for the `Endpoint` resource.

    /// The [`AWS::SageMaker::Endpoint.Alarm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-alarm.html) property type.
    #[derive(Debug, Default)]
    pub struct Alarm {
        /// Property [`AlarmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-alarm.html#cfn-sagemaker-endpoint-alarm-alarmname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarm_name: ::Value<String>,
    }

    impl ::codec::SerializeValue for Alarm {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlarmName", &self.alarm_name)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Alarm {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Alarm, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Alarm;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Alarm")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarm_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlarmName" => {
                                alarm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Alarm {
                        alarm_name: alarm_name.ok_or(::serde::de::Error::missing_field("AlarmName"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.AutoRollbackConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-autorollbackconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AutoRollbackConfig {
        /// Property [`Alarms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-autorollbackconfig.html#cfn-sagemaker-endpoint-autorollbackconfig-alarms).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub alarms: ::ValueList<Alarm>,
    }

    impl ::codec::SerializeValue for AutoRollbackConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Alarms", &self.alarms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AutoRollbackConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AutoRollbackConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AutoRollbackConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AutoRollbackConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut alarms: Option<::ValueList<Alarm>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Alarms" => {
                                alarms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AutoRollbackConfig {
                        alarms: alarms.ok_or(::serde::de::Error::missing_field("Alarms"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.BlueGreenUpdatePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-bluegreenupdatepolicy.html) property type.
    #[derive(Debug, Default)]
    pub struct BlueGreenUpdatePolicy {
        /// Property [`MaximumExecutionTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-bluegreenupdatepolicy.html#cfn-sagemaker-endpoint-bluegreenupdatepolicy-maximumexecutiontimeoutinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub maximum_execution_timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`TerminationWaitInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-bluegreenupdatepolicy.html#cfn-sagemaker-endpoint-bluegreenupdatepolicy-terminationwaitinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub termination_wait_in_seconds: Option<::Value<u32>>,
        /// Property [`TrafficRoutingConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-bluegreenupdatepolicy.html#cfn-sagemaker-endpoint-bluegreenupdatepolicy-trafficroutingconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub traffic_routing_configuration: ::Value<TrafficRoutingConfig>,
    }

    impl ::codec::SerializeValue for BlueGreenUpdatePolicy {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref maximum_execution_timeout_in_seconds) = self.maximum_execution_timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaximumExecutionTimeoutInSeconds", maximum_execution_timeout_in_seconds)?;
            }
            if let Some(ref termination_wait_in_seconds) = self.termination_wait_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TerminationWaitInSeconds", termination_wait_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TrafficRoutingConfiguration", &self.traffic_routing_configuration)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BlueGreenUpdatePolicy {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BlueGreenUpdatePolicy, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BlueGreenUpdatePolicy;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BlueGreenUpdatePolicy")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut maximum_execution_timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut termination_wait_in_seconds: Option<::Value<u32>> = None;
                    let mut traffic_routing_configuration: Option<::Value<TrafficRoutingConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaximumExecutionTimeoutInSeconds" => {
                                maximum_execution_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TerminationWaitInSeconds" => {
                                termination_wait_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TrafficRoutingConfiguration" => {
                                traffic_routing_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BlueGreenUpdatePolicy {
                        maximum_execution_timeout_in_seconds: maximum_execution_timeout_in_seconds,
                        termination_wait_in_seconds: termination_wait_in_seconds,
                        traffic_routing_configuration: traffic_routing_configuration.ok_or(::serde::de::Error::missing_field("TrafficRoutingConfiguration"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.CapacitySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-capacitysize.html) property type.
    #[derive(Debug, Default)]
    pub struct CapacitySize {
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-capacitysize.html#cfn-sagemaker-endpoint-capacitysize-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`Value`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-capacitysize.html#cfn-sagemaker-endpoint-capacitysize-value).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub value: ::Value<u32>,
    }

    impl ::codec::SerializeValue for CapacitySize {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Value", &self.value)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CapacitySize {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CapacitySize, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CapacitySize;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CapacitySize")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut r#type: Option<::Value<String>> = None;
                    let mut value: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Value" => {
                                value = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CapacitySize {
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        value: value.ok_or(::serde::de::Error::missing_field("Value"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.DeploymentConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-deploymentconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DeploymentConfig {
        /// Property [`AutoRollbackConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-deploymentconfig.html#cfn-sagemaker-endpoint-deploymentconfig-autorollbackconfiguration).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub auto_rollback_configuration: Option<::Value<AutoRollbackConfig>>,
        /// Property [`BlueGreenUpdatePolicy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-deploymentconfig.html#cfn-sagemaker-endpoint-deploymentconfig-bluegreenupdatepolicy).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub blue_green_update_policy: ::Value<BlueGreenUpdatePolicy>,
    }

    impl ::codec::SerializeValue for DeploymentConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref auto_rollback_configuration) = self.auto_rollback_configuration {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AutoRollbackConfiguration", auto_rollback_configuration)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BlueGreenUpdatePolicy", &self.blue_green_update_policy)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DeploymentConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DeploymentConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DeploymentConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DeploymentConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut auto_rollback_configuration: Option<::Value<AutoRollbackConfig>> = None;
                    let mut blue_green_update_policy: Option<::Value<BlueGreenUpdatePolicy>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AutoRollbackConfiguration" => {
                                auto_rollback_configuration = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "BlueGreenUpdatePolicy" => {
                                blue_green_update_policy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DeploymentConfig {
                        auto_rollback_configuration: auto_rollback_configuration,
                        blue_green_update_policy: blue_green_update_policy.ok_or(::serde::de::Error::missing_field("BlueGreenUpdatePolicy"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.TrafficRoutingConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct TrafficRoutingConfig {
        /// Property [`CanarySize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html#cfn-sagemaker-endpoint-trafficroutingconfig-canarysize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub canary_size: Option<::Value<CapacitySize>>,
        /// Property [`LinearStepSize`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html#cfn-sagemaker-endpoint-trafficroutingconfig-linearstepsize).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub linear_step_size: Option<::Value<CapacitySize>>,
        /// Property [`Type`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html#cfn-sagemaker-endpoint-trafficroutingconfig-type).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r#type: ::Value<String>,
        /// Property [`WaitIntervalInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-trafficroutingconfig.html#cfn-sagemaker-endpoint-trafficroutingconfig-waitintervalinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub wait_interval_in_seconds: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for TrafficRoutingConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref canary_size) = self.canary_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CanarySize", canary_size)?;
            }
            if let Some(ref linear_step_size) = self.linear_step_size {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LinearStepSize", linear_step_size)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Type", &self.r#type)?;
            if let Some(ref wait_interval_in_seconds) = self.wait_interval_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "WaitIntervalInSeconds", wait_interval_in_seconds)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TrafficRoutingConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TrafficRoutingConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TrafficRoutingConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TrafficRoutingConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut canary_size: Option<::Value<CapacitySize>> = None;
                    let mut linear_step_size: Option<::Value<CapacitySize>> = None;
                    let mut r#type: Option<::Value<String>> = None;
                    let mut wait_interval_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CanarySize" => {
                                canary_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LinearStepSize" => {
                                linear_step_size = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Type" => {
                                r#type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "WaitIntervalInSeconds" => {
                                wait_interval_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TrafficRoutingConfig {
                        canary_size: canary_size,
                        linear_step_size: linear_step_size,
                        r#type: r#type.ok_or(::serde::de::Error::missing_field("Type"))?,
                        wait_interval_in_seconds: wait_interval_in_seconds,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Endpoint.VariantProperty`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-variantproperty.html) property type.
    #[derive(Debug, Default)]
    pub struct VariantProperty {
        /// Property [`VariantPropertyType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpoint-variantproperty.html#cfn-sagemaker-endpoint-variantproperty-variantpropertytype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub variant_property_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for VariantProperty {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref variant_property_type) = self.variant_property_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariantPropertyType", variant_property_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VariantProperty {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VariantProperty, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VariantProperty;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VariantProperty")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut variant_property_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "VariantPropertyType" => {
                                variant_property_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VariantProperty {
                        variant_property_type: variant_property_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod endpoint_config {
    //! Property types for the `EndpointConfig` resource.

    /// The [`AWS::SageMaker::EndpointConfig.AsyncInferenceClientConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceclientconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AsyncInferenceClientConfig {
        /// Property [`MaxConcurrentInvocationsPerInstance`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceclientconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceclientconfig-maxconcurrentinvocationsperinstance).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_concurrent_invocations_per_instance: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for AsyncInferenceClientConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref max_concurrent_invocations_per_instance) = self.max_concurrent_invocations_per_instance {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentInvocationsPerInstance", max_concurrent_invocations_per_instance)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AsyncInferenceClientConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AsyncInferenceClientConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AsyncInferenceClientConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AsyncInferenceClientConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_concurrent_invocations_per_instance: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConcurrentInvocationsPerInstance" => {
                                max_concurrent_invocations_per_instance = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AsyncInferenceClientConfig {
                        max_concurrent_invocations_per_instance: max_concurrent_invocations_per_instance,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.AsyncInferenceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AsyncInferenceConfig {
        /// Property [`ClientConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceconfig-clientconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub client_config: Option<::Value<AsyncInferenceClientConfig>>,
        /// Property [`OutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceconfig-outputconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub output_config: ::Value<AsyncInferenceOutputConfig>,
    }

    impl ::codec::SerializeValue for AsyncInferenceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref client_config) = self.client_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClientConfig", client_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutputConfig", &self.output_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AsyncInferenceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AsyncInferenceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AsyncInferenceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AsyncInferenceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut client_config: Option<::Value<AsyncInferenceClientConfig>> = None;
                    let mut output_config: Option<::Value<AsyncInferenceOutputConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClientConfig" => {
                                client_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OutputConfig" => {
                                output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AsyncInferenceConfig {
                        client_config: client_config,
                        output_config: output_config.ok_or(::serde::de::Error::missing_field("OutputConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.AsyncInferenceNotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferencenotificationconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AsyncInferenceNotificationConfig {
        /// Property [`ErrorTopic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferencenotificationconfig.html#cfn-sagemaker-endpointconfig-asyncinferencenotificationconfig-errortopic).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub error_topic: Option<::Value<String>>,
        /// Property [`SuccessTopic`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferencenotificationconfig.html#cfn-sagemaker-endpointconfig-asyncinferencenotificationconfig-successtopic).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub success_topic: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for AsyncInferenceNotificationConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref error_topic) = self.error_topic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ErrorTopic", error_topic)?;
            }
            if let Some(ref success_topic) = self.success_topic {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SuccessTopic", success_topic)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AsyncInferenceNotificationConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AsyncInferenceNotificationConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AsyncInferenceNotificationConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AsyncInferenceNotificationConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut error_topic: Option<::Value<String>> = None;
                    let mut success_topic: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ErrorTopic" => {
                                error_topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SuccessTopic" => {
                                success_topic = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AsyncInferenceNotificationConfig {
                        error_topic: error_topic,
                        success_topic: success_topic,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.AsyncInferenceOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct AsyncInferenceOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceoutputconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceoutputconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`NotificationConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceoutputconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceoutputconfig-notificationconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub notification_config: Option<::Value<AsyncInferenceNotificationConfig>>,
        /// Property [`S3OutputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-asyncinferenceoutputconfig.html#cfn-sagemaker-endpointconfig-asyncinferenceoutputconfig-s3outputpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for AsyncInferenceOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            if let Some(ref notification_config) = self.notification_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationConfig", notification_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputPath", &self.s3_output_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AsyncInferenceOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AsyncInferenceOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AsyncInferenceOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AsyncInferenceOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut notification_config: Option<::Value<AsyncInferenceNotificationConfig>> = None;
                    let mut s3_output_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NotificationConfig" => {
                                notification_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputPath" => {
                                s3_output_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AsyncInferenceOutputConfig {
                        kms_key_id: kms_key_id,
                        notification_config: notification_config,
                        s3_output_path: s3_output_path.ok_or(::serde::de::Error::missing_field("S3OutputPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.CaptureContentTypeHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptureContentTypeHeader {
        /// Property [`CsvContentTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader.html#cfn-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader-csvcontenttypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub csv_content_types: Option<::ValueList<String>>,
        /// Property [`JsonContentTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader.html#cfn-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader-jsoncontenttypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub json_content_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for CaptureContentTypeHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref csv_content_types) = self.csv_content_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CsvContentTypes", csv_content_types)?;
            }
            if let Some(ref json_content_types) = self.json_content_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JsonContentTypes", json_content_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptureContentTypeHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptureContentTypeHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptureContentTypeHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptureContentTypeHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut csv_content_types: Option<::ValueList<String>> = None;
                    let mut json_content_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CsvContentTypes" => {
                                csv_content_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JsonContentTypes" => {
                                json_content_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptureContentTypeHeader {
                        csv_content_types: csv_content_types,
                        json_content_types: json_content_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.CaptureOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-captureoption.html) property type.
    #[derive(Debug, Default)]
    pub struct CaptureOption {
        /// Property [`CaptureMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-captureoption.html#cfn-sagemaker-endpointconfig-captureoption-capturemode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub capture_mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for CaptureOption {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureMode", &self.capture_mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CaptureOption {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CaptureOption, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CaptureOption;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CaptureOption")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capture_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptureMode" => {
                                capture_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CaptureOption {
                        capture_mode: capture_mode.ok_or(::serde::de::Error::missing_field("CaptureMode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyExplainerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyexplainerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyExplainerConfig {
        /// Property [`EnableExplanations`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyexplainerconfig.html#cfn-sagemaker-endpointconfig-clarifyexplainerconfig-enableexplanations).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_explanations: Option<::Value<String>>,
        /// Property [`InferenceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyexplainerconfig.html#cfn-sagemaker-endpointconfig-clarifyexplainerconfig-inferenceconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inference_config: Option<::Value<ClarifyInferenceConfig>>,
        /// Property [`ShapConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyexplainerconfig.html#cfn-sagemaker-endpointconfig-clarifyexplainerconfig-shapconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub shap_config: ::Value<ClarifyShapConfig>,
    }

    impl ::codec::SerializeValue for ClarifyExplainerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_explanations) = self.enable_explanations {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableExplanations", enable_explanations)?;
            }
            if let Some(ref inference_config) = self.inference_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceConfig", inference_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShapConfig", &self.shap_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyExplainerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyExplainerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyExplainerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyExplainerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_explanations: Option<::Value<String>> = None;
                    let mut inference_config: Option<::Value<ClarifyInferenceConfig>> = None;
                    let mut shap_config: Option<::Value<ClarifyShapConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableExplanations" => {
                                enable_explanations = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceConfig" => {
                                inference_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShapConfig" => {
                                shap_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClarifyExplainerConfig {
                        enable_explanations: enable_explanations,
                        inference_config: inference_config,
                        shap_config: shap_config.ok_or(::serde::de::Error::missing_field("ShapConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyFeatureType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyfeaturetype.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyFeatureType {
    }

    impl ::codec::SerializeValue for ClarifyFeatureType {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyFeatureType {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyFeatureType, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyFeatureType;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyFeatureType")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ClarifyFeatureType {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyheader.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyHeader {
    }

    impl ::codec::SerializeValue for ClarifyHeader {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyHeader {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyHeader, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyHeader;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyHeader")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(ClarifyHeader {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyInferenceConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyInferenceConfig {
        /// Property [`ContentTemplate`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-contenttemplate).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_template: Option<::Value<String>>,
        /// Property [`FeatureHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-featureheaders).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_headers: Option<::ValueList<ClarifyHeader>>,
        /// Property [`FeatureTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-featuretypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_types: Option<::ValueList<ClarifyFeatureType>>,
        /// Property [`FeaturesAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-featuresattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub features_attribute: Option<::Value<String>>,
        /// Property [`LabelAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-labelattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub label_attribute: Option<::Value<String>>,
        /// Property [`LabelHeaders`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-labelheaders).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub label_headers: Option<::ValueList<ClarifyHeader>>,
        /// Property [`LabelIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-labelindex).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub label_index: Option<::Value<u32>>,
        /// Property [`MaxPayloadInMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-maxpayloadinmb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_payload_in_mb: Option<::Value<u32>>,
        /// Property [`MaxRecordCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-maxrecordcount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_record_count: Option<::Value<u32>>,
        /// Property [`ProbabilityAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-probabilityattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_attribute: Option<::Value<String>>,
        /// Property [`ProbabilityIndex`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyinferenceconfig.html#cfn-sagemaker-endpointconfig-clarifyinferenceconfig-probabilityindex).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_index: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ClarifyInferenceConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_template) = self.content_template {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentTemplate", content_template)?;
            }
            if let Some(ref feature_headers) = self.feature_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureHeaders", feature_headers)?;
            }
            if let Some(ref feature_types) = self.feature_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureTypes", feature_types)?;
            }
            if let Some(ref features_attribute) = self.features_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeaturesAttribute", features_attribute)?;
            }
            if let Some(ref label_attribute) = self.label_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelAttribute", label_attribute)?;
            }
            if let Some(ref label_headers) = self.label_headers {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelHeaders", label_headers)?;
            }
            if let Some(ref label_index) = self.label_index {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "LabelIndex", label_index)?;
            }
            if let Some(ref max_payload_in_mb) = self.max_payload_in_mb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxPayloadInMB", max_payload_in_mb)?;
            }
            if let Some(ref max_record_count) = self.max_record_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRecordCount", max_record_count)?;
            }
            if let Some(ref probability_attribute) = self.probability_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityAttribute", probability_attribute)?;
            }
            if let Some(ref probability_index) = self.probability_index {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityIndex", probability_index)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyInferenceConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyInferenceConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyInferenceConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyInferenceConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_template: Option<::Value<String>> = None;
                    let mut feature_headers: Option<::ValueList<ClarifyHeader>> = None;
                    let mut feature_types: Option<::ValueList<ClarifyFeatureType>> = None;
                    let mut features_attribute: Option<::Value<String>> = None;
                    let mut label_attribute: Option<::Value<String>> = None;
                    let mut label_headers: Option<::ValueList<ClarifyHeader>> = None;
                    let mut label_index: Option<::Value<u32>> = None;
                    let mut max_payload_in_mb: Option<::Value<u32>> = None;
                    let mut max_record_count: Option<::Value<u32>> = None;
                    let mut probability_attribute: Option<::Value<String>> = None;
                    let mut probability_index: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentTemplate" => {
                                content_template = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureHeaders" => {
                                feature_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureTypes" => {
                                feature_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeaturesAttribute" => {
                                features_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelAttribute" => {
                                label_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelHeaders" => {
                                label_headers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LabelIndex" => {
                                label_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxPayloadInMB" => {
                                max_payload_in_mb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxRecordCount" => {
                                max_record_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityAttribute" => {
                                probability_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityIndex" => {
                                probability_index = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClarifyInferenceConfig {
                        content_template: content_template,
                        feature_headers: feature_headers,
                        feature_types: feature_types,
                        features_attribute: features_attribute,
                        label_attribute: label_attribute,
                        label_headers: label_headers,
                        label_index: label_index,
                        max_payload_in_mb: max_payload_in_mb,
                        max_record_count: max_record_count,
                        probability_attribute: probability_attribute,
                        probability_index: probability_index,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyShapBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapbaselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyShapBaselineConfig {
        /// Property [`MimeType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapbaselineconfig.html#cfn-sagemaker-endpointconfig-clarifyshapbaselineconfig-mimetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mime_type: Option<::Value<String>>,
        /// Property [`ShapBaseline`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapbaselineconfig.html#cfn-sagemaker-endpointconfig-clarifyshapbaselineconfig-shapbaseline).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub shap_baseline: Option<::Value<String>>,
        /// Property [`ShapBaselineUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapbaselineconfig.html#cfn-sagemaker-endpointconfig-clarifyshapbaselineconfig-shapbaselineuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub shap_baseline_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ClarifyShapBaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref mime_type) = self.mime_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MimeType", mime_type)?;
            }
            if let Some(ref shap_baseline) = self.shap_baseline {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShapBaseline", shap_baseline)?;
            }
            if let Some(ref shap_baseline_uri) = self.shap_baseline_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShapBaselineUri", shap_baseline_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyShapBaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyShapBaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyShapBaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyShapBaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mime_type: Option<::Value<String>> = None;
                    let mut shap_baseline: Option<::Value<String>> = None;
                    let mut shap_baseline_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MimeType" => {
                                mime_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShapBaseline" => {
                                shap_baseline = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShapBaselineUri" => {
                                shap_baseline_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClarifyShapBaselineConfig {
                        mime_type: mime_type,
                        shap_baseline: shap_baseline,
                        shap_baseline_uri: shap_baseline_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyShapConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyShapConfig {
        /// Property [`NumberOfSamples`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html#cfn-sagemaker-endpointconfig-clarifyshapconfig-numberofsamples).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub number_of_samples: Option<::Value<u32>>,
        /// Property [`Seed`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html#cfn-sagemaker-endpointconfig-clarifyshapconfig-seed).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub seed: Option<::Value<u32>>,
        /// Property [`ShapBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html#cfn-sagemaker-endpointconfig-clarifyshapconfig-shapbaselineconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub shap_baseline_config: ::Value<ClarifyShapBaselineConfig>,
        /// Property [`TextConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html#cfn-sagemaker-endpointconfig-clarifyshapconfig-textconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub text_config: Option<::Value<ClarifyTextConfig>>,
        /// Property [`UseLogit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifyshapconfig.html#cfn-sagemaker-endpointconfig-clarifyshapconfig-uselogit).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub use_logit: Option<::Value<bool>>,
    }

    impl ::codec::SerializeValue for ClarifyShapConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref number_of_samples) = self.number_of_samples {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NumberOfSamples", number_of_samples)?;
            }
            if let Some(ref seed) = self.seed {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Seed", seed)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ShapBaselineConfig", &self.shap_baseline_config)?;
            if let Some(ref text_config) = self.text_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "TextConfig", text_config)?;
            }
            if let Some(ref use_logit) = self.use_logit {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UseLogit", use_logit)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyShapConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyShapConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyShapConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyShapConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut number_of_samples: Option<::Value<u32>> = None;
                    let mut seed: Option<::Value<u32>> = None;
                    let mut shap_baseline_config: Option<::Value<ClarifyShapBaselineConfig>> = None;
                    let mut text_config: Option<::Value<ClarifyTextConfig>> = None;
                    let mut use_logit: Option<::Value<bool>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NumberOfSamples" => {
                                number_of_samples = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Seed" => {
                                seed = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ShapBaselineConfig" => {
                                shap_baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TextConfig" => {
                                text_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UseLogit" => {
                                use_logit = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClarifyShapConfig {
                        number_of_samples: number_of_samples,
                        seed: seed,
                        shap_baseline_config: shap_baseline_config.ok_or(::serde::de::Error::missing_field("ShapBaselineConfig"))?,
                        text_config: text_config,
                        use_logit: use_logit,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ClarifyTextConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifytextconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClarifyTextConfig {
        /// Property [`Granularity`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifytextconfig.html#cfn-sagemaker-endpointconfig-clarifytextconfig-granularity).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub granularity: ::Value<String>,
        /// Property [`Language`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-clarifytextconfig.html#cfn-sagemaker-endpointconfig-clarifytextconfig-language).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub language: ::Value<String>,
    }

    impl ::codec::SerializeValue for ClarifyTextConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Granularity", &self.granularity)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Language", &self.language)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClarifyTextConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClarifyTextConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClarifyTextConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClarifyTextConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut granularity: Option<::Value<String>> = None;
                    let mut language: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Granularity" => {
                                granularity = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Language" => {
                                language = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClarifyTextConfig {
                        granularity: granularity.ok_or(::serde::de::Error::missing_field("Granularity"))?,
                        language: language.ok_or(::serde::de::Error::missing_field("Language"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.DataCaptureConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DataCaptureConfig {
        /// Property [`CaptureContentTypeHeader`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-capturecontenttypeheader).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub capture_content_type_header: Option<::Value<CaptureContentTypeHeader>>,
        /// Property [`CaptureOptions`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-captureoptions).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub capture_options: ::ValueList<CaptureOption>,
        /// Property [`DestinationS3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-destinations3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub destination_s3_uri: ::Value<String>,
        /// Property [`EnableCapture`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-enablecapture).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_capture: Option<::Value<bool>>,
        /// Property [`InitialSamplingPercentage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-initialsamplingpercentage).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub initial_sampling_percentage: ::Value<u32>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-datacaptureconfig.html#cfn-sagemaker-endpointconfig-datacaptureconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for DataCaptureConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref capture_content_type_header) = self.capture_content_type_header {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureContentTypeHeader", capture_content_type_header)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CaptureOptions", &self.capture_options)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DestinationS3Uri", &self.destination_s3_uri)?;
            if let Some(ref enable_capture) = self.enable_capture {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableCapture", enable_capture)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialSamplingPercentage", &self.initial_sampling_percentage)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataCaptureConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataCaptureConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataCaptureConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataCaptureConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut capture_content_type_header: Option<::Value<CaptureContentTypeHeader>> = None;
                    let mut capture_options: Option<::ValueList<CaptureOption>> = None;
                    let mut destination_s3_uri: Option<::Value<String>> = None;
                    let mut enable_capture: Option<::Value<bool>> = None;
                    let mut initial_sampling_percentage: Option<::Value<u32>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CaptureContentTypeHeader" => {
                                capture_content_type_header = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CaptureOptions" => {
                                capture_options = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DestinationS3Uri" => {
                                destination_s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableCapture" => {
                                enable_capture = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitialSamplingPercentage" => {
                                initial_sampling_percentage = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataCaptureConfig {
                        capture_content_type_header: capture_content_type_header,
                        capture_options: capture_options.ok_or(::serde::de::Error::missing_field("CaptureOptions"))?,
                        destination_s3_uri: destination_s3_uri.ok_or(::serde::de::Error::missing_field("DestinationS3Uri"))?,
                        enable_capture: enable_capture,
                        initial_sampling_percentage: initial_sampling_percentage.ok_or(::serde::de::Error::missing_field("InitialSamplingPercentage"))?,
                        kms_key_id: kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ExplainerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-explainerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ExplainerConfig {
        /// Property [`ClarifyExplainerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-explainerconfig.html#cfn-sagemaker-endpointconfig-explainerconfig-clarifyexplainerconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub clarify_explainer_config: Option<::Value<ClarifyExplainerConfig>>,
    }

    impl ::codec::SerializeValue for ExplainerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref clarify_explainer_config) = self.clarify_explainer_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClarifyExplainerConfig", clarify_explainer_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ExplainerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ExplainerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ExplainerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ExplainerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut clarify_explainer_config: Option<::Value<ClarifyExplainerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClarifyExplainerConfig" => {
                                clarify_explainer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ExplainerConfig {
                        clarify_explainer_config: clarify_explainer_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ProductionVariant`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html) property type.
    #[derive(Debug, Default)]
    pub struct ProductionVariant {
        /// Property [`AcceleratorType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-acceleratortype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub accelerator_type: Option<::Value<String>>,
        /// Property [`ContainerStartupHealthCheckTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-containerstartuphealthchecktimeoutinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_startup_health_check_timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`InitialInstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-initialinstancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub initial_instance_count: Option<::Value<u32>>,
        /// Property [`InitialVariantWeight`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-initialvariantweight).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub initial_variant_weight: ::Value<f64>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`ModelDataDownloadTimeoutInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-modeldatadownloadtimeoutinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_data_download_timeout_in_seconds: Option<::Value<u32>>,
        /// Property [`ModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-modelname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_name: ::Value<String>,
        /// Property [`ServerlessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-serverlessconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub serverless_config: Option<::Value<ServerlessConfig>>,
        /// Property [`VariantName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-variantname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub variant_name: ::Value<String>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant.html#cfn-sagemaker-endpointconfig-productionvariant-volumesizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size_in_gb: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for ProductionVariant {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accelerator_type) = self.accelerator_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AcceleratorType", accelerator_type)?;
            }
            if let Some(ref container_startup_health_check_timeout_in_seconds) = self.container_startup_health_check_timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerStartupHealthCheckTimeoutInSeconds", container_startup_health_check_timeout_in_seconds)?;
            }
            if let Some(ref initial_instance_count) = self.initial_instance_count {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialInstanceCount", initial_instance_count)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InitialVariantWeight", &self.initial_variant_weight)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref model_data_download_timeout_in_seconds) = self.model_data_download_timeout_in_seconds {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataDownloadTimeoutInSeconds", model_data_download_timeout_in_seconds)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelName", &self.model_name)?;
            if let Some(ref serverless_config) = self.serverless_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ServerlessConfig", serverless_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VariantName", &self.variant_name)?;
            if let Some(ref volume_size_in_gb) = self.volume_size_in_gb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", volume_size_in_gb)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProductionVariant {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProductionVariant, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProductionVariant;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProductionVariant")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accelerator_type: Option<::Value<String>> = None;
                    let mut container_startup_health_check_timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut initial_instance_count: Option<::Value<u32>> = None;
                    let mut initial_variant_weight: Option<::Value<f64>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut model_data_download_timeout_in_seconds: Option<::Value<u32>> = None;
                    let mut model_name: Option<::Value<String>> = None;
                    let mut serverless_config: Option<::Value<ServerlessConfig>> = None;
                    let mut variant_name: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AcceleratorType" => {
                                accelerator_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerStartupHealthCheckTimeoutInSeconds" => {
                                container_startup_health_check_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitialInstanceCount" => {
                                initial_instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InitialVariantWeight" => {
                                initial_variant_weight = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataDownloadTimeoutInSeconds" => {
                                model_data_download_timeout_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelName" => {
                                model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ServerlessConfig" => {
                                serverless_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VariantName" => {
                                variant_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProductionVariant {
                        accelerator_type: accelerator_type,
                        container_startup_health_check_timeout_in_seconds: container_startup_health_check_timeout_in_seconds,
                        initial_instance_count: initial_instance_count,
                        initial_variant_weight: initial_variant_weight.ok_or(::serde::de::Error::missing_field("InitialVariantWeight"))?,
                        instance_type: instance_type,
                        model_data_download_timeout_in_seconds: model_data_download_timeout_in_seconds,
                        model_name: model_name.ok_or(::serde::de::Error::missing_field("ModelName"))?,
                        serverless_config: serverless_config,
                        variant_name: variant_name.ok_or(::serde::de::Error::missing_field("VariantName"))?,
                        volume_size_in_gb: volume_size_in_gb,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::EndpointConfig.ServerlessConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-serverlessconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ServerlessConfig {
        /// Property [`MaxConcurrency`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-serverlessconfig.html#cfn-sagemaker-endpointconfig-productionvariant-serverlessconfig-maxconcurrency).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_concurrency: ::Value<u32>,
        /// Property [`MemorySizeInMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-endpointconfig-productionvariant-serverlessconfig.html#cfn-sagemaker-endpointconfig-productionvariant-serverlessconfig-memorysizeinmb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub memory_size_in_mb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ServerlessConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrency", &self.max_concurrency)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MemorySizeInMB", &self.memory_size_in_mb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ServerlessConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ServerlessConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ServerlessConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ServerlessConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_concurrency: Option<::Value<u32>> = None;
                    let mut memory_size_in_mb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxConcurrency" => {
                                max_concurrency = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MemorySizeInMB" => {
                                memory_size_in_mb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ServerlessConfig {
                        max_concurrency: max_concurrency.ok_or(::serde::de::Error::missing_field("MaxConcurrency"))?,
                        memory_size_in_mb: memory_size_in_mb.ok_or(::serde::de::Error::missing_field("MemorySizeInMB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod feature_group {
    //! Property types for the `FeatureGroup` resource.

    /// The [`AWS::SageMaker::FeatureGroup.FeatureDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-featuredefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct FeatureDefinition {
        /// Property [`FeatureName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-featuredefinition.html#cfn-sagemaker-featuregroup-featuredefinition-featurename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_name: ::Value<String>,
        /// Property [`FeatureType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-featuregroup-featuredefinition.html#cfn-sagemaker-featuregroup-featuredefinition-featuretype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub feature_type: ::Value<String>,
    }

    impl ::codec::SerializeValue for FeatureDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureName", &self.feature_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeatureType", &self.feature_type)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FeatureDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FeatureDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FeatureDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FeatureDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut feature_name: Option<::Value<String>> = None;
                    let mut feature_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FeatureName" => {
                                feature_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeatureType" => {
                                feature_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FeatureDefinition {
                        feature_name: feature_name.ok_or(::serde::de::Error::missing_field("FeatureName"))?,
                        feature_type: feature_type.ok_or(::serde::de::Error::missing_field("FeatureType"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod model {
    //! Property types for the `Model` resource.

    /// The [`AWS::SageMaker::Model.ContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ContainerDefinition {
        /// Property [`ContainerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-containerhostname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_hostname: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::Value<::json::Value>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-image).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image: Option<::Value<String>>,
        /// Property [`ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-imageconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_config: Option<::Value<ImageConfig>>,
        /// Property [`InferenceSpecificationName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-inferencespecificationname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inference_specification_name: Option<::Value<String>>,
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-mode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mode: Option<::Value<String>>,
        /// Property [`ModelDataUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-modeldataurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_data_url: Option<::Value<String>>,
        /// Property [`ModelPackageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-modelpackagename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_package_name: Option<::Value<String>>,
        /// Property [`MultiModelConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition.html#cfn-sagemaker-model-containerdefinition-multimodelconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub multi_model_config: Option<::Value<MultiModelConfig>>,
    }

    impl ::codec::SerializeValue for ContainerDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_hostname) = self.container_hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerHostname", container_hostname)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref image) = self.image {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", image)?;
            }
            if let Some(ref image_config) = self.image_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageConfig", image_config)?;
            }
            if let Some(ref inference_specification_name) = self.inference_specification_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceSpecificationName", inference_specification_name)?;
            }
            if let Some(ref mode) = self.mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", mode)?;
            }
            if let Some(ref model_data_url) = self.model_data_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataUrl", model_data_url)?;
            }
            if let Some(ref model_package_name) = self.model_package_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelPackageName", model_package_name)?;
            }
            if let Some(ref multi_model_config) = self.multi_model_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MultiModelConfig", multi_model_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ContainerDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ContainerDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ContainerDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ContainerDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_hostname: Option<::Value<String>> = None;
                    let mut environment: Option<::Value<::json::Value>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut image_config: Option<::Value<ImageConfig>> = None;
                    let mut inference_specification_name: Option<::Value<String>> = None;
                    let mut mode: Option<::Value<String>> = None;
                    let mut model_data_url: Option<::Value<String>> = None;
                    let mut model_package_name: Option<::Value<String>> = None;
                    let mut multi_model_config: Option<::Value<MultiModelConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerHostname" => {
                                container_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageConfig" => {
                                image_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceSpecificationName" => {
                                inference_specification_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataUrl" => {
                                model_data_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelPackageName" => {
                                model_package_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MultiModelConfig" => {
                                multi_model_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ContainerDefinition {
                        container_hostname: container_hostname,
                        environment: environment,
                        image: image,
                        image_config: image_config,
                        inference_specification_name: inference_specification_name,
                        mode: mode,
                        model_data_url: model_data_url,
                        model_package_name: model_package_name,
                        multi_model_config: multi_model_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Model.ImageConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ImageConfig {
        /// Property [`RepositoryAccessMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig.html#cfn-sagemaker-model-containerdefinition-imageconfig-repositoryaccessmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_access_mode: ::Value<String>,
        /// Property [`RepositoryAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig.html#cfn-sagemaker-model-containerdefinition-imageconfig-repositoryauthconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_auth_config: Option<::Value<RepositoryAuthConfig>>,
    }

    impl ::codec::SerializeValue for ImageConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryAccessMode", &self.repository_access_mode)?;
            if let Some(ref repository_auth_config) = self.repository_auth_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryAuthConfig", repository_auth_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ImageConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ImageConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ImageConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ImageConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut repository_access_mode: Option<::Value<String>> = None;
                    let mut repository_auth_config: Option<::Value<RepositoryAuthConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RepositoryAccessMode" => {
                                repository_access_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RepositoryAuthConfig" => {
                                repository_auth_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ImageConfig {
                        repository_access_mode: repository_access_mode.ok_or(::serde::de::Error::missing_field("RepositoryAccessMode"))?,
                        repository_auth_config: repository_auth_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Model.InferenceExecutionConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-inferenceexecutionconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct InferenceExecutionConfig {
        /// Property [`Mode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-inferenceexecutionconfig.html#cfn-sagemaker-model-inferenceexecutionconfig-mode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub mode: ::Value<String>,
    }

    impl ::codec::SerializeValue for InferenceExecutionConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Mode", &self.mode)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InferenceExecutionConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InferenceExecutionConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InferenceExecutionConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InferenceExecutionConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Mode" => {
                                mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InferenceExecutionConfig {
                        mode: mode.ok_or(::serde::de::Error::missing_field("Mode"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Model.MultiModelConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-multimodelconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MultiModelConfig {
        /// Property [`ModelCacheSetting`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-multimodelconfig.html#cfn-sagemaker-model-containerdefinition-multimodelconfig-modelcachesetting).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_cache_setting: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MultiModelConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref model_cache_setting) = self.model_cache_setting {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelCacheSetting", model_cache_setting)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MultiModelConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MultiModelConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MultiModelConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MultiModelConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut model_cache_setting: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ModelCacheSetting" => {
                                model_cache_setting = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MultiModelConfig {
                        model_cache_setting: model_cache_setting,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Model.RepositoryAuthConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig-repositoryauthconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct RepositoryAuthConfig {
        /// Property [`RepositoryCredentialsProviderArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-containerdefinition-imageconfig-repositoryauthconfig.html#cfn-sagemaker-model-containerdefinition-imageconfig-repositoryauthconfig-repositorycredentialsproviderarn).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository_credentials_provider_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for RepositoryAuthConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RepositoryCredentialsProviderArn", &self.repository_credentials_provider_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RepositoryAuthConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RepositoryAuthConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RepositoryAuthConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RepositoryAuthConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut repository_credentials_provider_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "RepositoryCredentialsProviderArn" => {
                                repository_credentials_provider_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RepositoryAuthConfig {
                        repository_credentials_provider_arn: repository_credentials_provider_arn.ok_or(::serde::de::Error::missing_field("RepositoryCredentialsProviderArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Model.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-vpcconfig.html#cfn-sagemaker-model-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-model-vpcconfig.html#cfn-sagemaker-model-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod model_bias_job_definition {
    //! Property types for the `ModelBiasJobDefinition` resource.

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html#cfn-sagemaker-modelbiasjobdefinition-clusterconfig-instancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html#cfn-sagemaker-modelbiasjobdefinition-clusterconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html#cfn-sagemaker-modelbiasjobdefinition-clusterconfig-volumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-clusterconfig.html#cfn-sagemaker-modelbiasjobdefinition-clusterconfig-volumesizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-constraintsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct ConstraintsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-constraintsresource.html#cfn-sagemaker-modelbiasjobdefinition-constraintsresource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConstraintsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConstraintsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConstraintsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConstraintsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConstraintsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConstraintsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointInput {
        /// Property [`EndTimeOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-endtimeoffset).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub end_time_offset: Option<::Value<String>>,
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-endpointname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_name: ::Value<String>,
        /// Property [`FeaturesAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-featuresattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub features_attribute: Option<::Value<String>>,
        /// Property [`InferenceAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-inferenceattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inference_attribute: Option<::Value<String>>,
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`ProbabilityAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-probabilityattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_attribute: Option<::Value<String>>,
        /// Property [`ProbabilityThresholdAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-probabilitythresholdattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_threshold_attribute: Option<::Value<f64>>,
        /// Property [`S3DataDistributionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-s3datadistributiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_distribution_type: Option<::Value<String>>,
        /// Property [`S3InputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-s3inputmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_input_mode: Option<::Value<String>>,
        /// Property [`StartTimeOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-endpointinput.html#cfn-sagemaker-modelbiasjobdefinition-endpointinput-starttimeoffset).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub start_time_offset: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_time_offset) = self.end_time_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeOffset", end_time_offset)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
            if let Some(ref features_attribute) = self.features_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeaturesAttribute", features_attribute)?;
            }
            if let Some(ref inference_attribute) = self.inference_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceAttribute", inference_attribute)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref probability_attribute) = self.probability_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityAttribute", probability_attribute)?;
            }
            if let Some(ref probability_threshold_attribute) = self.probability_threshold_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityThresholdAttribute", probability_threshold_attribute)?;
            }
            if let Some(ref s3_data_distribution_type) = self.s3_data_distribution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataDistributionType", s3_data_distribution_type)?;
            }
            if let Some(ref s3_input_mode) = self.s3_input_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputMode", s3_input_mode)?;
            }
            if let Some(ref start_time_offset) = self.start_time_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeOffset", start_time_offset)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time_offset: Option<::Value<String>> = None;
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut features_attribute: Option<::Value<String>> = None;
                    let mut inference_attribute: Option<::Value<String>> = None;
                    let mut local_path: Option<::Value<String>> = None;
                    let mut probability_attribute: Option<::Value<String>> = None;
                    let mut probability_threshold_attribute: Option<::Value<f64>> = None;
                    let mut s3_data_distribution_type: Option<::Value<String>> = None;
                    let mut s3_input_mode: Option<::Value<String>> = None;
                    let mut start_time_offset: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTimeOffset" => {
                                end_time_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeaturesAttribute" => {
                                features_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceAttribute" => {
                                inference_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityAttribute" => {
                                probability_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityThresholdAttribute" => {
                                probability_threshold_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DataDistributionType" => {
                                s3_data_distribution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputMode" => {
                                s3_input_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeOffset" => {
                                start_time_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointInput {
                        end_time_offset: end_time_offset,
                        endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                        features_attribute: features_attribute,
                        inference_attribute: inference_attribute,
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        probability_attribute: probability_attribute,
                        probability_threshold_attribute: probability_threshold_attribute,
                        s3_data_distribution_type: s3_data_distribution_type,
                        s3_input_mode: s3_input_mode,
                        start_time_offset: start_time_offset,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.ModelBiasAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasappspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelBiasAppSpecification {
        /// Property [`ConfigUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasappspecification.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasappspecification-configuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub config_uri: ::Value<String>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasappspecification.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasappspecification-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::ValueMap<String>>,
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasappspecification.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasappspecification-imageuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for ModelBiasAppSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigUri", &self.config_uri)?;
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", &self.image_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelBiasAppSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelBiasAppSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelBiasAppSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelBiasAppSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut config_uri: Option<::Value<String>> = None;
                    let mut environment: Option<::ValueMap<String>> = None;
                    let mut image_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigUri" => {
                                config_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelBiasAppSpecification {
                        config_uri: config_uri.ok_or(::serde::de::Error::missing_field("ConfigUri"))?,
                        environment: environment,
                        image_uri: image_uri.ok_or(::serde::de::Error::missing_field("ImageUri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.ModelBiasBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelBiasBaselineConfig {
        /// Property [`BaseliningJobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig-baseliningjobname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub baselining_job_name: Option<::Value<String>>,
        /// Property [`ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasbaselineconfig-constraintsresource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints_resource: Option<::Value<ConstraintsResource>>,
    }

    impl ::codec::SerializeValue for ModelBiasBaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref baselining_job_name) = self.baselining_job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseliningJobName", baselining_job_name)?;
            }
            if let Some(ref constraints_resource) = self.constraints_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstraintsResource", constraints_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelBiasBaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelBiasBaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelBiasBaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelBiasBaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut baselining_job_name: Option<::Value<String>> = None;
                    let mut constraints_resource: Option<::Value<ConstraintsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseliningJobName" => {
                                baselining_job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstraintsResource" => {
                                constraints_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelBiasBaselineConfig {
                        baselining_job_name: baselining_job_name,
                        constraints_resource: constraints_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.ModelBiasJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasjobinput.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelBiasJobInput {
        /// Property [`EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasjobinput.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasjobinput-endpointinput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_input: ::Value<EndpointInput>,
        /// Property [`GroundTruthS3Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-modelbiasjobinput.html#cfn-sagemaker-modelbiasjobdefinition-modelbiasjobinput-groundtruths3input).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ground_truth_s3_input: ::Value<MonitoringGroundTruthS3Input>,
    }

    impl ::codec::SerializeValue for ModelBiasJobInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointInput", &self.endpoint_input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroundTruthS3Input", &self.ground_truth_s3_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelBiasJobInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelBiasJobInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelBiasJobInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelBiasJobInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_input: Option<::Value<EndpointInput>> = None;
                    let mut ground_truth_s3_input: Option<::Value<MonitoringGroundTruthS3Input>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointInput" => {
                                endpoint_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroundTruthS3Input" => {
                                ground_truth_s3_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelBiasJobInput {
                        endpoint_input: endpoint_input.ok_or(::serde::de::Error::missing_field("EndpointInput"))?,
                        ground_truth_s3_input: ground_truth_s3_input.ok_or(::serde::de::Error::missing_field("GroundTruthS3Input"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.MonitoringGroundTruthS3Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringgroundtruths3input.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringGroundTruthS3Input {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringgroundtruths3input.html#cfn-sagemaker-modelbiasjobdefinition-monitoringgroundtruths3input-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for MonitoringGroundTruthS3Input {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringGroundTruthS3Input {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringGroundTruthS3Input, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringGroundTruthS3Input;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringGroundTruthS3Input")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringGroundTruthS3Input {
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.MonitoringOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutput {
        /// Property [`S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutput.html#cfn-sagemaker-modelbiasjobdefinition-monitoringoutput-s3output).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output: ::Value<S3Output>,
    }

    impl ::codec::SerializeValue for MonitoringOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Output", &self.s3_output)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_output: Option<::Value<S3Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Output" => {
                                s3_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutput {
                        s3_output: s3_output.ok_or(::serde::de::Error::missing_field("S3Output"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelbiasjobdefinition-monitoringoutputconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MonitoringOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelbiasjobdefinition-monitoringoutputconfig-monitoringoutputs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub monitoring_outputs: ::ValueList<MonitoringOutput>,
    }

    impl ::codec::SerializeValue for MonitoringOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputs", &self.monitoring_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut monitoring_outputs: Option<::ValueList<MonitoringOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputs" => {
                                monitoring_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutputConfig {
                        kms_key_id: kms_key_id,
                        monitoring_outputs: monitoring_outputs.ok_or(::serde::de::Error::missing_field("MonitoringOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringResources {
        /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-monitoringresources.html#cfn-sagemaker-modelbiasjobdefinition-monitoringresources-clusterconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cluster_config: ::Value<ClusterConfig>,
    }

    impl ::codec::SerializeValue for MonitoringResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", &self.cluster_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_config: Option<::Value<ClusterConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterConfig" => {
                                cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringResources {
                        cluster_config: cluster_config.ok_or(::serde::de::Error::missing_field("ClusterConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-networkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfig {
        /// Property [`EnableInterContainerTrafficEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-networkconfig.html#cfn-sagemaker-modelbiasjobdefinition-networkconfig-enableintercontainertrafficencryption).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_inter_container_traffic_encryption: Option<::Value<bool>>,
        /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-networkconfig.html#cfn-sagemaker-modelbiasjobdefinition-networkconfig-enablenetworkisolation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_network_isolation: Option<::Value<bool>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-networkconfig.html#cfn-sagemaker-modelbiasjobdefinition-networkconfig-vpcconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for NetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_inter_container_traffic_encryption) = self.enable_inter_container_traffic_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableInterContainerTrafficEncryption", enable_inter_container_traffic_encryption)?;
            }
            if let Some(ref enable_network_isolation) = self.enable_network_isolation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_inter_container_traffic_encryption: Option<::Value<bool>> = None;
                    let mut enable_network_isolation: Option<::Value<bool>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                enable_inter_container_traffic_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNetworkIsolation" => {
                                enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfig {
                        enable_inter_container_traffic_encryption: enable_inter_container_traffic_encryption,
                        enable_network_isolation: enable_network_isolation,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-s3output.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Output {
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-s3output.html#cfn-sagemaker-modelbiasjobdefinition-s3output-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3UploadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-s3output.html#cfn-sagemaker-modelbiasjobdefinition-s3output-s3uploadmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_upload_mode: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-s3output.html#cfn-sagemaker-modelbiasjobdefinition-s3output-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_upload_mode) = self.s3_upload_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3UploadMode", s3_upload_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_upload_mode: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3UploadMode" => {
                                s3_upload_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Output {
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_upload_mode: s3_upload_mode,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-stoppingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct StoppingCondition {
        /// Property [`MaxRuntimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-stoppingcondition.html#cfn-sagemaker-modelbiasjobdefinition-stoppingcondition-maxruntimeinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_runtime_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StoppingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuntimeInSeconds", &self.max_runtime_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoppingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoppingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoppingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoppingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_runtime_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                max_runtime_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoppingCondition {
                        max_runtime_in_seconds: max_runtime_in_seconds.ok_or(::serde::de::Error::missing_field("MaxRuntimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelBiasJobDefinition.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-vpcconfig.html#cfn-sagemaker-modelbiasjobdefinition-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelbiasjobdefinition-vpcconfig.html#cfn-sagemaker-modelbiasjobdefinition-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod model_explainability_job_definition {
    //! Property types for the `ModelExplainabilityJobDefinition` resource.

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-clusterconfig-instancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-clusterconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-clusterconfig-volumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-clusterconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-clusterconfig-volumesizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-constraintsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct ConstraintsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-constraintsresource.html#cfn-sagemaker-modelexplainabilityjobdefinition-constraintsresource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConstraintsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConstraintsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConstraintsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConstraintsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConstraintsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConstraintsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointInput {
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-endpointname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_name: ::Value<String>,
        /// Property [`FeaturesAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-featuresattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub features_attribute: Option<::Value<String>>,
        /// Property [`InferenceAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-inferenceattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inference_attribute: Option<::Value<String>>,
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`ProbabilityAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-probabilityattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_attribute: Option<::Value<String>>,
        /// Property [`S3DataDistributionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-s3datadistributiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_distribution_type: Option<::Value<String>>,
        /// Property [`S3InputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-endpointinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-endpointinput-s3inputmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_input_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
            if let Some(ref features_attribute) = self.features_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FeaturesAttribute", features_attribute)?;
            }
            if let Some(ref inference_attribute) = self.inference_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceAttribute", inference_attribute)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref probability_attribute) = self.probability_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityAttribute", probability_attribute)?;
            }
            if let Some(ref s3_data_distribution_type) = self.s3_data_distribution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataDistributionType", s3_data_distribution_type)?;
            }
            if let Some(ref s3_input_mode) = self.s3_input_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputMode", s3_input_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut features_attribute: Option<::Value<String>> = None;
                    let mut inference_attribute: Option<::Value<String>> = None;
                    let mut local_path: Option<::Value<String>> = None;
                    let mut probability_attribute: Option<::Value<String>> = None;
                    let mut s3_data_distribution_type: Option<::Value<String>> = None;
                    let mut s3_input_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FeaturesAttribute" => {
                                features_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceAttribute" => {
                                inference_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityAttribute" => {
                                probability_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DataDistributionType" => {
                                s3_data_distribution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputMode" => {
                                s3_input_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointInput {
                        endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                        features_attribute: features_attribute,
                        inference_attribute: inference_attribute,
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        probability_attribute: probability_attribute,
                        s3_data_distribution_type: s3_data_distribution_type,
                        s3_input_mode: s3_input_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelExplainabilityAppSpecification {
        /// Property [`ConfigUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification-configuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub config_uri: ::Value<String>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::ValueMap<String>>,
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityappspecification-imageuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for ModelExplainabilityAppSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigUri", &self.config_uri)?;
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", &self.image_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelExplainabilityAppSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelExplainabilityAppSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelExplainabilityAppSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelExplainabilityAppSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut config_uri: Option<::Value<String>> = None;
                    let mut environment: Option<::ValueMap<String>> = None;
                    let mut image_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigUri" => {
                                config_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelExplainabilityAppSpecification {
                        config_uri: config_uri.ok_or(::serde::de::Error::missing_field("ConfigUri"))?,
                        environment: environment,
                        image_uri: image_uri.ok_or(::serde::de::Error::missing_field("ImageUri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelExplainabilityBaselineConfig {
        /// Property [`BaseliningJobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig-baseliningjobname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub baselining_job_name: Option<::Value<String>>,
        /// Property [`ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilitybaselineconfig-constraintsresource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints_resource: Option<::Value<ConstraintsResource>>,
    }

    impl ::codec::SerializeValue for ModelExplainabilityBaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref baselining_job_name) = self.baselining_job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseliningJobName", baselining_job_name)?;
            }
            if let Some(ref constraints_resource) = self.constraints_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstraintsResource", constraints_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelExplainabilityBaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelExplainabilityBaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelExplainabilityBaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelExplainabilityBaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut baselining_job_name: Option<::Value<String>> = None;
                    let mut constraints_resource: Option<::Value<ConstraintsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseliningJobName" => {
                                baselining_job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstraintsResource" => {
                                constraints_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelExplainabilityBaselineConfig {
                        baselining_job_name: baselining_job_name,
                        constraints_resource: constraints_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.ModelExplainabilityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjobinput.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelExplainabilityJobInput {
        /// Property [`EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjobinput.html#cfn-sagemaker-modelexplainabilityjobdefinition-modelexplainabilityjobinput-endpointinput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_input: ::Value<EndpointInput>,
    }

    impl ::codec::SerializeValue for ModelExplainabilityJobInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointInput", &self.endpoint_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelExplainabilityJobInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelExplainabilityJobInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelExplainabilityJobInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelExplainabilityJobInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_input: Option<::Value<EndpointInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointInput" => {
                                endpoint_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelExplainabilityJobInput {
                        endpoint_input: endpoint_input.ok_or(::serde::de::Error::missing_field("EndpointInput"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutput {
        /// Property [`S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutput.html#cfn-sagemaker-modelexplainabilityjobdefinition-monitoringoutput-s3output).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output: ::Value<S3Output>,
    }

    impl ::codec::SerializeValue for MonitoringOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Output", &self.s3_output)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_output: Option<::Value<S3Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Output" => {
                                s3_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutput {
                        s3_output: s3_output.ok_or(::serde::de::Error::missing_field("S3Output"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MonitoringOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-monitoringoutputconfig-monitoringoutputs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub monitoring_outputs: ::ValueList<MonitoringOutput>,
    }

    impl ::codec::SerializeValue for MonitoringOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputs", &self.monitoring_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut monitoring_outputs: Option<::ValueList<MonitoringOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputs" => {
                                monitoring_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutputConfig {
                        kms_key_id: kms_key_id,
                        monitoring_outputs: monitoring_outputs.ok_or(::serde::de::Error::missing_field("MonitoringOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringResources {
        /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-monitoringresources.html#cfn-sagemaker-modelexplainabilityjobdefinition-monitoringresources-clusterconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cluster_config: ::Value<ClusterConfig>,
    }

    impl ::codec::SerializeValue for MonitoringResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", &self.cluster_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_config: Option<::Value<ClusterConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterConfig" => {
                                cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringResources {
                        cluster_config: cluster_config.ok_or(::serde::de::Error::missing_field("ClusterConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-networkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfig {
        /// Property [`EnableInterContainerTrafficEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-networkconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-networkconfig-enableintercontainertrafficencryption).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_inter_container_traffic_encryption: Option<::Value<bool>>,
        /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-networkconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-networkconfig-enablenetworkisolation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_network_isolation: Option<::Value<bool>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-networkconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-networkconfig-vpcconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for NetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_inter_container_traffic_encryption) = self.enable_inter_container_traffic_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableInterContainerTrafficEncryption", enable_inter_container_traffic_encryption)?;
            }
            if let Some(ref enable_network_isolation) = self.enable_network_isolation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_inter_container_traffic_encryption: Option<::Value<bool>> = None;
                    let mut enable_network_isolation: Option<::Value<bool>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                enable_inter_container_traffic_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNetworkIsolation" => {
                                enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfig {
                        enable_inter_container_traffic_encryption: enable_inter_container_traffic_encryption,
                        enable_network_isolation: enable_network_isolation,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-s3output.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Output {
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-s3output.html#cfn-sagemaker-modelexplainabilityjobdefinition-s3output-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3UploadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-s3output.html#cfn-sagemaker-modelexplainabilityjobdefinition-s3output-s3uploadmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_upload_mode: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-s3output.html#cfn-sagemaker-modelexplainabilityjobdefinition-s3output-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_upload_mode) = self.s3_upload_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3UploadMode", s3_upload_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_upload_mode: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3UploadMode" => {
                                s3_upload_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Output {
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_upload_mode: s3_upload_mode,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-stoppingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct StoppingCondition {
        /// Property [`MaxRuntimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-stoppingcondition.html#cfn-sagemaker-modelexplainabilityjobdefinition-stoppingcondition-maxruntimeinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_runtime_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StoppingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuntimeInSeconds", &self.max_runtime_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoppingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoppingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoppingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoppingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_runtime_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                max_runtime_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoppingCondition {
                        max_runtime_in_seconds: max_runtime_in_seconds.ok_or(::serde::de::Error::missing_field("MaxRuntimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelExplainabilityJobDefinition.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-vpcconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelexplainabilityjobdefinition-vpcconfig.html#cfn-sagemaker-modelexplainabilityjobdefinition-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod model_package {
    //! Property types for the `ModelPackage` resource.

    /// The [`AWS::SageMaker::ModelPackage.AdditionalInferenceSpecificationDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct AdditionalInferenceSpecificationDefinition {
        /// Property [`Containers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-containers).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub containers: ::ValueList<ModelPackageContainerDefinition>,
        /// Property [`Description`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-description).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub description: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`SupportedContentTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-supportedcontenttypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supported_content_types: Option<::ValueList<String>>,
        /// Property [`SupportedRealtimeInferenceInstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-supportedrealtimeinferenceinstancetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supported_realtime_inference_instance_types: Option<::ValueList<String>>,
        /// Property [`SupportedResponseMIMETypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-supportedresponsemimetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supported_response_mime_types: Option<::ValueList<String>>,
        /// Property [`SupportedTransformInstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-additionalinferencespecificationdefinition.html#cfn-sagemaker-modelpackage-additionalinferencespecificationdefinition-supportedtransforminstancetypes).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub supported_transform_instance_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for AdditionalInferenceSpecificationDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Containers", &self.containers)?;
            if let Some(ref description) = self.description {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Description", description)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            if let Some(ref supported_content_types) = self.supported_content_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedContentTypes", supported_content_types)?;
            }
            if let Some(ref supported_realtime_inference_instance_types) = self.supported_realtime_inference_instance_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedRealtimeInferenceInstanceTypes", supported_realtime_inference_instance_types)?;
            }
            if let Some(ref supported_response_mime_types) = self.supported_response_mime_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedResponseMIMETypes", supported_response_mime_types)?;
            }
            if let Some(ref supported_transform_instance_types) = self.supported_transform_instance_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedTransformInstanceTypes", supported_transform_instance_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for AdditionalInferenceSpecificationDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<AdditionalInferenceSpecificationDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = AdditionalInferenceSpecificationDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type AdditionalInferenceSpecificationDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut containers: Option<::ValueList<ModelPackageContainerDefinition>> = None;
                    let mut description: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut supported_content_types: Option<::ValueList<String>> = None;
                    let mut supported_realtime_inference_instance_types: Option<::ValueList<String>> = None;
                    let mut supported_response_mime_types: Option<::ValueList<String>> = None;
                    let mut supported_transform_instance_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Containers" => {
                                containers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Description" => {
                                description = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedContentTypes" => {
                                supported_content_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedRealtimeInferenceInstanceTypes" => {
                                supported_realtime_inference_instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedResponseMIMETypes" => {
                                supported_response_mime_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedTransformInstanceTypes" => {
                                supported_transform_instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(AdditionalInferenceSpecificationDefinition {
                        containers: containers.ok_or(::serde::de::Error::missing_field("Containers"))?,
                        description: description,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        supported_content_types: supported_content_types,
                        supported_realtime_inference_instance_types: supported_realtime_inference_instance_types,
                        supported_response_mime_types: supported_response_mime_types,
                        supported_transform_instance_types: supported_transform_instance_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.Bias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-bias.html) property type.
    #[derive(Debug, Default)]
    pub struct Bias {
        /// Property [`PostTrainingReport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-bias.html#cfn-sagemaker-modelpackage-bias-posttrainingreport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub post_training_report: Option<::Value<MetricsSource>>,
        /// Property [`PreTrainingReport`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-bias.html#cfn-sagemaker-modelpackage-bias-pretrainingreport).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pre_training_report: Option<::Value<MetricsSource>>,
        /// Property [`Report`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-bias.html#cfn-sagemaker-modelpackage-bias-report).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub report: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for Bias {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref post_training_report) = self.post_training_report {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostTrainingReport", post_training_report)?;
            }
            if let Some(ref pre_training_report) = self.pre_training_report {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreTrainingReport", pre_training_report)?;
            }
            if let Some(ref report) = self.report {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Report", report)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Bias {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Bias, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Bias;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Bias")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut post_training_report: Option<::Value<MetricsSource>> = None;
                    let mut pre_training_report: Option<::Value<MetricsSource>> = None;
                    let mut report: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PostTrainingReport" => {
                                post_training_report = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreTrainingReport" => {
                                pre_training_report = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Report" => {
                                report = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Bias {
                        post_training_report: post_training_report,
                        pre_training_report: pre_training_report,
                        report: report,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-datasource.html) property type.
    #[derive(Debug, Default)]
    pub struct DataSource {
        /// Property [`S3DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-datasource.html#cfn-sagemaker-modelpackage-datasource-s3datasource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_source: ::Value<S3DataSource>,
    }

    impl ::codec::SerializeValue for DataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataSource", &self.s3_data_source)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_data_source: Option<::Value<S3DataSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3DataSource" => {
                                s3_data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DataSource {
                        s3_data_source: s3_data_source.ok_or(::serde::de::Error::missing_field("S3DataSource"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DriftCheckBaselines`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html) property type.
    #[derive(Debug, Default)]
    pub struct DriftCheckBaselines {
        /// Property [`Bias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html#cfn-sagemaker-modelpackage-driftcheckbaselines-bias).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bias: Option<::Value<DriftCheckBias>>,
        /// Property [`Explainability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html#cfn-sagemaker-modelpackage-driftcheckbaselines-explainability).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub explainability: Option<::Value<DriftCheckExplainability>>,
        /// Property [`ModelDataQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html#cfn-sagemaker-modelpackage-driftcheckbaselines-modeldataquality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_data_quality: Option<::Value<DriftCheckModelDataQuality>>,
        /// Property [`ModelQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbaselines.html#cfn-sagemaker-modelpackage-driftcheckbaselines-modelquality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_quality: Option<::Value<DriftCheckModelQuality>>,
    }

    impl ::codec::SerializeValue for DriftCheckBaselines {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bias) = self.bias {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bias", bias)?;
            }
            if let Some(ref explainability) = self.explainability {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Explainability", explainability)?;
            }
            if let Some(ref model_data_quality) = self.model_data_quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataQuality", model_data_quality)?;
            }
            if let Some(ref model_quality) = self.model_quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQuality", model_quality)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DriftCheckBaselines {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DriftCheckBaselines, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DriftCheckBaselines;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DriftCheckBaselines")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bias: Option<::Value<DriftCheckBias>> = None;
                    let mut explainability: Option<::Value<DriftCheckExplainability>> = None;
                    let mut model_data_quality: Option<::Value<DriftCheckModelDataQuality>> = None;
                    let mut model_quality: Option<::Value<DriftCheckModelQuality>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bias" => {
                                bias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Explainability" => {
                                explainability = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataQuality" => {
                                model_data_quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelQuality" => {
                                model_quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DriftCheckBaselines {
                        bias: bias,
                        explainability: explainability,
                        model_data_quality: model_data_quality,
                        model_quality: model_quality,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DriftCheckBias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbias.html) property type.
    #[derive(Debug, Default)]
    pub struct DriftCheckBias {
        /// Property [`ConfigFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbias.html#cfn-sagemaker-modelpackage-driftcheckbias-configfile).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub config_file: Option<::Value<FileSource>>,
        /// Property [`PostTrainingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbias.html#cfn-sagemaker-modelpackage-driftcheckbias-posttrainingconstraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub post_training_constraints: Option<::Value<MetricsSource>>,
        /// Property [`PreTrainingConstraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckbias.html#cfn-sagemaker-modelpackage-driftcheckbias-pretrainingconstraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub pre_training_constraints: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for DriftCheckBias {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref config_file) = self.config_file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigFile", config_file)?;
            }
            if let Some(ref post_training_constraints) = self.post_training_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostTrainingConstraints", post_training_constraints)?;
            }
            if let Some(ref pre_training_constraints) = self.pre_training_constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PreTrainingConstraints", pre_training_constraints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DriftCheckBias {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DriftCheckBias, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DriftCheckBias;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DriftCheckBias")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut config_file: Option<::Value<FileSource>> = None;
                    let mut post_training_constraints: Option<::Value<MetricsSource>> = None;
                    let mut pre_training_constraints: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigFile" => {
                                config_file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostTrainingConstraints" => {
                                post_training_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PreTrainingConstraints" => {
                                pre_training_constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DriftCheckBias {
                        config_file: config_file,
                        post_training_constraints: post_training_constraints,
                        pre_training_constraints: pre_training_constraints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DriftCheckExplainability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckexplainability.html) property type.
    #[derive(Debug, Default)]
    pub struct DriftCheckExplainability {
        /// Property [`ConfigFile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckexplainability.html#cfn-sagemaker-modelpackage-driftcheckexplainability-configfile).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub config_file: Option<::Value<FileSource>>,
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckexplainability.html#cfn-sagemaker-modelpackage-driftcheckexplainability-constraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for DriftCheckExplainability {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref config_file) = self.config_file {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConfigFile", config_file)?;
            }
            if let Some(ref constraints) = self.constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DriftCheckExplainability {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DriftCheckExplainability, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DriftCheckExplainability;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DriftCheckExplainability")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut config_file: Option<::Value<FileSource>> = None;
                    let mut constraints: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConfigFile" => {
                                config_file = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DriftCheckExplainability {
                        config_file: config_file,
                        constraints: constraints,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DriftCheckModelDataQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodeldataquality.html) property type.
    #[derive(Debug, Default)]
    pub struct DriftCheckModelDataQuality {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodeldataquality.html#cfn-sagemaker-modelpackage-driftcheckmodeldataquality-constraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints: Option<::Value<MetricsSource>>,
        /// Property [`Statistics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodeldataquality.html#cfn-sagemaker-modelpackage-driftcheckmodeldataquality-statistics).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub statistics: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for DriftCheckModelDataQuality {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constraints) = self.constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
            }
            if let Some(ref statistics) = self.statistics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistics", statistics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DriftCheckModelDataQuality {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DriftCheckModelDataQuality, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DriftCheckModelDataQuality;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DriftCheckModelDataQuality")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints: Option<::Value<MetricsSource>> = None;
                    let mut statistics: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistics" => {
                                statistics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DriftCheckModelDataQuality {
                        constraints: constraints,
                        statistics: statistics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.DriftCheckModelQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodelquality.html) property type.
    #[derive(Debug, Default)]
    pub struct DriftCheckModelQuality {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodelquality.html#cfn-sagemaker-modelpackage-driftcheckmodelquality-constraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints: Option<::Value<MetricsSource>>,
        /// Property [`Statistics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-driftcheckmodelquality.html#cfn-sagemaker-modelpackage-driftcheckmodelquality-statistics).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub statistics: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for DriftCheckModelQuality {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constraints) = self.constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
            }
            if let Some(ref statistics) = self.statistics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistics", statistics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DriftCheckModelQuality {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DriftCheckModelQuality, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DriftCheckModelQuality;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DriftCheckModelQuality")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints: Option<::Value<MetricsSource>> = None;
                    let mut statistics: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistics" => {
                                statistics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DriftCheckModelQuality {
                        constraints: constraints,
                        statistics: statistics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-environment.html) property type.
    #[derive(Debug, Default)]
    pub struct Environment {
    }

    impl ::codec::SerializeValue for Environment {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Environment {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Environment, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Environment;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Environment")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, _map: A) -> Result<Self::Value, A::Error> {
                    Ok(Environment {})
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.Explainability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-explainability.html) property type.
    #[derive(Debug, Default)]
    pub struct Explainability {
        /// Property [`Report`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-explainability.html#cfn-sagemaker-modelpackage-explainability-report).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub report: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for Explainability {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref report) = self.report {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Report", report)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for Explainability {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<Explainability, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = Explainability;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type Explainability")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut report: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Report" => {
                                report = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(Explainability {
                        report: report,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.FileSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-filesource.html) property type.
    #[derive(Debug, Default)]
    pub struct FileSource {
        /// Property [`ContentDigest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-filesource.html#cfn-sagemaker-modelpackage-filesource-contentdigest).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_digest: Option<::Value<String>>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-filesource.html#cfn-sagemaker-modelpackage-filesource-contenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-filesource.html#cfn-sagemaker-modelpackage-filesource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for FileSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_digest) = self.content_digest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentDigest", content_digest)?;
            }
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for FileSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<FileSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = FileSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type FileSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_digest: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentDigest" => {
                                content_digest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(FileSource {
                        content_digest: content_digest,
                        content_type: content_type,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.InferenceSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html) property type.
    #[derive(Debug, Default)]
    pub struct InferenceSpecification {
        /// Property [`Containers`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html#cfn-sagemaker-modelpackage-inferencespecification-containers).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub containers: ::ValueList<ModelPackageContainerDefinition>,
        /// Property [`SupportedContentTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html#cfn-sagemaker-modelpackage-inferencespecification-supportedcontenttypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub supported_content_types: ::ValueList<String>,
        /// Property [`SupportedRealtimeInferenceInstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html#cfn-sagemaker-modelpackage-inferencespecification-supportedrealtimeinferenceinstancetypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub supported_realtime_inference_instance_types: Option<::ValueList<String>>,
        /// Property [`SupportedResponseMIMETypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html#cfn-sagemaker-modelpackage-inferencespecification-supportedresponsemimetypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub supported_response_mime_types: ::ValueList<String>,
        /// Property [`SupportedTransformInstanceTypes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-inferencespecification.html#cfn-sagemaker-modelpackage-inferencespecification-supportedtransforminstancetypes).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub supported_transform_instance_types: Option<::ValueList<String>>,
    }

    impl ::codec::SerializeValue for InferenceSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Containers", &self.containers)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedContentTypes", &self.supported_content_types)?;
            if let Some(ref supported_realtime_inference_instance_types) = self.supported_realtime_inference_instance_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedRealtimeInferenceInstanceTypes", supported_realtime_inference_instance_types)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedResponseMIMETypes", &self.supported_response_mime_types)?;
            if let Some(ref supported_transform_instance_types) = self.supported_transform_instance_types {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SupportedTransformInstanceTypes", supported_transform_instance_types)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InferenceSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InferenceSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InferenceSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InferenceSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut containers: Option<::ValueList<ModelPackageContainerDefinition>> = None;
                    let mut supported_content_types: Option<::ValueList<String>> = None;
                    let mut supported_realtime_inference_instance_types: Option<::ValueList<String>> = None;
                    let mut supported_response_mime_types: Option<::ValueList<String>> = None;
                    let mut supported_transform_instance_types: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Containers" => {
                                containers = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedContentTypes" => {
                                supported_content_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedRealtimeInferenceInstanceTypes" => {
                                supported_realtime_inference_instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedResponseMIMETypes" => {
                                supported_response_mime_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SupportedTransformInstanceTypes" => {
                                supported_transform_instance_types = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InferenceSpecification {
                        containers: containers.ok_or(::serde::de::Error::missing_field("Containers"))?,
                        supported_content_types: supported_content_types.ok_or(::serde::de::Error::missing_field("SupportedContentTypes"))?,
                        supported_realtime_inference_instance_types: supported_realtime_inference_instance_types,
                        supported_response_mime_types: supported_response_mime_types.ok_or(::serde::de::Error::missing_field("SupportedResponseMIMETypes"))?,
                        supported_transform_instance_types: supported_transform_instance_types,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.MetadataProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html) property type.
    #[derive(Debug, Default)]
    pub struct MetadataProperties {
        /// Property [`CommitId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html#cfn-sagemaker-modelpackage-metadataproperties-commitid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub commit_id: Option<::Value<String>>,
        /// Property [`GeneratedBy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html#cfn-sagemaker-modelpackage-metadataproperties-generatedby).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub generated_by: Option<::Value<String>>,
        /// Property [`ProjectId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html#cfn-sagemaker-modelpackage-metadataproperties-projectid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub project_id: Option<::Value<String>>,
        /// Property [`Repository`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metadataproperties.html#cfn-sagemaker-modelpackage-metadataproperties-repository).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub repository: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MetadataProperties {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref commit_id) = self.commit_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CommitId", commit_id)?;
            }
            if let Some(ref generated_by) = self.generated_by {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "GeneratedBy", generated_by)?;
            }
            if let Some(ref project_id) = self.project_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProjectId", project_id)?;
            }
            if let Some(ref repository) = self.repository {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Repository", repository)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetadataProperties {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetadataProperties, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetadataProperties;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetadataProperties")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut commit_id: Option<::Value<String>> = None;
                    let mut generated_by: Option<::Value<String>> = None;
                    let mut project_id: Option<::Value<String>> = None;
                    let mut repository: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CommitId" => {
                                commit_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GeneratedBy" => {
                                generated_by = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProjectId" => {
                                project_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Repository" => {
                                repository = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetadataProperties {
                        commit_id: commit_id,
                        generated_by: generated_by,
                        project_id: project_id,
                        repository: repository,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.MetricsSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metricssource.html) property type.
    #[derive(Debug, Default)]
    pub struct MetricsSource {
        /// Property [`ContentDigest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metricssource.html#cfn-sagemaker-modelpackage-metricssource-contentdigest).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_digest: Option<::Value<String>>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metricssource.html#cfn-sagemaker-modelpackage-metricssource-contenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_type: ::Value<String>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-metricssource.html#cfn-sagemaker-modelpackage-metricssource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for MetricsSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content_digest) = self.content_digest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentDigest", content_digest)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", &self.content_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MetricsSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MetricsSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MetricsSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MetricsSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content_digest: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContentDigest" => {
                                content_digest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MetricsSource {
                        content_digest: content_digest,
                        content_type: content_type.ok_or(::serde::de::Error::missing_field("ContentType"))?,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelDataQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modeldataquality.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelDataQuality {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modeldataquality.html#cfn-sagemaker-modelpackage-modeldataquality-constraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints: Option<::Value<MetricsSource>>,
        /// Property [`Statistics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modeldataquality.html#cfn-sagemaker-modelpackage-modeldataquality-statistics).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub statistics: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for ModelDataQuality {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constraints) = self.constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
            }
            if let Some(ref statistics) = self.statistics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistics", statistics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelDataQuality {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelDataQuality, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelDataQuality;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelDataQuality")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints: Option<::Value<MetricsSource>> = None;
                    let mut statistics: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistics" => {
                                statistics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelDataQuality {
                        constraints: constraints,
                        statistics: statistics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelMetrics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelMetrics {
        /// Property [`Bias`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html#cfn-sagemaker-modelpackage-modelmetrics-bias).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub bias: Option<::Value<Bias>>,
        /// Property [`Explainability`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html#cfn-sagemaker-modelpackage-modelmetrics-explainability).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub explainability: Option<::Value<Explainability>>,
        /// Property [`ModelDataQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html#cfn-sagemaker-modelpackage-modelmetrics-modeldataquality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_data_quality: Option<::Value<ModelDataQuality>>,
        /// Property [`ModelQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelmetrics.html#cfn-sagemaker-modelpackage-modelmetrics-modelquality).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_quality: Option<::Value<ModelQuality>>,
    }

    impl ::codec::SerializeValue for ModelMetrics {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref bias) = self.bias {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Bias", bias)?;
            }
            if let Some(ref explainability) = self.explainability {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Explainability", explainability)?;
            }
            if let Some(ref model_data_quality) = self.model_data_quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataQuality", model_data_quality)?;
            }
            if let Some(ref model_quality) = self.model_quality {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelQuality", model_quality)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelMetrics {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelMetrics, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelMetrics;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelMetrics")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bias: Option<::Value<Bias>> = None;
                    let mut explainability: Option<::Value<Explainability>> = None;
                    let mut model_data_quality: Option<::Value<ModelDataQuality>> = None;
                    let mut model_quality: Option<::Value<ModelQuality>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Bias" => {
                                bias = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Explainability" => {
                                explainability = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataQuality" => {
                                model_data_quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelQuality" => {
                                model_quality = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelMetrics {
                        bias: bias,
                        explainability: explainability,
                        model_data_quality: model_data_quality,
                        model_quality: model_quality,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelPackageContainerDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelPackageContainerDefinition {
        /// Property [`ContainerHostname`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-containerhostname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_hostname: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<::Value<Environment>>,
        /// Property [`Framework`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-framework).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework: Option<::Value<String>>,
        /// Property [`FrameworkVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-frameworkversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub framework_version: Option<::Value<String>>,
        /// Property [`Image`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-image).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image: ::Value<String>,
        /// Property [`ImageDigest`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-imagedigest).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_digest: Option<::Value<String>>,
        /// Property [`ModelDataUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-modeldataurl).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model_data_url: Option<::Value<String>>,
        /// Property [`ModelInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-modelinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub model_input: Option<::Value<::json::Value>>,
        /// Property [`NearestModelName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-nearestmodelname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub nearest_model_name: Option<::Value<String>>,
        /// Property [`ProductId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagecontainerdefinition.html#cfn-sagemaker-modelpackage-modelpackagecontainerdefinition-productid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub product_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ModelPackageContainerDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_hostname) = self.container_hostname {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerHostname", container_hostname)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref framework) = self.framework {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Framework", framework)?;
            }
            if let Some(ref framework_version) = self.framework_version {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FrameworkVersion", framework_version)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Image", &self.image)?;
            if let Some(ref image_digest) = self.image_digest {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageDigest", image_digest)?;
            }
            if let Some(ref model_data_url) = self.model_data_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataUrl", model_data_url)?;
            }
            if let Some(ref model_input) = self.model_input {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelInput", model_input)?;
            }
            if let Some(ref nearest_model_name) = self.nearest_model_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NearestModelName", nearest_model_name)?;
            }
            if let Some(ref product_id) = self.product_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProductId", product_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelPackageContainerDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelPackageContainerDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelPackageContainerDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelPackageContainerDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_hostname: Option<::Value<String>> = None;
                    let mut environment: Option<::Value<Environment>> = None;
                    let mut framework: Option<::Value<String>> = None;
                    let mut framework_version: Option<::Value<String>> = None;
                    let mut image: Option<::Value<String>> = None;
                    let mut image_digest: Option<::Value<String>> = None;
                    let mut model_data_url: Option<::Value<String>> = None;
                    let mut model_input: Option<::Value<::json::Value>> = None;
                    let mut nearest_model_name: Option<::Value<String>> = None;
                    let mut product_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerHostname" => {
                                container_hostname = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Framework" => {
                                framework = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FrameworkVersion" => {
                                framework_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Image" => {
                                image = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageDigest" => {
                                image_digest = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataUrl" => {
                                model_data_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelInput" => {
                                model_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NearestModelName" => {
                                nearest_model_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProductId" => {
                                product_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelPackageContainerDefinition {
                        container_hostname: container_hostname,
                        environment: environment,
                        framework: framework,
                        framework_version: framework_version,
                        image: image.ok_or(::serde::de::Error::missing_field("Image"))?,
                        image_digest: image_digest,
                        model_data_url: model_data_url,
                        model_input: model_input,
                        nearest_model_name: nearest_model_name,
                        product_id: product_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelPackageStatusDetails`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusdetails.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelPackageStatusDetails {
        /// Property [`ImageScanStatuses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusdetails.html#cfn-sagemaker-modelpackage-modelpackagestatusdetails-imagescanstatuses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_scan_statuses: Option<::ValueList<ModelPackageStatusItem>>,
        /// Property [`ValidationStatuses`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusdetails.html#cfn-sagemaker-modelpackage-modelpackagestatusdetails-validationstatuses).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub validation_statuses: ::ValueList<ModelPackageStatusItem>,
    }

    impl ::codec::SerializeValue for ModelPackageStatusDetails {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref image_scan_statuses) = self.image_scan_statuses {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageScanStatuses", image_scan_statuses)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationStatuses", &self.validation_statuses)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelPackageStatusDetails {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelPackageStatusDetails, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelPackageStatusDetails;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelPackageStatusDetails")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut image_scan_statuses: Option<::ValueList<ModelPackageStatusItem>> = None;
                    let mut validation_statuses: Option<::ValueList<ModelPackageStatusItem>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ImageScanStatuses" => {
                                image_scan_statuses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidationStatuses" => {
                                validation_statuses = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelPackageStatusDetails {
                        image_scan_statuses: image_scan_statuses,
                        validation_statuses: validation_statuses.ok_or(::serde::de::Error::missing_field("ValidationStatuses"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelPackageStatusItem`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusitem.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelPackageStatusItem {
        /// Property [`FailureReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusitem.html#cfn-sagemaker-modelpackage-modelpackagestatusitem-failurereason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_reason: Option<::Value<String>>,
        /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusitem.html#cfn-sagemaker-modelpackage-modelpackagestatusitem-name).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub name: ::Value<String>,
        /// Property [`Status`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelpackagestatusitem.html#cfn-sagemaker-modelpackage-modelpackagestatusitem-status).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub status: ::Value<String>,
    }

    impl ::codec::SerializeValue for ModelPackageStatusItem {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref failure_reason) = self.failure_reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureReason", failure_reason)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Status", &self.status)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelPackageStatusItem {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelPackageStatusItem, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelPackageStatusItem;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelPackageStatusItem")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut failure_reason: Option<::Value<String>> = None;
                    let mut name: Option<::Value<String>> = None;
                    let mut status: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "FailureReason" => {
                                failure_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Name" => {
                                name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Status" => {
                                status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelPackageStatusItem {
                        failure_reason: failure_reason,
                        name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                        status: status.ok_or(::serde::de::Error::missing_field("Status"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ModelQuality`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelquality.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelQuality {
        /// Property [`Constraints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelquality.html#cfn-sagemaker-modelpackage-modelquality-constraints).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints: Option<::Value<MetricsSource>>,
        /// Property [`Statistics`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-modelquality.html#cfn-sagemaker-modelpackage-modelquality-statistics).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub statistics: Option<::Value<MetricsSource>>,
    }

    impl ::codec::SerializeValue for ModelQuality {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constraints) = self.constraints {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Constraints", constraints)?;
            }
            if let Some(ref statistics) = self.statistics {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Statistics", statistics)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelQuality {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelQuality, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelQuality;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelQuality")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints: Option<::Value<MetricsSource>> = None;
                    let mut statistics: Option<::Value<MetricsSource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Constraints" => {
                                constraints = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Statistics" => {
                                statistics = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelQuality {
                        constraints: constraints,
                        statistics: statistics,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.S3DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-s3datasource.html) property type.
    #[derive(Debug, Default)]
    pub struct S3DataSource {
        /// Property [`S3DataType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-s3datasource.html#cfn-sagemaker-modelpackage-s3datasource-s3datatype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_type: ::Value<String>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-s3datasource.html#cfn-sagemaker-modelpackage-s3datasource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3DataSource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataType", &self.s3_data_type)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3DataSource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3DataSource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3DataSource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3DataSource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_data_type: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3DataType" => {
                                s3_data_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3DataSource {
                        s3_data_type: s3_data_type.ok_or(::serde::de::Error::missing_field("S3DataType"))?,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.SourceAlgorithm`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithm.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceAlgorithm {
        /// Property [`AlgorithmName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithm.html#cfn-sagemaker-modelpackage-sourcealgorithm-algorithmname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub algorithm_name: ::Value<String>,
        /// Property [`ModelDataUrl`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithm.html#cfn-sagemaker-modelpackage-sourcealgorithm-modeldataurl).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub model_data_url: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SourceAlgorithm {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AlgorithmName", &self.algorithm_name)?;
            if let Some(ref model_data_url) = self.model_data_url {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ModelDataUrl", model_data_url)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceAlgorithm {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceAlgorithm, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceAlgorithm;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceAlgorithm")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut algorithm_name: Option<::Value<String>> = None;
                    let mut model_data_url: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AlgorithmName" => {
                                algorithm_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ModelDataUrl" => {
                                model_data_url = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAlgorithm {
                        algorithm_name: algorithm_name.ok_or(::serde::de::Error::missing_field("AlgorithmName"))?,
                        model_data_url: model_data_url,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.SourceAlgorithmSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithmspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct SourceAlgorithmSpecification {
        /// Property [`SourceAlgorithms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-sourcealgorithmspecification.html#cfn-sagemaker-modelpackage-sourcealgorithmspecification-sourcealgorithms).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub source_algorithms: ::ValueList<SourceAlgorithm>,
    }

    impl ::codec::SerializeValue for SourceAlgorithmSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SourceAlgorithms", &self.source_algorithms)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SourceAlgorithmSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SourceAlgorithmSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SourceAlgorithmSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SourceAlgorithmSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut source_algorithms: Option<::ValueList<SourceAlgorithm>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SourceAlgorithms" => {
                                source_algorithms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SourceAlgorithmSpecification {
                        source_algorithms: source_algorithms.ok_or(::serde::de::Error::missing_field("SourceAlgorithms"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.TransformInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformInput {
        /// Property [`CompressionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html#cfn-sagemaker-modelpackage-transforminput-compressiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub compression_type: Option<::Value<String>>,
        /// Property [`ContentType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html#cfn-sagemaker-modelpackage-transforminput-contenttype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub content_type: Option<::Value<String>>,
        /// Property [`DataSource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html#cfn-sagemaker-modelpackage-transforminput-datasource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub data_source: ::Value<DataSource>,
        /// Property [`SplitType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transforminput.html#cfn-sagemaker-modelpackage-transforminput-splittype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub split_type: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TransformInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref compression_type) = self.compression_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CompressionType", compression_type)?;
            }
            if let Some(ref content_type) = self.content_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContentType", content_type)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "DataSource", &self.data_source)?;
            if let Some(ref split_type) = self.split_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SplitType", split_type)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut compression_type: Option<::Value<String>> = None;
                    let mut content_type: Option<::Value<String>> = None;
                    let mut data_source: Option<::Value<DataSource>> = None;
                    let mut split_type: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CompressionType" => {
                                compression_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContentType" => {
                                content_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DataSource" => {
                                data_source = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SplitType" => {
                                split_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformInput {
                        compression_type: compression_type,
                        content_type: content_type,
                        data_source: data_source.ok_or(::serde::de::Error::missing_field("DataSource"))?,
                        split_type: split_type,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.TransformJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformJobDefinition {
        /// Property [`BatchStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-batchstrategy).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub batch_strategy: Option<::Value<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::Value<Environment>>,
        /// Property [`MaxConcurrentTransforms`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-maxconcurrenttransforms).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_concurrent_transforms: Option<::Value<u32>>,
        /// Property [`MaxPayloadInMB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-maxpayloadinmb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_payload_in_mb: Option<::Value<u32>>,
        /// Property [`TransformInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-transforminput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transform_input: ::Value<TransformInput>,
        /// Property [`TransformOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-transformoutput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transform_output: ::Value<TransformOutput>,
        /// Property [`TransformResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformjobdefinition.html#cfn-sagemaker-modelpackage-transformjobdefinition-transformresources).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transform_resources: ::Value<TransformResources>,
    }

    impl ::codec::SerializeValue for TransformJobDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref batch_strategy) = self.batch_strategy {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BatchStrategy", batch_strategy)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            if let Some(ref max_concurrent_transforms) = self.max_concurrent_transforms {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxConcurrentTransforms", max_concurrent_transforms)?;
            }
            if let Some(ref max_payload_in_mb) = self.max_payload_in_mb {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxPayloadInMB", max_payload_in_mb)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformInput", &self.transform_input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformOutput", &self.transform_output)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformResources", &self.transform_resources)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformJobDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformJobDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformJobDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformJobDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut batch_strategy: Option<::Value<String>> = None;
                    let mut environment: Option<::Value<Environment>> = None;
                    let mut max_concurrent_transforms: Option<::Value<u32>> = None;
                    let mut max_payload_in_mb: Option<::Value<u32>> = None;
                    let mut transform_input: Option<::Value<TransformInput>> = None;
                    let mut transform_output: Option<::Value<TransformOutput>> = None;
                    let mut transform_resources: Option<::Value<TransformResources>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BatchStrategy" => {
                                batch_strategy = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxConcurrentTransforms" => {
                                max_concurrent_transforms = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MaxPayloadInMB" => {
                                max_payload_in_mb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformInput" => {
                                transform_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformOutput" => {
                                transform_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformResources" => {
                                transform_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformJobDefinition {
                        batch_strategy: batch_strategy,
                        environment: environment,
                        max_concurrent_transforms: max_concurrent_transforms,
                        max_payload_in_mb: max_payload_in_mb,
                        transform_input: transform_input.ok_or(::serde::de::Error::missing_field("TransformInput"))?,
                        transform_output: transform_output.ok_or(::serde::de::Error::missing_field("TransformOutput"))?,
                        transform_resources: transform_resources.ok_or(::serde::de::Error::missing_field("TransformResources"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.TransformOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformOutput {
        /// Property [`Accept`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html#cfn-sagemaker-modelpackage-transformoutput-accept).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub accept: Option<::Value<String>>,
        /// Property [`AssembleWith`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html#cfn-sagemaker-modelpackage-transformoutput-assemblewith).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub assemble_with: Option<::Value<String>>,
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html#cfn-sagemaker-modelpackage-transformoutput-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`S3OutputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformoutput.html#cfn-sagemaker-modelpackage-transformoutput-s3outputpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output_path: ::Value<String>,
    }

    impl ::codec::SerializeValue for TransformOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref accept) = self.accept {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Accept", accept)?;
            }
            if let Some(ref assemble_with) = self.assemble_with {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AssembleWith", assemble_with)?;
            }
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputPath", &self.s3_output_path)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut accept: Option<::Value<String>> = None;
                    let mut assemble_with: Option<::Value<String>> = None;
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut s3_output_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Accept" => {
                                accept = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "AssembleWith" => {
                                assemble_with = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputPath" => {
                                s3_output_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformOutput {
                        accept: accept,
                        assemble_with: assemble_with,
                        kms_key_id: kms_key_id,
                        s3_output_path: s3_output_path.ok_or(::serde::de::Error::missing_field("S3OutputPath"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.TransformResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformresources.html) property type.
    #[derive(Debug, Default)]
    pub struct TransformResources {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformresources.html#cfn-sagemaker-modelpackage-transformresources-instancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformresources.html#cfn-sagemaker-modelpackage-transformresources-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-transformresources.html#cfn-sagemaker-modelpackage-transformresources-volumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for TransformResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for TransformResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<TransformResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = TransformResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type TransformResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(TransformResources {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.UserContext`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-usercontext.html) property type.
    #[derive(Debug, Default)]
    pub struct UserContext {
        /// Property [`DomainId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-usercontext.html#cfn-sagemaker-modelpackage-usercontext-domainid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub domain_id: Option<::Value<String>>,
        /// Property [`UserProfileArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-usercontext.html#cfn-sagemaker-modelpackage-usercontext-userprofilearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_profile_arn: Option<::Value<String>>,
        /// Property [`UserProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-usercontext.html#cfn-sagemaker-modelpackage-usercontext-userprofilename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub user_profile_name: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for UserContext {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref domain_id) = self.domain_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DomainId", domain_id)?;
            }
            if let Some(ref user_profile_arn) = self.user_profile_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProfileArn", user_profile_arn)?;
            }
            if let Some(ref user_profile_name) = self.user_profile_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserProfileName", user_profile_name)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserContext {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserContext, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserContext;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserContext")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut domain_id: Option<::Value<String>> = None;
                    let mut user_profile_arn: Option<::Value<String>> = None;
                    let mut user_profile_name: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DomainId" => {
                                domain_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserProfileArn" => {
                                user_profile_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserProfileName" => {
                                user_profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserContext {
                        domain_id: domain_id,
                        user_profile_arn: user_profile_arn,
                        user_profile_name: user_profile_name,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ValidationProfile`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationprofile.html) property type.
    #[derive(Debug, Default)]
    pub struct ValidationProfile {
        /// Property [`ProfileName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationprofile.html#cfn-sagemaker-modelpackage-validationprofile-profilename).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub profile_name: ::Value<String>,
        /// Property [`TransformJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationprofile.html#cfn-sagemaker-modelpackage-validationprofile-transformjobdefinition).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub transform_job_definition: ::Value<TransformJobDefinition>,
    }

    impl ::codec::SerializeValue for ValidationProfile {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProfileName", &self.profile_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "TransformJobDefinition", &self.transform_job_definition)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValidationProfile {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValidationProfile, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValidationProfile;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValidationProfile")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut profile_name: Option<::Value<String>> = None;
                    let mut transform_job_definition: Option<::Value<TransformJobDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ProfileName" => {
                                profile_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "TransformJobDefinition" => {
                                transform_job_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValidationProfile {
                        profile_name: profile_name.ok_or(::serde::de::Error::missing_field("ProfileName"))?,
                        transform_job_definition: transform_job_definition.ok_or(::serde::de::Error::missing_field("TransformJobDefinition"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelPackage.ValidationSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ValidationSpecification {
        /// Property [`ValidationProfiles`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationspecification.html#cfn-sagemaker-modelpackage-validationspecification-validationprofiles).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub validation_profiles: ::ValueList<ValidationProfile>,
        /// Property [`ValidationRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelpackage-validationspecification.html#cfn-sagemaker-modelpackage-validationspecification-validationrole).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub validation_role: ::Value<String>,
    }

    impl ::codec::SerializeValue for ValidationSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationProfiles", &self.validation_profiles)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ValidationRole", &self.validation_role)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ValidationSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ValidationSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ValidationSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ValidationSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut validation_profiles: Option<::ValueList<ValidationProfile>> = None;
                    let mut validation_role: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ValidationProfiles" => {
                                validation_profiles = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ValidationRole" => {
                                validation_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ValidationSpecification {
                        validation_profiles: validation_profiles.ok_or(::serde::de::Error::missing_field("ValidationProfiles"))?,
                        validation_role: validation_role.ok_or(::serde::de::Error::missing_field("ValidationRole"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod model_quality_job_definition {
    //! Property types for the `ModelQualityJobDefinition` resource.

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html#cfn-sagemaker-modelqualityjobdefinition-clusterconfig-instancecount).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html#cfn-sagemaker-modelqualityjobdefinition-clusterconfig-instancetype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html#cfn-sagemaker-modelqualityjobdefinition-clusterconfig-volumekmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-clusterconfig.html#cfn-sagemaker-modelqualityjobdefinition-clusterconfig-volumesizeingb).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-constraintsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct ConstraintsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-constraintsresource.html#cfn-sagemaker-modelqualityjobdefinition-constraintsresource-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConstraintsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConstraintsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConstraintsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConstraintsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConstraintsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConstraintsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointInput {
        /// Property [`EndTimeOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-endtimeoffset).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub end_time_offset: Option<::Value<String>>,
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-endpointname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_name: ::Value<String>,
        /// Property [`InferenceAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-inferenceattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub inference_attribute: Option<::Value<String>>,
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`ProbabilityAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-probabilityattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_attribute: Option<::Value<String>>,
        /// Property [`ProbabilityThresholdAttribute`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-probabilitythresholdattribute).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub probability_threshold_attribute: Option<::Value<f64>>,
        /// Property [`S3DataDistributionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-s3datadistributiontype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_data_distribution_type: Option<::Value<String>>,
        /// Property [`S3InputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-s3inputmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_input_mode: Option<::Value<String>>,
        /// Property [`StartTimeOffset`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-endpointinput.html#cfn-sagemaker-modelqualityjobdefinition-endpointinput-starttimeoffset).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub start_time_offset: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref end_time_offset) = self.end_time_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndTimeOffset", end_time_offset)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
            if let Some(ref inference_attribute) = self.inference_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InferenceAttribute", inference_attribute)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref probability_attribute) = self.probability_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityAttribute", probability_attribute)?;
            }
            if let Some(ref probability_threshold_attribute) = self.probability_threshold_attribute {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProbabilityThresholdAttribute", probability_threshold_attribute)?;
            }
            if let Some(ref s3_data_distribution_type) = self.s3_data_distribution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataDistributionType", s3_data_distribution_type)?;
            }
            if let Some(ref s3_input_mode) = self.s3_input_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputMode", s3_input_mode)?;
            }
            if let Some(ref start_time_offset) = self.start_time_offset {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StartTimeOffset", start_time_offset)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut end_time_offset: Option<::Value<String>> = None;
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut inference_attribute: Option<::Value<String>> = None;
                    let mut local_path: Option<::Value<String>> = None;
                    let mut probability_attribute: Option<::Value<String>> = None;
                    let mut probability_threshold_attribute: Option<::Value<f64>> = None;
                    let mut s3_data_distribution_type: Option<::Value<String>> = None;
                    let mut s3_input_mode: Option<::Value<String>> = None;
                    let mut start_time_offset: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndTimeOffset" => {
                                end_time_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InferenceAttribute" => {
                                inference_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityAttribute" => {
                                probability_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProbabilityThresholdAttribute" => {
                                probability_threshold_attribute = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DataDistributionType" => {
                                s3_data_distribution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputMode" => {
                                s3_input_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StartTimeOffset" => {
                                start_time_offset = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointInput {
                        end_time_offset: end_time_offset,
                        endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                        inference_attribute: inference_attribute,
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        probability_attribute: probability_attribute,
                        probability_threshold_attribute: probability_threshold_attribute,
                        s3_data_distribution_type: s3_data_distribution_type,
                        s3_input_mode: s3_input_mode,
                        start_time_offset: start_time_offset,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.ModelQualityAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelQualityAppSpecification {
        /// Property [`ContainerArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-containerarguments).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_arguments: Option<::ValueList<String>>,
        /// Property [`ContainerEntrypoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-containerentrypoint).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub container_entrypoint: Option<::ValueList<String>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-environment).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub environment: Option<::ValueMap<String>>,
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-imageuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub image_uri: ::Value<String>,
        /// Property [`PostAnalyticsProcessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-postanalyticsprocessorsourceuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub post_analytics_processor_source_uri: Option<::Value<String>>,
        /// Property [`ProblemType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-problemtype).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub problem_type: ::Value<String>,
        /// Property [`RecordPreprocessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityappspecification.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityappspecification-recordpreprocessorsourceuri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub record_preprocessor_source_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ModelQualityAppSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_arguments) = self.container_arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerArguments", container_arguments)?;
            }
            if let Some(ref container_entrypoint) = self.container_entrypoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerEntrypoint", container_entrypoint)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", &self.image_uri)?;
            if let Some(ref post_analytics_processor_source_uri) = self.post_analytics_processor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAnalyticsProcessorSourceUri", post_analytics_processor_source_uri)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProblemType", &self.problem_type)?;
            if let Some(ref record_preprocessor_source_uri) = self.record_preprocessor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordPreprocessorSourceUri", record_preprocessor_source_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelQualityAppSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelQualityAppSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelQualityAppSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelQualityAppSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_arguments: Option<::ValueList<String>> = None;
                    let mut container_entrypoint: Option<::ValueList<String>> = None;
                    let mut environment: Option<::ValueMap<String>> = None;
                    let mut image_uri: Option<::Value<String>> = None;
                    let mut post_analytics_processor_source_uri: Option<::Value<String>> = None;
                    let mut problem_type: Option<::Value<String>> = None;
                    let mut record_preprocessor_source_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerArguments" => {
                                container_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerEntrypoint" => {
                                container_entrypoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostAnalyticsProcessorSourceUri" => {
                                post_analytics_processor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProblemType" => {
                                problem_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordPreprocessorSourceUri" => {
                                record_preprocessor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelQualityAppSpecification {
                        container_arguments: container_arguments,
                        container_entrypoint: container_entrypoint,
                        environment: environment,
                        image_uri: image_uri.ok_or(::serde::de::Error::missing_field("ImageUri"))?,
                        post_analytics_processor_source_uri: post_analytics_processor_source_uri,
                        problem_type: problem_type.ok_or(::serde::de::Error::missing_field("ProblemType"))?,
                        record_preprocessor_source_uri: record_preprocessor_source_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.ModelQualityBaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelQualityBaselineConfig {
        /// Property [`BaseliningJobName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig.html#cfn-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig-baseliningjobname).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub baselining_job_name: Option<::Value<String>>,
        /// Property [`ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig.html#cfn-sagemaker-modelqualityjobdefinition-modelqualitybaselineconfig-constraintsresource).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub constraints_resource: Option<::Value<ConstraintsResource>>,
    }

    impl ::codec::SerializeValue for ModelQualityBaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref baselining_job_name) = self.baselining_job_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaseliningJobName", baselining_job_name)?;
            }
            if let Some(ref constraints_resource) = self.constraints_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstraintsResource", constraints_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelQualityBaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelQualityBaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelQualityBaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelQualityBaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut baselining_job_name: Option<::Value<String>> = None;
                    let mut constraints_resource: Option<::Value<ConstraintsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaseliningJobName" => {
                                baselining_job_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConstraintsResource" => {
                                constraints_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelQualityBaselineConfig {
                        baselining_job_name: baselining_job_name,
                        constraints_resource: constraints_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.ModelQualityJobInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityjobinput.html) property type.
    #[derive(Debug, Default)]
    pub struct ModelQualityJobInput {
        /// Property [`EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityjobinput.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityjobinput-endpointinput).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub endpoint_input: ::Value<EndpointInput>,
        /// Property [`GroundTruthS3Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-modelqualityjobinput.html#cfn-sagemaker-modelqualityjobdefinition-modelqualityjobinput-groundtruths3input).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub ground_truth_s3_input: ::Value<MonitoringGroundTruthS3Input>,
    }

    impl ::codec::SerializeValue for ModelQualityJobInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointInput", &self.endpoint_input)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "GroundTruthS3Input", &self.ground_truth_s3_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ModelQualityJobInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ModelQualityJobInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ModelQualityJobInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ModelQualityJobInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_input: Option<::Value<EndpointInput>> = None;
                    let mut ground_truth_s3_input: Option<::Value<MonitoringGroundTruthS3Input>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointInput" => {
                                endpoint_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "GroundTruthS3Input" => {
                                ground_truth_s3_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ModelQualityJobInput {
                        endpoint_input: endpoint_input.ok_or(::serde::de::Error::missing_field("EndpointInput"))?,
                        ground_truth_s3_input: ground_truth_s3_input.ok_or(::serde::de::Error::missing_field("GroundTruthS3Input"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.MonitoringGroundTruthS3Input`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringgroundtruths3input.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringGroundTruthS3Input {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringgroundtruths3input.html#cfn-sagemaker-modelqualityjobdefinition-monitoringgroundtruths3input-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for MonitoringGroundTruthS3Input {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringGroundTruthS3Input {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringGroundTruthS3Input, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringGroundTruthS3Input;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringGroundTruthS3Input")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringGroundTruthS3Input {
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.MonitoringOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutput {
        /// Property [`S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutput.html#cfn-sagemaker-modelqualityjobdefinition-monitoringoutput-s3output).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_output: ::Value<S3Output>,
    }

    impl ::codec::SerializeValue for MonitoringOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Output", &self.s3_output)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_output: Option<::Value<S3Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Output" => {
                                s3_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutput {
                        s3_output: s3_output.ok_or(::serde::de::Error::missing_field("S3Output"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelqualityjobdefinition-monitoringoutputconfig-kmskeyid).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MonitoringOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringoutputconfig.html#cfn-sagemaker-modelqualityjobdefinition-monitoringoutputconfig-monitoringoutputs).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub monitoring_outputs: ::ValueList<MonitoringOutput>,
    }

    impl ::codec::SerializeValue for MonitoringOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputs", &self.monitoring_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut monitoring_outputs: Option<::ValueList<MonitoringOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputs" => {
                                monitoring_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutputConfig {
                        kms_key_id: kms_key_id,
                        monitoring_outputs: monitoring_outputs.ok_or(::serde::de::Error::missing_field("MonitoringOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringResources {
        /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-monitoringresources.html#cfn-sagemaker-modelqualityjobdefinition-monitoringresources-clusterconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cluster_config: ::Value<ClusterConfig>,
    }

    impl ::codec::SerializeValue for MonitoringResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", &self.cluster_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_config: Option<::Value<ClusterConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterConfig" => {
                                cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringResources {
                        cluster_config: cluster_config.ok_or(::serde::de::Error::missing_field("ClusterConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-networkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfig {
        /// Property [`EnableInterContainerTrafficEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-networkconfig.html#cfn-sagemaker-modelqualityjobdefinition-networkconfig-enableintercontainertrafficencryption).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_inter_container_traffic_encryption: Option<::Value<bool>>,
        /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-networkconfig.html#cfn-sagemaker-modelqualityjobdefinition-networkconfig-enablenetworkisolation).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub enable_network_isolation: Option<::Value<bool>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-networkconfig.html#cfn-sagemaker-modelqualityjobdefinition-networkconfig-vpcconfig).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for NetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_inter_container_traffic_encryption) = self.enable_inter_container_traffic_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableInterContainerTrafficEncryption", enable_inter_container_traffic_encryption)?;
            }
            if let Some(ref enable_network_isolation) = self.enable_network_isolation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_inter_container_traffic_encryption: Option<::Value<bool>> = None;
                    let mut enable_network_isolation: Option<::Value<bool>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                enable_inter_container_traffic_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNetworkIsolation" => {
                                enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfig {
                        enable_inter_container_traffic_encryption: enable_inter_container_traffic_encryption,
                        enable_network_isolation: enable_network_isolation,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-s3output.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Output {
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-s3output.html#cfn-sagemaker-modelqualityjobdefinition-s3output-localpath).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3UploadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-s3output.html#cfn-sagemaker-modelqualityjobdefinition-s3output-s3uploadmode).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_upload_mode: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-s3output.html#cfn-sagemaker-modelqualityjobdefinition-s3output-s3uri).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_upload_mode) = self.s3_upload_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3UploadMode", s3_upload_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_upload_mode: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3UploadMode" => {
                                s3_upload_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Output {
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_upload_mode: s3_upload_mode,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-stoppingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct StoppingCondition {
        /// Property [`MaxRuntimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-stoppingcondition.html#cfn-sagemaker-modelqualityjobdefinition-stoppingcondition-maxruntimeinseconds).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub max_runtime_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StoppingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuntimeInSeconds", &self.max_runtime_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoppingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoppingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoppingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoppingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_runtime_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                max_runtime_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoppingCondition {
                        max_runtime_in_seconds: max_runtime_in_seconds.ok_or(::serde::de::Error::missing_field("MaxRuntimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::ModelQualityJobDefinition.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-vpcconfig.html#cfn-sagemaker-modelqualityjobdefinition-vpcconfig-securitygroupids).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-modelqualityjobdefinition-vpcconfig.html#cfn-sagemaker-modelqualityjobdefinition-vpcconfig-subnets).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod monitoring_schedule {
    //! Property types for the `MonitoringSchedule` resource.

    /// The [`AWS::SageMaker::MonitoringSchedule.BaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-baselineconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct BaselineConfig {
        /// Property [`ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-baselineconfig.html#cfn-sagemaker-monitoringschedule-baselineconfig-constraintsresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub constraints_resource: Option<::Value<ConstraintsResource>>,
        /// Property [`StatisticsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-baselineconfig.html#cfn-sagemaker-monitoringschedule-baselineconfig-statisticsresource).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub statistics_resource: Option<::Value<StatisticsResource>>,
    }

    impl ::codec::SerializeValue for BaselineConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref constraints_resource) = self.constraints_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConstraintsResource", constraints_resource)?;
            }
            if let Some(ref statistics_resource) = self.statistics_resource {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StatisticsResource", statistics_resource)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for BaselineConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<BaselineConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = BaselineConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type BaselineConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut constraints_resource: Option<::Value<ConstraintsResource>> = None;
                    let mut statistics_resource: Option<::Value<StatisticsResource>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConstraintsResource" => {
                                constraints_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StatisticsResource" => {
                                statistics_resource = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(BaselineConfig {
                        constraints_resource: constraints_resource,
                        statistics_resource: statistics_resource,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ClusterConfig {
        /// Property [`InstanceCount`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html#cfn-sagemaker-monitoringschedule-clusterconfig-instancecount).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_count: ::Value<u32>,
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html#cfn-sagemaker-monitoringschedule-clusterconfig-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: ::Value<String>,
        /// Property [`VolumeKmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html#cfn-sagemaker-monitoringschedule-clusterconfig-volumekmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_kms_key_id: Option<::Value<String>>,
        /// Property [`VolumeSizeInGB`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-clusterconfig.html#cfn-sagemaker-monitoringschedule-clusterconfig-volumesizeingb).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub volume_size_in_gb: ::Value<u32>,
    }

    impl ::codec::SerializeValue for ClusterConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceCount", &self.instance_count)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", &self.instance_type)?;
            if let Some(ref volume_kms_key_id) = self.volume_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeKmsKeyId", volume_kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "VolumeSizeInGB", &self.volume_size_in_gb)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ClusterConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ClusterConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ClusterConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ClusterConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_count: Option<::Value<u32>> = None;
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut volume_kms_key_id: Option<::Value<String>> = None;
                    let mut volume_size_in_gb: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceCount" => {
                                instance_count = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeKmsKeyId" => {
                                volume_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VolumeSizeInGB" => {
                                volume_size_in_gb = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ClusterConfig {
                        instance_count: instance_count.ok_or(::serde::de::Error::missing_field("InstanceCount"))?,
                        instance_type: instance_type.ok_or(::serde::de::Error::missing_field("InstanceType"))?,
                        volume_kms_key_id: volume_kms_key_id,
                        volume_size_in_gb: volume_size_in_gb.ok_or(::serde::de::Error::missing_field("VolumeSizeInGB"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.ConstraintsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-constraintsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct ConstraintsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-constraintsresource.html#cfn-sagemaker-monitoringschedule-constraintsresource-s3uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ConstraintsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ConstraintsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ConstraintsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ConstraintsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ConstraintsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ConstraintsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html) property type.
    #[derive(Debug, Default)]
    pub struct EndpointInput {
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html#cfn-sagemaker-monitoringschedule-endpointinput-endpointname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_name: ::Value<String>,
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html#cfn-sagemaker-monitoringschedule-endpointinput-localpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3DataDistributionType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html#cfn-sagemaker-monitoringschedule-endpointinput-s3datadistributiontype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_data_distribution_type: Option<::Value<String>>,
        /// Property [`S3InputMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-endpointinput.html#cfn-sagemaker-monitoringschedule-endpointinput-s3inputmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_input_mode: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for EndpointInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", &self.endpoint_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_data_distribution_type) = self.s3_data_distribution_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3DataDistributionType", s3_data_distribution_type)?;
            }
            if let Some(ref s3_input_mode) = self.s3_input_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3InputMode", s3_input_mode)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for EndpointInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<EndpointInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = EndpointInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type EndpointInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_data_distribution_type: Option<::Value<String>> = None;
                    let mut s3_input_mode: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3DataDistributionType" => {
                                s3_data_distribution_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3InputMode" => {
                                s3_input_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(EndpointInput {
                        endpoint_name: endpoint_name.ok_or(::serde::de::Error::missing_field("EndpointName"))?,
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_data_distribution_type: s3_data_distribution_type,
                        s3_input_mode: s3_input_mode,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringAppSpecification {
        /// Property [`ContainerArguments`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html#cfn-sagemaker-monitoringschedule-monitoringappspecification-containerarguments).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_arguments: Option<::ValueList<String>>,
        /// Property [`ContainerEntrypoint`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html#cfn-sagemaker-monitoringschedule-monitoringappspecification-containerentrypoint).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub container_entrypoint: Option<::ValueList<String>>,
        /// Property [`ImageUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html#cfn-sagemaker-monitoringschedule-monitoringappspecification-imageuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_uri: ::Value<String>,
        /// Property [`PostAnalyticsProcessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html#cfn-sagemaker-monitoringschedule-monitoringappspecification-postanalyticsprocessorsourceuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub post_analytics_processor_source_uri: Option<::Value<String>>,
        /// Property [`RecordPreprocessorSourceUri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringappspecification.html#cfn-sagemaker-monitoringschedule-monitoringappspecification-recordpreprocessorsourceuri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub record_preprocessor_source_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for MonitoringAppSpecification {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref container_arguments) = self.container_arguments {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerArguments", container_arguments)?;
            }
            if let Some(ref container_entrypoint) = self.container_entrypoint {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ContainerEntrypoint", container_entrypoint)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageUri", &self.image_uri)?;
            if let Some(ref post_analytics_processor_source_uri) = self.post_analytics_processor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PostAnalyticsProcessorSourceUri", post_analytics_processor_source_uri)?;
            }
            if let Some(ref record_preprocessor_source_uri) = self.record_preprocessor_source_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RecordPreprocessorSourceUri", record_preprocessor_source_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringAppSpecification {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringAppSpecification, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringAppSpecification;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringAppSpecification")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut container_arguments: Option<::ValueList<String>> = None;
                    let mut container_entrypoint: Option<::ValueList<String>> = None;
                    let mut image_uri: Option<::Value<String>> = None;
                    let mut post_analytics_processor_source_uri: Option<::Value<String>> = None;
                    let mut record_preprocessor_source_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ContainerArguments" => {
                                container_arguments = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ContainerEntrypoint" => {
                                container_entrypoint = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageUri" => {
                                image_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "PostAnalyticsProcessorSourceUri" => {
                                post_analytics_processor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RecordPreprocessorSourceUri" => {
                                record_preprocessor_source_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringAppSpecification {
                        container_arguments: container_arguments,
                        container_entrypoint: container_entrypoint,
                        image_uri: image_uri.ok_or(::serde::de::Error::missing_field("ImageUri"))?,
                        post_analytics_processor_source_uri: post_analytics_processor_source_uri,
                        record_preprocessor_source_uri: record_preprocessor_source_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringExecutionSummary`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringExecutionSummary {
        /// Property [`CreationTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-creationtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub creation_time: ::Value<String>,
        /// Property [`EndpointName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-endpointname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_name: Option<::Value<String>>,
        /// Property [`FailureReason`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-failurereason).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub failure_reason: Option<::Value<String>>,
        /// Property [`LastModifiedTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-lastmodifiedtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub last_modified_time: ::Value<String>,
        /// Property [`MonitoringExecutionStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-monitoringexecutionstatus).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_execution_status: ::Value<String>,
        /// Property [`MonitoringScheduleName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-monitoringschedulename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_schedule_name: ::Value<String>,
        /// Property [`ProcessingJobArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-processingjobarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub processing_job_arn: Option<::Value<String>>,
        /// Property [`ScheduledTime`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringexecutionsummary.html#cfn-sagemaker-monitoringschedule-monitoringexecutionsummary-scheduledtime).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub scheduled_time: ::Value<String>,
    }

    impl ::codec::SerializeValue for MonitoringExecutionSummary {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CreationTime", &self.creation_time)?;
            if let Some(ref endpoint_name) = self.endpoint_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointName", endpoint_name)?;
            }
            if let Some(ref failure_reason) = self.failure_reason {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "FailureReason", failure_reason)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LastModifiedTime", &self.last_modified_time)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringExecutionStatus", &self.monitoring_execution_status)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringScheduleName", &self.monitoring_schedule_name)?;
            if let Some(ref processing_job_arn) = self.processing_job_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProcessingJobArn", processing_job_arn)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduledTime", &self.scheduled_time)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringExecutionSummary {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringExecutionSummary, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringExecutionSummary;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringExecutionSummary")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut creation_time: Option<::Value<String>> = None;
                    let mut endpoint_name: Option<::Value<String>> = None;
                    let mut failure_reason: Option<::Value<String>> = None;
                    let mut last_modified_time: Option<::Value<String>> = None;
                    let mut monitoring_execution_status: Option<::Value<String>> = None;
                    let mut monitoring_schedule_name: Option<::Value<String>> = None;
                    let mut processing_job_arn: Option<::Value<String>> = None;
                    let mut scheduled_time: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CreationTime" => {
                                creation_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EndpointName" => {
                                endpoint_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "FailureReason" => {
                                failure_reason = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "LastModifiedTime" => {
                                last_modified_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringExecutionStatus" => {
                                monitoring_execution_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringScheduleName" => {
                                monitoring_schedule_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProcessingJobArn" => {
                                processing_job_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduledTime" => {
                                scheduled_time = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringExecutionSummary {
                        creation_time: creation_time.ok_or(::serde::de::Error::missing_field("CreationTime"))?,
                        endpoint_name: endpoint_name,
                        failure_reason: failure_reason,
                        last_modified_time: last_modified_time.ok_or(::serde::de::Error::missing_field("LastModifiedTime"))?,
                        monitoring_execution_status: monitoring_execution_status.ok_or(::serde::de::Error::missing_field("MonitoringExecutionStatus"))?,
                        monitoring_schedule_name: monitoring_schedule_name.ok_or(::serde::de::Error::missing_field("MonitoringScheduleName"))?,
                        processing_job_arn: processing_job_arn,
                        scheduled_time: scheduled_time.ok_or(::serde::de::Error::missing_field("ScheduledTime"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringinput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringInput {
        /// Property [`EndpointInput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringinput.html#cfn-sagemaker-monitoringschedule-monitoringinput-endpointinput).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub endpoint_input: ::Value<EndpointInput>,
    }

    impl ::codec::SerializeValue for MonitoringInput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "EndpointInput", &self.endpoint_input)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringInput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringInput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringInput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringInput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut endpoint_input: Option<::Value<EndpointInput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EndpointInput" => {
                                endpoint_input = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringInput {
                        endpoint_input: endpoint_input.ok_or(::serde::de::Error::missing_field("EndpointInput"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringJobDefinition {
        /// Property [`BaselineConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-baselineconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub baseline_config: Option<::Value<BaselineConfig>>,
        /// Property [`Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-environment).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub environment: Option<::ValueMap<String>>,
        /// Property [`MonitoringAppSpecification`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-monitoringappspecification).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_app_specification: ::Value<MonitoringAppSpecification>,
        /// Property [`MonitoringInputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-monitoringinputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_inputs: ::ValueList<MonitoringInput>,
        /// Property [`MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-monitoringoutputconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_output_config: ::Value<MonitoringOutputConfig>,
        /// Property [`MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-monitoringresources).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_resources: ::Value<MonitoringResources>,
        /// Property [`NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-networkconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub network_config: Option<::Value<NetworkConfig>>,
        /// Property [`RoleArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-rolearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub role_arn: ::Value<String>,
        /// Property [`StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringjobdefinition.html#cfn-sagemaker-monitoringschedule-monitoringjobdefinition-stoppingcondition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub stopping_condition: Option<::Value<StoppingCondition>>,
    }

    impl ::codec::SerializeValue for MonitoringJobDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref baseline_config) = self.baseline_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "BaselineConfig", baseline_config)?;
            }
            if let Some(ref environment) = self.environment {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Environment", environment)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringAppSpecification", &self.monitoring_app_specification)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringInputs", &self.monitoring_inputs)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputConfig", &self.monitoring_output_config)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringResources", &self.monitoring_resources)?;
            if let Some(ref network_config) = self.network_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NetworkConfig", network_config)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "RoleArn", &self.role_arn)?;
            if let Some(ref stopping_condition) = self.stopping_condition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "StoppingCondition", stopping_condition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringJobDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringJobDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringJobDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringJobDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut baseline_config: Option<::Value<BaselineConfig>> = None;
                    let mut environment: Option<::ValueMap<String>> = None;
                    let mut monitoring_app_specification: Option<::Value<MonitoringAppSpecification>> = None;
                    let mut monitoring_inputs: Option<::ValueList<MonitoringInput>> = None;
                    let mut monitoring_output_config: Option<::Value<MonitoringOutputConfig>> = None;
                    let mut monitoring_resources: Option<::Value<MonitoringResources>> = None;
                    let mut network_config: Option<::Value<NetworkConfig>> = None;
                    let mut role_arn: Option<::Value<String>> = None;
                    let mut stopping_condition: Option<::Value<StoppingCondition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BaselineConfig" => {
                                baseline_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Environment" => {
                                environment = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringAppSpecification" => {
                                monitoring_app_specification = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringInputs" => {
                                monitoring_inputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputConfig" => {
                                monitoring_output_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringResources" => {
                                monitoring_resources = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "NetworkConfig" => {
                                network_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RoleArn" => {
                                role_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "StoppingCondition" => {
                                stopping_condition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringJobDefinition {
                        baseline_config: baseline_config,
                        environment: environment,
                        monitoring_app_specification: monitoring_app_specification.ok_or(::serde::de::Error::missing_field("MonitoringAppSpecification"))?,
                        monitoring_inputs: monitoring_inputs.ok_or(::serde::de::Error::missing_field("MonitoringInputs"))?,
                        monitoring_output_config: monitoring_output_config.ok_or(::serde::de::Error::missing_field("MonitoringOutputConfig"))?,
                        monitoring_resources: monitoring_resources.ok_or(::serde::de::Error::missing_field("MonitoringResources"))?,
                        network_config: network_config,
                        role_arn: role_arn.ok_or(::serde::de::Error::missing_field("RoleArn"))?,
                        stopping_condition: stopping_condition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringOutput`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutput.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutput {
        /// Property [`S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutput.html#cfn-sagemaker-monitoringschedule-monitoringoutput-s3output).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output: ::Value<S3Output>,
    }

    impl ::codec::SerializeValue for MonitoringOutput {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Output", &self.s3_output)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutput {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutput, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutput;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutput")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_output: Option<::Value<S3Output>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Output" => {
                                s3_output = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutput {
                        s3_output: s3_output.ok_or(::serde::de::Error::missing_field("S3Output"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringOutputConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutputconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringOutputConfig {
        /// Property [`KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutputconfig.html#cfn-sagemaker-monitoringschedule-monitoringoutputconfig-kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kms_key_id: Option<::Value<String>>,
        /// Property [`MonitoringOutputs`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringoutputconfig.html#cfn-sagemaker-monitoringschedule-monitoringoutputconfig-monitoringoutputs).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_outputs: ::ValueList<MonitoringOutput>,
    }

    impl ::codec::SerializeValue for MonitoringOutputConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref kms_key_id) = self.kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KmsKeyId", kms_key_id)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringOutputs", &self.monitoring_outputs)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringOutputConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringOutputConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringOutputConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringOutputConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut kms_key_id: Option<::Value<String>> = None;
                    let mut monitoring_outputs: Option<::ValueList<MonitoringOutput>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "KmsKeyId" => {
                                kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringOutputs" => {
                                monitoring_outputs = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringOutputConfig {
                        kms_key_id: kms_key_id,
                        monitoring_outputs: monitoring_outputs.ok_or(::serde::de::Error::missing_field("MonitoringOutputs"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringresources.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringResources {
        /// Property [`ClusterConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringresources.html#cfn-sagemaker-monitoringschedule-monitoringresources-clusterconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cluster_config: ::Value<ClusterConfig>,
    }

    impl ::codec::SerializeValue for MonitoringResources {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ClusterConfig", &self.cluster_config)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringResources {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringResources, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringResources;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringResources")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cluster_config: Option<::Value<ClusterConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ClusterConfig" => {
                                cluster_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringResources {
                        cluster_config: cluster_config.ok_or(::serde::de::Error::missing_field("ClusterConfig"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.MonitoringScheduleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct MonitoringScheduleConfig {
        /// Property [`MonitoringJobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html#cfn-sagemaker-monitoringschedule-monitoringscheduleconfig-monitoringjobdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_job_definition: Option<::Value<MonitoringJobDefinition>>,
        /// Property [`MonitoringJobDefinitionName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html#cfn-sagemaker-monitoringschedule-monitoringscheduleconfig-monitoringjobdefinitionname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_job_definition_name: Option<::Value<String>>,
        /// Property [`MonitoringType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html#cfn-sagemaker-monitoringschedule-monitoringscheduleconfig-monitoringtype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub monitoring_type: Option<::Value<String>>,
        /// Property [`ScheduleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-monitoringscheduleconfig.html#cfn-sagemaker-monitoringschedule-monitoringscheduleconfig-scheduleconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_config: Option<::Value<ScheduleConfig>>,
    }

    impl ::codec::SerializeValue for MonitoringScheduleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref monitoring_job_definition) = self.monitoring_job_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringJobDefinition", monitoring_job_definition)?;
            }
            if let Some(ref monitoring_job_definition_name) = self.monitoring_job_definition_name {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringJobDefinitionName", monitoring_job_definition_name)?;
            }
            if let Some(ref monitoring_type) = self.monitoring_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "MonitoringType", monitoring_type)?;
            }
            if let Some(ref schedule_config) = self.schedule_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleConfig", schedule_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MonitoringScheduleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MonitoringScheduleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MonitoringScheduleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MonitoringScheduleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut monitoring_job_definition: Option<::Value<MonitoringJobDefinition>> = None;
                    let mut monitoring_job_definition_name: Option<::Value<String>> = None;
                    let mut monitoring_type: Option<::Value<String>> = None;
                    let mut schedule_config: Option<::Value<ScheduleConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MonitoringJobDefinition" => {
                                monitoring_job_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringJobDefinitionName" => {
                                monitoring_job_definition_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "MonitoringType" => {
                                monitoring_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ScheduleConfig" => {
                                schedule_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MonitoringScheduleConfig {
                        monitoring_job_definition: monitoring_job_definition,
                        monitoring_job_definition_name: monitoring_job_definition_name,
                        monitoring_type: monitoring_type,
                        schedule_config: schedule_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.NetworkConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-networkconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct NetworkConfig {
        /// Property [`EnableInterContainerTrafficEncryption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-networkconfig.html#cfn-sagemaker-monitoringschedule-networkconfig-enableintercontainertrafficencryption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_inter_container_traffic_encryption: Option<::Value<bool>>,
        /// Property [`EnableNetworkIsolation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-networkconfig.html#cfn-sagemaker-monitoringschedule-networkconfig-enablenetworkisolation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub enable_network_isolation: Option<::Value<bool>>,
        /// Property [`VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-networkconfig.html#cfn-sagemaker-monitoringschedule-networkconfig-vpcconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub vpc_config: Option<::Value<VpcConfig>>,
    }

    impl ::codec::SerializeValue for NetworkConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref enable_inter_container_traffic_encryption) = self.enable_inter_container_traffic_encryption {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableInterContainerTrafficEncryption", enable_inter_container_traffic_encryption)?;
            }
            if let Some(ref enable_network_isolation) = self.enable_network_isolation {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "EnableNetworkIsolation", enable_network_isolation)?;
            }
            if let Some(ref vpc_config) = self.vpc_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "VpcConfig", vpc_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NetworkConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NetworkConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NetworkConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NetworkConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut enable_inter_container_traffic_encryption: Option<::Value<bool>> = None;
                    let mut enable_network_isolation: Option<::Value<bool>> = None;
                    let mut vpc_config: Option<::Value<VpcConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "EnableInterContainerTrafficEncryption" => {
                                enable_inter_container_traffic_encryption = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "EnableNetworkIsolation" => {
                                enable_network_isolation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "VpcConfig" => {
                                vpc_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NetworkConfig {
                        enable_inter_container_traffic_encryption: enable_inter_container_traffic_encryption,
                        enable_network_isolation: enable_network_isolation,
                        vpc_config: vpc_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.S3Output`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-s3output.html) property type.
    #[derive(Debug, Default)]
    pub struct S3Output {
        /// Property [`LocalPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-s3output.html#cfn-sagemaker-monitoringschedule-s3output-localpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub local_path: ::Value<String>,
        /// Property [`S3UploadMode`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-s3output.html#cfn-sagemaker-monitoringschedule-s3output-s3uploadmode).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_upload_mode: Option<::Value<String>>,
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-s3output.html#cfn-sagemaker-monitoringschedule-s3output-s3uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_uri: ::Value<String>,
    }

    impl ::codec::SerializeValue for S3Output {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "LocalPath", &self.local_path)?;
            if let Some(ref s3_upload_mode) = self.s3_upload_mode {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3UploadMode", s3_upload_mode)?;
            }
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", &self.s3_uri)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for S3Output {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<S3Output, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = S3Output;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type S3Output")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut local_path: Option<::Value<String>> = None;
                    let mut s3_upload_mode: Option<::Value<String>> = None;
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "LocalPath" => {
                                local_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3UploadMode" => {
                                s3_upload_mode = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(S3Output {
                        local_path: local_path.ok_or(::serde::de::Error::missing_field("LocalPath"))?,
                        s3_upload_mode: s3_upload_mode,
                        s3_uri: s3_uri.ok_or(::serde::de::Error::missing_field("S3Uri"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.ScheduleConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-scheduleconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ScheduleConfig {
        /// Property [`ScheduleExpression`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-scheduleconfig.html#cfn-sagemaker-monitoringschedule-scheduleconfig-scheduleexpression).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub schedule_expression: ::Value<String>,
    }

    impl ::codec::SerializeValue for ScheduleConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ScheduleExpression", &self.schedule_expression)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ScheduleConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ScheduleConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ScheduleConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ScheduleConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut schedule_expression: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ScheduleExpression" => {
                                schedule_expression = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ScheduleConfig {
                        schedule_expression: schedule_expression.ok_or(::serde::de::Error::missing_field("ScheduleExpression"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.StatisticsResource`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-statisticsresource.html) property type.
    #[derive(Debug, Default)]
    pub struct StatisticsResource {
        /// Property [`S3Uri`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-statisticsresource.html#cfn-sagemaker-monitoringschedule-statisticsresource-s3uri).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_uri: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for StatisticsResource {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref s3_uri) = self.s3_uri {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3Uri", s3_uri)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StatisticsResource {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StatisticsResource, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StatisticsResource;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StatisticsResource")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut s3_uri: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "S3Uri" => {
                                s3_uri = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StatisticsResource {
                        s3_uri: s3_uri,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.StoppingCondition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-stoppingcondition.html) property type.
    #[derive(Debug, Default)]
    pub struct StoppingCondition {
        /// Property [`MaxRuntimeInSeconds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-stoppingcondition.html#cfn-sagemaker-monitoringschedule-stoppingcondition-maxruntimeinseconds).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub max_runtime_in_seconds: ::Value<u32>,
    }

    impl ::codec::SerializeValue for StoppingCondition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MaxRuntimeInSeconds", &self.max_runtime_in_seconds)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for StoppingCondition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<StoppingCondition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = StoppingCondition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type StoppingCondition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut max_runtime_in_seconds: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MaxRuntimeInSeconds" => {
                                max_runtime_in_seconds = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(StoppingCondition {
                        max_runtime_in_seconds: max_runtime_in_seconds.ok_or(::serde::de::Error::missing_field("MaxRuntimeInSeconds"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::MonitoringSchedule.VpcConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-vpcconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct VpcConfig {
        /// Property [`SecurityGroupIds`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-vpcconfig.html#cfn-sagemaker-monitoringschedule-vpcconfig-securitygroupids).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_group_ids: ::ValueList<String>,
        /// Property [`Subnets`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-monitoringschedule-vpcconfig.html#cfn-sagemaker-monitoringschedule-vpcconfig-subnets).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub subnets: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for VpcConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroupIds", &self.security_group_ids)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Subnets", &self.subnets)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for VpcConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<VpcConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = VpcConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type VpcConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut security_group_ids: Option<::ValueList<String>> = None;
                    let mut subnets: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "SecurityGroupIds" => {
                                security_group_ids = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "Subnets" => {
                                subnets = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(VpcConfig {
                        security_group_ids: security_group_ids.ok_or(::serde::de::Error::missing_field("SecurityGroupIds"))?,
                        subnets: subnets.ok_or(::serde::de::Error::missing_field("Subnets"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod notebook_instance {
    //! Property types for the `NotebookInstance` resource.

    /// The [`AWS::SageMaker::NotebookInstance.InstanceMetadataServiceConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstance-instancemetadataserviceconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct InstanceMetadataServiceConfiguration {
        /// Property [`MinimumInstanceMetadataServiceVersion`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstance-instancemetadataserviceconfiguration.html#cfn-sagemaker-notebookinstance-instancemetadataserviceconfiguration-minimuminstancemetadataserviceversion).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub minimum_instance_metadata_service_version: ::Value<String>,
    }

    impl ::codec::SerializeValue for InstanceMetadataServiceConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "MinimumInstanceMetadataServiceVersion", &self.minimum_instance_metadata_service_version)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for InstanceMetadataServiceConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<InstanceMetadataServiceConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = InstanceMetadataServiceConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type InstanceMetadataServiceConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut minimum_instance_metadata_service_version: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "MinimumInstanceMetadataServiceVersion" => {
                                minimum_instance_metadata_service_version = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(InstanceMetadataServiceConfiguration {
                        minimum_instance_metadata_service_version: minimum_instance_metadata_service_version.ok_or(::serde::de::Error::missing_field("MinimumInstanceMetadataServiceVersion"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod notebook_instance_lifecycle_config {
    //! Property types for the `NotebookInstanceLifecycleConfig` resource.

    /// The [`AWS::SageMaker::NotebookInstanceLifecycleConfig.NotebookInstanceLifecycleHook`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstancelifecycleconfig-notebookinstancelifecyclehook.html) property type.
    #[derive(Debug, Default)]
    pub struct NotebookInstanceLifecycleHook {
        /// Property [`Content`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-notebookinstancelifecycleconfig-notebookinstancelifecyclehook.html#cfn-sagemaker-notebookinstancelifecycleconfig-notebookinstancelifecyclehook-content).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub content: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for NotebookInstanceLifecycleHook {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref content) = self.content {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "Content", content)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotebookInstanceLifecycleHook {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotebookInstanceLifecycleHook, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotebookInstanceLifecycleHook;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotebookInstanceLifecycleHook")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut content: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "Content" => {
                                content = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotebookInstanceLifecycleHook {
                        content: content,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod user_profile {
    //! Property types for the `UserProfile` resource.

    /// The [`AWS::SageMaker::UserProfile.CustomImage`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customimage.html) property type.
    #[derive(Debug, Default)]
    pub struct CustomImage {
        /// Property [`AppImageConfigName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customimage.html#cfn-sagemaker-userprofile-customimage-appimageconfigname).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub app_image_config_name: ::Value<String>,
        /// Property [`ImageName`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customimage.html#cfn-sagemaker-userprofile-customimage-imagename).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_name: ::Value<String>,
        /// Property [`ImageVersionNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-customimage.html#cfn-sagemaker-userprofile-customimage-imageversionnumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub image_version_number: Option<::Value<u32>>,
    }

    impl ::codec::SerializeValue for CustomImage {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "AppImageConfigName", &self.app_image_config_name)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageName", &self.image_name)?;
            if let Some(ref image_version_number) = self.image_version_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ImageVersionNumber", image_version_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CustomImage {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CustomImage, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CustomImage;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CustomImage")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut app_image_config_name: Option<::Value<String>> = None;
                    let mut image_name: Option<::Value<String>> = None;
                    let mut image_version_number: Option<::Value<u32>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AppImageConfigName" => {
                                app_image_config_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageName" => {
                                image_name = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ImageVersionNumber" => {
                                image_version_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CustomImage {
                        app_image_config_name: app_image_config_name.ok_or(::serde::de::Error::missing_field("AppImageConfigName"))?,
                        image_name: image_name.ok_or(::serde::de::Error::missing_field("ImageName"))?,
                        image_version_number: image_version_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.JupyterServerAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-jupyterserverappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct JupyterServerAppSettings {
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-jupyterserverappsettings.html#cfn-sagemaker-userprofile-jupyterserverappsettings-defaultresourcespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
    }

    impl ::codec::SerializeValue for JupyterServerAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for JupyterServerAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<JupyterServerAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = JupyterServerAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type JupyterServerAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(JupyterServerAppSettings {
                        default_resource_spec: default_resource_spec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.KernelGatewayAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-kernelgatewayappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct KernelGatewayAppSettings {
        /// Property [`CustomImages`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-kernelgatewayappsettings.html#cfn-sagemaker-userprofile-kernelgatewayappsettings-customimages).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub custom_images: Option<::ValueList<CustomImage>>,
        /// Property [`DefaultResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-kernelgatewayappsettings.html#cfn-sagemaker-userprofile-kernelgatewayappsettings-defaultresourcespec).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub default_resource_spec: Option<::Value<ResourceSpec>>,
    }

    impl ::codec::SerializeValue for KernelGatewayAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref custom_images) = self.custom_images {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CustomImages", custom_images)?;
            }
            if let Some(ref default_resource_spec) = self.default_resource_spec {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "DefaultResourceSpec", default_resource_spec)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for KernelGatewayAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<KernelGatewayAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = KernelGatewayAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type KernelGatewayAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut custom_images: Option<::ValueList<CustomImage>> = None;
                    let mut default_resource_spec: Option<::Value<ResourceSpec>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CustomImages" => {
                                custom_images = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "DefaultResourceSpec" => {
                                default_resource_spec = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(KernelGatewayAppSettings {
                        custom_images: custom_images,
                        default_resource_spec: default_resource_spec,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.RStudioServerProAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-rstudioserverproappsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct RStudioServerProAppSettings {
        /// Property [`AccessStatus`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-rstudioserverproappsettings.html#cfn-sagemaker-userprofile-rstudioserverproappsettings-accessstatus).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub access_status: Option<::Value<String>>,
        /// Property [`UserGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-rstudioserverproappsettings.html#cfn-sagemaker-userprofile-rstudioserverproappsettings-usergroup).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub user_group: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for RStudioServerProAppSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref access_status) = self.access_status {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "AccessStatus", access_status)?;
            }
            if let Some(ref user_group) = self.user_group {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "UserGroup", user_group)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for RStudioServerProAppSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<RStudioServerProAppSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = RStudioServerProAppSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type RStudioServerProAppSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut access_status: Option<::Value<String>> = None;
                    let mut user_group: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "AccessStatus" => {
                                access_status = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "UserGroup" => {
                                user_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(RStudioServerProAppSettings {
                        access_status: access_status,
                        user_group: user_group,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.ResourceSpec`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-resourcespec.html) property type.
    #[derive(Debug, Default)]
    pub struct ResourceSpec {
        /// Property [`InstanceType`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-resourcespec.html#cfn-sagemaker-userprofile-resourcespec-instancetype).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub instance_type: Option<::Value<String>>,
        /// Property [`SageMakerImageArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-resourcespec.html#cfn-sagemaker-userprofile-resourcespec-sagemakerimagearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_arn: Option<::Value<String>>,
        /// Property [`SageMakerImageVersionArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-resourcespec.html#cfn-sagemaker-userprofile-resourcespec-sagemakerimageversionarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sage_maker_image_version_arn: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for ResourceSpec {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref instance_type) = self.instance_type {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "InstanceType", instance_type)?;
            }
            if let Some(ref sage_maker_image_arn) = self.sage_maker_image_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageArn", sage_maker_image_arn)?;
            }
            if let Some(ref sage_maker_image_version_arn) = self.sage_maker_image_version_arn {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SageMakerImageVersionArn", sage_maker_image_version_arn)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ResourceSpec {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ResourceSpec, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ResourceSpec;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ResourceSpec")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut instance_type: Option<::Value<String>> = None;
                    let mut sage_maker_image_arn: Option<::Value<String>> = None;
                    let mut sage_maker_image_version_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "InstanceType" => {
                                instance_type = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageArn" => {
                                sage_maker_image_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SageMakerImageVersionArn" => {
                                sage_maker_image_version_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ResourceSpec {
                        instance_type: instance_type,
                        sage_maker_image_arn: sage_maker_image_arn,
                        sage_maker_image_version_arn: sage_maker_image_version_arn,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.SharingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-sharingsettings.html) property type.
    #[derive(Debug, Default)]
    pub struct SharingSettings {
        /// Property [`NotebookOutputOption`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-sharingsettings.html#cfn-sagemaker-userprofile-sharingsettings-notebookoutputoption).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notebook_output_option: Option<::Value<String>>,
        /// Property [`S3KmsKeyId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-sharingsettings.html#cfn-sagemaker-userprofile-sharingsettings-s3kmskeyid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_kms_key_id: Option<::Value<String>>,
        /// Property [`S3OutputPath`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-sharingsettings.html#cfn-sagemaker-userprofile-sharingsettings-s3outputpath).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub s3_output_path: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for SharingSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref notebook_output_option) = self.notebook_output_option {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotebookOutputOption", notebook_output_option)?;
            }
            if let Some(ref s3_kms_key_id) = self.s3_kms_key_id {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3KmsKeyId", s3_kms_key_id)?;
            }
            if let Some(ref s3_output_path) = self.s3_output_path {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "S3OutputPath", s3_output_path)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for SharingSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<SharingSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = SharingSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type SharingSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notebook_output_option: Option<::Value<String>> = None;
                    let mut s3_kms_key_id: Option<::Value<String>> = None;
                    let mut s3_output_path: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotebookOutputOption" => {
                                notebook_output_option = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3KmsKeyId" => {
                                s3_kms_key_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "S3OutputPath" => {
                                s3_output_path = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(SharingSettings {
                        notebook_output_option: notebook_output_option,
                        s3_kms_key_id: s3_kms_key_id,
                        s3_output_path: s3_output_path,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::UserProfile.UserSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html) property type.
    #[derive(Debug, Default)]
    pub struct UserSettings {
        /// Property [`ExecutionRole`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-executionrole).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub execution_role: Option<::Value<String>>,
        /// Property [`JupyterServerAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-jupyterserverappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub jupyter_server_app_settings: Option<::Value<JupyterServerAppSettings>>,
        /// Property [`KernelGatewayAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-kernelgatewayappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub kernel_gateway_app_settings: Option<::Value<KernelGatewayAppSettings>>,
        /// Property [`RStudioServerProAppSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-rstudioserverproappsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub r_studio_server_pro_app_settings: Option<::Value<RStudioServerProAppSettings>>,
        /// Property [`SecurityGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-securitygroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub security_groups: Option<::ValueList<String>>,
        /// Property [`SharingSettings`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-userprofile-usersettings.html#cfn-sagemaker-userprofile-usersettings-sharingsettings).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub sharing_settings: Option<::Value<SharingSettings>>,
    }

    impl ::codec::SerializeValue for UserSettings {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref execution_role) = self.execution_role {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ExecutionRole", execution_role)?;
            }
            if let Some(ref jupyter_server_app_settings) = self.jupyter_server_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "JupyterServerAppSettings", jupyter_server_app_settings)?;
            }
            if let Some(ref kernel_gateway_app_settings) = self.kernel_gateway_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "KernelGatewayAppSettings", kernel_gateway_app_settings)?;
            }
            if let Some(ref r_studio_server_pro_app_settings) = self.r_studio_server_pro_app_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "RStudioServerProAppSettings", r_studio_server_pro_app_settings)?;
            }
            if let Some(ref security_groups) = self.security_groups {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SecurityGroups", security_groups)?;
            }
            if let Some(ref sharing_settings) = self.sharing_settings {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "SharingSettings", sharing_settings)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for UserSettings {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<UserSettings, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = UserSettings;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type UserSettings")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut execution_role: Option<::Value<String>> = None;
                    let mut jupyter_server_app_settings: Option<::Value<JupyterServerAppSettings>> = None;
                    let mut kernel_gateway_app_settings: Option<::Value<KernelGatewayAppSettings>> = None;
                    let mut r_studio_server_pro_app_settings: Option<::Value<RStudioServerProAppSettings>> = None;
                    let mut security_groups: Option<::ValueList<String>> = None;
                    let mut sharing_settings: Option<::Value<SharingSettings>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ExecutionRole" => {
                                execution_role = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "JupyterServerAppSettings" => {
                                jupyter_server_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "KernelGatewayAppSettings" => {
                                kernel_gateway_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "RStudioServerProAppSettings" => {
                                r_studio_server_pro_app_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SecurityGroups" => {
                                security_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "SharingSettings" => {
                                sharing_settings = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(UserSettings {
                        execution_role: execution_role,
                        jupyter_server_app_settings: jupyter_server_app_settings,
                        kernel_gateway_app_settings: kernel_gateway_app_settings,
                        r_studio_server_pro_app_settings: r_studio_server_pro_app_settings,
                        security_groups: security_groups,
                        sharing_settings: sharing_settings,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

pub mod workteam {
    //! Property types for the `Workteam` resource.

    /// The [`AWS::SageMaker::Workteam.CognitoMemberDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-cognitomemberdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct CognitoMemberDefinition {
        /// Property [`CognitoClientId`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-cognitomemberdefinition.html#cfn-sagemaker-workteam-cognitomemberdefinition-cognitoclientid).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cognito_client_id: ::Value<String>,
        /// Property [`CognitoUserGroup`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-cognitomemberdefinition.html#cfn-sagemaker-workteam-cognitomemberdefinition-cognitousergroup).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cognito_user_group: ::Value<String>,
        /// Property [`CognitoUserPool`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-cognitomemberdefinition.html#cfn-sagemaker-workteam-cognitomemberdefinition-cognitouserpool).
        ///
        /// Update type: _Immutable_.
        /// AWS CloudFormation replaces the resource when you change this property.
        pub cognito_user_pool: ::Value<String>,
    }

    impl ::codec::SerializeValue for CognitoMemberDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoClientId", &self.cognito_client_id)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoUserGroup", &self.cognito_user_group)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoUserPool", &self.cognito_user_pool)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for CognitoMemberDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<CognitoMemberDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = CognitoMemberDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type CognitoMemberDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cognito_client_id: Option<::Value<String>> = None;
                    let mut cognito_user_group: Option<::Value<String>> = None;
                    let mut cognito_user_pool: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CognitoClientId" => {
                                cognito_client_id = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CognitoUserGroup" => {
                                cognito_user_group = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "CognitoUserPool" => {
                                cognito_user_pool = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(CognitoMemberDefinition {
                        cognito_client_id: cognito_client_id.ok_or(::serde::de::Error::missing_field("CognitoClientId"))?,
                        cognito_user_group: cognito_user_group.ok_or(::serde::de::Error::missing_field("CognitoUserGroup"))?,
                        cognito_user_pool: cognito_user_pool.ok_or(::serde::de::Error::missing_field("CognitoUserPool"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Workteam.MemberDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-memberdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct MemberDefinition {
        /// Property [`CognitoMemberDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-memberdefinition.html#cfn-sagemaker-workteam-memberdefinition-cognitomemberdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub cognito_member_definition: Option<::Value<CognitoMemberDefinition>>,
        /// Property [`OidcMemberDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-memberdefinition.html#cfn-sagemaker-workteam-memberdefinition-oidcmemberdefinition).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oidc_member_definition: Option<::Value<OidcMemberDefinition>>,
    }

    impl ::codec::SerializeValue for MemberDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref cognito_member_definition) = self.cognito_member_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "CognitoMemberDefinition", cognito_member_definition)?;
            }
            if let Some(ref oidc_member_definition) = self.oidc_member_definition {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "OidcMemberDefinition", oidc_member_definition)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for MemberDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<MemberDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = MemberDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type MemberDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut cognito_member_definition: Option<::Value<CognitoMemberDefinition>> = None;
                    let mut oidc_member_definition: Option<::Value<OidcMemberDefinition>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "CognitoMemberDefinition" => {
                                cognito_member_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "OidcMemberDefinition" => {
                                oidc_member_definition = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(MemberDefinition {
                        cognito_member_definition: cognito_member_definition,
                        oidc_member_definition: oidc_member_definition,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Workteam.NotificationConfiguration`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-notificationconfiguration.html) property type.
    #[derive(Debug, Default)]
    pub struct NotificationConfiguration {
        /// Property [`NotificationTopicArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-notificationconfiguration.html#cfn-sagemaker-workteam-notificationconfiguration-notificationtopicarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub notification_topic_arn: ::Value<String>,
    }

    impl ::codec::SerializeValue for NotificationConfiguration {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "NotificationTopicArn", &self.notification_topic_arn)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for NotificationConfiguration {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<NotificationConfiguration, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = NotificationConfiguration;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type NotificationConfiguration")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut notification_topic_arn: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "NotificationTopicArn" => {
                                notification_topic_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(NotificationConfiguration {
                        notification_topic_arn: notification_topic_arn.ok_or(::serde::de::Error::missing_field("NotificationTopicArn"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::SageMaker::Workteam.OidcMemberDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-oidcmemberdefinition.html) property type.
    #[derive(Debug, Default)]
    pub struct OidcMemberDefinition {
        /// Property [`OidcGroups`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sagemaker-workteam-oidcmemberdefinition.html#cfn-sagemaker-workteam-oidcmemberdefinition-oidcgroups).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub oidc_groups: ::ValueList<String>,
    }

    impl ::codec::SerializeValue for OidcMemberDefinition {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "OidcGroups", &self.oidc_groups)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OidcMemberDefinition {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OidcMemberDefinition, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OidcMemberDefinition;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OidcMemberDefinition")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut oidc_groups: Option<::ValueList<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "OidcGroups" => {
                                oidc_groups = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OidcMemberDefinition {
                        oidc_groups: oidc_groups.ok_or(::serde::de::Error::missing_field("OidcGroups"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

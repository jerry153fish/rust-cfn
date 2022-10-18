//! Types for the `ConnectCampaigns` service.

/// The [`AWS::ConnectCampaigns::Campaign`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html) resource type.
#[derive(Debug, Default)]
pub struct Campaign {
    properties: CampaignProperties
}

/// Properties for the `Campaign` resource.
#[derive(Debug, Default)]
pub struct CampaignProperties {
    /// Property [`ConnectInstanceArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html#cfn-connectcampaigns-campaign-connectinstancearn).
    ///
    /// Update type: _Immutable_.
    /// AWS CloudFormation replaces the resource when you change this property.
    pub connect_instance_arn: ::Value<String>,
    /// Property [`DialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html#cfn-connectcampaigns-campaign-dialerconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub dialer_config: ::Value<self::campaign::DialerConfig>,
    /// Property [`Name`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html#cfn-connectcampaigns-campaign-name).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub name: ::Value<String>,
    /// Property [`OutboundCallConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html#cfn-connectcampaigns-campaign-outboundcallconfig).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub outbound_call_config: ::Value<self::campaign::OutboundCallConfig>,
    /// Property [`Tags`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaigns-campaign.html#cfn-connectcampaigns-campaign-tags).
    ///
    /// Update type: _Mutable_.
    /// AWS CloudFormation doesn't replace the resource when you change this property.
    pub tags: Option<::ValueList<::Tag>>,
}

impl ::serde::Serialize for CampaignProperties {
    fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = ::serde::Serializer::serialize_map(s, None)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectInstanceArn", &self.connect_instance_arn)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "DialerConfig", &self.dialer_config)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "Name", &self.name)?;
        ::serde::ser::SerializeMap::serialize_entry(&mut map, "OutboundCallConfig", &self.outbound_call_config)?;
        if let Some(ref tags) = self.tags {
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "Tags", tags)?;
        }
        ::serde::ser::SerializeMap::end(map)
    }
}

impl<'de> ::serde::Deserialize<'de> for CampaignProperties {
    fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<CampaignProperties, D::Error> {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = CampaignProperties;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "a struct of type CampaignProperties")
            }

            fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut connect_instance_arn: Option<::Value<String>> = None;
                let mut dialer_config: Option<::Value<self::campaign::DialerConfig>> = None;
                let mut name: Option<::Value<String>> = None;
                let mut outbound_call_config: Option<::Value<self::campaign::OutboundCallConfig>> = None;
                let mut tags: Option<::ValueList<::Tag>> = None;

                while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                    match __cfn_key.as_ref() {
                        "ConnectInstanceArn" => {
                            connect_instance_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "DialerConfig" => {
                            dialer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Name" => {
                            name = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "OutboundCallConfig" => {
                            outbound_call_config = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        "Tags" => {
                            tags = ::serde::de::MapAccess::next_value(&mut map)?;
                        }
                        _ => {}
                    }
                }

                Ok(CampaignProperties {
                    connect_instance_arn: connect_instance_arn.ok_or(::serde::de::Error::missing_field("ConnectInstanceArn"))?,
                    dialer_config: dialer_config.ok_or(::serde::de::Error::missing_field("DialerConfig"))?,
                    name: name.ok_or(::serde::de::Error::missing_field("Name"))?,
                    outbound_call_config: outbound_call_config.ok_or(::serde::de::Error::missing_field("OutboundCallConfig"))?,
                    tags: tags,
                })
            }
        }

        d.deserialize_map(Visitor)
    }
}

impl ::Resource for Campaign {
    type Properties = CampaignProperties;
    const TYPE: &'static str = "AWS::ConnectCampaigns::Campaign";
    fn properties(&self) -> &CampaignProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut CampaignProperties {
        &mut self.properties
    }
}

impl ::private::Sealed for Campaign {}

impl From<CampaignProperties> for Campaign {
    fn from(properties: CampaignProperties) -> Campaign {
        Campaign { properties }
    }
}

pub mod campaign {
    //! Property types for the `Campaign` resource.

    /// The [`AWS::ConnectCampaigns::Campaign.DialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-dialerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct DialerConfig {
        /// Property [`PredictiveDialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-dialerconfig.html#cfn-connectcampaigns-campaign-dialerconfig-predictivedialerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub predictive_dialer_config: Option<::Value<PredictiveDialerConfig>>,
        /// Property [`ProgressiveDialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-dialerconfig.html#cfn-connectcampaigns-campaign-dialerconfig-progressivedialerconfig).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub progressive_dialer_config: Option<::Value<ProgressiveDialerConfig>>,
    }

    impl ::codec::SerializeValue for DialerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            if let Some(ref predictive_dialer_config) = self.predictive_dialer_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "PredictiveDialerConfig", predictive_dialer_config)?;
            }
            if let Some(ref progressive_dialer_config) = self.progressive_dialer_config {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ProgressiveDialerConfig", progressive_dialer_config)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for DialerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<DialerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = DialerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type DialerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut predictive_dialer_config: Option<::Value<PredictiveDialerConfig>> = None;
                    let mut progressive_dialer_config: Option<::Value<ProgressiveDialerConfig>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "PredictiveDialerConfig" => {
                                predictive_dialer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ProgressiveDialerConfig" => {
                                progressive_dialer_config = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(DialerConfig {
                        predictive_dialer_config: predictive_dialer_config,
                        progressive_dialer_config: progressive_dialer_config,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ConnectCampaigns::Campaign.OutboundCallConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-outboundcallconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct OutboundCallConfig {
        /// Property [`ConnectContactFlowArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-outboundcallconfig.html#cfn-connectcampaigns-campaign-outboundcallconfig-connectcontactflowarn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connect_contact_flow_arn: ::Value<String>,
        /// Property [`ConnectQueueArn`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-outboundcallconfig.html#cfn-connectcampaigns-campaign-outboundcallconfig-connectqueuearn).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connect_queue_arn: ::Value<String>,
        /// Property [`ConnectSourcePhoneNumber`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-outboundcallconfig.html#cfn-connectcampaigns-campaign-outboundcallconfig-connectsourcephonenumber).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub connect_source_phone_number: Option<::Value<String>>,
    }

    impl ::codec::SerializeValue for OutboundCallConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectContactFlowArn", &self.connect_contact_flow_arn)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectQueueArn", &self.connect_queue_arn)?;
            if let Some(ref connect_source_phone_number) = self.connect_source_phone_number {
                ::serde::ser::SerializeMap::serialize_entry(&mut map, "ConnectSourcePhoneNumber", connect_source_phone_number)?;
            }
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for OutboundCallConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<OutboundCallConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = OutboundCallConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type OutboundCallConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut connect_contact_flow_arn: Option<::Value<String>> = None;
                    let mut connect_queue_arn: Option<::Value<String>> = None;
                    let mut connect_source_phone_number: Option<::Value<String>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "ConnectContactFlowArn" => {
                                connect_contact_flow_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectQueueArn" => {
                                connect_queue_arn = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            "ConnectSourcePhoneNumber" => {
                                connect_source_phone_number = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(OutboundCallConfig {
                        connect_contact_flow_arn: connect_contact_flow_arn.ok_or(::serde::de::Error::missing_field("ConnectContactFlowArn"))?,
                        connect_queue_arn: connect_queue_arn.ok_or(::serde::de::Error::missing_field("ConnectQueueArn"))?,
                        connect_source_phone_number: connect_source_phone_number,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ConnectCampaigns::Campaign.PredictiveDialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-predictivedialerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct PredictiveDialerConfig {
        /// Property [`BandwidthAllocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-predictivedialerconfig.html#cfn-connectcampaigns-campaign-predictivedialerconfig-bandwidthallocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bandwidth_allocation: ::Value<f64>,
    }

    impl ::codec::SerializeValue for PredictiveDialerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BandwidthAllocation", &self.bandwidth_allocation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for PredictiveDialerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<PredictiveDialerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = PredictiveDialerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type PredictiveDialerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bandwidth_allocation: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BandwidthAllocation" => {
                                bandwidth_allocation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(PredictiveDialerConfig {
                        bandwidth_allocation: bandwidth_allocation.ok_or(::serde::de::Error::missing_field("BandwidthAllocation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }

    /// The [`AWS::ConnectCampaigns::Campaign.ProgressiveDialerConfig`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-progressivedialerconfig.html) property type.
    #[derive(Debug, Default)]
    pub struct ProgressiveDialerConfig {
        /// Property [`BandwidthAllocation`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaigns-campaign-progressivedialerconfig.html#cfn-connectcampaigns-campaign-progressivedialerconfig-bandwidthallocation).
        ///
        /// Update type: _Mutable_.
        /// AWS CloudFormation doesn't replace the resource when you change this property.
        pub bandwidth_allocation: ::Value<f64>,
    }

    impl ::codec::SerializeValue for ProgressiveDialerConfig {
        fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            let mut map = ::serde::Serializer::serialize_map(s, None)?;
            ::serde::ser::SerializeMap::serialize_entry(&mut map, "BandwidthAllocation", &self.bandwidth_allocation)?;
            ::serde::ser::SerializeMap::end(map)
        }
    }

    impl ::codec::DeserializeValue for ProgressiveDialerConfig {
        fn deserialize<'de, D: ::serde::Deserializer<'de>>(d: D) -> Result<ProgressiveDialerConfig, D::Error> {
            struct Visitor;

            impl<'de> ::serde::de::Visitor<'de> for Visitor {
                type Value = ProgressiveDialerConfig;

                fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, "a struct of type ProgressiveDialerConfig")
                }

                fn visit_map<A: ::serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    let mut bandwidth_allocation: Option<::Value<f64>> = None;

                    while let Some(__cfn_key) = ::serde::de::MapAccess::next_key::<String>(&mut map)? {
                        match __cfn_key.as_ref() {
                            "BandwidthAllocation" => {
                                bandwidth_allocation = ::serde::de::MapAccess::next_value(&mut map)?;
                            }
                            _ => {}
                        }
                    }

                    Ok(ProgressiveDialerConfig {
                        bandwidth_allocation: bandwidth_allocation.ok_or(::serde::de::Error::missing_field("BandwidthAllocation"))?,
                    })
                }
            }

            d.deserialize_map(Visitor)
        }
    }
}

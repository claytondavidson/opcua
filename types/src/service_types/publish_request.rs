// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2020 Adam Lock

// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]

use std::io::{Read, Write};

#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    request_header::RequestHeader,
    service_types::SubscriptionAcknowledgement,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PublishRequest {
    pub request_header: RequestHeader,
    pub subscription_acknowledgements: Option<Vec<SubscriptionAcknowledgement>>,
}

impl MessageInfo for PublishRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::PublishRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<PublishRequest> for PublishRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += byte_len_array(&self.subscription_acknowledgements);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += write_array(stream, &self.subscription_acknowledgements)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let subscription_acknowledgements: Option<Vec<SubscriptionAcknowledgement>> = read_array(stream, decoding_options)?;
        Ok(PublishRequest {
            request_header,
            subscription_acknowledgements,
        })
    }
}

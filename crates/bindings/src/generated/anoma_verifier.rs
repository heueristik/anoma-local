///Module containing a contract's types and functions.
/**

```solidity
library IAnomaVerifier {
    type DeletionCriterion is uint8;
    struct Action { ResourceInstance consumed; ResourceInstance created; bytes32 commitmentTreeRoot; }
    struct AppData { ExpirableBlob[] resourcePayload; ExpirableBlob[] discoveryPayload; ExpirableBlob[] externalPayload; ExpirableBlob[] applicationPayload; }
    struct ExpirableBlob { DeletionCriterion deletionCriterion; bytes blob; }
    struct ResourceInstance { bytes32 tag; bytes32 logicRef; AppData appData; }
    struct Transaction { Action[] actions; bytes proof; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAnomaVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeletionCriterion(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<DeletionCriterion> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        impl DeletionCriterion {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(DeletionCriterion);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for DeletionCriterion {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<DeletionCriterion> for u8 {
            fn from(value: DeletionCriterion) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for DeletionCriterion {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeletionCriterion {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Action { ResourceInstance consumed; ResourceInstance created; bytes32 commitmentTreeRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Action {
        #[allow(missing_docs)]
        pub consumed: <ResourceInstance as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub created: <ResourceInstance as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub commitmentTreeRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            ResourceInstance,
            ResourceInstance,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ResourceInstance as alloy::sol_types::SolType>::RustType,
            <ResourceInstance as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Action> for UnderlyingRustTuple<'_> {
            fn from(value: Action) -> Self {
                (value.consumed, value.created, value.commitmentTreeRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Action {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    consumed: tuple.0,
                    created: tuple.1,
                    commitmentTreeRoot: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Action {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Action {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <ResourceInstance as alloy_sol_types::SolType>::tokenize(
                        &self.consumed,
                    ),
                    <ResourceInstance as alloy_sol_types::SolType>::tokenize(
                        &self.created,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.commitmentTreeRoot),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Action {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Action {
            const NAME: &'static str = "Action";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Action(ResourceInstance consumed,ResourceInstance created,bytes32 commitmentTreeRoot)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <ResourceInstance as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ResourceInstance as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ResourceInstance as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ResourceInstance as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <ResourceInstance as alloy_sol_types::SolType>::eip712_data_word(
                            &self.consumed,
                        )
                        .0,
                    <ResourceInstance as alloy_sol_types::SolType>::eip712_data_word(
                            &self.created,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.commitmentTreeRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Action {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <ResourceInstance as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.consumed,
                    )
                    + <ResourceInstance as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.created,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.commitmentTreeRoot,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <ResourceInstance as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.consumed,
                    out,
                );
                <ResourceInstance as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.created,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.commitmentTreeRoot,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct AppData { ExpirableBlob[] resourcePayload; ExpirableBlob[] discoveryPayload; ExpirableBlob[] externalPayload; ExpirableBlob[] applicationPayload; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AppData {
        #[allow(missing_docs)]
        pub resourcePayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub discoveryPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub externalPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub applicationPayload: alloy::sol_types::private::Vec<
            <ExpirableBlob as alloy::sol_types::SolType>::RustType,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
            alloy::sol_types::sol_data::Array<ExpirableBlob>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Vec<
                <ExpirableBlob as alloy::sol_types::SolType>::RustType,
            >,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AppData> for UnderlyingRustTuple<'_> {
            fn from(value: AppData) -> Self {
                (
                    value.resourcePayload,
                    value.discoveryPayload,
                    value.externalPayload,
                    value.applicationPayload,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AppData {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    resourcePayload: tuple.0,
                    discoveryPayload: tuple.1,
                    externalPayload: tuple.2,
                    applicationPayload: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AppData {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AppData {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.resourcePayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.discoveryPayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.externalPayload),
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::tokenize(&self.applicationPayload),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for AppData {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for AppData {
            const NAME: &'static str = "AppData";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AppData(ExpirableBlob[] resourcePayload,ExpirableBlob[] discoveryPayload,ExpirableBlob[] externalPayload,ExpirableBlob[] applicationPayload)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ExpirableBlob as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.resourcePayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.discoveryPayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.externalPayload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.applicationPayload,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AppData {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.resourcePayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.discoveryPayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.externalPayload,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ExpirableBlob,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.applicationPayload,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.resourcePayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.discoveryPayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.externalPayload,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ExpirableBlob,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.applicationPayload,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ExpirableBlob { DeletionCriterion deletionCriterion; bytes blob; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpirableBlob {
        #[allow(missing_docs)]
        pub deletionCriterion: <DeletionCriterion as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            DeletionCriterion,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <DeletionCriterion as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpirableBlob> for UnderlyingRustTuple<'_> {
            fn from(value: ExpirableBlob) -> Self {
                (value.deletionCriterion, value.blob)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpirableBlob {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    deletionCriterion: tuple.0,
                    blob: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ExpirableBlob {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ExpirableBlob {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <DeletionCriterion as alloy_sol_types::SolType>::tokenize(
                        &self.deletionCriterion,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ExpirableBlob {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ExpirableBlob {
            const NAME: &'static str = "ExpirableBlob";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ExpirableBlob(uint8 deletionCriterion,bytes blob)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <DeletionCriterion as alloy_sol_types::SolType>::eip712_data_word(
                            &self.deletionCriterion,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blob,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ExpirableBlob {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <DeletionCriterion as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.deletionCriterion,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blob,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <DeletionCriterion as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.deletionCriterion,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blob,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ResourceInstance { bytes32 tag; bytes32 logicRef; AppData appData; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ResourceInstance {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub logicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub appData: <AppData as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            AppData,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            <AppData as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ResourceInstance> for UnderlyingRustTuple<'_> {
            fn from(value: ResourceInstance) -> Self {
                (value.tag, value.logicRef, value.appData)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ResourceInstance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tag: tuple.0,
                    logicRef: tuple.1,
                    appData: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ResourceInstance {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ResourceInstance {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.tag),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.logicRef),
                    <AppData as alloy_sol_types::SolType>::tokenize(&self.appData),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ResourceInstance {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ResourceInstance {
            const NAME: &'static str = "ResourceInstance";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ResourceInstance(bytes32 tag,bytes32 logicRef,AppData appData)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<AppData as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <AppData as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tag)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.logicRef)
                        .0,
                    <AppData as alloy_sol_types::SolType>::eip712_data_word(
                            &self.appData,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ResourceInstance {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.tag)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.logicRef,
                    )
                    + <AppData as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.appData,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.tag, out);
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.logicRef,
                    out,
                );
                <AppData as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.appData,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Transaction { Action[] actions; bytes proof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Transaction {
        #[allow(missing_docs)]
        pub actions: alloy::sol_types::private::Vec<
            <Action as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<Action>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <Action as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Transaction> for UnderlyingRustTuple<'_> {
            fn from(value: Transaction) -> Self {
                (value.actions, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Transaction {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    actions: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Transaction {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Transaction {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::SolType>::tokenize(&self.actions),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Transaction {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Transaction {
            const NAME: &'static str = "Transaction";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Transaction(Action[] actions,bytes proof)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Action as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<Action as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.actions)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Transaction {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        Action,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.actions,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    Action,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.actions,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAnomaVerifier`](self) contract instance.

See the [wrapper's documentation](`IAnomaVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> IAnomaVerifierInstance<P, N> {
        IAnomaVerifierInstance::<P, N>::new(address, __provider)
    }
    /**A [`IAnomaVerifier`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAnomaVerifier`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAnomaVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IAnomaVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAnomaVerifierInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAnomaVerifierInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IAnomaVerifier`](self) contract instance.

See the [wrapper's documentation](`IAnomaVerifierInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IAnomaVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAnomaVerifierInstance<P, N> {
            IAnomaVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAnomaVerifierInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IAnomaVerifierInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IAnomaVerifier {
    type DeletionCriterion is uint8;
    struct Action {
        ResourceInstance consumed;
        ResourceInstance created;
        bytes32 commitmentTreeRoot;
    }
    struct AppData {
        ExpirableBlob[] resourcePayload;
        ExpirableBlob[] discoveryPayload;
        ExpirableBlob[] externalPayload;
        ExpirableBlob[] applicationPayload;
    }
    struct ExpirableBlob {
        DeletionCriterion deletionCriterion;
        bytes blob;
    }
    struct ResourceInstance {
        bytes32 tag;
        bytes32 logicRef;
        AppData appData;
    }
    struct Transaction {
        Action[] actions;
        bytes proof;
    }
}

interface AnomaVerifier {
    error EmptyTransactionNotAllowed();
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);
    error LeafCountTooLarge(uint256 leafCount);
    error LeafNotInField(bytes32 leaf);
    error NonExistingRoot(bytes32 root);
    error PreExistingNullifier(bytes32 nullifier);
    error ReentrancyGuardReentrantCall();
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error ZeroGroth16VerifierNotAllowed();

    event ActionExecuted(bytes32 indexed nullifier, bytes32 indexed commitment, bytes32 consumedLogicRef, bytes32 createdLogicRef);
    event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event CommitmentTreeRootAdded(bytes32 root);
    event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);
    event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
    event TransactionExecuted(bytes32 indexed actionDigest);

    constructor(address groth16Verifier);

    function GROTH16_VERIFIER() external view returns (address);
    function commitmentCount() external view returns (uint256 count);
    function execute(IAnomaVerifier.Transaction memory transaction) external;
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
    function latestCommitmentTreeRoot() external view returns (bytes32 root);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "groth16Verifier",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "GROTH16_VERIFIER",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "commitmentCount",
    "inputs": [],
    "outputs": [
      {
        "name": "count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
    "inputs": [
      {
        "name": "transaction",
        "type": "tuple",
        "internalType": "struct IAnomaVerifier.Transaction",
        "components": [
          {
            "name": "actions",
            "type": "tuple[]",
            "internalType": "struct IAnomaVerifier.Action[]",
            "components": [
              {
                "name": "consumed",
                "type": "tuple",
                "internalType": "struct IAnomaVerifier.ResourceInstance",
                "components": [
                  {
                    "name": "tag",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IAnomaVerifier.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "created",
                "type": "tuple",
                "internalType": "struct IAnomaVerifier.ResourceInstance",
                "components": [
                  {
                    "name": "tag",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "logicRef",
                    "type": "bytes32",
                    "internalType": "bytes32"
                  },
                  {
                    "name": "appData",
                    "type": "tuple",
                    "internalType": "struct IAnomaVerifier.AppData",
                    "components": [
                      {
                        "name": "resourcePayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "discoveryPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "externalPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      },
                      {
                        "name": "applicationPayload",
                        "type": "tuple[]",
                        "internalType": "struct IAnomaVerifier.ExpirableBlob[]",
                        "components": [
                          {
                            "name": "deletionCriterion",
                            "type": "uint8",
                            "internalType": "enum IAnomaVerifier.DeletionCriterion"
                          },
                          {
                            "name": "blob",
                            "type": "bytes",
                            "internalType": "bytes"
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "name": "commitmentTreeRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isCommitmentTreeRootHistorical",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "isHistorical",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isNullifierContained",
    "inputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "isContained",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestCommitmentTreeRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "ActionExecuted",
    "inputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "commitment",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "consumedLogicRef",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "createdLogicRef",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ApplicationPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CommitmentTreeRootAdded",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DiscoveryPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExternalPayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ForwarderCallExecuted",
    "inputs": [
      {
        "name": "untrustedForwarder",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "input",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "output",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ResourcePayload",
    "inputs": [
      {
        "name": "tag",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "blob",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TransactionExecuted",
    "inputs": [
      {
        "name": "actionDigest",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "EmptyTransactionNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ForwarderCallOutputMismatch",
    "inputs": [
      {
        "name": "expected",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "actual",
        "type": "bytes",
        "internalType": "bytes"
      }
    ]
  },
  {
    "type": "error",
    "name": "LeafCountTooLarge",
    "inputs": [
      {
        "name": "leafCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "LeafNotInField",
    "inputs": [
      {
        "name": "leaf",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "NonExistingRoot",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "PreExistingNullifier",
    "inputs": [
      {
        "name": "nullifier",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ReentrancyGuardReentrantCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroGroth16VerifierNotAllowed",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod AnomaVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610380604052346100d557604051601f614bc138819003918201601f19168301916001600160401b038311848410176100d9578084926020946040528339810103126100d557516001600160a01b0381168082036100d557156100c6576080527f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d2602061008a610135565b805f52610181825260405f20600160ff19825416179055604051908152a160405161315e9081611a23823960805181818160bb01526102140152f35b63362dd41f60e01b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b919082039182116100fa57565b634e487b7160e01b5f52601160045260245ffd5b9060808110156101215760030201905f90565b634e487b7160e01b5f52603260045260245ffd5b5f5480156104cf5760405190608082016001600160401b038111838210176100d957604052602082016040830190600183016101d9815f516020614b815f395f51905f526f8421084210842108cc6318c6db6d54be8260018060801b031060071b831560081b1783811c60018060401b031060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b905f196001831b019186528083528060011c828260021c19831917821616855219161660608501526004830183116100fa57835190515f93600501818118929183831c61038657506102b694505f19925f516020614b815f395f51905f52601f6f8421084210842108cc6318c6db6d54be831560081b6001600160801b03851160071b1784811c6001600160401b031060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a17905090030192905b6102a984835f6104d4565b93919490939582516100ed565b91600192835b6102dc575b505050505f146102d857906102d59161062a565b90565b5090565b80969293961561037e575f190161030d8185600180926060826020850151831c168201831b930151901c1690030190565b60808110156101215760030287015484515f198101949085116100fa578894830361033c5750965b93926102bc565b91969095848085518a1c16145f1461036c5715610366579061035d9161062a565b94829496610335565b5061035d565b9561037891979261062a565b96610335565b8296506102c1565b9082935060028193011c156103a4575b505050916102b6919261029e565b6102b69450196001831b5f190116806104575750815b60018181019290831b5f190116806103e0575050905b8180821091180218915f80610396565b5f516020614b815f395f51905f526f8421084210842108cc6318c6db6d54be6001600160801b03831160071b831560081b1783811c6001600160401b031060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790039190506103d0565b5f516020614b815f395f51905f526f8421084210842108cc6318c6db6d54be6001600160801b03831160071b831560081b1783811c6001600160401b031060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1782035f1901916103ba565b505f90565b925f935f93801561061d576104ea9084516100ed565b906040840151821c6001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a178201908183116100fa57600101916105b36105ad8287600180926060826020850151831c168201831b930151901c1690030190565b8461010e565b5090820361061457545b93600182018092116100fa57805182106105d657505050565b600294965061060b9395509061060591600180926060826020850151831c168201831b930151901c1690030190565b9061010e565b50015491600191565b600101546105bd565b505f945084935083925050565b905f516020614ba15f395f51905f5280925f086101c0525f08604051610160819052610bc081019081106001600160401b038211176100d957604052610ba06101605152602061016051017f19b849f69450b06848da1d39bd5e4a4302bb86744edc26238b0878e269ed23e581527f265ddfe127dd51bd7239347b758f0a1320eb2cc7450acc1dad47f80c8dcf34d660406101605101527f199750ec472f1809e0f66a545e1e51624108ac845015c2aa3dfc36bab497d8aa60606101605101527f157ff3fe65ac7208110f06a5f74302b14d743ea25067f0ffd032f787c7f1cdf860806101605101527f2e49c43c4569dd9c5fd35ac45fca33f10b15c590692f8beefe18f4896ac9490260a06101605101527f0e35fb89981890520d4aef2b6d6506c3cb2f0b6973c24fa82731345ffa2d1f1e60c06101605101527f251ad47cb15c4f1105f109ae5e944f1ba9d9e7806d667ffec6fe723002e0b99660e06101605101527f13da07dc64d428369873e97160234641f8beb56fdd05e5f3563fa39d9c22df4e6101006101605101527f0c009b84e650e6d23dc00c7dccef7483a553939689d350cd46e7b89055fd47386101206101605101527f011f16b1c63a854f01992e3956f42d8b04eb650c6d535eb0203dec74befdca066101406101605101527f0ed69e5e383a688f209d9a561daa79612f3f78d0467ad45485df07093f367549610160805101527f04dba94a7b0ce9e221acad41472b6bbe3aec507f5eb3d33f463672264c9f789b6101806101605101527f0a3f2637d840f3a16eb094271c9d237b6036757d4bb50bf7ce732ff1d4fa28e86101a06101605101527f259a666f129eea198f8a1c502fdb38fa39b1f075569564b6e54a485d1182323f6101c06101605101527f28bf7459c9b2f4c6d8e7d06a4ee3a47f7745d4271038e5157a32fdf7ede0d6a16101e06101605101527f0a1ca941f057037526ea200f489be8d4c37c85bbcce6a2aeec91bd69414324476102006101605101527f0c6f8f958be0e93053d7fd4fc54512855535ed1539f051dcb43a26fd926361cf6102206101605101527f123106a93cd17578d426e8128ac9d90aa9e8a00708e296e084dd57e69caaf8116102406101605101527f26e1ba52ad9285d97dd3ab52f8e840085e8fa83ff1e8f1877b074867cd2dee756102606101605101527f1cb55cad7bd133de18a64c5c47b9c97cbe4d8b7bf9e095864471537e6a4ae2c56102806101605101527f1dcd73e46acd8f8e0e2c7ce04bde7f6d2a53043d5060a41c7143f08e6e9055d06102a06101605101527f011003e32f6d9c66f5852f05474a4def0cda294a0eb4e9b9b12b9bb4512e55746102c06101605101527f2b1e809ac1d10ab29ad5f20d03a57dfebadfe5903f58bafed7c508dd2287ae8c6102e06101605101527f2539de1785b735999fb4dac35ee17ed0ef995d05ab2fc5faeaa69ae87bcec0a56103006101605101527f0c246c5a2ef8ee0126497f222b3e0a0ef4e1c3d41c86d46e43982cb11d77951d6103206101605101527f192089c4974f68e95408148f7c0632edbb09e6a6ad1a1c2f3f0305f5d03b527b6103406101605101527f1eae0ad8ab68b2f06a0ee36eeb0d0c058529097d91096b756d8fdc2fb5a60d856103606101605101527f179190e5d0e22179e46f8282872abc88db6e2fdc0dee99e69768bd98c5d06bfb6103806101605101527f29bb9e2c9076732576e9a81c7ac4b83214528f7db00f31bf6cafe794a9b3cd1c6103a06101605101527f225d394e42207599403efd0c2464a90d52652645882aac35b10e590e6e691e086103c06101605101527f064760623c25c8cf753d238055b444532be13557451c087de09efd454b23fd596103e06101605101527f10ba3a0e01df92e87f301c4b716d8a394d67f4bf42a75c10922910a78f6b5b876104006101605101527f0e070bf53f8451b24f9c6e96b0c2a801cb511bc0c242eb9d361b77693f21471c6104206101605101527f1b94cd61b051b04dd39755ff93821a73ccd6cb11d2491d8aa7f921014de252fb6104406101605101527f1d7cb39bafb8c744e148787a2e70230f9d4e917d5713bb050487b5aa7d74070b6104606101605101527f2ec93189bd1ab4f69117d0fe980c80ff8785c2961829f701bb74ac1f303b17db6104806101605101527f2db366bfdd36d277a692bb825b86275beac404a19ae07a9082ea46bd835179266104a06101605101527f062100eb485db06269655cf186a68532985275428450359adc99cec6960711b86104c06101605101527f0761d33c66614aaa570e7f1e8244ca1120243f92fa59e4f900c567bf41f5a59b6104e06101605101527f20fc411a114d13992c2705aa034e3f315d78608a0f7de4ccf7a72e494855ad0d6105006101605101527f25b5c004a4bdfcb5add9ec4e9ab219ba102c67e8b3effb5fc3a30f317250bc5a6105206101605101527f23b1822d278ed632a494e58f6df6f5ed038b186d8474155ad87e7dff62b37f4b6105406101605101527f22734b4c5c3f9493606c4ba9012499bf0f14d13bfcfcccaa16102a29cc2f69e06105606101605101527f26c0c8fe09eb30b7e27a74dc33492347e5bdff409aa3610254413d3fad795ce56105806101605101527f070dd0ccb6bd7bbae88eac03fa1fbb26196be3083a809829bbd626df348ccad96105a06101605101527f12b6595bdb329b6fb043ba78bb28c3bec2c0a6de46d8c5ad6067c4ebfd4250da6105c06101605101527f248d97d7f76283d63bec30e7a5876c11c06fca9b275c671c5e33d95bb7e8d7296105e06101605101527f1a306d439d463b0816fc6fd64cc939318b45eb759ddde4aa106d15d9bd9baaaa6106006101605101527f28a8f8372e3c38daced7c00421cb4621f4f1b54ddc27821b0d62d3d6ec7c56cf6106206101605101527e94975717f9a8a8bb35152f24d43294071ce320c829f388bc852183e1e2ce7e6106406101605101527f04d5ee4c3aa78f7d80fde60d716480d3593f74d4f653ae83f4103246db2e8d656106606101605101527f2a6cf5e9aa03d4336349ad6fb8ed2269c7bef54b8822cc76d08495c12efde1876106806101605101527f2304d31eaab960ba9274da43e19ddeb7f792180808fd6e43baae48d7efcba3f36106a06101605101527f03fd9ac865a4b2a6d5e7009785817249bff08a7e0726fcb4e1c11d39d199f0b06106c06101605101527eb7258ded52bbda2248404d55ee5044798afc3a209193073f7954d4d63b0b646106e06101605101527f159f81ada0771799ec38fca2d4bf65ebb13d3a74f3298db36272c5ca65e92d9a6107006101605101527f1ef90e67437fbc8550237a75bc28e3bb9000130ea25f0c5471e144cf4264431f6107206101605101527f1e65f838515e5ff0196b49aa41a2d2568df739bc176b08ec95a79ed82932e30d6107406101605101527f2b1b045def3a166cec6ce768d079ba74b18c844e570e1f826575c1068c94c33f6107606101605101527f0832e5753ceb0ff6402543b1109229c165dc2d73bef715e3f1c6e07c168bb1736107806101605101527f02f614e9cedfb3dc6b762ae0a37d41bab1b841c2e8b6451bc5a8e3c390b6ad166107a06101605101527f0e2427d38bd46a60dd640b8e362cad967370ebb777bedff40f6a0be27e7ed7056107c06101605101527f0493630b7c670b6deb7c84d414e7ce79049f0ec098c3c7c50768bbe29214a53a6107e06101605101527f22ead100e8e482674decdab17066c5a26bb1515355d5461a3dc06cc85327cea96108006101605101527f25b3e56e655b42cdaae2626ed2554d48583f1ae35626d04de5084e0b6d2a6f166108206101605101527f1e32752ada8836ef5837a6cde8ff13dbb599c336349e4c584b4fdc0a0cf6f9d06108406101605101527f2fa2a871c15a387cc50f68f6f3c3455b23c00995f05078f672a9864074d412e56108606101605101527f2f569b8a9a4424c9278e1db7311e889f54ccbf10661bab7fcd18e7c7a7d835056108806101605101527f044cb455110a8fdd531ade530234c518a7df93f7332ffd2144165374b246b43d6108a06101605101527f227808de93906d5d420246157f2e42b191fe8c90adfe118178ddc723a53190256108c06101605101527f02fcca2934e046bc623adead873579865d03781ae090ad4a8579d2e7a68003556108e06101605101527f0ef915f0ac120b876abccceb344a1d36bad3f3c5ab91a8ddcbec2e060d8befac6109006101605101527f1797130f4b7a3e1777eb757bc6f287f6ab0fb85f6be63b09f3b16ef2b1405d386109206101605101527f0a76225dc04170ae3306c85abab59e608c7f497c20156d4d36c668555decc6e56109406101605101527f1fffb9ec1992d66ba1e77a7b93209af6f8fa76d48acb664796174b5326a31a5c6109606101605101527f25721c4fc15a3f2853b57c338fa538d85f8fbba6c6b9c6090611889b797b9c5f6109806101605101527f0c817fd42d5f7a41215e3d07ba197216adb4c3790705da95eb63b982bfcaf75a6109a06101605101527f13abe3f5239915d39f7e13c2c24970b6df8cf86ce00a22002bc15866e52b5a966109c06101605101527f2106feea546224ea12ef7f39987a46c85c1bc3dc29bdbd7a92cd60acb4d391ce6109e06101605101527f21ca859468a746b6aaa79474a37dab49f1ca5a28c748bc7157e1b3345bb0f959610a006101605101527f05ccd6255c1e6f0c5cf1f0df934194c62911d14d0321662a8f1a48999e34185b610a206101605101527f0f0e34a64b70a626e464d846674c4c8816c4fb267fe44fe6ea28678cb09490a4610a406101605101527f0558531a4e25470c6157794ca36d0e9647dbfcfe350d64838f5b1a8a2de0d4bf610a606101605101527f09d3dca9173ed2faceea125157683d18924cadad3f655a60b72f5864961f1455610a806101605101527f0328cbd54e8c0913493f866ed03d218bf23f92d68aaec48617d4c722e5bd4335610aa06101605101527f2bf07216e2aff0a223a487b1a7094e07e79e7bcc9798c648ee3347dd5329d34b610ac06101605101527f1daf345a58006b736499c583cb76c316d6f78ed6a6dffc82111e11a63fe412df610ae06101605101527f176563472456aaa746b694c60e1823611ef39039b2edc7ff391e6f2293d2c404610b00610160510152610b206101605101905f516020614ba15f395f51905f5282527f10dc6e9c006ea38b04b1e03b4bd9490c0d03f98929ca1d7fb56821fd19d3b6e7610b406101605101527f0c28145b6a44df3e0149b3d0a30b3bb599df9756d4dd9b84a86b38cfb45a740b610b606101605101527e544b8338791518b2c7645a50392798b21f75bb60e3596170067d00141cac15610b806101605101527f222c01175718386f2e2e82eb122789e352e105a3b8fa852613bc534433ee428b610ba06101605101526803000000000000000060e0528260c0526101c05161014052600960a05261010052516101e0526101e0516101e05160e05183086101e05160e0518408086101e051906101e0516101e0516101c05160a051086101e05160e051800808906101e051908008086101a0526101e0516101e0516101c05160a051086101e0516101c05160a05108086101e051906101e0516101e05160e05185086101e0516101c051800808906101e05190800808610180526101e051610180516101e0516101e0516101c05160a051086101e05160e051800808086101e0516101a05160e05192939192909183918291086101e0516101c051800808086101a05160a092909252610180516101405260c05260e0525b610220610160510161010051106117e35761010051610700016101205261016051610b40810151610b60820151610b80830151610ba09093015192905b610120516101005110611750575050505061020061010051015b80610100511061171257506101605160405260a05190565b61172f6101e0516101005160e05160c0516101405160a051611821565b9260a0929192526101405260c05260e05260806101005101610100526116fa565b6101e051610100515160a051086101e0518180096101e051919082908009096101e0516101e05160e05160c051086101e0516101405184080890816101e05191866101e05191090860a0526101e051816101e05184610140510908610140526101e051816101e0518560c051090860c0526101e051906101e0518660e051090860e05260206101005101610100526116e0565b6118006101e0516101005160e05160c0516101405160a051611821565b9260a0929192526101405260c05260e05260806101005101610100526116a3565b9391949290946102e052610340526102a0526103605261036051906102a05151900861032052610360519060206102a0510151900861030052610360516103005161030051096102c0526103605160406102a05101516102e051086103605160606102a05101516103405108610260526103605161026051610260510961024052610360519061036051610260516103605161024051610240510909610360519091818082800961036051838009090908610280526103605161028051610360516103605161030051610360516102c0516102c051090961036051610300516102c051829080090908086102205261036051610300516102c051829182918290800909610360516103205181808280096103605161032051800909090861036051610360516102605161036051610240516102405109096103605161026051610240518290800909080861020052610360516102805180086103605161020051909182908008089061036051610360516103605161030051610360516102c0516102c05109096103605161032051818082800961036051610320518009090908610360516103605161030051610360516102c0516102c051090961036051610320518180828009610360516103205180090909080861036051610220519091829080080891610360518361020051089161036051826102205108929392919056fe61036060405260033611610011575f80fd5b5f5f3560e01c8063467b50111461014f578063bdeb442d1461012c578063c1b0bed7146100fc578063c44956d1146100df578063db19aa791461008e5763f8adf4c31461005c575f80fd5b3461008b57602060031936011261008b5760ff6040602092600435815261018184522054166040519015158152f35b80fd5b503461008b578060031936011261008b57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461008b578060031936011261008b5760209054604051908152f35b503461008b57602060031936011261008b5760ff6040602092600435815261018284522054166040519015158152f35b503461008b578060031936011261008b576020610147610ef7565b604051908152f35b50346104055760206003193601126104055760043567ffffffffffffffff8111610405578060040160406003198336030112610405577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c61068f5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101d981806106b7565b90918115610667575f936101ec83610746565b915f5b848110610436575061023c90602473ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016930190610805565b9190604051906020820182811067ffffffffffffffff82111761040957604052878252823b156104055792906102a56040519485937f8d8f8a5c000000000000000000000000000000000000000000000000000000008552604060048601526044850191610856565b905f602484015b600182106103eb575050509181805f9403915afa80156103e0576103ca575b5060206102f87f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d292610903565b80875261018182526040872060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00825416179055604051908152a1835b81811061038a575050507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce8280a2807f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d80f35b806103a96103a461039e60019486886107aa565b80610777565b6117c7565b6103c46103a46103ba8386886107aa565b6020810190610777565b01610336565b6103d79195505f9061070b565b5f9360206102cb565b6040513d5f823e3d90fd5b8251815286945060209283019260019290920191016102ac565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b956104428786886107aa565b6040810135805f5261018160205260ff60405f2054161561063c576104678280610777565b35805f5261018260205260ff60405f205416610611579160209391610559845f95865261018287526040862060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556104ce6104c98580610777565b611165565b868401906104df6104c98387610777565b6104e98286610777565b35907f9857d0b88f01dca7bb1205d68c307d28d8d4002ff5486007ac4c443596dcea2160408a6105198980610777565b01358b610526878b610777565b013582519182528c820152a38c6105486105408387610777565b35918c6107f1565b526105538480610777565b93610777565b8561056f61056a6040860186610876565b6113c0565b9161058061056a6040830183610876565b92826040519681880198895280356040890152013560608701526080860152803560a0860152013560c084015260e083015261010082015261010081526105c96101208261070b565b604051918291518091835e8101838152039060025afa156103e05760017f1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5f511696016101ef565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610405570180359067ffffffffffffffff821161040557602001918160051b3603831361040557565b90601f601f19910116810190811067ffffffffffffffff82111761040957604052565b67ffffffffffffffff81116104095760051b60200190565b906107508261072e565b61075d604051918261070b565b828152601f1961076d829461072e565b0190602036910137565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301821215610405570190565b908210156107c4576107c19160051b810190610777565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80518210156107c45760209160051b010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610405570180359067ffffffffffffffff82116104055760200191813603831361040557565b601f8260209493601f1993818652868601375f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215610405570190565b919082018092116108b657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9060808110156107c45760030201905f90565b919082039182116108b657565b5f5490805115610ee0576109188151836108a9565b67fffffffffffffffe8111610eb55781515f5b818110610e4557505061093d816114a4565b905f82518060011b907f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036108b657600281018091116108b65761098590929192610746565b9585918551945f5b8487821015610b65575f1981146108b65760019687838d6109c28d6109bb85600298019d8e169485926107f1565b51926107f1565b521b1795018087116108b6578681166109e2575b5060010194939461098d565b9094610a71826001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b908160011b917f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811681036108b65760018301808411806108b65760018085841c16145f14610b4e576108b657610ac8908e6107f1565b51935b8d8460018086831c16145f14610b3657610ae4916107f1565b515b60028501958686116108b657600184018094116108b6578f90610b2890610b22610b1a60019a8b809860039a1c16906108a9565b948592611b3e565b926107f1565b521b921b19161794906109d6565b505060808410156107c4576003840260010154610ae6565b5060808110156107c4576003026001015493610acb565b50509450945094905081925f9180610e0a575b50600485018086116108b657610b8e9082611588565b91610b9a81835f6119e2565b939194909395610bac835192836108f6565b92610bb785846108f6565b94610ddc578b9493925f5b855f198301928311965b610cba5750505050505050505f14610cb45790610be891611b3e565b93816001925b610bf9575050505f55565b610c86816001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b610c9081846107f1565b519060808110156107c4576003028401555f198101908082116108b6571680610bee565b50610be8565b90610cc987989b969792610eeb565b95610cee8786600180926060826020850151831c168201831b930151901c1690030190565b809c60018089841c16145f14610dc05790610d08916107f1565b51925b6108b657868403610d275750508490995b948e97969591610bcc565b91986001806040879d949d0151891c16145f14610d8e578692919015610d875790610d5191611b3e565b975b889960019988841015610d69575b505099610d1c565b90925060808310156107c457600260038894028b0101555f80610d61565b5097610d53565b98610d9c919a508692611b3e565b99868210610d1c57905060808110156107c457600260038202018a90558490610d1c565b5050915060808b10156107c457600160038c0201549188610d0b565b610e008582600180926060826020850151831c168201831b930151901c1690030190565b928c959493610bc2565b909150610e16816114a4565b610e208682611588565b91600481018091116108b657610e3591611588565b908180821091180218905f610b78565b7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001610e7082866107f1565b511015610e7f5760010161092b565b610e8990846107f1565b517fc53e726b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f084d73d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b50506107c15f611004565b80156108b6575f190190565b5f548015610fff57610f08816114a4565b90600481018091116108b657610f21610f3a9183611588565b91610f2d83825f6119e2565b92919390929482516108f6565b806001915b610f5a575050505f14610f5657906107c191611b3e565b5090565b610f6390610eeb565b94610f888684600180926060826020850151831c168201831b930151901c1690030190565b60808110156107c4576003028201548351915f1983019283116108b65787928303610fb4575095610f3f565b92959186806040879894980151831c16145f14610feb575015610fe55790610fdb91611b3e565b9284819395610f3f565b50610fdb565b91959294610ff99195611b3e565b95610f3f565b505f90565b8054801561110857611015816114a4565b91600482018092116108b65761102e6110499284611588565b92600161103c8583856119e2565b94919590949684516108f6565b910190805b611065575050505f14610f5657906107c191611b3e565b61106e90610eeb565b9461109c6110968785600180926060826020850151831c168201831b930151901c1690030190565b836108e3565b50548351915f1983019283116108b657879283036110bb57509561104e565b9295916001806040879894980151831c16145f146110f45750156110ee57906110e391611b3e565b92846001939561104e565b506110e3565b919592946111029195611b3e565b9561104e565b50505f90565b91908110156107c45760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215610405570190565b6040906107c1949281528160208201520191610856565b803590604081019061118061117a8383610876565b806106b7565b5f5b818110611351575050506111a36111998383610876565b60208101906106b7565b5f5b8181106112e2575050506111c66111bc8383610876565b60408101906106b7565b5f5b818110611273575050506111e9916111df91610876565b60608101906106b7565b90915f5b8281106111fa5750505050565b61120581848661110e565b35906002821015610405576001809214611220575b016111ed565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c61125a61125084888a61110e565b6020810190610805565b9061126b604051928392878461114e565b0390a261121a565b61127e81838561110e565b35906002821015610405576001809214611299575b016111c8565b867f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596112c961125084878961110e565b906112da604051928392878461114e565b0390a2611293565b6112ed81838561110e565b35906002821015610405576001809214611308575b016111a5565b867f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361133861125084878961110e565b90611349604051928392878461114e565b0390a2611302565b61135c81838561110e565b35906002821015610405576001809214611377575b01611182565b867f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66113a761125084878961110e565b906113b8604051928392878461114e565b0390a2611371565b5f602091826114626113db6113d584806106b7565b90612d3f565b92826113ec6113d5828401846106b7565b918161140f6113d56114046113d560408601866106b7565b9360608101906106b7565b9181604051978896818089019b8051918291018d5e8801908282018d8152815193849201905e010190898252805192839101825e0190878252805192839101825e0185815203601f19810183528261070b565b604051918291518091835e8101838152039060025afa156103e0577f1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5f511690565b906040516080810181811067ffffffffffffffff821117610409576040526060600182940161155a817f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b905f196001831b019184528060208501528060011c828260021c198319178216166040860152191616910152565b91905f6020600185519301940151938085189485841c155f146116425750505061163c5f1992937f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b90030190565b9080929550600284011c1561165657505050565b9293509091196001831b5f1901168061172b5750815b60018101915f196001841b011680155f14611690575050905b818082109118021890565b6117239192507f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b900390611685565b6117be5f19917f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b8303019161166c565b6117d76111bc6040830183610876565b60205f930135925b8181106117ec5750505050565b6117fa61125082848661110e565b81016060828203126104055781359173ffffffffffffffffffffffffffffffffffffffff831680930361040557602081013567ffffffffffffffff81116104055782611847918301612e7c565b91604082013567ffffffffffffffff8111610405576118669201612e7c565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352876004840152604060248401525f83806118ab6044820186612ec2565b038183885af19283156103e0575f93611966575b50825181518082149182611950575b505015611915575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf19161190c60405192839283612ee7565b0390a2016117df565b905061194c6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401612ee7565b0390fd5b9091506020850120906020830120145f806118ce565b9092503d805f833e611978818361070b565b8101906020818303126104055780519067ffffffffffffffff8211610405570181601f82011215610405578051906119af82612e60565b926119bd604051948561070b565b8284526020838301011161040557815f9260208093018386015e83010152915f6118bf565b925f935f938015611b31576119f89084516108f6565b906001611a97611a916040870151851c6001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b846108a9565b910191611ac7611ac18287600180926060826020850151831c168201831b930151901c1690030190565b846108e3565b50908203611b2857545b93600182018092116108b65780518210611aea57505050565b6002949650611b1f93955090611b1991600180926060826020850151831c168201831b930151901c1690030190565b906108e3565b50015491600191565b60010154611ad1565b505f945084935083925050565b907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000180925f086101a0525f08611b80610bc0604051610140526101405161070b565b610ba06101405152602061014051017f19b849f69450b06848da1d39bd5e4a4302bb86744edc26238b0878e269ed23e581527f265ddfe127dd51bd7239347b758f0a1320eb2cc7450acc1dad47f80c8dcf34d660406101405101527f199750ec472f1809e0f66a545e1e51624108ac845015c2aa3dfc36bab497d8aa60606101405101527f157ff3fe65ac7208110f06a5f74302b14d743ea25067f0ffd032f787c7f1cdf860806101405101527f2e49c43c4569dd9c5fd35ac45fca33f10b15c590692f8beefe18f4896ac9490260a06101405101527f0e35fb89981890520d4aef2b6d6506c3cb2f0b6973c24fa82731345ffa2d1f1e60c06101405101527f251ad47cb15c4f1105f109ae5e944f1ba9d9e7806d667ffec6fe723002e0b99660e06101405101527f13da07dc64d428369873e97160234641f8beb56fdd05e5f3563fa39d9c22df4e6101006101405101527f0c009b84e650e6d23dc00c7dccef7483a553939689d350cd46e7b89055fd47386101206101405101527f011f16b1c63a854f01992e3956f42d8b04eb650c6d535eb0203dec74befdca06610140805101527f0ed69e5e383a688f209d9a561daa79612f3f78d0467ad45485df07093f3675496101606101405101527f04dba94a7b0ce9e221acad41472b6bbe3aec507f5eb3d33f463672264c9f789b6101806101405101527f0a3f2637d840f3a16eb094271c9d237b6036757d4bb50bf7ce732ff1d4fa28e86101a06101405101527f259a666f129eea198f8a1c502fdb38fa39b1f075569564b6e54a485d1182323f6101c06101405101527f28bf7459c9b2f4c6d8e7d06a4ee3a47f7745d4271038e5157a32fdf7ede0d6a16101e06101405101527f0a1ca941f057037526ea200f489be8d4c37c85bbcce6a2aeec91bd69414324476102006101405101527f0c6f8f958be0e93053d7fd4fc54512855535ed1539f051dcb43a26fd926361cf6102206101405101527f123106a93cd17578d426e8128ac9d90aa9e8a00708e296e084dd57e69caaf8116102406101405101527f26e1ba52ad9285d97dd3ab52f8e840085e8fa83ff1e8f1877b074867cd2dee756102606101405101527f1cb55cad7bd133de18a64c5c47b9c97cbe4d8b7bf9e095864471537e6a4ae2c56102806101405101527f1dcd73e46acd8f8e0e2c7ce04bde7f6d2a53043d5060a41c7143f08e6e9055d06102a06101405101527f011003e32f6d9c66f5852f05474a4def0cda294a0eb4e9b9b12b9bb4512e55746102c06101405101527f2b1e809ac1d10ab29ad5f20d03a57dfebadfe5903f58bafed7c508dd2287ae8c6102e06101405101527f2539de1785b735999fb4dac35ee17ed0ef995d05ab2fc5faeaa69ae87bcec0a56103006101405101527f0c246c5a2ef8ee0126497f222b3e0a0ef4e1c3d41c86d46e43982cb11d77951d6103206101405101527f192089c4974f68e95408148f7c0632edbb09e6a6ad1a1c2f3f0305f5d03b527b6103406101405101527f1eae0ad8ab68b2f06a0ee36eeb0d0c058529097d91096b756d8fdc2fb5a60d856103606101405101527f179190e5d0e22179e46f8282872abc88db6e2fdc0dee99e69768bd98c5d06bfb6103806101405101527f29bb9e2c9076732576e9a81c7ac4b83214528f7db00f31bf6cafe794a9b3cd1c6103a06101405101527f225d394e42207599403efd0c2464a90d52652645882aac35b10e590e6e691e086103c06101405101527f064760623c25c8cf753d238055b444532be13557451c087de09efd454b23fd596103e06101405101527f10ba3a0e01df92e87f301c4b716d8a394d67f4bf42a75c10922910a78f6b5b876104006101405101527f0e070bf53f8451b24f9c6e96b0c2a801cb511bc0c242eb9d361b77693f21471c6104206101405101527f1b94cd61b051b04dd39755ff93821a73ccd6cb11d2491d8aa7f921014de252fb6104406101405101527f1d7cb39bafb8c744e148787a2e70230f9d4e917d5713bb050487b5aa7d74070b6104606101405101527f2ec93189bd1ab4f69117d0fe980c80ff8785c2961829f701bb74ac1f303b17db6104806101405101527f2db366bfdd36d277a692bb825b86275beac404a19ae07a9082ea46bd835179266104a06101405101527f062100eb485db06269655cf186a68532985275428450359adc99cec6960711b86104c06101405101527f0761d33c66614aaa570e7f1e8244ca1120243f92fa59e4f900c567bf41f5a59b6104e06101405101527f20fc411a114d13992c2705aa034e3f315d78608a0f7de4ccf7a72e494855ad0d6105006101405101527f25b5c004a4bdfcb5add9ec4e9ab219ba102c67e8b3effb5fc3a30f317250bc5a6105206101405101527f23b1822d278ed632a494e58f6df6f5ed038b186d8474155ad87e7dff62b37f4b6105406101405101527f22734b4c5c3f9493606c4ba9012499bf0f14d13bfcfcccaa16102a29cc2f69e06105606101405101527f26c0c8fe09eb30b7e27a74dc33492347e5bdff409aa3610254413d3fad795ce56105806101405101527f070dd0ccb6bd7bbae88eac03fa1fbb26196be3083a809829bbd626df348ccad96105a06101405101527f12b6595bdb329b6fb043ba78bb28c3bec2c0a6de46d8c5ad6067c4ebfd4250da6105c06101405101527f248d97d7f76283d63bec30e7a5876c11c06fca9b275c671c5e33d95bb7e8d7296105e06101405101527f1a306d439d463b0816fc6fd64cc939318b45eb759ddde4aa106d15d9bd9baaaa6106006101405101527f28a8f8372e3c38daced7c00421cb4621f4f1b54ddc27821b0d62d3d6ec7c56cf6106206101405101527e94975717f9a8a8bb35152f24d43294071ce320c829f388bc852183e1e2ce7e6106406101405101527f04d5ee4c3aa78f7d80fde60d716480d3593f74d4f653ae83f4103246db2e8d656106606101405101527f2a6cf5e9aa03d4336349ad6fb8ed2269c7bef54b8822cc76d08495c12efde1876106806101405101527f2304d31eaab960ba9274da43e19ddeb7f792180808fd6e43baae48d7efcba3f36106a06101405101527f03fd9ac865a4b2a6d5e7009785817249bff08a7e0726fcb4e1c11d39d199f0b06106c06101405101527eb7258ded52bbda2248404d55ee5044798afc3a209193073f7954d4d63b0b646106e06101405101527f159f81ada0771799ec38fca2d4bf65ebb13d3a74f3298db36272c5ca65e92d9a6107006101405101527f1ef90e67437fbc8550237a75bc28e3bb9000130ea25f0c5471e144cf4264431f6107206101405101527f1e65f838515e5ff0196b49aa41a2d2568df739bc176b08ec95a79ed82932e30d6107406101405101527f2b1b045def3a166cec6ce768d079ba74b18c844e570e1f826575c1068c94c33f6107606101405101527f0832e5753ceb0ff6402543b1109229c165dc2d73bef715e3f1c6e07c168bb1736107806101405101527f02f614e9cedfb3dc6b762ae0a37d41bab1b841c2e8b6451bc5a8e3c390b6ad166107a06101405101527f0e2427d38bd46a60dd640b8e362cad967370ebb777bedff40f6a0be27e7ed7056107c06101405101527f0493630b7c670b6deb7c84d414e7ce79049f0ec098c3c7c50768bbe29214a53a6107e06101405101527f22ead100e8e482674decdab17066c5a26bb1515355d5461a3dc06cc85327cea96108006101405101527f25b3e56e655b42cdaae2626ed2554d48583f1ae35626d04de5084e0b6d2a6f166108206101405101527f1e32752ada8836ef5837a6cde8ff13dbb599c336349e4c584b4fdc0a0cf6f9d06108406101405101527f2fa2a871c15a387cc50f68f6f3c3455b23c00995f05078f672a9864074d412e56108606101405101527f2f569b8a9a4424c9278e1db7311e889f54ccbf10661bab7fcd18e7c7a7d835056108806101405101527f044cb455110a8fdd531ade530234c518a7df93f7332ffd2144165374b246b43d6108a06101405101527f227808de93906d5d420246157f2e42b191fe8c90adfe118178ddc723a53190256108c06101405101527f02fcca2934e046bc623adead873579865d03781ae090ad4a8579d2e7a68003556108e06101405101527f0ef915f0ac120b876abccceb344a1d36bad3f3c5ab91a8ddcbec2e060d8befac6109006101405101527f1797130f4b7a3e1777eb757bc6f287f6ab0fb85f6be63b09f3b16ef2b1405d386109206101405101527f0a76225dc04170ae3306c85abab59e608c7f497c20156d4d36c668555decc6e56109406101405101527f1fffb9ec1992d66ba1e77a7b93209af6f8fa76d48acb664796174b5326a31a5c6109606101405101527f25721c4fc15a3f2853b57c338fa538d85f8fbba6c6b9c6090611889b797b9c5f6109806101405101527f0c817fd42d5f7a41215e3d07ba197216adb4c3790705da95eb63b982bfcaf75a6109a06101405101527f13abe3f5239915d39f7e13c2c24970b6df8cf86ce00a22002bc15866e52b5a966109c06101405101527f2106feea546224ea12ef7f39987a46c85c1bc3dc29bdbd7a92cd60acb4d391ce6109e06101405101527f21ca859468a746b6aaa79474a37dab49f1ca5a28c748bc7157e1b3345bb0f959610a006101405101527f05ccd6255c1e6f0c5cf1f0df934194c62911d14d0321662a8f1a48999e34185b610a206101405101527f0f0e34a64b70a626e464d846674c4c8816c4fb267fe44fe6ea28678cb09490a4610a406101405101527f0558531a4e25470c6157794ca36d0e9647dbfcfe350d64838f5b1a8a2de0d4bf610a606101405101527f09d3dca9173ed2faceea125157683d18924cadad3f655a60b72f5864961f1455610a806101405101527f0328cbd54e8c0913493f866ed03d218bf23f92d68aaec48617d4c722e5bd4335610aa06101405101527f2bf07216e2aff0a223a487b1a7094e07e79e7bcc9798c648ee3347dd5329d34b610ac06101405101527f1daf345a58006b736499c583cb76c316d6f78ed6a6dffc82111e11a63fe412df610ae06101405101527f176563472456aaa746b694c60e1823611ef39039b2edc7ff391e6f2293d2c404610b00610140510152610b206101405101907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000182527f10dc6e9c006ea38b04b1e03b4bd9490c0d03f98929ca1d7fb56821fd19d3b6e7610b406101405101527f0c28145b6a44df3e0149b3d0a30b3bb599df9756d4dd9b84a86b38cfb45a740b610b606101405101527e544b8338791518b2c7645a50392798b21f75bb60e3596170067d00141cac15610b806101405101527f222c01175718386f2e2e82eb122789e352e105a3b8fa852613bc534433ee428b610ba06101405101526803000000000000000060c0528260a0526101a05161012052600960805260e052516101c0526101c0516101c05160c05183086101c05160c0518408086101c051906101c0516101c0516101a051608051086101c05160c051800808906101c05190800808610180526101c0516101c0516101a051608051086101c0516101a05160805108086101c051906101c0516101c05160c05185086101c0516101a051800808906101c05190800808610160526101c051610160516101c0516101c0516101a051608051086101c05160c051800808086101c0516101805160c05192939192909183918291086101c0516101a0518008080861018051608092909252610160516101205260a05260c0525b610220610140510160e05110612d045760e051610700016101005261014051610b40810151610b60820151610b80830151610ba09093015192905b6101005160e05110612c74575050505061020060e051015b8060e05110612c3957506101405160405260805190565b612c556101c05160e05160c05160a05161012051608051612f50565b926080929192526101205260a05260c052608060e0510160e052612c22565b6101c05160e05151608051086101c0518180096101c051919082908009096101c0516101c05160c05160a051086101c0516101205184080890816101c05191866101c0519109086080526101c051816101c05184610120510908610120526101c051816101c0518560a051090860a0526101c051906101c0518660c051090860c052602060e0510160e052612c0a565b612d206101c05160e05160c05160a05161012051608051612f50565b926080929192526101205260a05260c052608060e0510160e052612bcf565b919091612d4b83612f0c565b7fffffffff000000000000000000000000000000000000000000000000000000006040519160e01b16602082015260048152612d8860248261070b565b925f915b818310612d9857505050565b909193612da961125086858561110e565b612db487868661110e565b35600281101561040557600193602083600593612e579583977fffffffff00000000000000000000000000000000000000000000000000000000612df783612f0c565b60e01b167fff000000000000000000000000000000000000000000000000000000000000006040519a888c995191829101898b015e88019460f81b16868501526021840152602583013701015f838201520301601f19810183528261070b565b94019190612d8c565b67ffffffffffffffff811161040957601f01601f191660200190565b81601f8201121561040557803590612e9382612e60565b92612ea1604051948561070b565b8284526020838301011161040557815f926020809301838601378301015290565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9091612efe6107c193604084526040840190612ec2565b916020818403910152612ec2565b63ffffffff8111612f205763ffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b9391949290946102c052610320526102805261034052610340519061028051519008610300526103405190602061028051015190086102e052610340516102e0516102e051096102a0526103405160406102805101516102c05108610340516060610280510151610320510861024052610340516102405161024051096102205261034051906103405161024051610340516102205161022051090961034051909181808280096103405183800909090861026052610340516102605161034051610340516102e051610340516102a0516102a0510909610340516102e0516102a0518290800909080861020052610340516102e0516102a05182918291829080090961034051610300518180828009610340516103005180090909086103405161034051610240516103405161022051610220510909610340516102405161022051829080090908086101e05261034051610260518008610340516101e05190918290800808906103405161034051610340516102e051610340516102a0516102a0510909610340516103005181808280096103405161030051800909090861034051610340516102e051610340516102a0516102a05109096103405161030051818082800961034051610300518009090908086103405161020051909182908008089161034051836101e051089161034051826102005108929392919056fea164736f6c6343000826000a070606050602050406020302050403010605020503030401050503040000000030644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x03\x80`@R4a\0\xD5W`@Q`\x1FaK\xC18\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xD9W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\xD5WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x80\x82\x03a\0\xD5W\x15a\0\xC6W`\x80R\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2` a\0\x8Aa\x015V[\x80_Ra\x01\x81\x82R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA1`@Qa1^\x90\x81a\x1A#\x829`\x80Q\x81\x81\x81`\xBB\x01Ra\x02\x14\x01R\xF3[c6-\xD4\x1F`\xE0\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x91\x90\x82\x03\x91\x82\x11a\0\xFAWV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90`\x80\x81\x10\x15a\x01!W`\x03\x02\x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_T\x80\x15a\x04\xCFW`@Q\x90`\x80\x82\x01`\x01`\x01`@\x1B\x03\x81\x11\x83\x82\x10\x17a\0\xD9W`@R` \x82\x01`@\x83\x01\x90`\x01\x83\x01a\x01\xD9\x81_Q` aK\x81_9_Q\x90_Ro\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82`\x01\x80`\x80\x1B\x03\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1C`\x01\x80`@\x1B\x03\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90_\x19`\x01\x83\x1B\x01\x91\x86R\x80\x83R\x80`\x01\x1C\x82\x82`\x02\x1C\x19\x83\x19\x17\x82\x16\x16\x85R\x19\x16\x16``\x85\x01R`\x04\x83\x01\x83\x11a\0\xFAW\x83Q\x90Q_\x93`\x05\x01\x81\x81\x18\x92\x91\x83\x83\x1Ca\x03\x86WPa\x02\xB6\x94P_\x19\x92_Q` aK\x81_9_Q\x90_R`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1B`\x01`\x01`\x80\x1B\x03\x85\x11`\x07\x1B\x17\x84\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90P\x90\x03\x01\x92\x90[a\x02\xA9\x84\x83_a\x04\xD4V[\x93\x91\x94\x90\x93\x95\x82Qa\0\xEDV[\x91`\x01\x92\x83[a\x02\xDCW[PPPP_\x14a\x02\xD8W\x90a\x02\xD5\x91a\x06*V[\x90V[P\x90V[\x80\x96\x92\x93\x96\x15a\x03~W_\x19\x01a\x03\r\x81\x85`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[`\x80\x81\x10\x15a\x01!W`\x03\x02\x87\x01T\x84Q_\x19\x81\x01\x94\x90\x85\x11a\0\xFAW\x88\x94\x83\x03a\x03<WP\x96[\x93\x92a\x02\xBCV[\x91\x96\x90\x95\x84\x80\x85Q\x8A\x1C\x16\x14_\x14a\x03lW\x15a\x03fW\x90a\x03]\x91a\x06*V[\x94\x82\x94\x96a\x035V[Pa\x03]V[\x95a\x03x\x91\x97\x92a\x06*V[\x96a\x035V[\x82\x96Pa\x02\xC1V[\x90\x82\x93P`\x02\x81\x93\x01\x1C\x15a\x03\xA4W[PPP\x91a\x02\xB6\x91\x92a\x02\x9EV[a\x02\xB6\x94P\x19`\x01\x83\x1B_\x19\x01\x16\x80a\x04WWP\x81[`\x01\x81\x81\x01\x92\x90\x83\x1B_\x19\x01\x16\x80a\x03\xE0WPP\x90[\x81\x80\x82\x10\x91\x18\x02\x18\x91_\x80a\x03\x96V[_Q` aK\x81_9_Q\x90_Ro\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE`\x01`\x01`\x80\x1B\x03\x83\x11`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90\x03\x91\x90Pa\x03\xD0V[_Q` aK\x81_9_Q\x90_Ro\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE`\x01`\x01`\x80\x1B\x03\x83\x11`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x82\x03_\x19\x01\x91a\x03\xBAV[P_\x90V[\x92_\x93_\x93\x80\x15a\x06\x1DWa\x04\xEA\x90\x84Qa\0\xEDV[\x90`@\x84\x01Q\x82\x1C`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x82\x01\x90\x81\x83\x11a\0\xFAW`\x01\x01\x91a\x05\xB3a\x05\xAD\x82\x87`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x84a\x01\x0EV[P\x90\x82\x03a\x06\x14WT[\x93`\x01\x82\x01\x80\x92\x11a\0\xFAW\x80Q\x82\x10a\x05\xD6WPPPV[`\x02\x94\x96Pa\x06\x0B\x93\x95P\x90a\x06\x05\x91`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x90a\x01\x0EV[P\x01T\x91`\x01\x91V[`\x01\x01Ta\x05\xBDV[P_\x94P\x84\x93P\x83\x92PPV[\x90_Q` aK\xA1_9_Q\x90_R\x80\x92_\x08a\x01\xC0R_\x08`@Qa\x01`\x81\x90Ra\x0B\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\0\xD9W`@Ra\x0B\xA0a\x01`QR` a\x01`Q\x01\x7F\x19\xB8I\xF6\x94P\xB0hH\xDA\x1D9\xBD^JC\x02\xBB\x86tN\xDC&#\x8B\x08x\xE2i\xED#\xE5\x81R\x7F&]\xDF\xE1'\xDDQ\xBDr94{u\x8F\n\x13 \xEB,\xC7E\n\xCC\x1D\xADG\xF8\x0C\x8D\xCF4\xD6`@a\x01`Q\x01R\x7F\x19\x97P\xECG/\x18\t\xE0\xF6jT^\x1EQbA\x08\xAC\x84P\x15\xC2\xAA=\xFC6\xBA\xB4\x97\xD8\xAA``a\x01`Q\x01R\x7F\x15\x7F\xF3\xFEe\xACr\x08\x11\x0F\x06\xA5\xF7C\x02\xB1Mt>\xA2Pg\xF0\xFF\xD02\xF7\x87\xC7\xF1\xCD\xF8`\x80a\x01`Q\x01R\x7F.I\xC4<Ei\xDD\x9C_\xD3Z\xC4_\xCA3\xF1\x0B\x15\xC5\x90i/\x8B\xEE\xFE\x18\xF4\x89j\xC9I\x02`\xA0a\x01`Q\x01R\x7F\x0E5\xFB\x89\x98\x18\x90R\rJ\xEF+me\x06\xC3\xCB/\x0Bis\xC2O\xA8'14_\xFA-\x1F\x1E`\xC0a\x01`Q\x01R\x7F%\x1A\xD4|\xB1\\O\x11\x05\xF1\t\xAE^\x94O\x1B\xA9\xD9\xE7\x80mf\x7F\xFE\xC6\xFEr0\x02\xE0\xB9\x96`\xE0a\x01`Q\x01R\x7F\x13\xDA\x07\xDCd\xD4(6\x98s\xE9q`#FA\xF8\xBE\xB5o\xDD\x05\xE5\xF3V?\xA3\x9D\x9C\"\xDFNa\x01\0a\x01`Q\x01R\x7F\x0C\0\x9B\x84\xE6P\xE6\xD2=\xC0\x0C}\xCC\xEFt\x83\xA5S\x93\x96\x89\xD3P\xCDF\xE7\xB8\x90U\xFDG8a\x01 a\x01`Q\x01R\x7F\x01\x1F\x16\xB1\xC6:\x85O\x01\x99.9V\xF4-\x8B\x04\xEBe\x0CmS^\xB0 =\xECt\xBE\xFD\xCA\x06a\x01@a\x01`Q\x01R\x7F\x0E\xD6\x9E^8:h\x8F \x9D\x9AV\x1D\xAAya/?x\xD0Fz\xD4T\x85\xDF\x07\t?6uIa\x01`\x80Q\x01R\x7F\x04\xDB\xA9J{\x0C\xE9\xE2!\xAC\xADAG+k\xBE:\xECP\x7F^\xB3\xD3?F6r&L\x9Fx\x9Ba\x01\x80a\x01`Q\x01R\x7F\n?&7\xD8@\xF3\xA1n\xB0\x94'\x1C\x9D#{`6u}K\xB5\x0B\xF7\xCEs/\xF1\xD4\xFA(\xE8a\x01\xA0a\x01`Q\x01R\x7F%\x9Afo\x12\x9E\xEA\x19\x8F\x8A\x1CP/\xDB8\xFA9\xB1\xF0uV\x95d\xB6\xE5JH]\x11\x822?a\x01\xC0a\x01`Q\x01R\x7F(\xBFtY\xC9\xB2\xF4\xC6\xD8\xE7\xD0jN\xE3\xA4\x7FwE\xD4'\x108\xE5\x15z2\xFD\xF7\xED\xE0\xD6\xA1a\x01\xE0a\x01`Q\x01R\x7F\n\x1C\xA9A\xF0W\x03u&\xEA \x0FH\x9B\xE8\xD4\xC3|\x85\xBB\xCC\xE6\xA2\xAE\xEC\x91\xBDiAC$Ga\x02\0a\x01`Q\x01R\x7F\x0Co\x8F\x95\x8B\xE0\xE90S\xD7\xFDO\xC5E\x12\x85U5\xED\x159\xF0Q\xDC\xB4:&\xFD\x92ca\xCFa\x02 a\x01`Q\x01R\x7F\x121\x06\xA9<\xD1ux\xD4&\xE8\x12\x8A\xC9\xD9\n\xA9\xE8\xA0\x07\x08\xE2\x96\xE0\x84\xDDW\xE6\x9C\xAA\xF8\x11a\x02@a\x01`Q\x01R\x7F&\xE1\xBAR\xAD\x92\x85\xD9}\xD3\xABR\xF8\xE8@\x08^\x8F\xA8?\xF1\xE8\xF1\x87{\x07Hg\xCD-\xEEua\x02`a\x01`Q\x01R\x7F\x1C\xB5\\\xAD{\xD13\xDE\x18\xA6L\\G\xB9\xC9|\xBEM\x8B{\xF9\xE0\x95\x86DqS~jJ\xE2\xC5a\x02\x80a\x01`Q\x01R\x7F\x1D\xCDs\xE4j\xCD\x8F\x8E\x0E,|\xE0K\xDE\x7Fm*S\x04=P`\xA4\x1CqC\xF0\x8En\x90U\xD0a\x02\xA0a\x01`Q\x01R\x7F\x01\x10\x03\xE3/m\x9Cf\xF5\x85/\x05GJM\xEF\x0C\xDA)J\x0E\xB4\xE9\xB9\xB1+\x9B\xB4Q.Uta\x02\xC0a\x01`Q\x01R\x7F+\x1E\x80\x9A\xC1\xD1\n\xB2\x9A\xD5\xF2\r\x03\xA5}\xFE\xBA\xDF\xE5\x90?X\xBA\xFE\xD7\xC5\x08\xDD\"\x87\xAE\x8Ca\x02\xE0a\x01`Q\x01R\x7F%9\xDE\x17\x85\xB75\x99\x9F\xB4\xDA\xC3^\xE1~\xD0\xEF\x99]\x05\xAB/\xC5\xFA\xEA\xA6\x9A\xE8{\xCE\xC0\xA5a\x03\0a\x01`Q\x01R\x7F\x0C$lZ.\xF8\xEE\x01&I\x7F\"+>\n\x0E\xF4\xE1\xC3\xD4\x1C\x86\xD4nC\x98,\xB1\x1Dw\x95\x1Da\x03 a\x01`Q\x01R\x7F\x19 \x89\xC4\x97Oh\xE9T\x08\x14\x8F|\x062\xED\xBB\t\xE6\xA6\xAD\x1A\x1C/?\x03\x05\xF5\xD0;R{a\x03@a\x01`Q\x01R\x7F\x1E\xAE\n\xD8\xABh\xB2\xF0j\x0E\xE3n\xEB\r\x0C\x05\x85)\t}\x91\tkum\x8F\xDC/\xB5\xA6\r\x85a\x03`a\x01`Q\x01R\x7F\x17\x91\x90\xE5\xD0\xE2!y\xE4o\x82\x82\x87*\xBC\x88\xDBn/\xDC\r\xEE\x99\xE6\x97h\xBD\x98\xC5\xD0k\xFBa\x03\x80a\x01`Q\x01R\x7F)\xBB\x9E,\x90vs%v\xE9\xA8\x1Cz\xC4\xB82\x14R\x8F}\xB0\x0F1\xBFl\xAF\xE7\x94\xA9\xB3\xCD\x1Ca\x03\xA0a\x01`Q\x01R\x7F\"]9NB u\x99@>\xFD\x0C$d\xA9\rRe&E\x88*\xAC5\xB1\x0EY\x0Eni\x1E\x08a\x03\xC0a\x01`Q\x01R\x7F\x06G`b<%\xC8\xCFu=#\x80U\xB4DS+\xE15WE\x1C\x08}\xE0\x9E\xFDEK#\xFDYa\x03\xE0a\x01`Q\x01R\x7F\x10\xBA:\x0E\x01\xDF\x92\xE8\x7F0\x1CKqm\x8A9Mg\xF4\xBFB\xA7\\\x10\x92)\x10\xA7\x8Fk[\x87a\x04\0a\x01`Q\x01R\x7F\x0E\x07\x0B\xF5?\x84Q\xB2O\x9Cn\x96\xB0\xC2\xA8\x01\xCBQ\x1B\xC0\xC2B\xEB\x9D6\x1Bwi?!G\x1Ca\x04 a\x01`Q\x01R\x7F\x1B\x94\xCDa\xB0Q\xB0M\xD3\x97U\xFF\x93\x82\x1As\xCC\xD6\xCB\x11\xD2I\x1D\x8A\xA7\xF9!\x01M\xE2R\xFBa\x04@a\x01`Q\x01R\x7F\x1D|\xB3\x9B\xAF\xB8\xC7D\xE1Hxz.p#\x0F\x9DN\x91}W\x13\xBB\x05\x04\x87\xB5\xAA}t\x07\x0Ba\x04`a\x01`Q\x01R\x7F.\xC91\x89\xBD\x1A\xB4\xF6\x91\x17\xD0\xFE\x98\x0C\x80\xFF\x87\x85\xC2\x96\x18)\xF7\x01\xBBt\xAC\x1F0;\x17\xDBa\x04\x80a\x01`Q\x01R\x7F-\xB3f\xBF\xDD6\xD2w\xA6\x92\xBB\x82[\x86'[\xEA\xC4\x04\xA1\x9A\xE0z\x90\x82\xEAF\xBD\x83Qy&a\x04\xA0a\x01`Q\x01R\x7F\x06!\0\xEBH]\xB0bie\\\xF1\x86\xA6\x852\x98RuB\x84P5\x9A\xDC\x99\xCE\xC6\x96\x07\x11\xB8a\x04\xC0a\x01`Q\x01R\x7F\x07a\xD3<faJ\xAAW\x0E\x7F\x1E\x82D\xCA\x11 $?\x92\xFAY\xE4\xF9\0\xC5g\xBFA\xF5\xA5\x9Ba\x04\xE0a\x01`Q\x01R\x7F \xFCA\x1A\x11M\x13\x99,'\x05\xAA\x03N?1]x`\x8A\x0F}\xE4\xCC\xF7\xA7.IHU\xAD\ra\x05\0a\x01`Q\x01R\x7F%\xB5\xC0\x04\xA4\xBD\xFC\xB5\xAD\xD9\xECN\x9A\xB2\x19\xBA\x10,g\xE8\xB3\xEF\xFB_\xC3\xA3\x0F1rP\xBCZa\x05 a\x01`Q\x01R\x7F#\xB1\x82-'\x8E\xD62\xA4\x94\xE5\x8Fm\xF6\xF5\xED\x03\x8B\x18m\x84t\x15Z\xD8~}\xFFb\xB3\x7FKa\x05@a\x01`Q\x01R\x7F\"sKL\\?\x94\x93`lK\xA9\x01$\x99\xBF\x0F\x14\xD1;\xFC\xFC\xCC\xAA\x16\x10*)\xCC/i\xE0a\x05`a\x01`Q\x01R\x7F&\xC0\xC8\xFE\t\xEB0\xB7\xE2zt\xDC3I#G\xE5\xBD\xFF@\x9A\xA3a\x02TA=?\xADy\\\xE5a\x05\x80a\x01`Q\x01R\x7F\x07\r\xD0\xCC\xB6\xBD{\xBA\xE8\x8E\xAC\x03\xFA\x1F\xBB&\x19k\xE3\x08:\x80\x98)\xBB\xD6&\xDF4\x8C\xCA\xD9a\x05\xA0a\x01`Q\x01R\x7F\x12\xB6Y[\xDB2\x9Bo\xB0C\xBAx\xBB(\xC3\xBE\xC2\xC0\xA6\xDEF\xD8\xC5\xAD`g\xC4\xEB\xFDBP\xDAa\x05\xC0a\x01`Q\x01R\x7F$\x8D\x97\xD7\xF7b\x83\xD6;\xEC0\xE7\xA5\x87l\x11\xC0o\xCA\x9B'\\g\x1C^3\xD9[\xB7\xE8\xD7)a\x05\xE0a\x01`Q\x01R\x7F\x1A0mC\x9DF;\x08\x16\xFCo\xD6L\xC991\x8BE\xEBu\x9D\xDD\xE4\xAA\x10m\x15\xD9\xBD\x9B\xAA\xAAa\x06\0a\x01`Q\x01R\x7F(\xA8\xF87.<8\xDA\xCE\xD7\xC0\x04!\xCBF!\xF4\xF1\xB5M\xDC'\x82\x1B\rb\xD3\xD6\xEC|V\xCFa\x06 a\x01`Q\x01R~\x94\x97W\x17\xF9\xA8\xA8\xBB5\x15/$\xD42\x94\x07\x1C\xE3 \xC8)\xF3\x88\xBC\x85!\x83\xE1\xE2\xCE~a\x06@a\x01`Q\x01R\x7F\x04\xD5\xEEL:\xA7\x8F}\x80\xFD\xE6\rqd\x80\xD3Y?t\xD4\xF6S\xAE\x83\xF4\x102F\xDB.\x8Dea\x06`a\x01`Q\x01R\x7F*l\xF5\xE9\xAA\x03\xD43cI\xADo\xB8\xED\"i\xC7\xBE\xF5K\x88\"\xCCv\xD0\x84\x95\xC1.\xFD\xE1\x87a\x06\x80a\x01`Q\x01R\x7F#\x04\xD3\x1E\xAA\xB9`\xBA\x92t\xDAC\xE1\x9D\xDE\xB7\xF7\x92\x18\x08\x08\xFDnC\xBA\xAEH\xD7\xEF\xCB\xA3\xF3a\x06\xA0a\x01`Q\x01R\x7F\x03\xFD\x9A\xC8e\xA4\xB2\xA6\xD5\xE7\0\x97\x85\x81rI\xBF\xF0\x8A~\x07&\xFC\xB4\xE1\xC1\x1D9\xD1\x99\xF0\xB0a\x06\xC0a\x01`Q\x01R~\xB7%\x8D\xEDR\xBB\xDA\"H@MU\xEEPDy\x8A\xFC: \x91\x93\x07?yT\xD4\xD6;\x0Bda\x06\xE0a\x01`Q\x01R\x7F\x15\x9F\x81\xAD\xA0w\x17\x99\xEC8\xFC\xA2\xD4\xBFe\xEB\xB1=:t\xF3)\x8D\xB3br\xC5\xCAe\xE9-\x9Aa\x07\0a\x01`Q\x01R\x7F\x1E\xF9\x0EgC\x7F\xBC\x85P#zu\xBC(\xE3\xBB\x90\0\x13\x0E\xA2_\x0CTq\xE1D\xCFBdC\x1Fa\x07 a\x01`Q\x01R\x7F\x1Ee\xF88Q^_\xF0\x19kI\xAAA\xA2\xD2V\x8D\xF79\xBC\x17k\x08\xEC\x95\xA7\x9E\xD8)2\xE3\ra\x07@a\x01`Q\x01R\x7F+\x1B\x04]\xEF:\x16l\xECl\xE7h\xD0y\xBAt\xB1\x8C\x84NW\x0E\x1F\x82eu\xC1\x06\x8C\x94\xC3?a\x07`a\x01`Q\x01R\x7F\x082\xE5u<\xEB\x0F\xF6@%C\xB1\x10\x92)\xC1e\xDC-s\xBE\xF7\x15\xE3\xF1\xC6\xE0|\x16\x8B\xB1sa\x07\x80a\x01`Q\x01R\x7F\x02\xF6\x14\xE9\xCE\xDF\xB3\xDCkv*\xE0\xA3}A\xBA\xB1\xB8A\xC2\xE8\xB6E\x1B\xC5\xA8\xE3\xC3\x90\xB6\xAD\x16a\x07\xA0a\x01`Q\x01R\x7F\x0E$'\xD3\x8B\xD4j`\xDDd\x0B\x8E6,\xAD\x96sp\xEB\xB7w\xBE\xDF\xF4\x0Fj\x0B\xE2~~\xD7\x05a\x07\xC0a\x01`Q\x01R\x7F\x04\x93c\x0B|g\x0Bm\xEB|\x84\xD4\x14\xE7\xCEy\x04\x9F\x0E\xC0\x98\xC3\xC7\xC5\x07h\xBB\xE2\x92\x14\xA5:a\x07\xE0a\x01`Q\x01R\x7F\"\xEA\xD1\0\xE8\xE4\x82gM\xEC\xDA\xB1pf\xC5\xA2k\xB1QSU\xD5F\x1A=\xC0l\xC8S'\xCE\xA9a\x08\0a\x01`Q\x01R\x7F%\xB3\xE5ne[B\xCD\xAA\xE2bn\xD2UMHX?\x1A\xE3V&\xD0M\xE5\x08N\x0Bm*o\x16a\x08 a\x01`Q\x01R\x7F\x1E2u*\xDA\x886\xEFX7\xA6\xCD\xE8\xFF\x13\xDB\xB5\x99\xC364\x9ELXKO\xDC\n\x0C\xF6\xF9\xD0a\x08@a\x01`Q\x01R\x7F/\xA2\xA8q\xC1Z8|\xC5\x0Fh\xF6\xF3\xC3E[#\xC0\t\x95\xF0Px\xF6r\xA9\x86@t\xD4\x12\xE5a\x08`a\x01`Q\x01R\x7F/V\x9B\x8A\x9AD$\xC9'\x8E\x1D\xB71\x1E\x88\x9FT\xCC\xBF\x10f\x1B\xAB\x7F\xCD\x18\xE7\xC7\xA7\xD85\x05a\x08\x80a\x01`Q\x01R\x7F\x04L\xB4U\x11\n\x8F\xDDS\x1A\xDES\x024\xC5\x18\xA7\xDF\x93\xF73/\xFD!D\x16St\xB2F\xB4=a\x08\xA0a\x01`Q\x01R\x7F\"x\x08\xDE\x93\x90m]B\x02F\x15\x7F.B\xB1\x91\xFE\x8C\x90\xAD\xFE\x11\x81x\xDD\xC7#\xA51\x90%a\x08\xC0a\x01`Q\x01R\x7F\x02\xFC\xCA)4\xE0F\xBCb:\xDE\xAD\x875y\x86]\x03x\x1A\xE0\x90\xADJ\x85y\xD2\xE7\xA6\x80\x03Ua\x08\xE0a\x01`Q\x01R\x7F\x0E\xF9\x15\xF0\xAC\x12\x0B\x87j\xBC\xCC\xEB4J\x1D6\xBA\xD3\xF3\xC5\xAB\x91\xA8\xDD\xCB\xEC.\x06\r\x8B\xEF\xACa\t\0a\x01`Q\x01R\x7F\x17\x97\x13\x0FKz>\x17w\xEBu{\xC6\xF2\x87\xF6\xAB\x0F\xB8_k\xE6;\t\xF3\xB1n\xF2\xB1@]8a\t a\x01`Q\x01R\x7F\nv\"]\xC0Ap\xAE3\x06\xC8Z\xBA\xB5\x9E`\x8C\x7FI| \x15mM6\xC6hU]\xEC\xC6\xE5a\t@a\x01`Q\x01R\x7F\x1F\xFF\xB9\xEC\x19\x92\xD6k\xA1\xE7z{\x93 \x9A\xF6\xF8\xFAv\xD4\x8A\xCBfG\x96\x17KS&\xA3\x1A\\a\t`a\x01`Q\x01R\x7F%r\x1CO\xC1Z?(S\xB5|3\x8F\xA58\xD8_\x8F\xBB\xA6\xC6\xB9\xC6\t\x06\x11\x88\x9By{\x9C_a\t\x80a\x01`Q\x01R\x7F\x0C\x81\x7F\xD4-_zA!^=\x07\xBA\x19r\x16\xAD\xB4\xC3y\x07\x05\xDA\x95\xEBc\xB9\x82\xBF\xCA\xF7Za\t\xA0a\x01`Q\x01R\x7F\x13\xAB\xE3\xF5#\x99\x15\xD3\x9F~\x13\xC2\xC2Ip\xB6\xDF\x8C\xF8l\xE0\n\"\0+\xC1Xf\xE5+Z\x96a\t\xC0a\x01`Q\x01R\x7F!\x06\xFE\xEATb$\xEA\x12\xEF\x7F9\x98zF\xC8\\\x1B\xC3\xDC)\xBD\xBDz\x92\xCD`\xAC\xB4\xD3\x91\xCEa\t\xE0a\x01`Q\x01R\x7F!\xCA\x85\x94h\xA7F\xB6\xAA\xA7\x94t\xA3}\xABI\xF1\xCAZ(\xC7H\xBCqW\xE1\xB34[\xB0\xF9Ya\n\0a\x01`Q\x01R\x7F\x05\xCC\xD6%\\\x1Eo\x0C\\\xF1\xF0\xDF\x93A\x94\xC6)\x11\xD1M\x03!f*\x8F\x1AH\x99\x9E4\x18[a\n a\x01`Q\x01R\x7F\x0F\x0E4\xA6Kp\xA6&\xE4d\xD8FgLL\x88\x16\xC4\xFB&\x7F\xE4O\xE6\xEA(g\x8C\xB0\x94\x90\xA4a\n@a\x01`Q\x01R\x7F\x05XS\x1AN%G\x0CaWyL\xA3m\x0E\x96G\xDB\xFC\xFE5\rd\x83\x8F[\x1A\x8A-\xE0\xD4\xBFa\n`a\x01`Q\x01R\x7F\t\xD3\xDC\xA9\x17>\xD2\xFA\xCE\xEA\x12QWh=\x18\x92L\xAD\xAD?eZ`\xB7/Xd\x96\x1F\x14Ua\n\x80a\x01`Q\x01R\x7F\x03(\xCB\xD5N\x8C\t\x13I?\x86n\xD0=!\x8B\xF2?\x92\xD6\x8A\xAE\xC4\x86\x17\xD4\xC7\"\xE5\xBDC5a\n\xA0a\x01`Q\x01R\x7F+\xF0r\x16\xE2\xAF\xF0\xA2#\xA4\x87\xB1\xA7\tN\x07\xE7\x9E{\xCC\x97\x98\xC6H\xEE3G\xDDS)\xD3Ka\n\xC0a\x01`Q\x01R\x7F\x1D\xAF4ZX\0ksd\x99\xC5\x83\xCBv\xC3\x16\xD6\xF7\x8E\xD6\xA6\xDF\xFC\x82\x11\x1E\x11\xA6?\xE4\x12\xDFa\n\xE0a\x01`Q\x01R\x7F\x17ecG$V\xAA\xA7F\xB6\x94\xC6\x0E\x18#a\x1E\xF3\x909\xB2\xED\xC7\xFF9\x1Eo\"\x93\xD2\xC4\x04a\x0B\0a\x01`Q\x01Ra\x0B a\x01`Q\x01\x90_Q` aK\xA1_9_Q\x90_R\x82R\x7F\x10\xDCn\x9C\0n\xA3\x8B\x04\xB1\xE0;K\xD9I\x0C\r\x03\xF9\x89)\xCA\x1D\x7F\xB5h!\xFD\x19\xD3\xB6\xE7a\x0B@a\x01`Q\x01R\x7F\x0C(\x14[jD\xDF>\x01I\xB3\xD0\xA3\x0B;\xB5\x99\xDF\x97V\xD4\xDD\x9B\x84\xA8k8\xCF\xB4Zt\x0Ba\x0B`a\x01`Q\x01R~TK\x838y\x15\x18\xB2\xC7dZP9'\x98\xB2\x1Fu\xBB`\xE3Yap\x06}\0\x14\x1C\xAC\x15a\x0B\x80a\x01`Q\x01R\x7F\",\x01\x17W\x188o..\x82\xEB\x12'\x89\xE3R\xE1\x05\xA3\xB8\xFA\x85&\x13\xBCSD3\xEEB\x8Ba\x0B\xA0a\x01`Q\x01Rh\x03\0\0\0\0\0\0\0\0`\xE0R\x82`\xC0Ra\x01\xC0Qa\x01@R`\t`\xA0Ra\x01\0RQa\x01\xE0Ra\x01\xE0Qa\x01\xE0Q`\xE0Q\x83\x08a\x01\xE0Q`\xE0Q\x84\x08\x08a\x01\xE0Q\x90a\x01\xE0Qa\x01\xE0Qa\x01\xC0Q`\xA0Q\x08a\x01\xE0Q`\xE0Q\x80\x08\x08\x90a\x01\xE0Q\x90\x80\x08\x08a\x01\xA0Ra\x01\xE0Qa\x01\xE0Qa\x01\xC0Q`\xA0Q\x08a\x01\xE0Qa\x01\xC0Q`\xA0Q\x08\x08a\x01\xE0Q\x90a\x01\xE0Qa\x01\xE0Q`\xE0Q\x85\x08a\x01\xE0Qa\x01\xC0Q\x80\x08\x08\x90a\x01\xE0Q\x90\x80\x08\x08a\x01\x80Ra\x01\xE0Qa\x01\x80Qa\x01\xE0Qa\x01\xE0Qa\x01\xC0Q`\xA0Q\x08a\x01\xE0Q`\xE0Q\x80\x08\x08\x08a\x01\xE0Qa\x01\xA0Q`\xE0Q\x92\x93\x91\x92\x90\x91\x83\x91\x82\x91\x08a\x01\xE0Qa\x01\xC0Q\x80\x08\x08\x08a\x01\xA0Q`\xA0\x92\x90\x92Ra\x01\x80Qa\x01@R`\xC0R`\xE0R[a\x02 a\x01`Q\x01a\x01\0Q\x10a\x17\xE3Wa\x01\0Qa\x07\0\x01a\x01 Ra\x01`Qa\x0B@\x81\x01Qa\x0B`\x82\x01Qa\x0B\x80\x83\x01Qa\x0B\xA0\x90\x93\x01Q\x92\x90[a\x01 Qa\x01\0Q\x10a\x17PWPPPPa\x02\0a\x01\0Q\x01[\x80a\x01\0Q\x10a\x17\x12WPa\x01`Q`@R`\xA0Q\x90V[a\x17/a\x01\xE0Qa\x01\0Q`\xE0Q`\xC0Qa\x01@Q`\xA0Qa\x18!V[\x92`\xA0\x92\x91\x92Ra\x01@R`\xC0R`\xE0R`\x80a\x01\0Q\x01a\x01\0Ra\x16\xFAV[a\x01\xE0Qa\x01\0QQ`\xA0Q\x08a\x01\xE0Q\x81\x80\ta\x01\xE0Q\x91\x90\x82\x90\x80\t\ta\x01\xE0Qa\x01\xE0Q`\xE0Q`\xC0Q\x08a\x01\xE0Qa\x01@Q\x84\x08\x08\x90\x81a\x01\xE0Q\x91\x86a\x01\xE0Q\x91\t\x08`\xA0Ra\x01\xE0Q\x81a\x01\xE0Q\x84a\x01@Q\t\x08a\x01@Ra\x01\xE0Q\x81a\x01\xE0Q\x85`\xC0Q\t\x08`\xC0Ra\x01\xE0Q\x90a\x01\xE0Q\x86`\xE0Q\t\x08`\xE0R` a\x01\0Q\x01a\x01\0Ra\x16\xE0V[a\x18\0a\x01\xE0Qa\x01\0Q`\xE0Q`\xC0Qa\x01@Q`\xA0Qa\x18!V[\x92`\xA0\x92\x91\x92Ra\x01@R`\xC0R`\xE0R`\x80a\x01\0Q\x01a\x01\0Ra\x16\xA3V[\x93\x91\x94\x92\x90\x94a\x02\xE0Ra\x03@Ra\x02\xA0Ra\x03`Ra\x03`Q\x90a\x02\xA0QQ\x90\x08a\x03 Ra\x03`Q\x90` a\x02\xA0Q\x01Q\x90\x08a\x03\0Ra\x03`Qa\x03\0Qa\x03\0Q\ta\x02\xC0Ra\x03`Q`@a\x02\xA0Q\x01Qa\x02\xE0Q\x08a\x03`Q``a\x02\xA0Q\x01Qa\x03@Q\x08a\x02`Ra\x03`Qa\x02`Qa\x02`Q\ta\x02@Ra\x03`Q\x90a\x03`Qa\x02`Qa\x03`Qa\x02@Qa\x02@Q\t\ta\x03`Q\x90\x91\x81\x80\x82\x80\ta\x03`Q\x83\x80\t\t\t\x08a\x02\x80Ra\x03`Qa\x02\x80Qa\x03`Qa\x03`Qa\x03\0Qa\x03`Qa\x02\xC0Qa\x02\xC0Q\t\ta\x03`Qa\x03\0Qa\x02\xC0Q\x82\x90\x80\t\t\x08\x08a\x02 Ra\x03`Qa\x03\0Qa\x02\xC0Q\x82\x91\x82\x91\x82\x90\x80\t\ta\x03`Qa\x03 Q\x81\x80\x82\x80\ta\x03`Qa\x03 Q\x80\t\t\t\x08a\x03`Qa\x03`Qa\x02`Qa\x03`Qa\x02@Qa\x02@Q\t\ta\x03`Qa\x02`Qa\x02@Q\x82\x90\x80\t\t\x08\x08a\x02\0Ra\x03`Qa\x02\x80Q\x80\x08a\x03`Qa\x02\0Q\x90\x91\x82\x90\x80\x08\x08\x90a\x03`Qa\x03`Qa\x03`Qa\x03\0Qa\x03`Qa\x02\xC0Qa\x02\xC0Q\t\ta\x03`Qa\x03 Q\x81\x80\x82\x80\ta\x03`Qa\x03 Q\x80\t\t\t\x08a\x03`Qa\x03`Qa\x03\0Qa\x03`Qa\x02\xC0Qa\x02\xC0Q\t\ta\x03`Qa\x03 Q\x81\x80\x82\x80\ta\x03`Qa\x03 Q\x80\t\t\t\x08\x08a\x03`Qa\x02 Q\x90\x91\x82\x90\x80\x08\x08\x91a\x03`Q\x83a\x02\0Q\x08\x91a\x03`Q\x82a\x02 Q\x08\x92\x93\x92\x91\x90V\xFEa\x03``@R`\x036\x11a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80cF{P\x11\x14a\x01OW\x80c\xBD\xEBD-\x14a\x01,W\x80c\xC1\xB0\xBE\xD7\x14a\0\xFCW\x80c\xC4IV\xD1\x14a\0\xDFW\x80c\xDB\x19\xAAy\x14a\0\x8EWc\xF8\xAD\xF4\xC3\x14a\0\\W_\x80\xFD[4a\0\x8BW` `\x03\x196\x01\x12a\0\x8BW`\xFF`@` \x92`\x045\x81Ra\x01\x81\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` \x90T`@Q\x90\x81R\xF3[P4a\0\x8BW` `\x03\x196\x01\x12a\0\x8BW`\xFF`@` \x92`\x045\x81Ra\x01\x82\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` a\x01Ga\x0E\xF7V[`@Q\x90\x81R\xF3[P4a\x04\x05W` `\x03\x196\x01\x12a\x04\x05W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05W\x80`\x04\x01`@`\x03\x19\x836\x03\x01\x12a\x04\x05W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x06\x8FW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01\xD9\x81\x80a\x06\xB7V[\x90\x91\x81\x15a\x06gW_\x93a\x01\xEC\x83a\x07FV[\x91_[\x84\x81\x10a\x046WPa\x02<\x90`$s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x01\x90a\x08\x05V[\x91\x90`@Q\x90` \x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@R\x87\x82R\x82;\x15a\x04\x05W\x92\x90a\x02\xA5`@Q\x94\x85\x93\x7F\x8D\x8F\x8A\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x91a\x08VV[\x90_`$\x84\x01[`\x01\x82\x10a\x03\xEBWPPP\x91\x81\x80_\x94\x03\x91Z\xFA\x80\x15a\x03\xE0Wa\x03\xCAW[P` a\x02\xF8\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92a\t\x03V[\x80\x87Ra\x01\x81\x82R`@\x87 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`@Q\x90\x81R\xA1\x83[\x81\x81\x10a\x03\x8AWPPP\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE\x82\x80\xA2\x80\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\x80\xF3[\x80a\x03\xA9a\x03\xA4a\x03\x9E`\x01\x94\x86\x88a\x07\xAAV[\x80a\x07wV[a\x17\xC7V[a\x03\xC4a\x03\xA4a\x03\xBA\x83\x86\x88a\x07\xAAV[` \x81\x01\x90a\x07wV[\x01a\x036V[a\x03\xD7\x91\x95P_\x90a\x07\x0BV[_\x93` a\x02\xCBV[`@Q=_\x82>=\x90\xFD[\x82Q\x81R\x86\x94P` \x92\x83\x01\x92`\x01\x92\x90\x92\x01\x91\x01a\x02\xACV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x95a\x04B\x87\x86\x88a\x07\xAAV[`@\x81\x015\x80_Ra\x01\x81` R`\xFF`@_ T\x16\x15a\x06<Wa\x04g\x82\x80a\x07wV[5\x80_Ra\x01\x82` R`\xFF`@_ T\x16a\x06\x11W\x91` \x93\x91a\x05Y\x84_\x95\x86Ra\x01\x82\x87R`@\x86 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Ua\x04\xCEa\x04\xC9\x85\x80a\x07wV[a\x11eV[\x86\x84\x01\x90a\x04\xDFa\x04\xC9\x83\x87a\x07wV[a\x04\xE9\x82\x86a\x07wV[5\x90\x7F\x98W\xD0\xB8\x8F\x01\xDC\xA7\xBB\x12\x05\xD6\x8C0}(\xD8\xD4\0/\xF5H`\x07\xACLD5\x96\xDC\xEA!`@\x8Aa\x05\x19\x89\x80a\x07wV[\x015\x8Ba\x05&\x87\x8Ba\x07wV[\x015\x82Q\x91\x82R\x8C\x82\x01R\xA3\x8Ca\x05Ha\x05@\x83\x87a\x07wV[5\x91\x8Ca\x07\xF1V[Ra\x05S\x84\x80a\x07wV[\x93a\x07wV[\x85a\x05oa\x05j`@\x86\x01\x86a\x08vV[a\x13\xC0V[\x91a\x05\x80a\x05j`@\x83\x01\x83a\x08vV[\x92\x82`@Q\x96\x81\x88\x01\x98\x89R\x805`@\x89\x01R\x015``\x87\x01R`\x80\x86\x01R\x805`\xA0\x86\x01R\x015`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01\0\x81Ra\x05\xC9a\x01 \x82a\x07\x0BV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x03\xE0W`\x01\x7F\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_Q\x16\x96\x01a\x01\xEFV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x04\x05WV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\tW`\x05\x1B` \x01\x90V[\x90a\x07P\x82a\x07.V[a\x07]`@Q\x91\x82a\x07\x0BV[\x82\x81R`\x1F\x19a\x07m\x82\x94a\x07.V[\x01\x90` 6\x91\x017V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[\x90\x82\x10\x15a\x07\xC4Wa\x07\xC1\x91`\x05\x1B\x81\x01\x90a\x07wV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x07\xC4W` \x91`\x05\x1B\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W` \x01\x91\x816\x03\x83\x13a\x04\x05WV[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x08\xB6WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x01\x90_\x90V[\x91\x90\x82\x03\x91\x82\x11a\x08\xB6WV[_T\x90\x80Q\x15a\x0E\xE0Wa\t\x18\x81Q\x83a\x08\xA9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x11a\x0E\xB5W\x81Q_[\x81\x81\x10a\x0EEWPPa\t=\x81a\x14\xA4V[\x90_\x82Q\x80`\x01\x1B\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x08\xB6W`\x02\x81\x01\x80\x91\x11a\x08\xB6Wa\t\x85\x90\x92\x91\x92a\x07FV[\x95\x85\x91\x85Q\x94_[\x84\x87\x82\x10\x15a\x0BeW_\x19\x81\x14a\x08\xB6W`\x01\x96\x87\x83\x8Da\t\xC2\x8Da\t\xBB\x85`\x02\x98\x01\x9D\x8E\x16\x94\x85\x92a\x07\xF1V[Q\x92a\x07\xF1V[R\x1B\x17\x95\x01\x80\x87\x11a\x08\xB6W\x86\x81\x16a\t\xE2W[P`\x01\x01\x94\x93\x94a\t\x8DV[\x90\x94a\nq\x82`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x90\x81`\x01\x1B\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x08\xB6W`\x01\x83\x01\x80\x84\x11\x80a\x08\xB6W`\x01\x80\x85\x84\x1C\x16\x14_\x14a\x0BNWa\x08\xB6Wa\n\xC8\x90\x8Ea\x07\xF1V[Q\x93[\x8D\x84`\x01\x80\x86\x83\x1C\x16\x14_\x14a\x0B6Wa\n\xE4\x91a\x07\xF1V[Q[`\x02\x85\x01\x95\x86\x86\x11a\x08\xB6W`\x01\x84\x01\x80\x94\x11a\x08\xB6W\x8F\x90a\x0B(\x90a\x0B\"a\x0B\x1A`\x01\x9A\x8B\x80\x98`\x03\x9A\x1C\x16\x90a\x08\xA9V[\x94\x85\x92a\x1B>V[\x92a\x07\xF1V[R\x1B\x92\x1B\x19\x16\x17\x94\x90a\t\xD6V[PP`\x80\x84\x10\x15a\x07\xC4W`\x03\x84\x02`\x01\x01Ta\n\xE6V[P`\x80\x81\x10\x15a\x07\xC4W`\x03\x02`\x01\x01T\x93a\n\xCBV[PP\x94P\x94P\x94\x90P\x81\x92_\x91\x80a\x0E\nW[P`\x04\x85\x01\x80\x86\x11a\x08\xB6Wa\x0B\x8E\x90\x82a\x15\x88V[\x91a\x0B\x9A\x81\x83_a\x19\xE2V[\x93\x91\x94\x90\x93\x95a\x0B\xAC\x83Q\x92\x83a\x08\xF6V[\x92a\x0B\xB7\x85\x84a\x08\xF6V[\x94a\r\xDCW\x8B\x94\x93\x92_[\x85_\x19\x83\x01\x92\x83\x11\x96[a\x0C\xBAWPPPPPPPP_\x14a\x0C\xB4W\x90a\x0B\xE8\x91a\x1B>V[\x93\x81`\x01\x92[a\x0B\xF9WPPP_UV[a\x0C\x86\x81`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[a\x0C\x90\x81\x84a\x07\xF1V[Q\x90`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x84\x01U_\x19\x81\x01\x90\x80\x82\x11a\x08\xB6W\x16\x80a\x0B\xEEV[Pa\x0B\xE8V[\x90a\x0C\xC9\x87\x98\x9B\x96\x97\x92a\x0E\xEBV[\x95a\x0C\xEE\x87\x86`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x80\x9C`\x01\x80\x89\x84\x1C\x16\x14_\x14a\r\xC0W\x90a\r\x08\x91a\x07\xF1V[Q\x92[a\x08\xB6W\x86\x84\x03a\r'WPP\x84\x90\x99[\x94\x8E\x97\x96\x95\x91a\x0B\xCCV[\x91\x98`\x01\x80`@\x87\x9D\x94\x9D\x01Q\x89\x1C\x16\x14_\x14a\r\x8EW\x86\x92\x91\x90\x15a\r\x87W\x90a\rQ\x91a\x1B>V[\x97[\x88\x99`\x01\x99\x88\x84\x10\x15a\riW[PP\x99a\r\x1CV[\x90\x92P`\x80\x83\x10\x15a\x07\xC4W`\x02`\x03\x88\x94\x02\x8B\x01\x01U_\x80a\raV[P\x97a\rSV[\x98a\r\x9C\x91\x9AP\x86\x92a\x1B>V[\x99\x86\x82\x10a\r\x1CW\x90P`\x80\x81\x10\x15a\x07\xC4W`\x02`\x03\x82\x02\x01\x8A\x90U\x84\x90a\r\x1CV[PP\x91P`\x80\x8B\x10\x15a\x07\xC4W`\x01`\x03\x8C\x02\x01T\x91\x88a\r\x0BV[a\x0E\0\x85\x82`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x92\x8C\x95\x94\x93a\x0B\xC2V[\x90\x91Pa\x0E\x16\x81a\x14\xA4V[a\x0E \x86\x82a\x15\x88V[\x91`\x04\x81\x01\x80\x91\x11a\x08\xB6Wa\x0E5\x91a\x15\x88V[\x90\x81\x80\x82\x10\x91\x18\x02\x18\x90_a\x0BxV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01a\x0Ep\x82\x86a\x07\xF1V[Q\x10\x15a\x0E\x7FW`\x01\x01a\t+V[a\x0E\x89\x90\x84a\x07\xF1V[Q\x7F\xC5>rk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\x08Ms\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[PPa\x07\xC1_a\x10\x04V[\x80\x15a\x08\xB6W_\x19\x01\x90V[_T\x80\x15a\x0F\xFFWa\x0F\x08\x81a\x14\xA4V[\x90`\x04\x81\x01\x80\x91\x11a\x08\xB6Wa\x0F!a\x0F:\x91\x83a\x15\x88V[\x91a\x0F-\x83\x82_a\x19\xE2V[\x92\x91\x93\x90\x92\x94\x82Qa\x08\xF6V[\x80`\x01\x91[a\x0FZWPPP_\x14a\x0FVW\x90a\x07\xC1\x91a\x1B>V[P\x90V[a\x0Fc\x90a\x0E\xEBV[\x94a\x0F\x88\x86\x84`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x82\x01T\x83Q\x91_\x19\x83\x01\x92\x83\x11a\x08\xB6W\x87\x92\x83\x03a\x0F\xB4WP\x95a\x0F?V[\x92\x95\x91\x86\x80`@\x87\x98\x94\x98\x01Q\x83\x1C\x16\x14_\x14a\x0F\xEBWP\x15a\x0F\xE5W\x90a\x0F\xDB\x91a\x1B>V[\x92\x84\x81\x93\x95a\x0F?V[Pa\x0F\xDBV[\x91\x95\x92\x94a\x0F\xF9\x91\x95a\x1B>V[\x95a\x0F?V[P_\x90V[\x80T\x80\x15a\x11\x08Wa\x10\x15\x81a\x14\xA4V[\x91`\x04\x82\x01\x80\x92\x11a\x08\xB6Wa\x10.a\x10I\x92\x84a\x15\x88V[\x92`\x01a\x10<\x85\x83\x85a\x19\xE2V[\x94\x91\x95\x90\x94\x96\x84Qa\x08\xF6V[\x91\x01\x90\x80[a\x10eWPPP_\x14a\x0FVW\x90a\x07\xC1\x91a\x1B>V[a\x10n\x90a\x0E\xEBV[\x94a\x10\x9Ca\x10\x96\x87\x85`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x83a\x08\xE3V[PT\x83Q\x91_\x19\x83\x01\x92\x83\x11a\x08\xB6W\x87\x92\x83\x03a\x10\xBBWP\x95a\x10NV[\x92\x95\x91`\x01\x80`@\x87\x98\x94\x98\x01Q\x83\x1C\x16\x14_\x14a\x10\xF4WP\x15a\x10\xEEW\x90a\x10\xE3\x91a\x1B>V[\x92\x84`\x01\x93\x95a\x10NV[Pa\x10\xE3V[\x91\x95\x92\x94a\x11\x02\x91\x95a\x1B>V[\x95a\x10NV[PP_\x90V[\x91\x90\x81\x10\x15a\x07\xC4W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[`@\x90a\x07\xC1\x94\x92\x81R\x81` \x82\x01R\x01\x91a\x08VV[\x805\x90`@\x81\x01\x90a\x11\x80a\x11z\x83\x83a\x08vV[\x80a\x06\xB7V[_[\x81\x81\x10a\x13QWPPPa\x11\xA3a\x11\x99\x83\x83a\x08vV[` \x81\x01\x90a\x06\xB7V[_[\x81\x81\x10a\x12\xE2WPPPa\x11\xC6a\x11\xBC\x83\x83a\x08vV[`@\x81\x01\x90a\x06\xB7V[_[\x81\x81\x10a\x12sWPPPa\x11\xE9\x91a\x11\xDF\x91a\x08vV[``\x81\x01\x90a\x06\xB7V[\x90\x91_[\x82\x81\x10a\x11\xFAWPPPPV[a\x12\x05\x81\x84\x86a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x12 W[\x01a\x11\xEDV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca\x12Za\x12P\x84\x88\x8Aa\x11\x0EV[` \x81\x01\x90a\x08\x05V[\x90a\x12k`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x12\x1AV[a\x12~\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x12\x99W[\x01a\x11\xC8V[\x86\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa\x12\xC9a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x12\xDA`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x12\x93V[a\x12\xED\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x13\x08W[\x01a\x11\xA5V[\x86\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a\x138a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x13I`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x13\x02V[a\x13\\\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x13wW[\x01a\x11\x82V[\x86\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a\x13\xA7a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x13\xB8`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x13qV[_` \x91\x82a\x14ba\x13\xDBa\x13\xD5\x84\x80a\x06\xB7V[\x90a-?V[\x92\x82a\x13\xECa\x13\xD5\x82\x84\x01\x84a\x06\xB7V[\x91\x81a\x14\x0Fa\x13\xD5a\x14\x04a\x13\xD5`@\x86\x01\x86a\x06\xB7V[\x93``\x81\x01\x90a\x06\xB7V[\x91\x81`@Q\x97\x88\x96\x81\x80\x89\x01\x9B\x80Q\x91\x82\x91\x01\x8D^\x88\x01\x90\x82\x82\x01\x8D\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01\x90\x89\x82R\x80Q\x92\x83\x91\x01\x82^\x01\x90\x87\x82R\x80Q\x92\x83\x91\x01\x82^\x01\x85\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x07\x0BV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x03\xE0W\x7F\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_Q\x16\x90V[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@R```\x01\x82\x94\x01a\x15Z\x81\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90_\x19`\x01\x83\x1B\x01\x91\x84R\x80` \x85\x01R\x80`\x01\x1C\x82\x82`\x02\x1C\x19\x83\x19\x17\x82\x16\x16`@\x86\x01R\x19\x16\x16\x91\x01RV[\x91\x90_` `\x01\x85Q\x93\x01\x94\x01Q\x93\x80\x85\x18\x94\x85\x84\x1C\x15_\x14a\x16BWPPPa\x16<_\x19\x92\x93\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90\x03\x01\x90V[\x90\x80\x92\x95P`\x02\x84\x01\x1C\x15a\x16VWPPPV[\x92\x93P\x90\x91\x19`\x01\x83\x1B_\x19\x01\x16\x80a\x17+WP\x81[`\x01\x81\x01\x91_\x19`\x01\x84\x1B\x01\x16\x80\x15_\x14a\x16\x90WPP\x90[\x81\x80\x82\x10\x91\x18\x02\x18\x90V[a\x17#\x91\x92P\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90\x03\x90a\x16\x85V[a\x17\xBE_\x19\x91\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x83\x03\x01\x91a\x16lV[a\x17\xD7a\x11\xBC`@\x83\x01\x83a\x08vV[` _\x93\x015\x92[\x81\x81\x10a\x17\xECWPPPPV[a\x17\xFAa\x12P\x82\x84\x86a\x11\x0EV[\x81\x01``\x82\x82\x03\x12a\x04\x05W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x04\x05W` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05W\x82a\x18G\x91\x83\x01a.|V[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05Wa\x18f\x92\x01a.|V[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x87`\x04\x84\x01R`@`$\x84\x01R_\x83\x80a\x18\xAB`D\x82\x01\x86a.\xC2V[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x03\xE0W_\x93a\x19fW[P\x82Q\x81Q\x80\x82\x14\x91\x82a\x19PW[PP\x15a\x19\x15WP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91a\x19\x0C`@Q\x92\x83\x92\x83a.\xE7V[\x03\x90\xA2\x01a\x17\xDFV[\x90Pa\x19L`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a.\xE7V[\x03\x90\xFD[\x90\x91P` \x85\x01 \x90` \x83\x01 \x14_\x80a\x18\xCEV[\x90\x92P=\x80_\x83>a\x19x\x81\x83a\x07\x0BV[\x81\x01\x90` \x81\x83\x03\x12a\x04\x05W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W\x01\x81`\x1F\x82\x01\x12\x15a\x04\x05W\x80Q\x90a\x19\xAF\x82a.`V[\x92a\x19\xBD`@Q\x94\x85a\x07\x0BV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\x05W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_a\x18\xBFV[\x92_\x93_\x93\x80\x15a\x1B1Wa\x19\xF8\x90\x84Qa\x08\xF6V[\x90`\x01a\x1A\x97a\x1A\x91`@\x87\x01Q\x85\x1C`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x84a\x08\xA9V[\x91\x01\x91a\x1A\xC7a\x1A\xC1\x82\x87`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x84a\x08\xE3V[P\x90\x82\x03a\x1B(WT[\x93`\x01\x82\x01\x80\x92\x11a\x08\xB6W\x80Q\x82\x10a\x1A\xEAWPPPV[`\x02\x94\x96Pa\x1B\x1F\x93\x95P\x90a\x1B\x19\x91`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x90a\x08\xE3V[P\x01T\x91`\x01\x91V[`\x01\x01Ta\x1A\xD1V[P_\x94P\x84\x93P\x83\x92PPV[\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x92_\x08a\x01\xA0R_\x08a\x1B\x80a\x0B\xC0`@Qa\x01@Ra\x01@Qa\x07\x0BV[a\x0B\xA0a\x01@QR` a\x01@Q\x01\x7F\x19\xB8I\xF6\x94P\xB0hH\xDA\x1D9\xBD^JC\x02\xBB\x86tN\xDC&#\x8B\x08x\xE2i\xED#\xE5\x81R\x7F&]\xDF\xE1'\xDDQ\xBDr94{u\x8F\n\x13 \xEB,\xC7E\n\xCC\x1D\xADG\xF8\x0C\x8D\xCF4\xD6`@a\x01@Q\x01R\x7F\x19\x97P\xECG/\x18\t\xE0\xF6jT^\x1EQbA\x08\xAC\x84P\x15\xC2\xAA=\xFC6\xBA\xB4\x97\xD8\xAA``a\x01@Q\x01R\x7F\x15\x7F\xF3\xFEe\xACr\x08\x11\x0F\x06\xA5\xF7C\x02\xB1Mt>\xA2Pg\xF0\xFF\xD02\xF7\x87\xC7\xF1\xCD\xF8`\x80a\x01@Q\x01R\x7F.I\xC4<Ei\xDD\x9C_\xD3Z\xC4_\xCA3\xF1\x0B\x15\xC5\x90i/\x8B\xEE\xFE\x18\xF4\x89j\xC9I\x02`\xA0a\x01@Q\x01R\x7F\x0E5\xFB\x89\x98\x18\x90R\rJ\xEF+me\x06\xC3\xCB/\x0Bis\xC2O\xA8'14_\xFA-\x1F\x1E`\xC0a\x01@Q\x01R\x7F%\x1A\xD4|\xB1\\O\x11\x05\xF1\t\xAE^\x94O\x1B\xA9\xD9\xE7\x80mf\x7F\xFE\xC6\xFEr0\x02\xE0\xB9\x96`\xE0a\x01@Q\x01R\x7F\x13\xDA\x07\xDCd\xD4(6\x98s\xE9q`#FA\xF8\xBE\xB5o\xDD\x05\xE5\xF3V?\xA3\x9D\x9C\"\xDFNa\x01\0a\x01@Q\x01R\x7F\x0C\0\x9B\x84\xE6P\xE6\xD2=\xC0\x0C}\xCC\xEFt\x83\xA5S\x93\x96\x89\xD3P\xCDF\xE7\xB8\x90U\xFDG8a\x01 a\x01@Q\x01R\x7F\x01\x1F\x16\xB1\xC6:\x85O\x01\x99.9V\xF4-\x8B\x04\xEBe\x0CmS^\xB0 =\xECt\xBE\xFD\xCA\x06a\x01@\x80Q\x01R\x7F\x0E\xD6\x9E^8:h\x8F \x9D\x9AV\x1D\xAAya/?x\xD0Fz\xD4T\x85\xDF\x07\t?6uIa\x01`a\x01@Q\x01R\x7F\x04\xDB\xA9J{\x0C\xE9\xE2!\xAC\xADAG+k\xBE:\xECP\x7F^\xB3\xD3?F6r&L\x9Fx\x9Ba\x01\x80a\x01@Q\x01R\x7F\n?&7\xD8@\xF3\xA1n\xB0\x94'\x1C\x9D#{`6u}K\xB5\x0B\xF7\xCEs/\xF1\xD4\xFA(\xE8a\x01\xA0a\x01@Q\x01R\x7F%\x9Afo\x12\x9E\xEA\x19\x8F\x8A\x1CP/\xDB8\xFA9\xB1\xF0uV\x95d\xB6\xE5JH]\x11\x822?a\x01\xC0a\x01@Q\x01R\x7F(\xBFtY\xC9\xB2\xF4\xC6\xD8\xE7\xD0jN\xE3\xA4\x7FwE\xD4'\x108\xE5\x15z2\xFD\xF7\xED\xE0\xD6\xA1a\x01\xE0a\x01@Q\x01R\x7F\n\x1C\xA9A\xF0W\x03u&\xEA \x0FH\x9B\xE8\xD4\xC3|\x85\xBB\xCC\xE6\xA2\xAE\xEC\x91\xBDiAC$Ga\x02\0a\x01@Q\x01R\x7F\x0Co\x8F\x95\x8B\xE0\xE90S\xD7\xFDO\xC5E\x12\x85U5\xED\x159\xF0Q\xDC\xB4:&\xFD\x92ca\xCFa\x02 a\x01@Q\x01R\x7F\x121\x06\xA9<\xD1ux\xD4&\xE8\x12\x8A\xC9\xD9\n\xA9\xE8\xA0\x07\x08\xE2\x96\xE0\x84\xDDW\xE6\x9C\xAA\xF8\x11a\x02@a\x01@Q\x01R\x7F&\xE1\xBAR\xAD\x92\x85\xD9}\xD3\xABR\xF8\xE8@\x08^\x8F\xA8?\xF1\xE8\xF1\x87{\x07Hg\xCD-\xEEua\x02`a\x01@Q\x01R\x7F\x1C\xB5\\\xAD{\xD13\xDE\x18\xA6L\\G\xB9\xC9|\xBEM\x8B{\xF9\xE0\x95\x86DqS~jJ\xE2\xC5a\x02\x80a\x01@Q\x01R\x7F\x1D\xCDs\xE4j\xCD\x8F\x8E\x0E,|\xE0K\xDE\x7Fm*S\x04=P`\xA4\x1CqC\xF0\x8En\x90U\xD0a\x02\xA0a\x01@Q\x01R\x7F\x01\x10\x03\xE3/m\x9Cf\xF5\x85/\x05GJM\xEF\x0C\xDA)J\x0E\xB4\xE9\xB9\xB1+\x9B\xB4Q.Uta\x02\xC0a\x01@Q\x01R\x7F+\x1E\x80\x9A\xC1\xD1\n\xB2\x9A\xD5\xF2\r\x03\xA5}\xFE\xBA\xDF\xE5\x90?X\xBA\xFE\xD7\xC5\x08\xDD\"\x87\xAE\x8Ca\x02\xE0a\x01@Q\x01R\x7F%9\xDE\x17\x85\xB75\x99\x9F\xB4\xDA\xC3^\xE1~\xD0\xEF\x99]\x05\xAB/\xC5\xFA\xEA\xA6\x9A\xE8{\xCE\xC0\xA5a\x03\0a\x01@Q\x01R\x7F\x0C$lZ.\xF8\xEE\x01&I\x7F\"+>\n\x0E\xF4\xE1\xC3\xD4\x1C\x86\xD4nC\x98,\xB1\x1Dw\x95\x1Da\x03 a\x01@Q\x01R\x7F\x19 \x89\xC4\x97Oh\xE9T\x08\x14\x8F|\x062\xED\xBB\t\xE6\xA6\xAD\x1A\x1C/?\x03\x05\xF5\xD0;R{a\x03@a\x01@Q\x01R\x7F\x1E\xAE\n\xD8\xABh\xB2\xF0j\x0E\xE3n\xEB\r\x0C\x05\x85)\t}\x91\tkum\x8F\xDC/\xB5\xA6\r\x85a\x03`a\x01@Q\x01R\x7F\x17\x91\x90\xE5\xD0\xE2!y\xE4o\x82\x82\x87*\xBC\x88\xDBn/\xDC\r\xEE\x99\xE6\x97h\xBD\x98\xC5\xD0k\xFBa\x03\x80a\x01@Q\x01R\x7F)\xBB\x9E,\x90vs%v\xE9\xA8\x1Cz\xC4\xB82\x14R\x8F}\xB0\x0F1\xBFl\xAF\xE7\x94\xA9\xB3\xCD\x1Ca\x03\xA0a\x01@Q\x01R\x7F\"]9NB u\x99@>\xFD\x0C$d\xA9\rRe&E\x88*\xAC5\xB1\x0EY\x0Eni\x1E\x08a\x03\xC0a\x01@Q\x01R\x7F\x06G`b<%\xC8\xCFu=#\x80U\xB4DS+\xE15WE\x1C\x08}\xE0\x9E\xFDEK#\xFDYa\x03\xE0a\x01@Q\x01R\x7F\x10\xBA:\x0E\x01\xDF\x92\xE8\x7F0\x1CKqm\x8A9Mg\xF4\xBFB\xA7\\\x10\x92)\x10\xA7\x8Fk[\x87a\x04\0a\x01@Q\x01R\x7F\x0E\x07\x0B\xF5?\x84Q\xB2O\x9Cn\x96\xB0\xC2\xA8\x01\xCBQ\x1B\xC0\xC2B\xEB\x9D6\x1Bwi?!G\x1Ca\x04 a\x01@Q\x01R\x7F\x1B\x94\xCDa\xB0Q\xB0M\xD3\x97U\xFF\x93\x82\x1As\xCC\xD6\xCB\x11\xD2I\x1D\x8A\xA7\xF9!\x01M\xE2R\xFBa\x04@a\x01@Q\x01R\x7F\x1D|\xB3\x9B\xAF\xB8\xC7D\xE1Hxz.p#\x0F\x9DN\x91}W\x13\xBB\x05\x04\x87\xB5\xAA}t\x07\x0Ba\x04`a\x01@Q\x01R\x7F.\xC91\x89\xBD\x1A\xB4\xF6\x91\x17\xD0\xFE\x98\x0C\x80\xFF\x87\x85\xC2\x96\x18)\xF7\x01\xBBt\xAC\x1F0;\x17\xDBa\x04\x80a\x01@Q\x01R\x7F-\xB3f\xBF\xDD6\xD2w\xA6\x92\xBB\x82[\x86'[\xEA\xC4\x04\xA1\x9A\xE0z\x90\x82\xEAF\xBD\x83Qy&a\x04\xA0a\x01@Q\x01R\x7F\x06!\0\xEBH]\xB0bie\\\xF1\x86\xA6\x852\x98RuB\x84P5\x9A\xDC\x99\xCE\xC6\x96\x07\x11\xB8a\x04\xC0a\x01@Q\x01R\x7F\x07a\xD3<faJ\xAAW\x0E\x7F\x1E\x82D\xCA\x11 $?\x92\xFAY\xE4\xF9\0\xC5g\xBFA\xF5\xA5\x9Ba\x04\xE0a\x01@Q\x01R\x7F \xFCA\x1A\x11M\x13\x99,'\x05\xAA\x03N?1]x`\x8A\x0F}\xE4\xCC\xF7\xA7.IHU\xAD\ra\x05\0a\x01@Q\x01R\x7F%\xB5\xC0\x04\xA4\xBD\xFC\xB5\xAD\xD9\xECN\x9A\xB2\x19\xBA\x10,g\xE8\xB3\xEF\xFB_\xC3\xA3\x0F1rP\xBCZa\x05 a\x01@Q\x01R\x7F#\xB1\x82-'\x8E\xD62\xA4\x94\xE5\x8Fm\xF6\xF5\xED\x03\x8B\x18m\x84t\x15Z\xD8~}\xFFb\xB3\x7FKa\x05@a\x01@Q\x01R\x7F\"sKL\\?\x94\x93`lK\xA9\x01$\x99\xBF\x0F\x14\xD1;\xFC\xFC\xCC\xAA\x16\x10*)\xCC/i\xE0a\x05`a\x01@Q\x01R\x7F&\xC0\xC8\xFE\t\xEB0\xB7\xE2zt\xDC3I#G\xE5\xBD\xFF@\x9A\xA3a\x02TA=?\xADy\\\xE5a\x05\x80a\x01@Q\x01R\x7F\x07\r\xD0\xCC\xB6\xBD{\xBA\xE8\x8E\xAC\x03\xFA\x1F\xBB&\x19k\xE3\x08:\x80\x98)\xBB\xD6&\xDF4\x8C\xCA\xD9a\x05\xA0a\x01@Q\x01R\x7F\x12\xB6Y[\xDB2\x9Bo\xB0C\xBAx\xBB(\xC3\xBE\xC2\xC0\xA6\xDEF\xD8\xC5\xAD`g\xC4\xEB\xFDBP\xDAa\x05\xC0a\x01@Q\x01R\x7F$\x8D\x97\xD7\xF7b\x83\xD6;\xEC0\xE7\xA5\x87l\x11\xC0o\xCA\x9B'\\g\x1C^3\xD9[\xB7\xE8\xD7)a\x05\xE0a\x01@Q\x01R\x7F\x1A0mC\x9DF;\x08\x16\xFCo\xD6L\xC991\x8BE\xEBu\x9D\xDD\xE4\xAA\x10m\x15\xD9\xBD\x9B\xAA\xAAa\x06\0a\x01@Q\x01R\x7F(\xA8\xF87.<8\xDA\xCE\xD7\xC0\x04!\xCBF!\xF4\xF1\xB5M\xDC'\x82\x1B\rb\xD3\xD6\xEC|V\xCFa\x06 a\x01@Q\x01R~\x94\x97W\x17\xF9\xA8\xA8\xBB5\x15/$\xD42\x94\x07\x1C\xE3 \xC8)\xF3\x88\xBC\x85!\x83\xE1\xE2\xCE~a\x06@a\x01@Q\x01R\x7F\x04\xD5\xEEL:\xA7\x8F}\x80\xFD\xE6\rqd\x80\xD3Y?t\xD4\xF6S\xAE\x83\xF4\x102F\xDB.\x8Dea\x06`a\x01@Q\x01R\x7F*l\xF5\xE9\xAA\x03\xD43cI\xADo\xB8\xED\"i\xC7\xBE\xF5K\x88\"\xCCv\xD0\x84\x95\xC1.\xFD\xE1\x87a\x06\x80a\x01@Q\x01R\x7F#\x04\xD3\x1E\xAA\xB9`\xBA\x92t\xDAC\xE1\x9D\xDE\xB7\xF7\x92\x18\x08\x08\xFDnC\xBA\xAEH\xD7\xEF\xCB\xA3\xF3a\x06\xA0a\x01@Q\x01R\x7F\x03\xFD\x9A\xC8e\xA4\xB2\xA6\xD5\xE7\0\x97\x85\x81rI\xBF\xF0\x8A~\x07&\xFC\xB4\xE1\xC1\x1D9\xD1\x99\xF0\xB0a\x06\xC0a\x01@Q\x01R~\xB7%\x8D\xEDR\xBB\xDA\"H@MU\xEEPDy\x8A\xFC: \x91\x93\x07?yT\xD4\xD6;\x0Bda\x06\xE0a\x01@Q\x01R\x7F\x15\x9F\x81\xAD\xA0w\x17\x99\xEC8\xFC\xA2\xD4\xBFe\xEB\xB1=:t\xF3)\x8D\xB3br\xC5\xCAe\xE9-\x9Aa\x07\0a\x01@Q\x01R\x7F\x1E\xF9\x0EgC\x7F\xBC\x85P#zu\xBC(\xE3\xBB\x90\0\x13\x0E\xA2_\x0CTq\xE1D\xCFBdC\x1Fa\x07 a\x01@Q\x01R\x7F\x1Ee\xF88Q^_\xF0\x19kI\xAAA\xA2\xD2V\x8D\xF79\xBC\x17k\x08\xEC\x95\xA7\x9E\xD8)2\xE3\ra\x07@a\x01@Q\x01R\x7F+\x1B\x04]\xEF:\x16l\xECl\xE7h\xD0y\xBAt\xB1\x8C\x84NW\x0E\x1F\x82eu\xC1\x06\x8C\x94\xC3?a\x07`a\x01@Q\x01R\x7F\x082\xE5u<\xEB\x0F\xF6@%C\xB1\x10\x92)\xC1e\xDC-s\xBE\xF7\x15\xE3\xF1\xC6\xE0|\x16\x8B\xB1sa\x07\x80a\x01@Q\x01R\x7F\x02\xF6\x14\xE9\xCE\xDF\xB3\xDCkv*\xE0\xA3}A\xBA\xB1\xB8A\xC2\xE8\xB6E\x1B\xC5\xA8\xE3\xC3\x90\xB6\xAD\x16a\x07\xA0a\x01@Q\x01R\x7F\x0E$'\xD3\x8B\xD4j`\xDDd\x0B\x8E6,\xAD\x96sp\xEB\xB7w\xBE\xDF\xF4\x0Fj\x0B\xE2~~\xD7\x05a\x07\xC0a\x01@Q\x01R\x7F\x04\x93c\x0B|g\x0Bm\xEB|\x84\xD4\x14\xE7\xCEy\x04\x9F\x0E\xC0\x98\xC3\xC7\xC5\x07h\xBB\xE2\x92\x14\xA5:a\x07\xE0a\x01@Q\x01R\x7F\"\xEA\xD1\0\xE8\xE4\x82gM\xEC\xDA\xB1pf\xC5\xA2k\xB1QSU\xD5F\x1A=\xC0l\xC8S'\xCE\xA9a\x08\0a\x01@Q\x01R\x7F%\xB3\xE5ne[B\xCD\xAA\xE2bn\xD2UMHX?\x1A\xE3V&\xD0M\xE5\x08N\x0Bm*o\x16a\x08 a\x01@Q\x01R\x7F\x1E2u*\xDA\x886\xEFX7\xA6\xCD\xE8\xFF\x13\xDB\xB5\x99\xC364\x9ELXKO\xDC\n\x0C\xF6\xF9\xD0a\x08@a\x01@Q\x01R\x7F/\xA2\xA8q\xC1Z8|\xC5\x0Fh\xF6\xF3\xC3E[#\xC0\t\x95\xF0Px\xF6r\xA9\x86@t\xD4\x12\xE5a\x08`a\x01@Q\x01R\x7F/V\x9B\x8A\x9AD$\xC9'\x8E\x1D\xB71\x1E\x88\x9FT\xCC\xBF\x10f\x1B\xAB\x7F\xCD\x18\xE7\xC7\xA7\xD85\x05a\x08\x80a\x01@Q\x01R\x7F\x04L\xB4U\x11\n\x8F\xDDS\x1A\xDES\x024\xC5\x18\xA7\xDF\x93\xF73/\xFD!D\x16St\xB2F\xB4=a\x08\xA0a\x01@Q\x01R\x7F\"x\x08\xDE\x93\x90m]B\x02F\x15\x7F.B\xB1\x91\xFE\x8C\x90\xAD\xFE\x11\x81x\xDD\xC7#\xA51\x90%a\x08\xC0a\x01@Q\x01R\x7F\x02\xFC\xCA)4\xE0F\xBCb:\xDE\xAD\x875y\x86]\x03x\x1A\xE0\x90\xADJ\x85y\xD2\xE7\xA6\x80\x03Ua\x08\xE0a\x01@Q\x01R\x7F\x0E\xF9\x15\xF0\xAC\x12\x0B\x87j\xBC\xCC\xEB4J\x1D6\xBA\xD3\xF3\xC5\xAB\x91\xA8\xDD\xCB\xEC.\x06\r\x8B\xEF\xACa\t\0a\x01@Q\x01R\x7F\x17\x97\x13\x0FKz>\x17w\xEBu{\xC6\xF2\x87\xF6\xAB\x0F\xB8_k\xE6;\t\xF3\xB1n\xF2\xB1@]8a\t a\x01@Q\x01R\x7F\nv\"]\xC0Ap\xAE3\x06\xC8Z\xBA\xB5\x9E`\x8C\x7FI| \x15mM6\xC6hU]\xEC\xC6\xE5a\t@a\x01@Q\x01R\x7F\x1F\xFF\xB9\xEC\x19\x92\xD6k\xA1\xE7z{\x93 \x9A\xF6\xF8\xFAv\xD4\x8A\xCBfG\x96\x17KS&\xA3\x1A\\a\t`a\x01@Q\x01R\x7F%r\x1CO\xC1Z?(S\xB5|3\x8F\xA58\xD8_\x8F\xBB\xA6\xC6\xB9\xC6\t\x06\x11\x88\x9By{\x9C_a\t\x80a\x01@Q\x01R\x7F\x0C\x81\x7F\xD4-_zA!^=\x07\xBA\x19r\x16\xAD\xB4\xC3y\x07\x05\xDA\x95\xEBc\xB9\x82\xBF\xCA\xF7Za\t\xA0a\x01@Q\x01R\x7F\x13\xAB\xE3\xF5#\x99\x15\xD3\x9F~\x13\xC2\xC2Ip\xB6\xDF\x8C\xF8l\xE0\n\"\0+\xC1Xf\xE5+Z\x96a\t\xC0a\x01@Q\x01R\x7F!\x06\xFE\xEATb$\xEA\x12\xEF\x7F9\x98zF\xC8\\\x1B\xC3\xDC)\xBD\xBDz\x92\xCD`\xAC\xB4\xD3\x91\xCEa\t\xE0a\x01@Q\x01R\x7F!\xCA\x85\x94h\xA7F\xB6\xAA\xA7\x94t\xA3}\xABI\xF1\xCAZ(\xC7H\xBCqW\xE1\xB34[\xB0\xF9Ya\n\0a\x01@Q\x01R\x7F\x05\xCC\xD6%\\\x1Eo\x0C\\\xF1\xF0\xDF\x93A\x94\xC6)\x11\xD1M\x03!f*\x8F\x1AH\x99\x9E4\x18[a\n a\x01@Q\x01R\x7F\x0F\x0E4\xA6Kp\xA6&\xE4d\xD8FgLL\x88\x16\xC4\xFB&\x7F\xE4O\xE6\xEA(g\x8C\xB0\x94\x90\xA4a\n@a\x01@Q\x01R\x7F\x05XS\x1AN%G\x0CaWyL\xA3m\x0E\x96G\xDB\xFC\xFE5\rd\x83\x8F[\x1A\x8A-\xE0\xD4\xBFa\n`a\x01@Q\x01R\x7F\t\xD3\xDC\xA9\x17>\xD2\xFA\xCE\xEA\x12QWh=\x18\x92L\xAD\xAD?eZ`\xB7/Xd\x96\x1F\x14Ua\n\x80a\x01@Q\x01R\x7F\x03(\xCB\xD5N\x8C\t\x13I?\x86n\xD0=!\x8B\xF2?\x92\xD6\x8A\xAE\xC4\x86\x17\xD4\xC7\"\xE5\xBDC5a\n\xA0a\x01@Q\x01R\x7F+\xF0r\x16\xE2\xAF\xF0\xA2#\xA4\x87\xB1\xA7\tN\x07\xE7\x9E{\xCC\x97\x98\xC6H\xEE3G\xDDS)\xD3Ka\n\xC0a\x01@Q\x01R\x7F\x1D\xAF4ZX\0ksd\x99\xC5\x83\xCBv\xC3\x16\xD6\xF7\x8E\xD6\xA6\xDF\xFC\x82\x11\x1E\x11\xA6?\xE4\x12\xDFa\n\xE0a\x01@Q\x01R\x7F\x17ecG$V\xAA\xA7F\xB6\x94\xC6\x0E\x18#a\x1E\xF3\x909\xB2\xED\xC7\xFF9\x1Eo\"\x93\xD2\xC4\x04a\x0B\0a\x01@Q\x01Ra\x0B a\x01@Q\x01\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x82R\x7F\x10\xDCn\x9C\0n\xA3\x8B\x04\xB1\xE0;K\xD9I\x0C\r\x03\xF9\x89)\xCA\x1D\x7F\xB5h!\xFD\x19\xD3\xB6\xE7a\x0B@a\x01@Q\x01R\x7F\x0C(\x14[jD\xDF>\x01I\xB3\xD0\xA3\x0B;\xB5\x99\xDF\x97V\xD4\xDD\x9B\x84\xA8k8\xCF\xB4Zt\x0Ba\x0B`a\x01@Q\x01R~TK\x838y\x15\x18\xB2\xC7dZP9'\x98\xB2\x1Fu\xBB`\xE3Yap\x06}\0\x14\x1C\xAC\x15a\x0B\x80a\x01@Q\x01R\x7F\",\x01\x17W\x188o..\x82\xEB\x12'\x89\xE3R\xE1\x05\xA3\xB8\xFA\x85&\x13\xBCSD3\xEEB\x8Ba\x0B\xA0a\x01@Q\x01Rh\x03\0\0\0\0\0\0\0\0`\xC0R\x82`\xA0Ra\x01\xA0Qa\x01 R`\t`\x80R`\xE0RQa\x01\xC0Ra\x01\xC0Qa\x01\xC0Q`\xC0Q\x83\x08a\x01\xC0Q`\xC0Q\x84\x08\x08a\x01\xC0Q\x90a\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Q`\xC0Q\x80\x08\x08\x90a\x01\xC0Q\x90\x80\x08\x08a\x01\x80Ra\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Qa\x01\xA0Q`\x80Q\x08\x08a\x01\xC0Q\x90a\x01\xC0Qa\x01\xC0Q`\xC0Q\x85\x08a\x01\xC0Qa\x01\xA0Q\x80\x08\x08\x90a\x01\xC0Q\x90\x80\x08\x08a\x01`Ra\x01\xC0Qa\x01`Qa\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Q`\xC0Q\x80\x08\x08\x08a\x01\xC0Qa\x01\x80Q`\xC0Q\x92\x93\x91\x92\x90\x91\x83\x91\x82\x91\x08a\x01\xC0Qa\x01\xA0Q\x80\x08\x08\x08a\x01\x80Q`\x80\x92\x90\x92Ra\x01`Qa\x01 R`\xA0R`\xC0R[a\x02 a\x01@Q\x01`\xE0Q\x10a-\x04W`\xE0Qa\x07\0\x01a\x01\0Ra\x01@Qa\x0B@\x81\x01Qa\x0B`\x82\x01Qa\x0B\x80\x83\x01Qa\x0B\xA0\x90\x93\x01Q\x92\x90[a\x01\0Q`\xE0Q\x10a,tWPPPPa\x02\0`\xE0Q\x01[\x80`\xE0Q\x10a,9WPa\x01@Q`@R`\x80Q\x90V[a,Ua\x01\xC0Q`\xE0Q`\xC0Q`\xA0Qa\x01 Q`\x80Qa/PV[\x92`\x80\x92\x91\x92Ra\x01 R`\xA0R`\xC0R`\x80`\xE0Q\x01`\xE0Ra,\"V[a\x01\xC0Q`\xE0QQ`\x80Q\x08a\x01\xC0Q\x81\x80\ta\x01\xC0Q\x91\x90\x82\x90\x80\t\ta\x01\xC0Qa\x01\xC0Q`\xC0Q`\xA0Q\x08a\x01\xC0Qa\x01 Q\x84\x08\x08\x90\x81a\x01\xC0Q\x91\x86a\x01\xC0Q\x91\t\x08`\x80Ra\x01\xC0Q\x81a\x01\xC0Q\x84a\x01 Q\t\x08a\x01 Ra\x01\xC0Q\x81a\x01\xC0Q\x85`\xA0Q\t\x08`\xA0Ra\x01\xC0Q\x90a\x01\xC0Q\x86`\xC0Q\t\x08`\xC0R` `\xE0Q\x01`\xE0Ra,\nV[a- a\x01\xC0Q`\xE0Q`\xC0Q`\xA0Qa\x01 Q`\x80Qa/PV[\x92`\x80\x92\x91\x92Ra\x01 R`\xA0R`\xC0R`\x80`\xE0Q\x01`\xE0Ra+\xCFV[\x91\x90\x91a-K\x83a/\x0CV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x91`\xE0\x1B\x16` \x82\x01R`\x04\x81Ra-\x88`$\x82a\x07\x0BV[\x92_\x91[\x81\x83\x10a-\x98WPPPV[\x90\x91\x93a-\xA9a\x12P\x86\x85\x85a\x11\x0EV[a-\xB4\x87\x86\x86a\x11\x0EV[5`\x02\x81\x10\x15a\x04\x05W`\x01\x93` \x83`\x05\x93a.W\x95\x83\x97\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a-\xF7\x83a/\x0CV[`\xE0\x1B\x16\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x9A\x88\x8C\x99Q\x91\x82\x91\x01\x89\x8B\x01^\x88\x01\x94`\xF8\x1B\x16\x86\x85\x01R`!\x84\x01R`%\x83\x017\x01\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x07\x0BV[\x94\x01\x91\x90a-\x8CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x04\x05W\x805\x90a.\x93\x82a.`V[\x92a.\xA1`@Q\x94\x85a\x07\x0BV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\x05W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x91a.\xFEa\x07\xC1\x93`@\x84R`@\x84\x01\x90a.\xC2V[\x91` \x81\x84\x03\x91\x01Ra.\xC2V[c\xFF\xFF\xFF\xFF\x81\x11a/ Wc\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[\x93\x91\x94\x92\x90\x94a\x02\xC0Ra\x03 Ra\x02\x80Ra\x03@Ra\x03@Q\x90a\x02\x80QQ\x90\x08a\x03\0Ra\x03@Q\x90` a\x02\x80Q\x01Q\x90\x08a\x02\xE0Ra\x03@Qa\x02\xE0Qa\x02\xE0Q\ta\x02\xA0Ra\x03@Q`@a\x02\x80Q\x01Qa\x02\xC0Q\x08a\x03@Q``a\x02\x80Q\x01Qa\x03 Q\x08a\x02@Ra\x03@Qa\x02@Qa\x02@Q\ta\x02 Ra\x03@Q\x90a\x03@Qa\x02@Qa\x03@Qa\x02 Qa\x02 Q\t\ta\x03@Q\x90\x91\x81\x80\x82\x80\ta\x03@Q\x83\x80\t\t\t\x08a\x02`Ra\x03@Qa\x02`Qa\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x02\xE0Qa\x02\xA0Q\x82\x90\x80\t\t\x08\x08a\x02\0Ra\x03@Qa\x02\xE0Qa\x02\xA0Q\x82\x91\x82\x91\x82\x90\x80\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08a\x03@Qa\x03@Qa\x02@Qa\x03@Qa\x02 Qa\x02 Q\t\ta\x03@Qa\x02@Qa\x02 Q\x82\x90\x80\t\t\x08\x08a\x01\xE0Ra\x03@Qa\x02`Q\x80\x08a\x03@Qa\x01\xE0Q\x90\x91\x82\x90\x80\x08\x08\x90a\x03@Qa\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08a\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08\x08a\x03@Qa\x02\0Q\x90\x91\x82\x90\x80\x08\x08\x91a\x03@Q\x83a\x01\xE0Q\x08\x91a\x03@Q\x82a\x02\0Q\x08\x92\x93\x92\x91\x90V\xFE\xA1dsolcC\0\x08&\0\n\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\x000dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x61036060405260033611610011575f80fd5b5f5f3560e01c8063467b50111461014f578063bdeb442d1461012c578063c1b0bed7146100fc578063c44956d1146100df578063db19aa791461008e5763f8adf4c31461005c575f80fd5b3461008b57602060031936011261008b5760ff6040602092600435815261018184522054166040519015158152f35b80fd5b503461008b578060031936011261008b57602060405173ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b503461008b578060031936011261008b5760209054604051908152f35b503461008b57602060031936011261008b5760ff6040602092600435815261018284522054166040519015158152f35b503461008b578060031936011261008b576020610147610ef7565b604051908152f35b50346104055760206003193601126104055760043567ffffffffffffffff8111610405578060040160406003198336030112610405577f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005c61068f5760017f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d6101d981806106b7565b90918115610667575f936101ec83610746565b915f5b848110610436575061023c90602473ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016930190610805565b9190604051906020820182811067ffffffffffffffff82111761040957604052878252823b156104055792906102a56040519485937f8d8f8a5c000000000000000000000000000000000000000000000000000000008552604060048601526044850191610856565b905f602484015b600182106103eb575050509181805f9403915afa80156103e0576103ca575b5060206102f87f0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d292610903565b80875261018182526040872060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00825416179055604051908152a1835b81811061038a575050507f862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce8280a2807f9b779b17422d0df92223018b32b4d1fa46e071723d6817e2486d003becc55f005d80f35b806103a96103a461039e60019486886107aa565b80610777565b6117c7565b6103c46103a46103ba8386886107aa565b6020810190610777565b01610336565b6103d79195505f9061070b565b5f9360206102cb565b6040513d5f823e3d90fd5b8251815286945060209283019260019290920191016102ac565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b956104428786886107aa565b6040810135805f5261018160205260ff60405f2054161561063c576104678280610777565b35805f5261018260205260ff60405f205416610611579160209391610559845f95865261018287526040862060017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff008254161790556104ce6104c98580610777565b611165565b868401906104df6104c98387610777565b6104e98286610777565b35907f9857d0b88f01dca7bb1205d68c307d28d8d4002ff5486007ac4c443596dcea2160408a6105198980610777565b01358b610526878b610777565b013582519182528c820152a38c6105486105408387610777565b35918c6107f1565b526105538480610777565b93610777565b8561056f61056a6040860186610876565b6113c0565b9161058061056a6040830183610876565b92826040519681880198895280356040890152013560608701526080860152803560a0860152013560c084015260e083015261010082015261010081526105c96101208261070b565b604051918291518091835e8101838152039060025afa156103e05760017f1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5f511696016101ef565b7f39a940c5000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7ff9849ea3000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7fcf1b093c000000000000000000000000000000000000000000000000000000005f5260045ffd5b7f3ee5aeb5000000000000000000000000000000000000000000000000000000005f5260045ffd5b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610405570180359067ffffffffffffffff821161040557602001918160051b3603831361040557565b90601f601f19910116810190811067ffffffffffffffff82111761040957604052565b67ffffffffffffffff81116104095760051b60200190565b906107508261072e565b61075d604051918261070b565b828152601f1961076d829461072e565b0190602036910137565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa181360301821215610405570190565b908210156107c4576107c19160051b810190610777565b90565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80518210156107c45760209160051b010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe181360301821215610405570180359067ffffffffffffffff82116104055760200191813603831361040557565b601f8260209493601f1993818652868601375f8582860101520116010190565b9035907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8181360301821215610405570190565b919082018092116108b657565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b9060808110156107c45760030201905f90565b919082039182116108b657565b5f5490805115610ee0576109188151836108a9565b67fffffffffffffffe8111610eb55781515f5b818110610e4557505061093d816114a4565b905f82518060011b907f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8116036108b657600281018091116108b65761098590929192610746565b9585918551945f5b8487821015610b65575f1981146108b65760019687838d6109c28d6109bb85600298019d8e169485926107f1565b51926107f1565b521b1795018087116108b6578681166109e2575b5060010194939461098d565b9094610a71826001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b908160011b917f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff811681036108b65760018301808411806108b65760018085841c16145f14610b4e576108b657610ac8908e6107f1565b51935b8d8460018086831c16145f14610b3657610ae4916107f1565b515b60028501958686116108b657600184018094116108b6578f90610b2890610b22610b1a60019a8b809860039a1c16906108a9565b948592611b3e565b926107f1565b521b921b19161794906109d6565b505060808410156107c4576003840260010154610ae6565b5060808110156107c4576003026001015493610acb565b50509450945094905081925f9180610e0a575b50600485018086116108b657610b8e9082611588565b91610b9a81835f6119e2565b939194909395610bac835192836108f6565b92610bb785846108f6565b94610ddc578b9493925f5b855f198301928311965b610cba5750505050505050505f14610cb45790610be891611b3e565b93816001925b610bf9575050505f55565b610c86816001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b610c9081846107f1565b519060808110156107c4576003028401555f198101908082116108b6571680610bee565b50610be8565b90610cc987989b969792610eeb565b95610cee8786600180926060826020850151831c168201831b930151901c1690030190565b809c60018089841c16145f14610dc05790610d08916107f1565b51925b6108b657868403610d275750508490995b948e97969591610bcc565b91986001806040879d949d0151891c16145f14610d8e578692919015610d875790610d5191611b3e565b975b889960019988841015610d69575b505099610d1c565b90925060808310156107c457600260038894028b0101555f80610d61565b5097610d53565b98610d9c919a508692611b3e565b99868210610d1c57905060808110156107c457600260038202018a90558490610d1c565b5050915060808b10156107c457600160038c0201549188610d0b565b610e008582600180926060826020850151831c168201831b930151901c1690030190565b928c959493610bc2565b909150610e16816114a4565b610e208682611588565b91600481018091116108b657610e3591611588565b908180821091180218905f610b78565b7f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001610e7082866107f1565b511015610e7f5760010161092b565b610e8990846107f1565b517fc53e726b000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b7f084d73d9000000000000000000000000000000000000000000000000000000005f5260045260245ffd5b50506107c15f611004565b80156108b6575f190190565b5f548015610fff57610f08816114a4565b90600481018091116108b657610f21610f3a9183611588565b91610f2d83825f6119e2565b92919390929482516108f6565b806001915b610f5a575050505f14610f5657906107c191611b3e565b5090565b610f6390610eeb565b94610f888684600180926060826020850151831c168201831b930151901c1690030190565b60808110156107c4576003028201548351915f1983019283116108b65787928303610fb4575095610f3f565b92959186806040879894980151831c16145f14610feb575015610fe55790610fdb91611b3e565b9284819395610f3f565b50610fdb565b91959294610ff99195611b3e565b95610f3f565b505f90565b8054801561110857611015816114a4565b91600482018092116108b65761102e6110499284611588565b92600161103c8583856119e2565b94919590949684516108f6565b910190805b611065575050505f14610f5657906107c191611b3e565b61106e90610eeb565b9461109c6110968785600180926060826020850151831c168201831b930151901c1690030190565b836108e3565b50548351915f1983019283116108b657879283036110bb57509561104e565b9295916001806040879894980151831c16145f146110f45750156110ee57906110e391611b3e565b92846001939561104e565b506110e3565b919592946111029195611b3e565b9561104e565b50505f90565b91908110156107c45760051b810135907fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc181360301821215610405570190565b6040906107c1949281528160208201520191610856565b803590604081019061118061117a8383610876565b806106b7565b5f5b818110611351575050506111a36111998383610876565b60208101906106b7565b5f5b8181106112e2575050506111c66111bc8383610876565b60408101906106b7565b5f5b818110611273575050506111e9916111df91610876565b60608101906106b7565b90915f5b8281106111fa5750505050565b61120581848661110e565b35906002821015610405576001809214611220575b016111ed565b827fa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c61125a61125084888a61110e565b6020810190610805565b9061126b604051928392878461114e565b0390a261121a565b61127e81838561110e565b35906002821015610405576001809214611299575b016111c8565b867f9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd596112c961125084878961110e565b906112da604051928392878461114e565b0390a2611293565b6112ed81838561110e565b35906002821015610405576001809214611308575b016111a5565b867f48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f361133861125084878961110e565b90611349604051928392878461114e565b0390a2611302565b61135c81838561110e565b35906002821015610405576001809214611377575b01611182565b867f3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae66113a761125084878961110e565b906113b8604051928392878461114e565b0390a2611371565b5f602091826114626113db6113d584806106b7565b90612d3f565b92826113ec6113d5828401846106b7565b918161140f6113d56114046113d560408601866106b7565b9360608101906106b7565b9181604051978896818089019b8051918291018d5e8801908282018d8152815193849201905e010190898252805192839101825e0190878252805192839101825e0185815203601f19810183528261070b565b604051918291518091835e8101838152039060025afa156103e0577f1fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5f511690565b906040516080810181811067ffffffffffffffff821117610409576040526060600182940161155a817f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b905f196001831b019184528060208501528060011c828260021c198319178216166040860152191616910152565b91905f6020600185519301940151938085189485841c155f146116425750505061163c5f1992937f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b90030190565b9080929550600284011c1561165657505050565b9293509091196001831b5f1901168061172b5750815b60018101915f196001841b011680155f14611690575050905b818082109118021890565b6117239192507f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b900390611685565b6117be5f19917f07060605060205040602030205040301060502050303040105050304000000006f8421084210842108cc6318c6db6d54be826fffffffffffffffffffffffffffffffff1060071b831560081b1783811c67ffffffffffffffff1060061b1783811c63ffffffff1060051b1783811c61ffff1060041b1783811c60ff1060031b1792831c1c601f161a1790565b8303019161166c565b6117d76111bc6040830183610876565b60205f930135925b8181106117ec5750505050565b6117fa61125082848661110e565b81016060828203126104055781359173ffffffffffffffffffffffffffffffffffffffff831680930361040557602081013567ffffffffffffffff81116104055782611847918301612e7c565b91604082013567ffffffffffffffff8111610405576118669201612e7c565b90604051917f33a89203000000000000000000000000000000000000000000000000000000008352876004840152604060248401525f83806118ab6044820186612ec2565b038183885af19283156103e0575f93611966575b50825181518082149182611950575b505015611915575060019392917fcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf19161190c60405192839283612ee7565b0390a2016117df565b905061194c6040519283927fc504fada00000000000000000000000000000000000000000000000000000000845260048401612ee7565b0390fd5b9091506020850120906020830120145f806118ce565b9092503d805f833e611978818361070b565b8101906020818303126104055780519067ffffffffffffffff8211610405570181601f82011215610405578051906119af82612e60565b926119bd604051948561070b565b8284526020838301011161040557815f9260208093018386015e83010152915f6118bf565b925f935f938015611b31576119f89084516108f6565b906001611a97611a916040870151851c6001811901167e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b1503160405601f6101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff860260f81c161b60f71c1692831c63d76453e004161a1790565b846108a9565b910191611ac7611ac18287600180926060826020850151831c168201831b930151901c1690030190565b846108e3565b50908203611b2857545b93600182018092116108b65780518210611aea57505050565b6002949650611b1f93955090611b1991600180926060826020850151831c168201831b930151901c1690030190565b906108e3565b50015491600191565b60010154611ad1565b505f945084935083925050565b907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000180925f086101a0525f08611b80610bc0604051610140526101405161070b565b610ba06101405152602061014051017f19b849f69450b06848da1d39bd5e4a4302bb86744edc26238b0878e269ed23e581527f265ddfe127dd51bd7239347b758f0a1320eb2cc7450acc1dad47f80c8dcf34d660406101405101527f199750ec472f1809e0f66a545e1e51624108ac845015c2aa3dfc36bab497d8aa60606101405101527f157ff3fe65ac7208110f06a5f74302b14d743ea25067f0ffd032f787c7f1cdf860806101405101527f2e49c43c4569dd9c5fd35ac45fca33f10b15c590692f8beefe18f4896ac9490260a06101405101527f0e35fb89981890520d4aef2b6d6506c3cb2f0b6973c24fa82731345ffa2d1f1e60c06101405101527f251ad47cb15c4f1105f109ae5e944f1ba9d9e7806d667ffec6fe723002e0b99660e06101405101527f13da07dc64d428369873e97160234641f8beb56fdd05e5f3563fa39d9c22df4e6101006101405101527f0c009b84e650e6d23dc00c7dccef7483a553939689d350cd46e7b89055fd47386101206101405101527f011f16b1c63a854f01992e3956f42d8b04eb650c6d535eb0203dec74befdca06610140805101527f0ed69e5e383a688f209d9a561daa79612f3f78d0467ad45485df07093f3675496101606101405101527f04dba94a7b0ce9e221acad41472b6bbe3aec507f5eb3d33f463672264c9f789b6101806101405101527f0a3f2637d840f3a16eb094271c9d237b6036757d4bb50bf7ce732ff1d4fa28e86101a06101405101527f259a666f129eea198f8a1c502fdb38fa39b1f075569564b6e54a485d1182323f6101c06101405101527f28bf7459c9b2f4c6d8e7d06a4ee3a47f7745d4271038e5157a32fdf7ede0d6a16101e06101405101527f0a1ca941f057037526ea200f489be8d4c37c85bbcce6a2aeec91bd69414324476102006101405101527f0c6f8f958be0e93053d7fd4fc54512855535ed1539f051dcb43a26fd926361cf6102206101405101527f123106a93cd17578d426e8128ac9d90aa9e8a00708e296e084dd57e69caaf8116102406101405101527f26e1ba52ad9285d97dd3ab52f8e840085e8fa83ff1e8f1877b074867cd2dee756102606101405101527f1cb55cad7bd133de18a64c5c47b9c97cbe4d8b7bf9e095864471537e6a4ae2c56102806101405101527f1dcd73e46acd8f8e0e2c7ce04bde7f6d2a53043d5060a41c7143f08e6e9055d06102a06101405101527f011003e32f6d9c66f5852f05474a4def0cda294a0eb4e9b9b12b9bb4512e55746102c06101405101527f2b1e809ac1d10ab29ad5f20d03a57dfebadfe5903f58bafed7c508dd2287ae8c6102e06101405101527f2539de1785b735999fb4dac35ee17ed0ef995d05ab2fc5faeaa69ae87bcec0a56103006101405101527f0c246c5a2ef8ee0126497f222b3e0a0ef4e1c3d41c86d46e43982cb11d77951d6103206101405101527f192089c4974f68e95408148f7c0632edbb09e6a6ad1a1c2f3f0305f5d03b527b6103406101405101527f1eae0ad8ab68b2f06a0ee36eeb0d0c058529097d91096b756d8fdc2fb5a60d856103606101405101527f179190e5d0e22179e46f8282872abc88db6e2fdc0dee99e69768bd98c5d06bfb6103806101405101527f29bb9e2c9076732576e9a81c7ac4b83214528f7db00f31bf6cafe794a9b3cd1c6103a06101405101527f225d394e42207599403efd0c2464a90d52652645882aac35b10e590e6e691e086103c06101405101527f064760623c25c8cf753d238055b444532be13557451c087de09efd454b23fd596103e06101405101527f10ba3a0e01df92e87f301c4b716d8a394d67f4bf42a75c10922910a78f6b5b876104006101405101527f0e070bf53f8451b24f9c6e96b0c2a801cb511bc0c242eb9d361b77693f21471c6104206101405101527f1b94cd61b051b04dd39755ff93821a73ccd6cb11d2491d8aa7f921014de252fb6104406101405101527f1d7cb39bafb8c744e148787a2e70230f9d4e917d5713bb050487b5aa7d74070b6104606101405101527f2ec93189bd1ab4f69117d0fe980c80ff8785c2961829f701bb74ac1f303b17db6104806101405101527f2db366bfdd36d277a692bb825b86275beac404a19ae07a9082ea46bd835179266104a06101405101527f062100eb485db06269655cf186a68532985275428450359adc99cec6960711b86104c06101405101527f0761d33c66614aaa570e7f1e8244ca1120243f92fa59e4f900c567bf41f5a59b6104e06101405101527f20fc411a114d13992c2705aa034e3f315d78608a0f7de4ccf7a72e494855ad0d6105006101405101527f25b5c004a4bdfcb5add9ec4e9ab219ba102c67e8b3effb5fc3a30f317250bc5a6105206101405101527f23b1822d278ed632a494e58f6df6f5ed038b186d8474155ad87e7dff62b37f4b6105406101405101527f22734b4c5c3f9493606c4ba9012499bf0f14d13bfcfcccaa16102a29cc2f69e06105606101405101527f26c0c8fe09eb30b7e27a74dc33492347e5bdff409aa3610254413d3fad795ce56105806101405101527f070dd0ccb6bd7bbae88eac03fa1fbb26196be3083a809829bbd626df348ccad96105a06101405101527f12b6595bdb329b6fb043ba78bb28c3bec2c0a6de46d8c5ad6067c4ebfd4250da6105c06101405101527f248d97d7f76283d63bec30e7a5876c11c06fca9b275c671c5e33d95bb7e8d7296105e06101405101527f1a306d439d463b0816fc6fd64cc939318b45eb759ddde4aa106d15d9bd9baaaa6106006101405101527f28a8f8372e3c38daced7c00421cb4621f4f1b54ddc27821b0d62d3d6ec7c56cf6106206101405101527e94975717f9a8a8bb35152f24d43294071ce320c829f388bc852183e1e2ce7e6106406101405101527f04d5ee4c3aa78f7d80fde60d716480d3593f74d4f653ae83f4103246db2e8d656106606101405101527f2a6cf5e9aa03d4336349ad6fb8ed2269c7bef54b8822cc76d08495c12efde1876106806101405101527f2304d31eaab960ba9274da43e19ddeb7f792180808fd6e43baae48d7efcba3f36106a06101405101527f03fd9ac865a4b2a6d5e7009785817249bff08a7e0726fcb4e1c11d39d199f0b06106c06101405101527eb7258ded52bbda2248404d55ee5044798afc3a209193073f7954d4d63b0b646106e06101405101527f159f81ada0771799ec38fca2d4bf65ebb13d3a74f3298db36272c5ca65e92d9a6107006101405101527f1ef90e67437fbc8550237a75bc28e3bb9000130ea25f0c5471e144cf4264431f6107206101405101527f1e65f838515e5ff0196b49aa41a2d2568df739bc176b08ec95a79ed82932e30d6107406101405101527f2b1b045def3a166cec6ce768d079ba74b18c844e570e1f826575c1068c94c33f6107606101405101527f0832e5753ceb0ff6402543b1109229c165dc2d73bef715e3f1c6e07c168bb1736107806101405101527f02f614e9cedfb3dc6b762ae0a37d41bab1b841c2e8b6451bc5a8e3c390b6ad166107a06101405101527f0e2427d38bd46a60dd640b8e362cad967370ebb777bedff40f6a0be27e7ed7056107c06101405101527f0493630b7c670b6deb7c84d414e7ce79049f0ec098c3c7c50768bbe29214a53a6107e06101405101527f22ead100e8e482674decdab17066c5a26bb1515355d5461a3dc06cc85327cea96108006101405101527f25b3e56e655b42cdaae2626ed2554d48583f1ae35626d04de5084e0b6d2a6f166108206101405101527f1e32752ada8836ef5837a6cde8ff13dbb599c336349e4c584b4fdc0a0cf6f9d06108406101405101527f2fa2a871c15a387cc50f68f6f3c3455b23c00995f05078f672a9864074d412e56108606101405101527f2f569b8a9a4424c9278e1db7311e889f54ccbf10661bab7fcd18e7c7a7d835056108806101405101527f044cb455110a8fdd531ade530234c518a7df93f7332ffd2144165374b246b43d6108a06101405101527f227808de93906d5d420246157f2e42b191fe8c90adfe118178ddc723a53190256108c06101405101527f02fcca2934e046bc623adead873579865d03781ae090ad4a8579d2e7a68003556108e06101405101527f0ef915f0ac120b876abccceb344a1d36bad3f3c5ab91a8ddcbec2e060d8befac6109006101405101527f1797130f4b7a3e1777eb757bc6f287f6ab0fb85f6be63b09f3b16ef2b1405d386109206101405101527f0a76225dc04170ae3306c85abab59e608c7f497c20156d4d36c668555decc6e56109406101405101527f1fffb9ec1992d66ba1e77a7b93209af6f8fa76d48acb664796174b5326a31a5c6109606101405101527f25721c4fc15a3f2853b57c338fa538d85f8fbba6c6b9c6090611889b797b9c5f6109806101405101527f0c817fd42d5f7a41215e3d07ba197216adb4c3790705da95eb63b982bfcaf75a6109a06101405101527f13abe3f5239915d39f7e13c2c24970b6df8cf86ce00a22002bc15866e52b5a966109c06101405101527f2106feea546224ea12ef7f39987a46c85c1bc3dc29bdbd7a92cd60acb4d391ce6109e06101405101527f21ca859468a746b6aaa79474a37dab49f1ca5a28c748bc7157e1b3345bb0f959610a006101405101527f05ccd6255c1e6f0c5cf1f0df934194c62911d14d0321662a8f1a48999e34185b610a206101405101527f0f0e34a64b70a626e464d846674c4c8816c4fb267fe44fe6ea28678cb09490a4610a406101405101527f0558531a4e25470c6157794ca36d0e9647dbfcfe350d64838f5b1a8a2de0d4bf610a606101405101527f09d3dca9173ed2faceea125157683d18924cadad3f655a60b72f5864961f1455610a806101405101527f0328cbd54e8c0913493f866ed03d218bf23f92d68aaec48617d4c722e5bd4335610aa06101405101527f2bf07216e2aff0a223a487b1a7094e07e79e7bcc9798c648ee3347dd5329d34b610ac06101405101527f1daf345a58006b736499c583cb76c316d6f78ed6a6dffc82111e11a63fe412df610ae06101405101527f176563472456aaa746b694c60e1823611ef39039b2edc7ff391e6f2293d2c404610b00610140510152610b206101405101907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f000000182527f10dc6e9c006ea38b04b1e03b4bd9490c0d03f98929ca1d7fb56821fd19d3b6e7610b406101405101527f0c28145b6a44df3e0149b3d0a30b3bb599df9756d4dd9b84a86b38cfb45a740b610b606101405101527e544b8338791518b2c7645a50392798b21f75bb60e3596170067d00141cac15610b806101405101527f222c01175718386f2e2e82eb122789e352e105a3b8fa852613bc534433ee428b610ba06101405101526803000000000000000060c0528260a0526101a05161012052600960805260e052516101c0526101c0516101c05160c05183086101c05160c0518408086101c051906101c0516101c0516101a051608051086101c05160c051800808906101c05190800808610180526101c0516101c0516101a051608051086101c0516101a05160805108086101c051906101c0516101c05160c05185086101c0516101a051800808906101c05190800808610160526101c051610160516101c0516101c0516101a051608051086101c05160c051800808086101c0516101805160c05192939192909183918291086101c0516101a0518008080861018051608092909252610160516101205260a05260c0525b610220610140510160e05110612d045760e051610700016101005261014051610b40810151610b60820151610b80830151610ba09093015192905b6101005160e05110612c74575050505061020060e051015b8060e05110612c3957506101405160405260805190565b612c556101c05160e05160c05160a05161012051608051612f50565b926080929192526101205260a05260c052608060e0510160e052612c22565b6101c05160e05151608051086101c0518180096101c051919082908009096101c0516101c05160c05160a051086101c0516101205184080890816101c05191866101c0519109086080526101c051816101c05184610120510908610120526101c051816101c0518560a051090860a0526101c051906101c0518660c051090860c052602060e0510160e052612c0a565b612d206101c05160e05160c05160a05161012051608051612f50565b926080929192526101205260a05260c052608060e0510160e052612bcf565b919091612d4b83612f0c565b7fffffffff000000000000000000000000000000000000000000000000000000006040519160e01b16602082015260048152612d8860248261070b565b925f915b818310612d9857505050565b909193612da961125086858561110e565b612db487868661110e565b35600281101561040557600193602083600593612e579583977fffffffff00000000000000000000000000000000000000000000000000000000612df783612f0c565b60e01b167fff000000000000000000000000000000000000000000000000000000000000006040519a888c995191829101898b015e88019460f81b16868501526021840152602583013701015f838201520301601f19810183528261070b565b94019190612d8c565b67ffffffffffffffff811161040957601f01601f191660200190565b81601f8201121561040557803590612e9382612e60565b92612ea1604051948561070b565b8284526020838301011161040557815f926020809301838601378301015290565b90601f19601f602080948051918291828752018686015e5f8582860101520116010190565b9091612efe6107c193604084526040840190612ec2565b916020818403910152612ec2565b63ffffffff8111612f205763ffffffff1690565b7f6dfcc650000000000000000000000000000000000000000000000000000000005f52602060045260245260445ffd5b9391949290946102c052610320526102805261034052610340519061028051519008610300526103405190602061028051015190086102e052610340516102e0516102e051096102a0526103405160406102805101516102c05108610340516060610280510151610320510861024052610340516102405161024051096102205261034051906103405161024051610340516102205161022051090961034051909181808280096103405183800909090861026052610340516102605161034051610340516102e051610340516102a0516102a0510909610340516102e0516102a0518290800909080861020052610340516102e0516102a05182918291829080090961034051610300518180828009610340516103005180090909086103405161034051610240516103405161022051610220510909610340516102405161022051829080090908086101e05261034051610260518008610340516101e05190918290800808906103405161034051610340516102e051610340516102a0516102a0510909610340516103005181808280096103405161030051800909090861034051610340516102e051610340516102a0516102a05109096103405161030051818082800961034051610300518009090908086103405161020051909182908008089161034051836101e051089161034051826102005108929392919056fea164736f6c6343000826000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x03``@R`\x036\x11a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80cF{P\x11\x14a\x01OW\x80c\xBD\xEBD-\x14a\x01,W\x80c\xC1\xB0\xBE\xD7\x14a\0\xFCW\x80c\xC4IV\xD1\x14a\0\xDFW\x80c\xDB\x19\xAAy\x14a\0\x8EWc\xF8\xAD\xF4\xC3\x14a\0\\W_\x80\xFD[4a\0\x8BW` `\x03\x196\x01\x12a\0\x8BW`\xFF`@` \x92`\x045\x81Ra\x01\x81\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` `@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` \x90T`@Q\x90\x81R\xF3[P4a\0\x8BW` `\x03\x196\x01\x12a\0\x8BW`\xFF`@` \x92`\x045\x81Ra\x01\x82\x84R T\x16`@Q\x90\x15\x15\x81R\xF3[P4a\0\x8BW\x80`\x03\x196\x01\x12a\0\x8BW` a\x01Ga\x0E\xF7V[`@Q\x90\x81R\xF3[P4a\x04\x05W` `\x03\x196\x01\x12a\x04\x05W`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05W\x80`\x04\x01`@`\x03\x19\x836\x03\x01\x12a\x04\x05W\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0\\a\x06\x8FW`\x01\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]a\x01\xD9\x81\x80a\x06\xB7V[\x90\x91\x81\x15a\x06gW_\x93a\x01\xEC\x83a\x07FV[\x91_[\x84\x81\x10a\x046WPa\x02<\x90`$s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93\x01\x90a\x08\x05V[\x91\x90`@Q\x90` \x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@R\x87\x82R\x82;\x15a\x04\x05W\x92\x90a\x02\xA5`@Q\x94\x85\x93\x7F\x8D\x8F\x8A\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85R`@`\x04\x86\x01R`D\x85\x01\x91a\x08VV[\x90_`$\x84\x01[`\x01\x82\x10a\x03\xEBWPPP\x91\x81\x80_\x94\x03\x91Z\xFA\x80\x15a\x03\xE0Wa\x03\xCAW[P` a\x02\xF8\x7F\n-\xC5H\xED\x95\n\xCC\xB4\r]xT\x1F9T\xC5\xE1\x82\xA8\xEC\xF1\x9EX\x1AO\"c\xF6\x1FY\xD2\x92a\t\x03V[\x80\x87Ra\x01\x81\x82R`@\x87 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90U`@Q\x90\x81R\xA1\x83[\x81\x81\x10a\x03\x8AWPPP\x7F\x86-\x83\xC6\xC5\xAF'^i{\xFDN'\xC82<\x19kD\xBD\xD0\x11\xDD\x9A\xAA\xB6\xDB\x0E\xC9\x94=\xCE\x82\x80\xA2\x80\x7F\x9Bw\x9B\x17B-\r\xF9\"#\x01\x8B2\xB4\xD1\xFAF\xE0qr=h\x17\xE2Hm\0;\xEC\xC5_\0]\x80\xF3[\x80a\x03\xA9a\x03\xA4a\x03\x9E`\x01\x94\x86\x88a\x07\xAAV[\x80a\x07wV[a\x17\xC7V[a\x03\xC4a\x03\xA4a\x03\xBA\x83\x86\x88a\x07\xAAV[` \x81\x01\x90a\x07wV[\x01a\x036V[a\x03\xD7\x91\x95P_\x90a\x07\x0BV[_\x93` a\x02\xCBV[`@Q=_\x82>=\x90\xFD[\x82Q\x81R\x86\x94P` \x92\x83\x01\x92`\x01\x92\x90\x92\x01\x91\x01a\x02\xACV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[\x95a\x04B\x87\x86\x88a\x07\xAAV[`@\x81\x015\x80_Ra\x01\x81` R`\xFF`@_ T\x16\x15a\x06<Wa\x04g\x82\x80a\x07wV[5\x80_Ra\x01\x82` R`\xFF`@_ T\x16a\x06\x11W\x91` \x93\x91a\x05Y\x84_\x95\x86Ra\x01\x82\x87R`@\x86 `\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x82T\x16\x17\x90Ua\x04\xCEa\x04\xC9\x85\x80a\x07wV[a\x11eV[\x86\x84\x01\x90a\x04\xDFa\x04\xC9\x83\x87a\x07wV[a\x04\xE9\x82\x86a\x07wV[5\x90\x7F\x98W\xD0\xB8\x8F\x01\xDC\xA7\xBB\x12\x05\xD6\x8C0}(\xD8\xD4\0/\xF5H`\x07\xACLD5\x96\xDC\xEA!`@\x8Aa\x05\x19\x89\x80a\x07wV[\x015\x8Ba\x05&\x87\x8Ba\x07wV[\x015\x82Q\x91\x82R\x8C\x82\x01R\xA3\x8Ca\x05Ha\x05@\x83\x87a\x07wV[5\x91\x8Ca\x07\xF1V[Ra\x05S\x84\x80a\x07wV[\x93a\x07wV[\x85a\x05oa\x05j`@\x86\x01\x86a\x08vV[a\x13\xC0V[\x91a\x05\x80a\x05j`@\x83\x01\x83a\x08vV[\x92\x82`@Q\x96\x81\x88\x01\x98\x89R\x805`@\x89\x01R\x015``\x87\x01R`\x80\x86\x01R\x805`\xA0\x86\x01R\x015`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01\0\x81Ra\x05\xC9a\x01 \x82a\x07\x0BV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x03\xE0W`\x01\x7F\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_Q\x16\x96\x01a\x01\xEFV[\x7F9\xA9@\xC5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xF9\x84\x9E\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\xCF\x1B\t<\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x7F>\xE5\xAE\xB5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04_\xFD[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x04\x05WV[\x90`\x1F`\x1F\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@RV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\tW`\x05\x1B` \x01\x90V[\x90a\x07P\x82a\x07.V[a\x07]`@Q\x91\x82a\x07\x0BV[\x82\x81R`\x1F\x19a\x07m\x82\x94a\x07.V[\x01\x90` 6\x91\x017V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[\x90\x82\x10\x15a\x07\xC4Wa\x07\xC1\x91`\x05\x1B\x81\x01\x90a\x07wV[\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a\x07\xC4W` \x91`\x05\x1B\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W` \x01\x91\x816\x03\x83\x13a\x04\x05WV[`\x1F\x82` \x94\x93`\x1F\x19\x93\x81\x86R\x86\x86\x017_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x905\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[\x91\x90\x82\x01\x80\x92\x11a\x08\xB6WV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x90`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x01\x90_\x90V[\x91\x90\x82\x03\x91\x82\x11a\x08\xB6WV[_T\x90\x80Q\x15a\x0E\xE0Wa\t\x18\x81Q\x83a\x08\xA9V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x81\x11a\x0E\xB5W\x81Q_[\x81\x81\x10a\x0EEWPPa\t=\x81a\x14\xA4V[\x90_\x82Q\x80`\x01\x1B\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x08\xB6W`\x02\x81\x01\x80\x91\x11a\x08\xB6Wa\t\x85\x90\x92\x91\x92a\x07FV[\x95\x85\x91\x85Q\x94_[\x84\x87\x82\x10\x15a\x0BeW_\x19\x81\x14a\x08\xB6W`\x01\x96\x87\x83\x8Da\t\xC2\x8Da\t\xBB\x85`\x02\x98\x01\x9D\x8E\x16\x94\x85\x92a\x07\xF1V[Q\x92a\x07\xF1V[R\x1B\x17\x95\x01\x80\x87\x11a\x08\xB6W\x86\x81\x16a\t\xE2W[P`\x01\x01\x94\x93\x94a\t\x8DV[\x90\x94a\nq\x82`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x90\x81`\x01\x1B\x91\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x08\xB6W`\x01\x83\x01\x80\x84\x11\x80a\x08\xB6W`\x01\x80\x85\x84\x1C\x16\x14_\x14a\x0BNWa\x08\xB6Wa\n\xC8\x90\x8Ea\x07\xF1V[Q\x93[\x8D\x84`\x01\x80\x86\x83\x1C\x16\x14_\x14a\x0B6Wa\n\xE4\x91a\x07\xF1V[Q[`\x02\x85\x01\x95\x86\x86\x11a\x08\xB6W`\x01\x84\x01\x80\x94\x11a\x08\xB6W\x8F\x90a\x0B(\x90a\x0B\"a\x0B\x1A`\x01\x9A\x8B\x80\x98`\x03\x9A\x1C\x16\x90a\x08\xA9V[\x94\x85\x92a\x1B>V[\x92a\x07\xF1V[R\x1B\x92\x1B\x19\x16\x17\x94\x90a\t\xD6V[PP`\x80\x84\x10\x15a\x07\xC4W`\x03\x84\x02`\x01\x01Ta\n\xE6V[P`\x80\x81\x10\x15a\x07\xC4W`\x03\x02`\x01\x01T\x93a\n\xCBV[PP\x94P\x94P\x94\x90P\x81\x92_\x91\x80a\x0E\nW[P`\x04\x85\x01\x80\x86\x11a\x08\xB6Wa\x0B\x8E\x90\x82a\x15\x88V[\x91a\x0B\x9A\x81\x83_a\x19\xE2V[\x93\x91\x94\x90\x93\x95a\x0B\xAC\x83Q\x92\x83a\x08\xF6V[\x92a\x0B\xB7\x85\x84a\x08\xF6V[\x94a\r\xDCW\x8B\x94\x93\x92_[\x85_\x19\x83\x01\x92\x83\x11\x96[a\x0C\xBAWPPPPPPPP_\x14a\x0C\xB4W\x90a\x0B\xE8\x91a\x1B>V[\x93\x81`\x01\x92[a\x0B\xF9WPPP_UV[a\x0C\x86\x81`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[a\x0C\x90\x81\x84a\x07\xF1V[Q\x90`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x84\x01U_\x19\x81\x01\x90\x80\x82\x11a\x08\xB6W\x16\x80a\x0B\xEEV[Pa\x0B\xE8V[\x90a\x0C\xC9\x87\x98\x9B\x96\x97\x92a\x0E\xEBV[\x95a\x0C\xEE\x87\x86`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x80\x9C`\x01\x80\x89\x84\x1C\x16\x14_\x14a\r\xC0W\x90a\r\x08\x91a\x07\xF1V[Q\x92[a\x08\xB6W\x86\x84\x03a\r'WPP\x84\x90\x99[\x94\x8E\x97\x96\x95\x91a\x0B\xCCV[\x91\x98`\x01\x80`@\x87\x9D\x94\x9D\x01Q\x89\x1C\x16\x14_\x14a\r\x8EW\x86\x92\x91\x90\x15a\r\x87W\x90a\rQ\x91a\x1B>V[\x97[\x88\x99`\x01\x99\x88\x84\x10\x15a\riW[PP\x99a\r\x1CV[\x90\x92P`\x80\x83\x10\x15a\x07\xC4W`\x02`\x03\x88\x94\x02\x8B\x01\x01U_\x80a\raV[P\x97a\rSV[\x98a\r\x9C\x91\x9AP\x86\x92a\x1B>V[\x99\x86\x82\x10a\r\x1CW\x90P`\x80\x81\x10\x15a\x07\xC4W`\x02`\x03\x82\x02\x01\x8A\x90U\x84\x90a\r\x1CV[PP\x91P`\x80\x8B\x10\x15a\x07\xC4W`\x01`\x03\x8C\x02\x01T\x91\x88a\r\x0BV[a\x0E\0\x85\x82`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x92\x8C\x95\x94\x93a\x0B\xC2V[\x90\x91Pa\x0E\x16\x81a\x14\xA4V[a\x0E \x86\x82a\x15\x88V[\x91`\x04\x81\x01\x80\x91\x11a\x08\xB6Wa\x0E5\x91a\x15\x88V[\x90\x81\x80\x82\x10\x91\x18\x02\x18\x90_a\x0BxV[\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01a\x0Ep\x82\x86a\x07\xF1V[Q\x10\x15a\x0E\x7FW`\x01\x01a\t+V[a\x0E\x89\x90\x84a\x07\xF1V[Q\x7F\xC5>rk\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[\x7F\x08Ms\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x04R`$_\xFD[PPa\x07\xC1_a\x10\x04V[\x80\x15a\x08\xB6W_\x19\x01\x90V[_T\x80\x15a\x0F\xFFWa\x0F\x08\x81a\x14\xA4V[\x90`\x04\x81\x01\x80\x91\x11a\x08\xB6Wa\x0F!a\x0F:\x91\x83a\x15\x88V[\x91a\x0F-\x83\x82_a\x19\xE2V[\x92\x91\x93\x90\x92\x94\x82Qa\x08\xF6V[\x80`\x01\x91[a\x0FZWPPP_\x14a\x0FVW\x90a\x07\xC1\x91a\x1B>V[P\x90V[a\x0Fc\x90a\x0E\xEBV[\x94a\x0F\x88\x86\x84`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[`\x80\x81\x10\x15a\x07\xC4W`\x03\x02\x82\x01T\x83Q\x91_\x19\x83\x01\x92\x83\x11a\x08\xB6W\x87\x92\x83\x03a\x0F\xB4WP\x95a\x0F?V[\x92\x95\x91\x86\x80`@\x87\x98\x94\x98\x01Q\x83\x1C\x16\x14_\x14a\x0F\xEBWP\x15a\x0F\xE5W\x90a\x0F\xDB\x91a\x1B>V[\x92\x84\x81\x93\x95a\x0F?V[Pa\x0F\xDBV[\x91\x95\x92\x94a\x0F\xF9\x91\x95a\x1B>V[\x95a\x0F?V[P_\x90V[\x80T\x80\x15a\x11\x08Wa\x10\x15\x81a\x14\xA4V[\x91`\x04\x82\x01\x80\x92\x11a\x08\xB6Wa\x10.a\x10I\x92\x84a\x15\x88V[\x92`\x01a\x10<\x85\x83\x85a\x19\xE2V[\x94\x91\x95\x90\x94\x96\x84Qa\x08\xF6V[\x91\x01\x90\x80[a\x10eWPPP_\x14a\x0FVW\x90a\x07\xC1\x91a\x1B>V[a\x10n\x90a\x0E\xEBV[\x94a\x10\x9Ca\x10\x96\x87\x85`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x83a\x08\xE3V[PT\x83Q\x91_\x19\x83\x01\x92\x83\x11a\x08\xB6W\x87\x92\x83\x03a\x10\xBBWP\x95a\x10NV[\x92\x95\x91`\x01\x80`@\x87\x98\x94\x98\x01Q\x83\x1C\x16\x14_\x14a\x10\xF4WP\x15a\x10\xEEW\x90a\x10\xE3\x91a\x1B>V[\x92\x84`\x01\x93\x95a\x10NV[Pa\x10\xE3V[\x91\x95\x92\x94a\x11\x02\x91\x95a\x1B>V[\x95a\x10NV[PP_\x90V[\x91\x90\x81\x10\x15a\x07\xC4W`\x05\x1B\x81\x015\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC1\x816\x03\x01\x82\x12\x15a\x04\x05W\x01\x90V[`@\x90a\x07\xC1\x94\x92\x81R\x81` \x82\x01R\x01\x91a\x08VV[\x805\x90`@\x81\x01\x90a\x11\x80a\x11z\x83\x83a\x08vV[\x80a\x06\xB7V[_[\x81\x81\x10a\x13QWPPPa\x11\xA3a\x11\x99\x83\x83a\x08vV[` \x81\x01\x90a\x06\xB7V[_[\x81\x81\x10a\x12\xE2WPPPa\x11\xC6a\x11\xBC\x83\x83a\x08vV[`@\x81\x01\x90a\x06\xB7V[_[\x81\x81\x10a\x12sWPPPa\x11\xE9\x91a\x11\xDF\x91a\x08vV[``\x81\x01\x90a\x06\xB7V[\x90\x91_[\x82\x81\x10a\x11\xFAWPPPPV[a\x12\x05\x81\x84\x86a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x12 W[\x01a\x11\xEDV[\x82\x7F\xA4\x94\xDA\xC4\xB7\x18HCX?\x97.\x06x>,;\xB4\x7FO\x017\xB8\xDFR\xA8`\xDF\x07!\x9F\x8Ca\x12Za\x12P\x84\x88\x8Aa\x11\x0EV[` \x81\x01\x90a\x08\x05V[\x90a\x12k`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x12\x1AV[a\x12~\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x12\x99W[\x01a\x11\xC8V[\x86\x7F\x9Ca\xB2\x90\xF61\t\x7FV'<\xF4\xDA\xF4\r\xF1\xFF\x9C\xCC3\xF1\x01\xD4d\x83}\xA1\xF5\xAE\x18\xBDYa\x12\xC9a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x12\xDA`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x12\x93V[a\x12\xED\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x13\x08W[\x01a\x11\xA5V[\x86\x7FH$8s\xB4u-\xDC\xB4^\r{\x11\xC4\xC2fX>^\t\x9A\x0By\x8F\xDD\x9C\x1A\xF7\xD4\x93$\xF3a\x138a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x13I`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x13\x02V[a\x13\\\x81\x83\x85a\x11\x0EV[5\x90`\x02\x82\x10\x15a\x04\x05W`\x01\x80\x92\x14a\x13wW[\x01a\x11\x82V[\x86\x7F:\x13M\x01\xC0x\x03\0<c0\x17\x17\xDD\xC4a.lG\xAE@\x8E\xEE\xA3\",\xDE\xD52\xD0*\xE6a\x13\xA7a\x12P\x84\x87\x89a\x11\x0EV[\x90a\x13\xB8`@Q\x92\x83\x92\x87\x84a\x11NV[\x03\x90\xA2a\x13qV[_` \x91\x82a\x14ba\x13\xDBa\x13\xD5\x84\x80a\x06\xB7V[\x90a-?V[\x92\x82a\x13\xECa\x13\xD5\x82\x84\x01\x84a\x06\xB7V[\x91\x81a\x14\x0Fa\x13\xD5a\x14\x04a\x13\xD5`@\x86\x01\x86a\x06\xB7V[\x93``\x81\x01\x90a\x06\xB7V[\x91\x81`@Q\x97\x88\x96\x81\x80\x89\x01\x9B\x80Q\x91\x82\x91\x01\x8D^\x88\x01\x90\x82\x82\x01\x8D\x81R\x81Q\x93\x84\x92\x01\x90^\x01\x01\x90\x89\x82R\x80Q\x92\x83\x91\x01\x82^\x01\x90\x87\x82R\x80Q\x92\x83\x91\x01\x82^\x01\x85\x81R\x03`\x1F\x19\x81\x01\x83R\x82a\x07\x0BV[`@Q\x91\x82\x91Q\x80\x91\x83^\x81\x01\x83\x81R\x03\x90`\x02Z\xFA\x15a\x03\xE0W\x7F\x1F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_Q\x16\x90V[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04\tW`@R```\x01\x82\x94\x01a\x15Z\x81\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90_\x19`\x01\x83\x1B\x01\x91\x84R\x80` \x85\x01R\x80`\x01\x1C\x82\x82`\x02\x1C\x19\x83\x19\x17\x82\x16\x16`@\x86\x01R\x19\x16\x16\x91\x01RV[\x91\x90_` `\x01\x85Q\x93\x01\x94\x01Q\x93\x80\x85\x18\x94\x85\x84\x1C\x15_\x14a\x16BWPPPa\x16<_\x19\x92\x93\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90\x03\x01\x90V[\x90\x80\x92\x95P`\x02\x84\x01\x1C\x15a\x16VWPPPV[\x92\x93P\x90\x91\x19`\x01\x83\x1B_\x19\x01\x16\x80a\x17+WP\x81[`\x01\x81\x01\x91_\x19`\x01\x84\x1B\x01\x16\x80\x15_\x14a\x16\x90WPP\x90[\x81\x80\x82\x10\x91\x18\x02\x18\x90V[a\x17#\x91\x92P\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x90\x03\x90a\x16\x85V[a\x17\xBE_\x19\x91\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0o\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x83\x15`\x08\x1B\x17\x83\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x83\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x83\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x83\x81\x1C`\xFF\x10`\x03\x1B\x17\x92\x83\x1C\x1C`\x1F\x16\x1A\x17\x90V[\x83\x03\x01\x91a\x16lV[a\x17\xD7a\x11\xBC`@\x83\x01\x83a\x08vV[` _\x93\x015\x92[\x81\x81\x10a\x17\xECWPPPPV[a\x17\xFAa\x12P\x82\x84\x86a\x11\x0EV[\x81\x01``\x82\x82\x03\x12a\x04\x05W\x815\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x80\x93\x03a\x04\x05W` \x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05W\x82a\x18G\x91\x83\x01a.|V[\x91`@\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\x05Wa\x18f\x92\x01a.|V[\x90`@Q\x91\x7F3\xA8\x92\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83R\x87`\x04\x84\x01R`@`$\x84\x01R_\x83\x80a\x18\xAB`D\x82\x01\x86a.\xC2V[\x03\x81\x83\x88Z\xF1\x92\x83\x15a\x03\xE0W_\x93a\x19fW[P\x82Q\x81Q\x80\x82\x14\x91\x82a\x19PW[PP\x15a\x19\x15WP`\x01\x93\x92\x91\x7F\xCD\xDB2z\xDB1\xFET7\xDF*\x8Ch0\x1B\xB1:k\xAA\xE42\xA8\x04\x83\x8C\xAA\xF6\x82Pj\xAD\xF1\x91a\x19\x0C`@Q\x92\x83\x92\x83a.\xE7V[\x03\x90\xA2\x01a\x17\xDFV[\x90Pa\x19L`@Q\x92\x83\x92\x7F\xC5\x04\xFA\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84R`\x04\x84\x01a.\xE7V[\x03\x90\xFD[\x90\x91P` \x85\x01 \x90` \x83\x01 \x14_\x80a\x18\xCEV[\x90\x92P=\x80_\x83>a\x19x\x81\x83a\x07\x0BV[\x81\x01\x90` \x81\x83\x03\x12a\x04\x05W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x04\x05W\x01\x81`\x1F\x82\x01\x12\x15a\x04\x05W\x80Q\x90a\x19\xAF\x82a.`V[\x92a\x19\xBD`@Q\x94\x85a\x07\x0BV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\x05W\x81_\x92` \x80\x93\x01\x83\x86\x01^\x83\x01\x01R\x91_a\x18\xBFV[\x92_\x93_\x93\x80\x15a\x1B1Wa\x19\xF8\x90\x84Qa\x08\xF6V[\x90`\x01a\x1A\x97a\x1A\x91`@\x87\x01Q\x85\x1C`\x01\x81\x19\x01\x16~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05`\x1Fa\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x86\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x92\x83\x1Cc\xD7dS\xE0\x04\x16\x1A\x17\x90V[\x84a\x08\xA9V[\x91\x01\x91a\x1A\xC7a\x1A\xC1\x82\x87`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x84a\x08\xE3V[P\x90\x82\x03a\x1B(WT[\x93`\x01\x82\x01\x80\x92\x11a\x08\xB6W\x80Q\x82\x10a\x1A\xEAWPPPV[`\x02\x94\x96Pa\x1B\x1F\x93\x95P\x90a\x1B\x19\x91`\x01\x80\x92``\x82` \x85\x01Q\x83\x1C\x16\x82\x01\x83\x1B\x93\x01Q\x90\x1C\x16\x90\x03\x01\x90V[\x90a\x08\xE3V[P\x01T\x91`\x01\x91V[`\x01\x01Ta\x1A\xD1V[P_\x94P\x84\x93P\x83\x92PPV[\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x80\x92_\x08a\x01\xA0R_\x08a\x1B\x80a\x0B\xC0`@Qa\x01@Ra\x01@Qa\x07\x0BV[a\x0B\xA0a\x01@QR` a\x01@Q\x01\x7F\x19\xB8I\xF6\x94P\xB0hH\xDA\x1D9\xBD^JC\x02\xBB\x86tN\xDC&#\x8B\x08x\xE2i\xED#\xE5\x81R\x7F&]\xDF\xE1'\xDDQ\xBDr94{u\x8F\n\x13 \xEB,\xC7E\n\xCC\x1D\xADG\xF8\x0C\x8D\xCF4\xD6`@a\x01@Q\x01R\x7F\x19\x97P\xECG/\x18\t\xE0\xF6jT^\x1EQbA\x08\xAC\x84P\x15\xC2\xAA=\xFC6\xBA\xB4\x97\xD8\xAA``a\x01@Q\x01R\x7F\x15\x7F\xF3\xFEe\xACr\x08\x11\x0F\x06\xA5\xF7C\x02\xB1Mt>\xA2Pg\xF0\xFF\xD02\xF7\x87\xC7\xF1\xCD\xF8`\x80a\x01@Q\x01R\x7F.I\xC4<Ei\xDD\x9C_\xD3Z\xC4_\xCA3\xF1\x0B\x15\xC5\x90i/\x8B\xEE\xFE\x18\xF4\x89j\xC9I\x02`\xA0a\x01@Q\x01R\x7F\x0E5\xFB\x89\x98\x18\x90R\rJ\xEF+me\x06\xC3\xCB/\x0Bis\xC2O\xA8'14_\xFA-\x1F\x1E`\xC0a\x01@Q\x01R\x7F%\x1A\xD4|\xB1\\O\x11\x05\xF1\t\xAE^\x94O\x1B\xA9\xD9\xE7\x80mf\x7F\xFE\xC6\xFEr0\x02\xE0\xB9\x96`\xE0a\x01@Q\x01R\x7F\x13\xDA\x07\xDCd\xD4(6\x98s\xE9q`#FA\xF8\xBE\xB5o\xDD\x05\xE5\xF3V?\xA3\x9D\x9C\"\xDFNa\x01\0a\x01@Q\x01R\x7F\x0C\0\x9B\x84\xE6P\xE6\xD2=\xC0\x0C}\xCC\xEFt\x83\xA5S\x93\x96\x89\xD3P\xCDF\xE7\xB8\x90U\xFDG8a\x01 a\x01@Q\x01R\x7F\x01\x1F\x16\xB1\xC6:\x85O\x01\x99.9V\xF4-\x8B\x04\xEBe\x0CmS^\xB0 =\xECt\xBE\xFD\xCA\x06a\x01@\x80Q\x01R\x7F\x0E\xD6\x9E^8:h\x8F \x9D\x9AV\x1D\xAAya/?x\xD0Fz\xD4T\x85\xDF\x07\t?6uIa\x01`a\x01@Q\x01R\x7F\x04\xDB\xA9J{\x0C\xE9\xE2!\xAC\xADAG+k\xBE:\xECP\x7F^\xB3\xD3?F6r&L\x9Fx\x9Ba\x01\x80a\x01@Q\x01R\x7F\n?&7\xD8@\xF3\xA1n\xB0\x94'\x1C\x9D#{`6u}K\xB5\x0B\xF7\xCEs/\xF1\xD4\xFA(\xE8a\x01\xA0a\x01@Q\x01R\x7F%\x9Afo\x12\x9E\xEA\x19\x8F\x8A\x1CP/\xDB8\xFA9\xB1\xF0uV\x95d\xB6\xE5JH]\x11\x822?a\x01\xC0a\x01@Q\x01R\x7F(\xBFtY\xC9\xB2\xF4\xC6\xD8\xE7\xD0jN\xE3\xA4\x7FwE\xD4'\x108\xE5\x15z2\xFD\xF7\xED\xE0\xD6\xA1a\x01\xE0a\x01@Q\x01R\x7F\n\x1C\xA9A\xF0W\x03u&\xEA \x0FH\x9B\xE8\xD4\xC3|\x85\xBB\xCC\xE6\xA2\xAE\xEC\x91\xBDiAC$Ga\x02\0a\x01@Q\x01R\x7F\x0Co\x8F\x95\x8B\xE0\xE90S\xD7\xFDO\xC5E\x12\x85U5\xED\x159\xF0Q\xDC\xB4:&\xFD\x92ca\xCFa\x02 a\x01@Q\x01R\x7F\x121\x06\xA9<\xD1ux\xD4&\xE8\x12\x8A\xC9\xD9\n\xA9\xE8\xA0\x07\x08\xE2\x96\xE0\x84\xDDW\xE6\x9C\xAA\xF8\x11a\x02@a\x01@Q\x01R\x7F&\xE1\xBAR\xAD\x92\x85\xD9}\xD3\xABR\xF8\xE8@\x08^\x8F\xA8?\xF1\xE8\xF1\x87{\x07Hg\xCD-\xEEua\x02`a\x01@Q\x01R\x7F\x1C\xB5\\\xAD{\xD13\xDE\x18\xA6L\\G\xB9\xC9|\xBEM\x8B{\xF9\xE0\x95\x86DqS~jJ\xE2\xC5a\x02\x80a\x01@Q\x01R\x7F\x1D\xCDs\xE4j\xCD\x8F\x8E\x0E,|\xE0K\xDE\x7Fm*S\x04=P`\xA4\x1CqC\xF0\x8En\x90U\xD0a\x02\xA0a\x01@Q\x01R\x7F\x01\x10\x03\xE3/m\x9Cf\xF5\x85/\x05GJM\xEF\x0C\xDA)J\x0E\xB4\xE9\xB9\xB1+\x9B\xB4Q.Uta\x02\xC0a\x01@Q\x01R\x7F+\x1E\x80\x9A\xC1\xD1\n\xB2\x9A\xD5\xF2\r\x03\xA5}\xFE\xBA\xDF\xE5\x90?X\xBA\xFE\xD7\xC5\x08\xDD\"\x87\xAE\x8Ca\x02\xE0a\x01@Q\x01R\x7F%9\xDE\x17\x85\xB75\x99\x9F\xB4\xDA\xC3^\xE1~\xD0\xEF\x99]\x05\xAB/\xC5\xFA\xEA\xA6\x9A\xE8{\xCE\xC0\xA5a\x03\0a\x01@Q\x01R\x7F\x0C$lZ.\xF8\xEE\x01&I\x7F\"+>\n\x0E\xF4\xE1\xC3\xD4\x1C\x86\xD4nC\x98,\xB1\x1Dw\x95\x1Da\x03 a\x01@Q\x01R\x7F\x19 \x89\xC4\x97Oh\xE9T\x08\x14\x8F|\x062\xED\xBB\t\xE6\xA6\xAD\x1A\x1C/?\x03\x05\xF5\xD0;R{a\x03@a\x01@Q\x01R\x7F\x1E\xAE\n\xD8\xABh\xB2\xF0j\x0E\xE3n\xEB\r\x0C\x05\x85)\t}\x91\tkum\x8F\xDC/\xB5\xA6\r\x85a\x03`a\x01@Q\x01R\x7F\x17\x91\x90\xE5\xD0\xE2!y\xE4o\x82\x82\x87*\xBC\x88\xDBn/\xDC\r\xEE\x99\xE6\x97h\xBD\x98\xC5\xD0k\xFBa\x03\x80a\x01@Q\x01R\x7F)\xBB\x9E,\x90vs%v\xE9\xA8\x1Cz\xC4\xB82\x14R\x8F}\xB0\x0F1\xBFl\xAF\xE7\x94\xA9\xB3\xCD\x1Ca\x03\xA0a\x01@Q\x01R\x7F\"]9NB u\x99@>\xFD\x0C$d\xA9\rRe&E\x88*\xAC5\xB1\x0EY\x0Eni\x1E\x08a\x03\xC0a\x01@Q\x01R\x7F\x06G`b<%\xC8\xCFu=#\x80U\xB4DS+\xE15WE\x1C\x08}\xE0\x9E\xFDEK#\xFDYa\x03\xE0a\x01@Q\x01R\x7F\x10\xBA:\x0E\x01\xDF\x92\xE8\x7F0\x1CKqm\x8A9Mg\xF4\xBFB\xA7\\\x10\x92)\x10\xA7\x8Fk[\x87a\x04\0a\x01@Q\x01R\x7F\x0E\x07\x0B\xF5?\x84Q\xB2O\x9Cn\x96\xB0\xC2\xA8\x01\xCBQ\x1B\xC0\xC2B\xEB\x9D6\x1Bwi?!G\x1Ca\x04 a\x01@Q\x01R\x7F\x1B\x94\xCDa\xB0Q\xB0M\xD3\x97U\xFF\x93\x82\x1As\xCC\xD6\xCB\x11\xD2I\x1D\x8A\xA7\xF9!\x01M\xE2R\xFBa\x04@a\x01@Q\x01R\x7F\x1D|\xB3\x9B\xAF\xB8\xC7D\xE1Hxz.p#\x0F\x9DN\x91}W\x13\xBB\x05\x04\x87\xB5\xAA}t\x07\x0Ba\x04`a\x01@Q\x01R\x7F.\xC91\x89\xBD\x1A\xB4\xF6\x91\x17\xD0\xFE\x98\x0C\x80\xFF\x87\x85\xC2\x96\x18)\xF7\x01\xBBt\xAC\x1F0;\x17\xDBa\x04\x80a\x01@Q\x01R\x7F-\xB3f\xBF\xDD6\xD2w\xA6\x92\xBB\x82[\x86'[\xEA\xC4\x04\xA1\x9A\xE0z\x90\x82\xEAF\xBD\x83Qy&a\x04\xA0a\x01@Q\x01R\x7F\x06!\0\xEBH]\xB0bie\\\xF1\x86\xA6\x852\x98RuB\x84P5\x9A\xDC\x99\xCE\xC6\x96\x07\x11\xB8a\x04\xC0a\x01@Q\x01R\x7F\x07a\xD3<faJ\xAAW\x0E\x7F\x1E\x82D\xCA\x11 $?\x92\xFAY\xE4\xF9\0\xC5g\xBFA\xF5\xA5\x9Ba\x04\xE0a\x01@Q\x01R\x7F \xFCA\x1A\x11M\x13\x99,'\x05\xAA\x03N?1]x`\x8A\x0F}\xE4\xCC\xF7\xA7.IHU\xAD\ra\x05\0a\x01@Q\x01R\x7F%\xB5\xC0\x04\xA4\xBD\xFC\xB5\xAD\xD9\xECN\x9A\xB2\x19\xBA\x10,g\xE8\xB3\xEF\xFB_\xC3\xA3\x0F1rP\xBCZa\x05 a\x01@Q\x01R\x7F#\xB1\x82-'\x8E\xD62\xA4\x94\xE5\x8Fm\xF6\xF5\xED\x03\x8B\x18m\x84t\x15Z\xD8~}\xFFb\xB3\x7FKa\x05@a\x01@Q\x01R\x7F\"sKL\\?\x94\x93`lK\xA9\x01$\x99\xBF\x0F\x14\xD1;\xFC\xFC\xCC\xAA\x16\x10*)\xCC/i\xE0a\x05`a\x01@Q\x01R\x7F&\xC0\xC8\xFE\t\xEB0\xB7\xE2zt\xDC3I#G\xE5\xBD\xFF@\x9A\xA3a\x02TA=?\xADy\\\xE5a\x05\x80a\x01@Q\x01R\x7F\x07\r\xD0\xCC\xB6\xBD{\xBA\xE8\x8E\xAC\x03\xFA\x1F\xBB&\x19k\xE3\x08:\x80\x98)\xBB\xD6&\xDF4\x8C\xCA\xD9a\x05\xA0a\x01@Q\x01R\x7F\x12\xB6Y[\xDB2\x9Bo\xB0C\xBAx\xBB(\xC3\xBE\xC2\xC0\xA6\xDEF\xD8\xC5\xAD`g\xC4\xEB\xFDBP\xDAa\x05\xC0a\x01@Q\x01R\x7F$\x8D\x97\xD7\xF7b\x83\xD6;\xEC0\xE7\xA5\x87l\x11\xC0o\xCA\x9B'\\g\x1C^3\xD9[\xB7\xE8\xD7)a\x05\xE0a\x01@Q\x01R\x7F\x1A0mC\x9DF;\x08\x16\xFCo\xD6L\xC991\x8BE\xEBu\x9D\xDD\xE4\xAA\x10m\x15\xD9\xBD\x9B\xAA\xAAa\x06\0a\x01@Q\x01R\x7F(\xA8\xF87.<8\xDA\xCE\xD7\xC0\x04!\xCBF!\xF4\xF1\xB5M\xDC'\x82\x1B\rb\xD3\xD6\xEC|V\xCFa\x06 a\x01@Q\x01R~\x94\x97W\x17\xF9\xA8\xA8\xBB5\x15/$\xD42\x94\x07\x1C\xE3 \xC8)\xF3\x88\xBC\x85!\x83\xE1\xE2\xCE~a\x06@a\x01@Q\x01R\x7F\x04\xD5\xEEL:\xA7\x8F}\x80\xFD\xE6\rqd\x80\xD3Y?t\xD4\xF6S\xAE\x83\xF4\x102F\xDB.\x8Dea\x06`a\x01@Q\x01R\x7F*l\xF5\xE9\xAA\x03\xD43cI\xADo\xB8\xED\"i\xC7\xBE\xF5K\x88\"\xCCv\xD0\x84\x95\xC1.\xFD\xE1\x87a\x06\x80a\x01@Q\x01R\x7F#\x04\xD3\x1E\xAA\xB9`\xBA\x92t\xDAC\xE1\x9D\xDE\xB7\xF7\x92\x18\x08\x08\xFDnC\xBA\xAEH\xD7\xEF\xCB\xA3\xF3a\x06\xA0a\x01@Q\x01R\x7F\x03\xFD\x9A\xC8e\xA4\xB2\xA6\xD5\xE7\0\x97\x85\x81rI\xBF\xF0\x8A~\x07&\xFC\xB4\xE1\xC1\x1D9\xD1\x99\xF0\xB0a\x06\xC0a\x01@Q\x01R~\xB7%\x8D\xEDR\xBB\xDA\"H@MU\xEEPDy\x8A\xFC: \x91\x93\x07?yT\xD4\xD6;\x0Bda\x06\xE0a\x01@Q\x01R\x7F\x15\x9F\x81\xAD\xA0w\x17\x99\xEC8\xFC\xA2\xD4\xBFe\xEB\xB1=:t\xF3)\x8D\xB3br\xC5\xCAe\xE9-\x9Aa\x07\0a\x01@Q\x01R\x7F\x1E\xF9\x0EgC\x7F\xBC\x85P#zu\xBC(\xE3\xBB\x90\0\x13\x0E\xA2_\x0CTq\xE1D\xCFBdC\x1Fa\x07 a\x01@Q\x01R\x7F\x1Ee\xF88Q^_\xF0\x19kI\xAAA\xA2\xD2V\x8D\xF79\xBC\x17k\x08\xEC\x95\xA7\x9E\xD8)2\xE3\ra\x07@a\x01@Q\x01R\x7F+\x1B\x04]\xEF:\x16l\xECl\xE7h\xD0y\xBAt\xB1\x8C\x84NW\x0E\x1F\x82eu\xC1\x06\x8C\x94\xC3?a\x07`a\x01@Q\x01R\x7F\x082\xE5u<\xEB\x0F\xF6@%C\xB1\x10\x92)\xC1e\xDC-s\xBE\xF7\x15\xE3\xF1\xC6\xE0|\x16\x8B\xB1sa\x07\x80a\x01@Q\x01R\x7F\x02\xF6\x14\xE9\xCE\xDF\xB3\xDCkv*\xE0\xA3}A\xBA\xB1\xB8A\xC2\xE8\xB6E\x1B\xC5\xA8\xE3\xC3\x90\xB6\xAD\x16a\x07\xA0a\x01@Q\x01R\x7F\x0E$'\xD3\x8B\xD4j`\xDDd\x0B\x8E6,\xAD\x96sp\xEB\xB7w\xBE\xDF\xF4\x0Fj\x0B\xE2~~\xD7\x05a\x07\xC0a\x01@Q\x01R\x7F\x04\x93c\x0B|g\x0Bm\xEB|\x84\xD4\x14\xE7\xCEy\x04\x9F\x0E\xC0\x98\xC3\xC7\xC5\x07h\xBB\xE2\x92\x14\xA5:a\x07\xE0a\x01@Q\x01R\x7F\"\xEA\xD1\0\xE8\xE4\x82gM\xEC\xDA\xB1pf\xC5\xA2k\xB1QSU\xD5F\x1A=\xC0l\xC8S'\xCE\xA9a\x08\0a\x01@Q\x01R\x7F%\xB3\xE5ne[B\xCD\xAA\xE2bn\xD2UMHX?\x1A\xE3V&\xD0M\xE5\x08N\x0Bm*o\x16a\x08 a\x01@Q\x01R\x7F\x1E2u*\xDA\x886\xEFX7\xA6\xCD\xE8\xFF\x13\xDB\xB5\x99\xC364\x9ELXKO\xDC\n\x0C\xF6\xF9\xD0a\x08@a\x01@Q\x01R\x7F/\xA2\xA8q\xC1Z8|\xC5\x0Fh\xF6\xF3\xC3E[#\xC0\t\x95\xF0Px\xF6r\xA9\x86@t\xD4\x12\xE5a\x08`a\x01@Q\x01R\x7F/V\x9B\x8A\x9AD$\xC9'\x8E\x1D\xB71\x1E\x88\x9FT\xCC\xBF\x10f\x1B\xAB\x7F\xCD\x18\xE7\xC7\xA7\xD85\x05a\x08\x80a\x01@Q\x01R\x7F\x04L\xB4U\x11\n\x8F\xDDS\x1A\xDES\x024\xC5\x18\xA7\xDF\x93\xF73/\xFD!D\x16St\xB2F\xB4=a\x08\xA0a\x01@Q\x01R\x7F\"x\x08\xDE\x93\x90m]B\x02F\x15\x7F.B\xB1\x91\xFE\x8C\x90\xAD\xFE\x11\x81x\xDD\xC7#\xA51\x90%a\x08\xC0a\x01@Q\x01R\x7F\x02\xFC\xCA)4\xE0F\xBCb:\xDE\xAD\x875y\x86]\x03x\x1A\xE0\x90\xADJ\x85y\xD2\xE7\xA6\x80\x03Ua\x08\xE0a\x01@Q\x01R\x7F\x0E\xF9\x15\xF0\xAC\x12\x0B\x87j\xBC\xCC\xEB4J\x1D6\xBA\xD3\xF3\xC5\xAB\x91\xA8\xDD\xCB\xEC.\x06\r\x8B\xEF\xACa\t\0a\x01@Q\x01R\x7F\x17\x97\x13\x0FKz>\x17w\xEBu{\xC6\xF2\x87\xF6\xAB\x0F\xB8_k\xE6;\t\xF3\xB1n\xF2\xB1@]8a\t a\x01@Q\x01R\x7F\nv\"]\xC0Ap\xAE3\x06\xC8Z\xBA\xB5\x9E`\x8C\x7FI| \x15mM6\xC6hU]\xEC\xC6\xE5a\t@a\x01@Q\x01R\x7F\x1F\xFF\xB9\xEC\x19\x92\xD6k\xA1\xE7z{\x93 \x9A\xF6\xF8\xFAv\xD4\x8A\xCBfG\x96\x17KS&\xA3\x1A\\a\t`a\x01@Q\x01R\x7F%r\x1CO\xC1Z?(S\xB5|3\x8F\xA58\xD8_\x8F\xBB\xA6\xC6\xB9\xC6\t\x06\x11\x88\x9By{\x9C_a\t\x80a\x01@Q\x01R\x7F\x0C\x81\x7F\xD4-_zA!^=\x07\xBA\x19r\x16\xAD\xB4\xC3y\x07\x05\xDA\x95\xEBc\xB9\x82\xBF\xCA\xF7Za\t\xA0a\x01@Q\x01R\x7F\x13\xAB\xE3\xF5#\x99\x15\xD3\x9F~\x13\xC2\xC2Ip\xB6\xDF\x8C\xF8l\xE0\n\"\0+\xC1Xf\xE5+Z\x96a\t\xC0a\x01@Q\x01R\x7F!\x06\xFE\xEATb$\xEA\x12\xEF\x7F9\x98zF\xC8\\\x1B\xC3\xDC)\xBD\xBDz\x92\xCD`\xAC\xB4\xD3\x91\xCEa\t\xE0a\x01@Q\x01R\x7F!\xCA\x85\x94h\xA7F\xB6\xAA\xA7\x94t\xA3}\xABI\xF1\xCAZ(\xC7H\xBCqW\xE1\xB34[\xB0\xF9Ya\n\0a\x01@Q\x01R\x7F\x05\xCC\xD6%\\\x1Eo\x0C\\\xF1\xF0\xDF\x93A\x94\xC6)\x11\xD1M\x03!f*\x8F\x1AH\x99\x9E4\x18[a\n a\x01@Q\x01R\x7F\x0F\x0E4\xA6Kp\xA6&\xE4d\xD8FgLL\x88\x16\xC4\xFB&\x7F\xE4O\xE6\xEA(g\x8C\xB0\x94\x90\xA4a\n@a\x01@Q\x01R\x7F\x05XS\x1AN%G\x0CaWyL\xA3m\x0E\x96G\xDB\xFC\xFE5\rd\x83\x8F[\x1A\x8A-\xE0\xD4\xBFa\n`a\x01@Q\x01R\x7F\t\xD3\xDC\xA9\x17>\xD2\xFA\xCE\xEA\x12QWh=\x18\x92L\xAD\xAD?eZ`\xB7/Xd\x96\x1F\x14Ua\n\x80a\x01@Q\x01R\x7F\x03(\xCB\xD5N\x8C\t\x13I?\x86n\xD0=!\x8B\xF2?\x92\xD6\x8A\xAE\xC4\x86\x17\xD4\xC7\"\xE5\xBDC5a\n\xA0a\x01@Q\x01R\x7F+\xF0r\x16\xE2\xAF\xF0\xA2#\xA4\x87\xB1\xA7\tN\x07\xE7\x9E{\xCC\x97\x98\xC6H\xEE3G\xDDS)\xD3Ka\n\xC0a\x01@Q\x01R\x7F\x1D\xAF4ZX\0ksd\x99\xC5\x83\xCBv\xC3\x16\xD6\xF7\x8E\xD6\xA6\xDF\xFC\x82\x11\x1E\x11\xA6?\xE4\x12\xDFa\n\xE0a\x01@Q\x01R\x7F\x17ecG$V\xAA\xA7F\xB6\x94\xC6\x0E\x18#a\x1E\xF3\x909\xB2\xED\xC7\xFF9\x1Eo\"\x93\xD2\xC4\x04a\x0B\0a\x01@Q\x01Ra\x0B a\x01@Q\x01\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x82R\x7F\x10\xDCn\x9C\0n\xA3\x8B\x04\xB1\xE0;K\xD9I\x0C\r\x03\xF9\x89)\xCA\x1D\x7F\xB5h!\xFD\x19\xD3\xB6\xE7a\x0B@a\x01@Q\x01R\x7F\x0C(\x14[jD\xDF>\x01I\xB3\xD0\xA3\x0B;\xB5\x99\xDF\x97V\xD4\xDD\x9B\x84\xA8k8\xCF\xB4Zt\x0Ba\x0B`a\x01@Q\x01R~TK\x838y\x15\x18\xB2\xC7dZP9'\x98\xB2\x1Fu\xBB`\xE3Yap\x06}\0\x14\x1C\xAC\x15a\x0B\x80a\x01@Q\x01R\x7F\",\x01\x17W\x188o..\x82\xEB\x12'\x89\xE3R\xE1\x05\xA3\xB8\xFA\x85&\x13\xBCSD3\xEEB\x8Ba\x0B\xA0a\x01@Q\x01Rh\x03\0\0\0\0\0\0\0\0`\xC0R\x82`\xA0Ra\x01\xA0Qa\x01 R`\t`\x80R`\xE0RQa\x01\xC0Ra\x01\xC0Qa\x01\xC0Q`\xC0Q\x83\x08a\x01\xC0Q`\xC0Q\x84\x08\x08a\x01\xC0Q\x90a\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Q`\xC0Q\x80\x08\x08\x90a\x01\xC0Q\x90\x80\x08\x08a\x01\x80Ra\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Qa\x01\xA0Q`\x80Q\x08\x08a\x01\xC0Q\x90a\x01\xC0Qa\x01\xC0Q`\xC0Q\x85\x08a\x01\xC0Qa\x01\xA0Q\x80\x08\x08\x90a\x01\xC0Q\x90\x80\x08\x08a\x01`Ra\x01\xC0Qa\x01`Qa\x01\xC0Qa\x01\xC0Qa\x01\xA0Q`\x80Q\x08a\x01\xC0Q`\xC0Q\x80\x08\x08\x08a\x01\xC0Qa\x01\x80Q`\xC0Q\x92\x93\x91\x92\x90\x91\x83\x91\x82\x91\x08a\x01\xC0Qa\x01\xA0Q\x80\x08\x08\x08a\x01\x80Q`\x80\x92\x90\x92Ra\x01`Qa\x01 R`\xA0R`\xC0R[a\x02 a\x01@Q\x01`\xE0Q\x10a-\x04W`\xE0Qa\x07\0\x01a\x01\0Ra\x01@Qa\x0B@\x81\x01Qa\x0B`\x82\x01Qa\x0B\x80\x83\x01Qa\x0B\xA0\x90\x93\x01Q\x92\x90[a\x01\0Q`\xE0Q\x10a,tWPPPPa\x02\0`\xE0Q\x01[\x80`\xE0Q\x10a,9WPa\x01@Q`@R`\x80Q\x90V[a,Ua\x01\xC0Q`\xE0Q`\xC0Q`\xA0Qa\x01 Q`\x80Qa/PV[\x92`\x80\x92\x91\x92Ra\x01 R`\xA0R`\xC0R`\x80`\xE0Q\x01`\xE0Ra,\"V[a\x01\xC0Q`\xE0QQ`\x80Q\x08a\x01\xC0Q\x81\x80\ta\x01\xC0Q\x91\x90\x82\x90\x80\t\ta\x01\xC0Qa\x01\xC0Q`\xC0Q`\xA0Q\x08a\x01\xC0Qa\x01 Q\x84\x08\x08\x90\x81a\x01\xC0Q\x91\x86a\x01\xC0Q\x91\t\x08`\x80Ra\x01\xC0Q\x81a\x01\xC0Q\x84a\x01 Q\t\x08a\x01 Ra\x01\xC0Q\x81a\x01\xC0Q\x85`\xA0Q\t\x08`\xA0Ra\x01\xC0Q\x90a\x01\xC0Q\x86`\xC0Q\t\x08`\xC0R` `\xE0Q\x01`\xE0Ra,\nV[a- a\x01\xC0Q`\xE0Q`\xC0Q`\xA0Qa\x01 Q`\x80Qa/PV[\x92`\x80\x92\x91\x92Ra\x01 R`\xA0R`\xC0R`\x80`\xE0Q\x01`\xE0Ra+\xCFV[\x91\x90\x91a-K\x83a/\x0CV[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x91`\xE0\x1B\x16` \x82\x01R`\x04\x81Ra-\x88`$\x82a\x07\x0BV[\x92_\x91[\x81\x83\x10a-\x98WPPPV[\x90\x91\x93a-\xA9a\x12P\x86\x85\x85a\x11\x0EV[a-\xB4\x87\x86\x86a\x11\x0EV[5`\x02\x81\x10\x15a\x04\x05W`\x01\x93` \x83`\x05\x93a.W\x95\x83\x97\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a-\xF7\x83a/\x0CV[`\xE0\x1B\x16\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x9A\x88\x8C\x99Q\x91\x82\x91\x01\x89\x8B\x01^\x88\x01\x94`\xF8\x1B\x16\x86\x85\x01R`!\x84\x01R`%\x83\x017\x01\x01_\x83\x82\x01R\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x07\x0BV[\x94\x01\x91\x90a-\x8CV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x04\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x81`\x1F\x82\x01\x12\x15a\x04\x05W\x805\x90a.\x93\x82a.`V[\x92a.\xA1`@Q\x94\x85a\x07\x0BV[\x82\x84R` \x83\x83\x01\x01\x11a\x04\x05W\x81_\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x90`\x1F\x19`\x1F` \x80\x94\x80Q\x91\x82\x91\x82\x87R\x01\x86\x86\x01^_\x85\x82\x86\x01\x01R\x01\x16\x01\x01\x90V[\x90\x91a.\xFEa\x07\xC1\x93`@\x84R`@\x84\x01\x90a.\xC2V[\x91` \x81\x84\x03\x91\x01Ra.\xC2V[c\xFF\xFF\xFF\xFF\x81\x11a/ Wc\xFF\xFF\xFF\xFF\x16\x90V[\x7Fm\xFC\xC6P\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R` `\x04R`$R`D_\xFD[\x93\x91\x94\x92\x90\x94a\x02\xC0Ra\x03 Ra\x02\x80Ra\x03@Ra\x03@Q\x90a\x02\x80QQ\x90\x08a\x03\0Ra\x03@Q\x90` a\x02\x80Q\x01Q\x90\x08a\x02\xE0Ra\x03@Qa\x02\xE0Qa\x02\xE0Q\ta\x02\xA0Ra\x03@Q`@a\x02\x80Q\x01Qa\x02\xC0Q\x08a\x03@Q``a\x02\x80Q\x01Qa\x03 Q\x08a\x02@Ra\x03@Qa\x02@Qa\x02@Q\ta\x02 Ra\x03@Q\x90a\x03@Qa\x02@Qa\x03@Qa\x02 Qa\x02 Q\t\ta\x03@Q\x90\x91\x81\x80\x82\x80\ta\x03@Q\x83\x80\t\t\t\x08a\x02`Ra\x03@Qa\x02`Qa\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x02\xE0Qa\x02\xA0Q\x82\x90\x80\t\t\x08\x08a\x02\0Ra\x03@Qa\x02\xE0Qa\x02\xA0Q\x82\x91\x82\x91\x82\x90\x80\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08a\x03@Qa\x03@Qa\x02@Qa\x03@Qa\x02 Qa\x02 Q\t\ta\x03@Qa\x02@Qa\x02 Q\x82\x90\x80\t\t\x08\x08a\x01\xE0Ra\x03@Qa\x02`Q\x80\x08a\x03@Qa\x01\xE0Q\x90\x91\x82\x90\x80\x08\x08\x90a\x03@Qa\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08a\x03@Qa\x03@Qa\x02\xE0Qa\x03@Qa\x02\xA0Qa\x02\xA0Q\t\ta\x03@Qa\x03\0Q\x81\x80\x82\x80\ta\x03@Qa\x03\0Q\x80\t\t\t\x08\x08a\x03@Qa\x02\0Q\x90\x91\x82\x90\x80\x08\x08\x91a\x03@Q\x83a\x01\xE0Q\x08\x91a\x03@Q\x82a\x02\0Q\x08\x92\x93\x92\x91\x90V\xFE\xA1dsolcC\0\x08&\0\n",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyTransactionNotAllowed()` and selector `0xcf1b093c`.
```solidity
error EmptyTransactionNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyTransactionNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyTransactionNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyTransactionNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyTransactionNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyTransactionNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyTransactionNotAllowed()";
            const SELECTOR: [u8; 4] = [207u8, 27u8, 9u8, 60u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ForwarderCallOutputMismatch(bytes,bytes)` and selector `0xc504fada`.
```solidity
error ForwarderCallOutputMismatch(bytes expected, bytes actual);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ForwarderCallOutputMismatch {
        #[allow(missing_docs)]
        pub expected: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub actual: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ForwarderCallOutputMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: ForwarderCallOutputMismatch) -> Self {
                (value.expected, value.actual)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ForwarderCallOutputMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    expected: tuple.0,
                    actual: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ForwarderCallOutputMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ForwarderCallOutputMismatch(bytes,bytes)";
            const SELECTOR: [u8; 4] = [197u8, 4u8, 250u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.expected,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.actual,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LeafCountTooLarge(uint256)` and selector `0x084d73d9`.
```solidity
error LeafCountTooLarge(uint256 leafCount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeafCountTooLarge {
        #[allow(missing_docs)]
        pub leafCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LeafCountTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: LeafCountTooLarge) -> Self {
                (value.leafCount,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeafCountTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { leafCount: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeafCountTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeafCountTooLarge(uint256)";
            const SELECTOR: [u8; 4] = [8u8, 77u8, 115u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.leafCount),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LeafNotInField(bytes32)` and selector `0xc53e726b`.
```solidity
error LeafNotInField(bytes32 leaf);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeafNotInField {
        #[allow(missing_docs)]
        pub leaf: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LeafNotInField> for UnderlyingRustTuple<'_> {
            fn from(value: LeafNotInField) -> Self {
                (value.leaf,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeafNotInField {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { leaf: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeafNotInField {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeafNotInField(bytes32)";
            const SELECTOR: [u8; 4] = [197u8, 62u8, 114u8, 107u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.leaf),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NonExistingRoot(bytes32)` and selector `0xf9849ea3`.
```solidity
error NonExistingRoot(bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonExistingRoot {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonExistingRoot> for UnderlyingRustTuple<'_> {
            fn from(value: NonExistingRoot) -> Self {
                (value.root,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonExistingRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { root: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonExistingRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonExistingRoot(bytes32)";
            const SELECTOR: [u8; 4] = [249u8, 132u8, 158u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PreExistingNullifier(bytes32)` and selector `0x39a940c5`.
```solidity
error PreExistingNullifier(bytes32 nullifier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PreExistingNullifier {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PreExistingNullifier> for UnderlyingRustTuple<'_> {
            fn from(value: PreExistingNullifier) -> Self {
                (value.nullifier,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PreExistingNullifier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { nullifier: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PreExistingNullifier {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PreExistingNullifier(bytes32)";
            const SELECTOR: [u8; 4] = [57u8, 169u8, 64u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifier),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ReentrancyGuardReentrantCall()` and selector `0x3ee5aeb5`.
```solidity
error ReentrancyGuardReentrantCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReentrancyGuardReentrantCall;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ReentrancyGuardReentrantCall>
        for UnderlyingRustTuple<'_> {
            fn from(value: ReentrancyGuardReentrantCall) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ReentrancyGuardReentrantCall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReentrancyGuardReentrantCall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReentrancyGuardReentrantCall()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 174u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
```solidity
error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        #[allow(missing_docs)]
        pub bits: u8,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SafeCastOverflowedUintDowncast>
        for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    bits: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.bits),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ZeroGroth16VerifierNotAllowed()` and selector `0x362dd41f`.
```solidity
error ZeroGroth16VerifierNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroGroth16VerifierNotAllowed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ZeroGroth16VerifierNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: ZeroGroth16VerifierNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ZeroGroth16VerifierNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroGroth16VerifierNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroGroth16VerifierNotAllowed()";
            const SELECTOR: [u8; 4] = [54u8, 45u8, 212u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                Self::abi_decode_raw_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ActionExecuted(bytes32,bytes32,bytes32,bytes32)` and selector `0x9857d0b88f01dca7bb1205d68c307d28d8d4002ff5486007ac4c443596dcea21`.
```solidity
event ActionExecuted(bytes32 indexed nullifier, bytes32 indexed commitment, bytes32 consumedLogicRef, bytes32 createdLogicRef);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActionExecuted {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub commitment: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub consumedLogicRef: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub createdLogicRef: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ActionExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ActionExecuted(bytes32,bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                152u8, 87u8, 208u8, 184u8, 143u8, 1u8, 220u8, 167u8, 187u8, 18u8, 5u8,
                214u8, 140u8, 48u8, 125u8, 40u8, 216u8, 212u8, 0u8, 47u8, 245u8, 72u8,
                96u8, 7u8, 172u8, 76u8, 68u8, 53u8, 150u8, 220u8, 234u8, 33u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    nullifier: topics.1,
                    commitment: topics.2,
                    consumedLogicRef: data.0,
                    createdLogicRef: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.consumedLogicRef),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.createdLogicRef),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.nullifier.clone(),
                    self.commitment.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.nullifier);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.commitment);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ActionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ApplicationPayload(bytes32,uint256,bytes)` and selector `0xa494dac4b7184843583f972e06783e2c3bb47f4f0137b8df52a860df07219f8c`.
```solidity
event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ApplicationPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ApplicationPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ApplicationPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 148u8, 218u8, 196u8, 183u8, 24u8, 72u8, 67u8, 88u8, 63u8, 151u8,
                46u8, 6u8, 120u8, 62u8, 44u8, 59u8, 180u8, 127u8, 79u8, 1u8, 55u8, 184u8,
                223u8, 82u8, 168u8, 96u8, 223u8, 7u8, 33u8, 159u8, 140u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ApplicationPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ApplicationPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ApplicationPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CommitmentTreeRootAdded(bytes32)` and selector `0x0a2dc548ed950accb40d5d78541f3954c5e182a8ecf19e581a4f2263f61f59d2`.
```solidity
event CommitmentTreeRootAdded(bytes32 root);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CommitmentTreeRootAdded {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for CommitmentTreeRootAdded {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CommitmentTreeRootAdded(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                10u8, 45u8, 197u8, 72u8, 237u8, 149u8, 10u8, 204u8, 180u8, 13u8, 93u8,
                120u8, 84u8, 31u8, 57u8, 84u8, 197u8, 225u8, 130u8, 168u8, 236u8, 241u8,
                158u8, 88u8, 26u8, 79u8, 34u8, 99u8, 246u8, 31u8, 89u8, 210u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { root: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CommitmentTreeRootAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CommitmentTreeRootAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CommitmentTreeRootAdded,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `DiscoveryPayload(bytes32,uint256,bytes)` and selector `0x48243873b4752ddcb45e0d7b11c4c266583e5e099a0b798fdd9c1af7d49324f3`.
```solidity
event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DiscoveryPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DiscoveryPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "DiscoveryPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                72u8, 36u8, 56u8, 115u8, 180u8, 117u8, 45u8, 220u8, 180u8, 94u8, 13u8,
                123u8, 17u8, 196u8, 194u8, 102u8, 88u8, 62u8, 94u8, 9u8, 154u8, 11u8,
                121u8, 143u8, 221u8, 156u8, 26u8, 247u8, 212u8, 147u8, 36u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DiscoveryPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DiscoveryPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DiscoveryPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExternalPayload(bytes32,uint256,bytes)` and selector `0x9c61b290f631097f56273cf4daf40df1ff9ccc33f101d464837da1f5ae18bd59`.
```solidity
event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExternalPayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExternalPayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ExternalPayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8, 97u8, 178u8, 144u8, 246u8, 49u8, 9u8, 127u8, 86u8, 39u8, 60u8,
                244u8, 218u8, 244u8, 13u8, 241u8, 255u8, 156u8, 204u8, 51u8, 241u8, 1u8,
                212u8, 100u8, 131u8, 125u8, 161u8, 245u8, 174u8, 24u8, 189u8, 89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExternalPayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExternalPayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExternalPayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ForwarderCallExecuted(address,bytes,bytes)` and selector `0xcddb327adb31fe5437df2a8c68301bb13a6baae432a804838caaf682506aadf1`.
```solidity
event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ForwarderCallExecuted {
        #[allow(missing_docs)]
        pub untrustedForwarder: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub input: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub output: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ForwarderCallExecuted {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ForwarderCallExecuted(address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                205u8, 219u8, 50u8, 122u8, 219u8, 49u8, 254u8, 84u8, 55u8, 223u8, 42u8,
                140u8, 104u8, 48u8, 27u8, 177u8, 58u8, 107u8, 170u8, 228u8, 50u8, 168u8,
                4u8, 131u8, 140u8, 170u8, 246u8, 130u8, 80u8, 106u8, 173u8, 241u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    untrustedForwarder: topics.1,
                    input: data.0,
                    output: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.input,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.output,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.untrustedForwarder.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.untrustedForwarder,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ForwarderCallExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ForwarderCallExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ForwarderCallExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ResourcePayload(bytes32,uint256,bytes)` and selector `0x3a134d01c07803003c63301717ddc4612e6c47ae408eeea3222cded532d02ae6`.
```solidity
event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ResourcePayload {
        #[allow(missing_docs)]
        pub tag: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub blob: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ResourcePayload {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ResourcePayload(bytes32,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                58u8, 19u8, 77u8, 1u8, 192u8, 120u8, 3u8, 0u8, 60u8, 99u8, 48u8, 23u8,
                23u8, 221u8, 196u8, 97u8, 46u8, 108u8, 71u8, 174u8, 64u8, 142u8, 238u8,
                163u8, 34u8, 44u8, 222u8, 213u8, 50u8, 208u8, 42u8, 230u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tag: topics.1,
                    index: data.0,
                    blob: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.blob,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.tag.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tag);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ResourcePayload {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ResourcePayload> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ResourcePayload) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransactionExecuted(bytes32)` and selector `0x862d83c6c5af275e697bfd4e27c8323c196b44bdd011dd9aaab6db0ec9943dce`.
```solidity
event TransactionExecuted(bytes32 indexed actionDigest);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransactionExecuted {
        #[allow(missing_docs)]
        pub actionDigest: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for TransactionExecuted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "TransactionExecuted(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                134u8, 45u8, 131u8, 198u8, 197u8, 175u8, 39u8, 94u8, 105u8, 123u8, 253u8,
                78u8, 39u8, 200u8, 50u8, 60u8, 25u8, 107u8, 68u8, 189u8, 208u8, 17u8,
                221u8, 154u8, 170u8, 182u8, 219u8, 14u8, 201u8, 148u8, 61u8, 206u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { actionDigest: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.actionDigest.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.actionDigest);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TransactionExecuted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransactionExecuted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransactionExecuted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address groth16Verifier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub groth16Verifier: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.groth16Verifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { groth16Verifier: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.groth16Verifier,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GROTH16_VERIFIER()` and selector `0xdb19aa79`.
```solidity
function GROTH16_VERIFIER() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GROTH16_VERIFIERCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GROTH16_VERIFIER()`](GROTH16_VERIFIERCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GROTH16_VERIFIERReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<GROTH16_VERIFIERCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: GROTH16_VERIFIERCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GROTH16_VERIFIERCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<GROTH16_VERIFIERReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: GROTH16_VERIFIERReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GROTH16_VERIFIERReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GROTH16_VERIFIERCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GROTH16_VERIFIER()";
            const SELECTOR: [u8; 4] = [219u8, 25u8, 170u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: GROTH16_VERIFIERReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: GROTH16_VERIFIERReturn = r.into();
                        r._0
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `commitmentCount()` and selector `0xc44956d1`.
```solidity
function commitmentCount() external view returns (uint256 count);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentCountCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`commitmentCount()`](commitmentCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct commitmentCountReturn {
        #[allow(missing_docs)]
        pub count: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: commitmentCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for commitmentCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<commitmentCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: commitmentCountReturn) -> Self {
                    (value.count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for commitmentCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { count: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for commitmentCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "commitmentCount()";
            const SELECTOR: [u8; 4] = [196u8, 73u8, 86u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: commitmentCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: commitmentCountReturn = r.into();
                        r.count
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `execute((((bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),bytes32)[],bytes))` and selector `0x467b5011`.
```solidity
function execute(IAnomaVerifier.Transaction memory transaction) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeCall {
        #[allow(missing_docs)]
        pub transaction: <IAnomaVerifier::Transaction as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`execute((((bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),bytes32)[],bytes))`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct executeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (IAnomaVerifier::Transaction,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAnomaVerifier::Transaction as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.transaction,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { transaction: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl executeReturn {
            fn _tokenize(
                &self,
            ) -> <executeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (IAnomaVerifier::Transaction,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute((((bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),(bytes32,bytes32,((uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[],(uint8,bytes)[])),bytes32)[],bytes))";
            const SELECTOR: [u8; 4] = [70u8, 123u8, 80u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAnomaVerifier::Transaction as alloy_sol_types::SolType>::tokenize(
                        &self.transaction,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                executeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCommitmentTreeRootHistorical(bytes32)` and selector `0xf8adf4c3`.
```solidity
function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootHistoricalCall {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCommitmentTreeRootHistorical(bytes32)`](isCommitmentTreeRootHistoricalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCommitmentTreeRootHistoricalReturn {
        #[allow(missing_docs)]
        pub isHistorical: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCommitmentTreeRootHistoricalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootHistoricalCall) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootHistoricalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { root: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isCommitmentTreeRootHistoricalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCommitmentTreeRootHistoricalReturn) -> Self {
                    (value.isHistorical,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCommitmentTreeRootHistoricalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isHistorical: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCommitmentTreeRootHistoricalCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCommitmentTreeRootHistorical(bytes32)";
            const SELECTOR: [u8; 4] = [248u8, 173u8, 244u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isCommitmentTreeRootHistoricalReturn = r.into();
                        r.isHistorical
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isCommitmentTreeRootHistoricalReturn = r.into();
                        r.isHistorical
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isNullifierContained(bytes32)` and selector `0xc1b0bed7`.
```solidity
function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNullifierContainedCall {
        #[allow(missing_docs)]
        pub nullifier: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isNullifierContained(bytes32)`](isNullifierContainedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNullifierContainedReturn {
        #[allow(missing_docs)]
        pub isContained: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isNullifierContainedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isNullifierContainedCall) -> Self {
                    (value.nullifier,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isNullifierContainedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nullifier: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isNullifierContainedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isNullifierContainedReturn) -> Self {
                    (value.isContained,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isNullifierContainedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isContained: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isNullifierContainedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isNullifierContained(bytes32)";
            const SELECTOR: [u8; 4] = [193u8, 176u8, 190u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.nullifier),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: isNullifierContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: isNullifierContainedReturn = r.into();
                        r.isContained
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestCommitmentTreeRoot()` and selector `0xbdeb442d`.
```solidity
function latestCommitmentTreeRoot() external view returns (bytes32 root);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestCommitmentTreeRootCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestCommitmentTreeRoot()`](latestCommitmentTreeRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestCommitmentTreeRootReturn {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestCommitmentTreeRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestCommitmentTreeRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestCommitmentTreeRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<latestCommitmentTreeRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: latestCommitmentTreeRootReturn) -> Self {
                    (value.root,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for latestCommitmentTreeRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { root: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestCommitmentTreeRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestCommitmentTreeRoot()";
            const SELECTOR: [u8; 4] = [189u8, 235u8, 68u8, 45u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: latestCommitmentTreeRootReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_with_config(
                data: &[u8],
                config: alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence_with_config(
                        data,
                        config,
                    )
                    .map(|r| {
                        let r: latestCommitmentTreeRootReturn = r.into();
                        r.root
                    })
            }
            #[inline]
            fn abi_decode_returns_validate(
                data: &[u8],
            ) -> alloy_sol_types::Result<Self::Return> {
                Self::abi_decode_returns_with_config(
                    data,
                    alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
                )
            }
        }
    };
    ///Container for all the [`AnomaVerifier`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum AnomaVerifierCalls {
        #[allow(missing_docs)]
        GROTH16_VERIFIER(GROTH16_VERIFIERCall),
        #[allow(missing_docs)]
        commitmentCount(commitmentCountCall),
        #[allow(missing_docs)]
        execute(executeCall),
        #[allow(missing_docs)]
        isCommitmentTreeRootHistorical(isCommitmentTreeRootHistoricalCall),
        #[allow(missing_docs)]
        isNullifierContained(isNullifierContainedCall),
        #[allow(missing_docs)]
        latestCommitmentTreeRoot(latestCommitmentTreeRootCall),
    }
    impl AnomaVerifierCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [70u8, 123u8, 80u8, 17u8],
            [189u8, 235u8, 68u8, 45u8],
            [193u8, 176u8, 190u8, 215u8],
            [196u8, 73u8, 86u8, 209u8],
            [219u8, 25u8, 170u8, 121u8],
            [248u8, 173u8, 244u8, 195u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(execute),
            ::core::stringify!(latestCommitmentTreeRoot),
            ::core::stringify!(isNullifierContained),
            ::core::stringify!(commitmentCount),
            ::core::stringify!(GROTH16_VERIFIER),
            ::core::stringify!(isCommitmentTreeRootHistorical),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <executeCall as alloy_sol_types::SolCall>::SIGNATURE,
            <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isNullifierContainedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <commitmentCountCall as alloy_sol_types::SolCall>::SIGNATURE,
            <GROTH16_VERIFIERCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AnomaVerifierCalls {
        const NAME: &'static str = "AnomaVerifierCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 6usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GROTH16_VERIFIER(_) => {
                    <GROTH16_VERIFIERCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::commitmentCount(_) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isCommitmentTreeRootHistorical(_) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNullifierContained(_) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestCommitmentTreeRoot(_) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<AnomaVerifierCalls>] = &[
                {
                    fn execute(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::execute)
                    }
                    execute
                },
                {
                    fn latestCommitmentTreeRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::latestCommitmentTreeRoot)
                    }
                    latestCommitmentTreeRoot
                },
                {
                    fn isNullifierContained(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::isNullifierContained)
                    }
                    isNullifierContained
                },
                {
                    fn commitmentCount(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <commitmentCountCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::commitmentCount)
                    }
                    commitmentCount
                },
                {
                    fn GROTH16_VERIFIER(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <GROTH16_VERIFIERCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::GROTH16_VERIFIER)
                    }
                    GROTH16_VERIFIER
                },
                {
                    fn isCommitmentTreeRootHistorical(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierCalls> {
                        <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierCalls::isCommitmentTreeRootHistorical)
                    }
                    isCommitmentTreeRootHistorical
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::GROTH16_VERIFIER(inner) => {
                    <GROTH16_VERIFIERCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::commitmentCount(inner) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isCommitmentTreeRootHistorical(inner) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isNullifierContained(inner) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestCommitmentTreeRoot(inner) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::GROTH16_VERIFIER(inner) => {
                    <GROTH16_VERIFIERCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::commitmentCount(inner) => {
                    <commitmentCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isCommitmentTreeRootHistorical(inner) => {
                    <isCommitmentTreeRootHistoricalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isNullifierContained(inner) => {
                    <isNullifierContainedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestCommitmentTreeRoot(inner) => {
                    <latestCommitmentTreeRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AnomaVerifier`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum AnomaVerifierErrors {
        #[allow(missing_docs)]
        EmptyTransactionNotAllowed(EmptyTransactionNotAllowed),
        #[allow(missing_docs)]
        ForwarderCallOutputMismatch(ForwarderCallOutputMismatch),
        #[allow(missing_docs)]
        LeafCountTooLarge(LeafCountTooLarge),
        #[allow(missing_docs)]
        LeafNotInField(LeafNotInField),
        #[allow(missing_docs)]
        NonExistingRoot(NonExistingRoot),
        #[allow(missing_docs)]
        PreExistingNullifier(PreExistingNullifier),
        #[allow(missing_docs)]
        ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall),
        #[allow(missing_docs)]
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        #[allow(missing_docs)]
        ZeroGroth16VerifierNotAllowed(ZeroGroth16VerifierNotAllowed),
    }
    impl AnomaVerifierErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 77u8, 115u8, 217u8],
            [54u8, 45u8, 212u8, 31u8],
            [57u8, 169u8, 64u8, 197u8],
            [62u8, 229u8, 174u8, 181u8],
            [109u8, 252u8, 198u8, 80u8],
            [197u8, 4u8, 250u8, 218u8],
            [197u8, 62u8, 114u8, 107u8],
            [207u8, 27u8, 9u8, 60u8],
            [249u8, 132u8, 158u8, 163u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(LeafCountTooLarge),
            ::core::stringify!(ZeroGroth16VerifierNotAllowed),
            ::core::stringify!(PreExistingNullifier),
            ::core::stringify!(ReentrancyGuardReentrantCall),
            ::core::stringify!(SafeCastOverflowedUintDowncast),
            ::core::stringify!(ForwarderCallOutputMismatch),
            ::core::stringify!(LeafNotInField),
            ::core::stringify!(EmptyTransactionNotAllowed),
            ::core::stringify!(NonExistingRoot),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <LeafCountTooLarge as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroGroth16VerifierNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <PreExistingNullifier as alloy_sol_types::SolError>::SIGNATURE,
            <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SIGNATURE,
            <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SIGNATURE,
            <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <LeafNotInField as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <NonExistingRoot as alloy_sol_types::SolError>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 4usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AnomaVerifierErrors {
        const NAME: &'static str = "AnomaVerifierErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 9usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EmptyTransactionNotAllowed(_) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ForwarderCallOutputMismatch(_) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeafCountTooLarge(_) => {
                    <LeafCountTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeafNotInField(_) => {
                    <LeafNotInField as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonExistingRoot(_) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PreExistingNullifier(_) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReentrancyGuardReentrantCall(_) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroGroth16VerifierNotAllowed(_) => {
                    <ZeroGroth16VerifierNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_with_config(
            selector: [u8; 4],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                alloy_sol_types::abi::AbiDecoderConfig,
            ) -> alloy_sol_types::Result<AnomaVerifierErrors>] = &[
                {
                    fn LeafCountTooLarge(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <LeafCountTooLarge as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::LeafCountTooLarge)
                    }
                    LeafCountTooLarge
                },
                {
                    fn ZeroGroth16VerifierNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <ZeroGroth16VerifierNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::ZeroGroth16VerifierNotAllowed)
                    }
                    ZeroGroth16VerifierNotAllowed
                },
                {
                    fn PreExistingNullifier(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <PreExistingNullifier as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::PreExistingNullifier)
                    }
                    PreExistingNullifier
                },
                {
                    fn ReentrancyGuardReentrantCall(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::ReentrancyGuardReentrantCall)
                    }
                    ReentrancyGuardReentrantCall
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ForwarderCallOutputMismatch(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::ForwarderCallOutputMismatch)
                    }
                    ForwarderCallOutputMismatch
                },
                {
                    fn LeafNotInField(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <LeafNotInField as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::LeafNotInField)
                    }
                    LeafNotInField
                },
                {
                    fn EmptyTransactionNotAllowed(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::EmptyTransactionNotAllowed)
                    }
                    EmptyTransactionNotAllowed
                },
                {
                    fn NonExistingRoot(
                        data: &[u8],
                        config: alloy_sol_types::abi::AbiDecoderConfig,
                    ) -> alloy_sol_types::Result<AnomaVerifierErrors> {
                        <NonExistingRoot as alloy_sol_types::SolError>::abi_decode_raw_with_config(
                                data,
                                config,
                            )
                            .map(AnomaVerifierErrors::NonExistingRoot)
                    }
                    NonExistingRoot
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, config)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            Self::abi_decode_raw_with_config(
                selector,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::new().validate(true),
            )
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::EmptyTransactionNotAllowed(inner) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ForwarderCallOutputMismatch(inner) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeafCountTooLarge(inner) => {
                    <LeafCountTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeafNotInField(inner) => {
                    <LeafNotInField as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonExistingRoot(inner) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PreExistingNullifier(inner) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ZeroGroth16VerifierNotAllowed(inner) => {
                    <ZeroGroth16VerifierNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EmptyTransactionNotAllowed(inner) => {
                    <EmptyTransactionNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ForwarderCallOutputMismatch(inner) => {
                    <ForwarderCallOutputMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeafCountTooLarge(inner) => {
                    <LeafCountTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeafNotInField(inner) => {
                    <LeafNotInField as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonExistingRoot(inner) => {
                    <NonExistingRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PreExistingNullifier(inner) => {
                    <PreExistingNullifier as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReentrancyGuardReentrantCall(inner) => {
                    <ReentrancyGuardReentrantCall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ZeroGroth16VerifierNotAllowed(inner) => {
                    <ZeroGroth16VerifierNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl AnomaVerifierErrors {
        /**Creates a [`EmptyTransactionNotAllowed`] error.

```solidity
error EmptyTransactionNotAllowed()
```*/
        #[inline]
        pub fn empty_transaction_not_allowed() -> Self {
            Self::EmptyTransactionNotAllowed(EmptyTransactionNotAllowed)
        }
        /**Creates a [`ForwarderCallOutputMismatch`] error.

```solidity
error ForwarderCallOutputMismatch(bytes,bytes)
```*/
        #[inline]
        pub fn forwarder_call_output_mismatch(
            expected: alloy::sol_types::private::Bytes,
            actual: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ForwarderCallOutputMismatch(ForwarderCallOutputMismatch {
                expected: expected,
                actual: actual,
            })
        }
        /**Creates a [`LeafCountTooLarge`] error.

```solidity
error LeafCountTooLarge(uint256)
```*/
        #[inline]
        pub fn leaf_count_too_large(
            leaf_count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::LeafCountTooLarge(LeafCountTooLarge {
                leafCount: leaf_count,
            })
        }
        /**Creates a [`LeafNotInField`] error.

```solidity
error LeafNotInField(bytes32)
```*/
        #[inline]
        pub fn leaf_not_in_field(
            leaf: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::LeafNotInField(LeafNotInField { leaf: leaf })
        }
        /**Creates a [`NonExistingRoot`] error.

```solidity
error NonExistingRoot(bytes32)
```*/
        #[inline]
        pub fn non_existing_root(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::NonExistingRoot(NonExistingRoot { root: root })
        }
        /**Creates a [`PreExistingNullifier`] error.

```solidity
error PreExistingNullifier(bytes32)
```*/
        #[inline]
        pub fn pre_existing_nullifier(
            nullifier: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::PreExistingNullifier(PreExistingNullifier {
                nullifier: nullifier,
            })
        }
        /**Creates a [`ReentrancyGuardReentrantCall`] error.

```solidity
error ReentrancyGuardReentrantCall()
```*/
        #[inline]
        pub fn reentrancy_guard_reentrant_call() -> Self {
            Self::ReentrancyGuardReentrantCall(ReentrancyGuardReentrantCall)
        }
        /**Creates a [`SafeCastOverflowedUintDowncast`] error.

```solidity
error SafeCastOverflowedUintDowncast(uint8,uint256)
```*/
        #[inline]
        pub fn safe_cast_overflowed_uint_downcast(
            bits: u8,
            value: alloy::sol_types::private::primitives::aliases::U256,
        ) -> Self {
            Self::SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast {
                bits: bits,
                value: value,
            })
        }
        /**Creates a [`ZeroGroth16VerifierNotAllowed`] error.

```solidity
error ZeroGroth16VerifierNotAllowed()
```*/
        #[inline]
        pub fn zero_groth_16_verifier_not_allowed() -> Self {
            Self::ZeroGroth16VerifierNotAllowed(ZeroGroth16VerifierNotAllowed)
        }
    }
    ///Container for all the [`AnomaVerifier`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum AnomaVerifierEvents {
        #[allow(missing_docs)]
        ActionExecuted(ActionExecuted),
        #[allow(missing_docs)]
        ApplicationPayload(ApplicationPayload),
        #[allow(missing_docs)]
        CommitmentTreeRootAdded(CommitmentTreeRootAdded),
        #[allow(missing_docs)]
        DiscoveryPayload(DiscoveryPayload),
        #[allow(missing_docs)]
        ExternalPayload(ExternalPayload),
        #[allow(missing_docs)]
        ForwarderCallExecuted(ForwarderCallExecuted),
        #[allow(missing_docs)]
        ResourcePayload(ResourcePayload),
        #[allow(missing_docs)]
        TransactionExecuted(TransactionExecuted),
    }
    impl AnomaVerifierEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 45u8, 197u8, 72u8, 237u8, 149u8, 10u8, 204u8, 180u8, 13u8, 93u8,
                120u8, 84u8, 31u8, 57u8, 84u8, 197u8, 225u8, 130u8, 168u8, 236u8, 241u8,
                158u8, 88u8, 26u8, 79u8, 34u8, 99u8, 246u8, 31u8, 89u8, 210u8,
            ],
            [
                58u8, 19u8, 77u8, 1u8, 192u8, 120u8, 3u8, 0u8, 60u8, 99u8, 48u8, 23u8,
                23u8, 221u8, 196u8, 97u8, 46u8, 108u8, 71u8, 174u8, 64u8, 142u8, 238u8,
                163u8, 34u8, 44u8, 222u8, 213u8, 50u8, 208u8, 42u8, 230u8,
            ],
            [
                72u8, 36u8, 56u8, 115u8, 180u8, 117u8, 45u8, 220u8, 180u8, 94u8, 13u8,
                123u8, 17u8, 196u8, 194u8, 102u8, 88u8, 62u8, 94u8, 9u8, 154u8, 11u8,
                121u8, 143u8, 221u8, 156u8, 26u8, 247u8, 212u8, 147u8, 36u8, 243u8,
            ],
            [
                134u8, 45u8, 131u8, 198u8, 197u8, 175u8, 39u8, 94u8, 105u8, 123u8, 253u8,
                78u8, 39u8, 200u8, 50u8, 60u8, 25u8, 107u8, 68u8, 189u8, 208u8, 17u8,
                221u8, 154u8, 170u8, 182u8, 219u8, 14u8, 201u8, 148u8, 61u8, 206u8,
            ],
            [
                152u8, 87u8, 208u8, 184u8, 143u8, 1u8, 220u8, 167u8, 187u8, 18u8, 5u8,
                214u8, 140u8, 48u8, 125u8, 40u8, 216u8, 212u8, 0u8, 47u8, 245u8, 72u8,
                96u8, 7u8, 172u8, 76u8, 68u8, 53u8, 150u8, 220u8, 234u8, 33u8,
            ],
            [
                156u8, 97u8, 178u8, 144u8, 246u8, 49u8, 9u8, 127u8, 86u8, 39u8, 60u8,
                244u8, 218u8, 244u8, 13u8, 241u8, 255u8, 156u8, 204u8, 51u8, 241u8, 1u8,
                212u8, 100u8, 131u8, 125u8, 161u8, 245u8, 174u8, 24u8, 189u8, 89u8,
            ],
            [
                164u8, 148u8, 218u8, 196u8, 183u8, 24u8, 72u8, 67u8, 88u8, 63u8, 151u8,
                46u8, 6u8, 120u8, 62u8, 44u8, 59u8, 180u8, 127u8, 79u8, 1u8, 55u8, 184u8,
                223u8, 82u8, 168u8, 96u8, 223u8, 7u8, 33u8, 159u8, 140u8,
            ],
            [
                205u8, 219u8, 50u8, 122u8, 219u8, 49u8, 254u8, 84u8, 55u8, 223u8, 42u8,
                140u8, 104u8, 48u8, 27u8, 177u8, 58u8, 107u8, 170u8, 228u8, 50u8, 168u8,
                4u8, 131u8, 140u8, 170u8, 246u8, 130u8, 80u8, 106u8, 173u8, 241u8,
            ],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(CommitmentTreeRootAdded),
            ::core::stringify!(ResourcePayload),
            ::core::stringify!(DiscoveryPayload),
            ::core::stringify!(TransactionExecuted),
            ::core::stringify!(ActionExecuted),
            ::core::stringify!(ExternalPayload),
            ::core::stringify!(ApplicationPayload),
            ::core::stringify!(ForwarderCallExecuted),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE,
            <ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
            <ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE,
            <ForwarderCallExecuted as alloy_sol_types::SolEvent>::SIGNATURE,
        ];
        /// Returns the signature for the given selector, if known.
        #[inline]
        pub fn signature_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            match Self::SELECTORS.binary_search(&selector) {
                ::core::result::Result::Ok(idx) => {
                    ::core::option::Option::Some(Self::SIGNATURES[idx])
                }
                ::core::result::Result::Err(_) => ::core::option::Option::None,
            }
        }
        /// Returns the enum variant name for the given selector, if known.
        #[inline]
        pub fn name_by_selector(
            selector: [u8; 32usize],
        ) -> ::core::option::Option<&'static str> {
            let sig = Self::signature_by_selector(selector)?;
            sig.split_once('(').map(|(name, _)| name)
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AnomaVerifierEvents {
        const NAME: &'static str = "AnomaVerifierEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            <Self as alloy_sol_types::SolEventInterface>::decode_raw_log_with_config(
                topics,
                data,
                alloy_sol_types::abi::AbiDecoderConfig::default(),
            )
        }
        fn decode_raw_log_with_config(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            config: alloy_sol_types::abi::AbiDecoderConfig,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ActionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActionExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ActionExecuted)
                }
                Some(
                    <ApplicationPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ApplicationPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ApplicationPayload)
                }
                Some(
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CommitmentTreeRootAdded as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::CommitmentTreeRootAdded)
                }
                Some(<DiscoveryPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DiscoveryPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::DiscoveryPayload)
                }
                Some(<ExternalPayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExternalPayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ExternalPayload)
                }
                Some(
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ForwarderCallExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ForwarderCallExecuted)
                }
                Some(<ResourcePayload as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ResourcePayload as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::ResourcePayload)
                }
                Some(
                    <TransactionExecuted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransactionExecuted as alloy_sol_types::SolEvent>::decode_raw_log_with_config(
                            topics,
                            data,
                            config,
                        )
                        .map(Self::TransactionExecuted)
                }
                _ => {
                    if topics
                        .len()
                        .checked_mul(alloy_sol_types::Word::len_bytes())
                        .and_then(|len| len.checked_add(data.len()))
                        .is_none_or(|len| len > config.get_memory_limit())
                    {
                        return alloy_sol_types::private::Err(
                            alloy_sol_types::Error::MemoryLimitExceeded(
                                config.get_memory_limit(),
                            ),
                        );
                    }
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for AnomaVerifierEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ApplicationPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CommitmentTreeRootAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DiscoveryPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExternalPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ForwarderCallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ResourcePayload(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ActionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ApplicationPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CommitmentTreeRootAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DiscoveryPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExternalPayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ForwarderCallExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ResourcePayload(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransactionExecuted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    #[automatically_derived]
    impl AnomaVerifierEvents {
        /**Creates a [`ActionExecuted`] event.

```solidity
event ActionExecuted(bytes32,bytes32,bytes32,bytes32)
```*/
        #[inline]
        pub fn action_executed(
            nullifier: alloy::sol_types::private::FixedBytes<32>,
            commitment: alloy::sol_types::private::FixedBytes<32>,
            consumed_logic_ref: alloy::sol_types::private::FixedBytes<32>,
            created_logic_ref: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::ActionExecuted(ActionExecuted {
                nullifier: nullifier,
                commitment: commitment,
                consumedLogicRef: consumed_logic_ref,
                createdLogicRef: created_logic_ref,
            })
        }
        /**Creates a [`ApplicationPayload`] event.

```solidity
event ApplicationPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn application_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ApplicationPayload(ApplicationPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`CommitmentTreeRootAdded`] event.

```solidity
event CommitmentTreeRootAdded(bytes32)
```*/
        #[inline]
        pub fn commitment_tree_root_added(
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::CommitmentTreeRootAdded(CommitmentTreeRootAdded {
                root: root,
            })
        }
        /**Creates a [`DiscoveryPayload`] event.

```solidity
event DiscoveryPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn discovery_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::DiscoveryPayload(DiscoveryPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`ExternalPayload`] event.

```solidity
event ExternalPayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn external_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ExternalPayload(ExternalPayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`ForwarderCallExecuted`] event.

```solidity
event ForwarderCallExecuted(address,bytes,bytes)
```*/
        #[inline]
        pub fn forwarder_call_executed(
            untrusted_forwarder: alloy::sol_types::private::Address,
            input: alloy::sol_types::private::Bytes,
            output: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ForwarderCallExecuted(ForwarderCallExecuted {
                untrustedForwarder: untrusted_forwarder,
                input: input,
                output: output,
            })
        }
        /**Creates a [`ResourcePayload`] event.

```solidity
event ResourcePayload(bytes32,uint256,bytes)
```*/
        #[inline]
        pub fn resource_payload(
            tag: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
            blob: alloy::sol_types::private::Bytes,
        ) -> Self {
            Self::ResourcePayload(ResourcePayload {
                tag: tag,
                index: index,
                blob: blob,
            })
        }
        /**Creates a [`TransactionExecuted`] event.

```solidity
event TransactionExecuted(bytes32)
```*/
        #[inline]
        pub fn transaction_executed(
            action_digest: alloy::sol_types::private::FixedBytes<32>,
        ) -> Self {
            Self::TransactionExecuted(TransactionExecuted {
                actionDigest: action_digest,
            })
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AnomaVerifier`](self) contract instance.

See the [wrapper's documentation](`AnomaVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> AnomaVerifierInstance<P, N> {
        AnomaVerifierInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        groth16Verifier: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AnomaVerifierInstance<P, N>>,
    > {
        AnomaVerifierInstance::<P, N>::deploy(__provider, groth16Verifier)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
        groth16Verifier: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        AnomaVerifierInstance::<P, N>::deploy_builder(__provider, groth16Verifier)
    }
    /**A [`AnomaVerifier`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AnomaVerifier`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AnomaVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for AnomaVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AnomaVerifierInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AnomaVerifierInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`AnomaVerifier`](self) contract instance.

See the [wrapper's documentation](`AnomaVerifierInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            __provider: P,
        ) -> Self {
            Self {
                address,
                provider: __provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
            groth16Verifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AnomaVerifierInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider, groth16Verifier);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            __provider: P,
            groth16Verifier: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { groth16Verifier },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> AnomaVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AnomaVerifierInstance<P, N> {
            AnomaVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AnomaVerifierInstance<P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`GROTH16_VERIFIER`] function.
        pub fn GROTH16_VERIFIER(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GROTH16_VERIFIERCall, N> {
            self.call_builder(&GROTH16_VERIFIERCall)
        }
        ///Creates a new call builder for the [`commitmentCount`] function.
        pub fn commitmentCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, commitmentCountCall, N> {
            self.call_builder(&commitmentCountCall)
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            transaction: <IAnomaVerifier::Transaction as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, executeCall, N> {
            self.call_builder(&executeCall { transaction })
        }
        ///Creates a new call builder for the [`isCommitmentTreeRootHistorical`] function.
        pub fn isCommitmentTreeRootHistorical(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isCommitmentTreeRootHistoricalCall, N> {
            self.call_builder(
                &isCommitmentTreeRootHistoricalCall {
                    root,
                },
            )
        }
        ///Creates a new call builder for the [`isNullifierContained`] function.
        pub fn isNullifierContained(
            &self,
            nullifier: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isNullifierContainedCall, N> {
            self.call_builder(
                &isNullifierContainedCall {
                    nullifier,
                },
            )
        }
        ///Creates a new call builder for the [`latestCommitmentTreeRoot`] function.
        pub fn latestCommitmentTreeRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, latestCommitmentTreeRootCall, N> {
            self.call_builder(&latestCommitmentTreeRootCall)
        }
    }
    /// Event filters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > AnomaVerifierInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ActionExecuted`] event.
        pub fn ActionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ActionExecuted, N> {
            self.event_filter::<ActionExecuted>()
        }
        ///Creates a new event filter for the [`ApplicationPayload`] event.
        pub fn ApplicationPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ApplicationPayload, N> {
            self.event_filter::<ApplicationPayload>()
        }
        ///Creates a new event filter for the [`CommitmentTreeRootAdded`] event.
        pub fn CommitmentTreeRootAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, CommitmentTreeRootAdded, N> {
            self.event_filter::<CommitmentTreeRootAdded>()
        }
        ///Creates a new event filter for the [`DiscoveryPayload`] event.
        pub fn DiscoveryPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, DiscoveryPayload, N> {
            self.event_filter::<DiscoveryPayload>()
        }
        ///Creates a new event filter for the [`ExternalPayload`] event.
        pub fn ExternalPayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ExternalPayload, N> {
            self.event_filter::<ExternalPayload>()
        }
        ///Creates a new event filter for the [`ForwarderCallExecuted`] event.
        pub fn ForwarderCallExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, ForwarderCallExecuted, N> {
            self.event_filter::<ForwarderCallExecuted>()
        }
        ///Creates a new event filter for the [`ResourcePayload`] event.
        pub fn ResourcePayload_filter(
            &self,
        ) -> alloy_contract::Event<&P, ResourcePayload, N> {
            self.event_filter::<ResourcePayload>()
        }
        ///Creates a new event filter for the [`TransactionExecuted`] event.
        pub fn TransactionExecuted_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransactionExecuted, N> {
            self.event_filter::<TransactionExecuted>()
        }
    }
}

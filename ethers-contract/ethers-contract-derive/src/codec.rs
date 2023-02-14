//! Helper functions for deriving `EthAbiType`

use ethers_core::macros::ethers_core_crate;
use quote::quote;
use syn::DeriveInput;

/// Generates the `AbiEncode` + `AbiDecode` implementation
pub fn derive_codec_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ethers_core = ethers_core_crate();

    quote! {
        impl #ethers_core::abi::AbiDecode for #name {
            fn decode(bytes: impl AsRef<[u8]>) -> ::std::result::Result<Self, #ethers_core::abi::AbiError> {
                if let param @ #ethers_core::abi::ParamType::Tuple(_) = <Self as #ethers_core::abi::AbiType>::param_type() {
                    let token = #ethers_core::abi::decode(&[param], bytes.as_ref())?
                        .pop().expect("the vec of tokens should have one element - a tuple");
                    Ok(<Self as #ethers_core::abi::Tokenizable>::from_token(token)?)
                } else {
                    Err(
                        #ethers_core::abi::InvalidOutputType("Expected tuple".to_string()).into()
                    )
                }
            }
        }

        impl #ethers_core::abi::AbiEncode for #name {
            fn encode(self) -> ::std::vec::Vec<u8> {
                let tokens = #ethers_core::abi::Tokenize::into_tokens(self);
                #ethers_core::abi::encode(&tokens)
            }
        }
    }
}

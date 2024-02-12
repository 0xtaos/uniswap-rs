pub use i_universal_router_commands::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_universal_router_commands {
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("cryptopunks"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("cryptopunks"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("punkId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("foundation"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("foundation"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("looksRare1155"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("looksRare1155"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("looksRare721"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("looksRare721"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nft20"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nft20"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nftx"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nftx"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerCheck1155"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerCheck1155"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("minBalance"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownerCheck721"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerCheck721"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payPortion"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payPortion"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bips"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2Permit"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit2Permit"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitSingle"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(160usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(48usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(48usize),
                                                ],
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.PermitSingle",
                                        ),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2PermitBatch"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("permit2PermitBatch"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("permitBatch"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers_core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(160usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(48usize),
                                                            ::ethers_core::abi::ethabi::ParamType::Uint(48usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.PermitBatch",
                                        ),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2TransferFrom"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "permit2TransferFrom",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(160usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("permit2TransferFromBatch"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "permit2TransferFromBatch",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("batchDetails"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                                    ::ethers_core::abi::ethabi::ParamType::Uint(160usize),
                                                    ::ethers_core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IAllowanceTransfer.AllowanceTransferDetails[]",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("seaport"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("seaport"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sudoswap"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sudoswap"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweep"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweep"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMin"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepErc1155"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepErc1155"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sweepErc721"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sweepErc721"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transfer"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unwrapWeth"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unwrapWeth"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMin"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v2SwapExactIn"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v2SwapExactIn"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payerIsUser"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v2SwapExactOut"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v2SwapExactOut"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payerIsUser"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v3SwapExactIn"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v3SwapExactIn"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOutMin"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payerIsUser"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("v3SwapExactOut"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("v3SwapExactOut"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountInMax"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payerIsUser"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("wrapEth"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("wrapEth"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountMin"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("x2y21155"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("x2y21155"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("x2y2721"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("x2y2721"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recipient"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IUNIVERSALROUTERCOMMANDS_ABI: ::ethers_contract::Lazy<
        ::ethers_core::abi::Abi,
    > = ::ethers_contract::Lazy::new(__abi);
    pub struct IUniversalRouterCommands<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniversalRouterCommands<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniversalRouterCommands<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniversalRouterCommands<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniversalRouterCommands<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IUniversalRouterCommands))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IUniversalRouterCommands<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IUNIVERSALROUTERCOMMANDS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `cryptopunks` (0xeda3083f) function
        pub fn cryptopunks(
            &self,
            punk_id: ::ethers_core::types::U256,
            recipient: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 163, 8, 63], (punk_id, recipient, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `foundation` (0xa5c51c5c) function
        pub fn foundation(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            recipient: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 197, 28, 92], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `looksRare1155` (0x564c2f7e) function
        pub fn looks_rare_1155(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            recipient: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [86, 76, 47, 126],
                    (value, data, recipient, token, id, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `looksRare721` (0x9289a6bd) function
        pub fn looks_rare_721(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            recipient: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 137, 166, 189], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nft20` (0x7bf8e837) function
        pub fn nft_20(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 248, 232, 55], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nftx` (0x9d49b90a) function
        pub fn nftx(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 73, 185, 10], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerCheck1155` (0x3e148b68) function
        pub fn owner_check_1155(
            &self,
            owner: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
            min_balance: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 20, 139, 104], (owner, token, id, min_balance))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerCheck721` (0xe0d26a5a) function
        pub fn owner_check_721(
            &self,
            owner: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 210, 106, 90], (owner, token, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payPortion` (0xd5092a81) function
        pub fn pay_portion(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            bips: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 9, 42, 129], (token, recipient, bips))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2Permit` (0x9d8e5ab3) function
        pub fn permit_2_permit(
            &self,
            permit_single: PermitSingle,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 142, 90, 179], (permit_single, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2PermitBatch` (0x5b9f25d3) function
        pub fn permit_2_permit_batch(
            &self,
            permit_batch: PermitBatch,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 159, 37, 211], (permit_batch, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2TransferFrom` (0xf7223d79) function
        pub fn permit_2_transfer_from(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 34, 61, 121], (token, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit2TransferFromBatch` (0x812a07a3) function
        pub fn permit_2_transfer_from_batch(
            &self,
            batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 42, 7, 163], batch_details)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `seaport` (0x41cd3002) function
        pub fn seaport(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 205, 48, 2], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sudoswap` (0x9211495f) function
        pub fn sudoswap(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 17, 73, 95], (value, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweep` (0x62c06767) function
        pub fn sweep(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            amount_min: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 192, 103, 103], (token, recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepErc1155` (0xb930d445) function
        pub fn sweep_erc_1155(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([185, 48, 212, 69], (token, recipient, id, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sweepErc721` (0x40e2cf43) function
        pub fn sweep_erc_721(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 226, 207, 67], (token, recipient, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xbeabacc8) function
        pub fn transfer(
            &self,
            token: ::ethers_core::types::Address,
            recipient: ::ethers_core::types::Address,
            value: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 171, 172, 200], (token, recipient, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWeth` (0x521b6390) function
        pub fn unwrap_weth(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_min: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([82, 27, 99, 144], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v2SwapExactIn` (0xdcd5db1f) function
        pub fn v_2_swap_exact_in(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_in: ::ethers_core::types::U256,
            amount_out_min: ::ethers_core::types::U256,
            path: ::std::vec::Vec<::ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [220, 213, 219, 31],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v2SwapExactOut` (0xe868b367) function
        pub fn v_2_swap_exact_out(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_out: ::ethers_core::types::U256,
            amount_in_max: ::ethers_core::types::U256,
            path: ::std::vec::Vec<::ethers_core::types::Address>,
            payer_is_user: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [232, 104, 179, 103],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v3SwapExactIn` (0x5a58ec2d) function
        pub fn v_3_swap_exact_in(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_in: ::ethers_core::types::U256,
            amount_out_min: ::ethers_core::types::U256,
            path: ::ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [90, 88, 236, 45],
                    (recipient, amount_in, amount_out_min, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `v3SwapExactOut` (0x14cd4424) function
        pub fn v_3_swap_exact_out(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_out: ::ethers_core::types::U256,
            amount_in_max: ::ethers_core::types::U256,
            path: ::ethers_core::types::Bytes,
            payer_is_user: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [20, 205, 68, 36],
                    (recipient, amount_out, amount_in_max, path, payer_is_user),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapEth` (0xb3bc1a13) function
        pub fn wrap_eth(
            &self,
            recipient: ::ethers_core::types::Address,
            amount_min: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 188, 26, 19], (recipient, amount_min))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `x2y21155` (0x108ccf02) function
        pub fn x_2y_21155(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            recipient: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
            amount: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [16, 140, 207, 2],
                    (value, data, recipient, token, id, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `x2y2721` (0xc059e681) function
        pub fn x_2y_2721(
            &self,
            value: ::ethers_core::types::U256,
            data: ::ethers_core::types::Bytes,
            recipient: ::ethers_core::types::Address,
            token: ::ethers_core::types::Address,
            id: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 89, 230, 129], (value, data, recipient, token, id))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IUniversalRouterCommands<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `cryptopunks` function with signature `cryptopunks(uint256,address,uint256)` and selector `0xeda3083f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "cryptopunks", abi = "cryptopunks(uint256,address,uint256)")]
    pub struct CryptopunksCall {
        pub punk_id: ::ethers_core::types::U256,
        pub recipient: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `foundation` function with signature `foundation(uint256,bytes,address,address,uint256)` and selector `0xa5c51c5c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "foundation",
        abi = "foundation(uint256,bytes,address,address,uint256)"
    )]
    pub struct FoundationCall {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub recipient: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `looksRare1155` function with signature `looksRare1155(uint256,bytes,address,address,uint256,uint256)` and selector `0x564c2f7e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "looksRare1155",
        abi = "looksRare1155(uint256,bytes,address,address,uint256,uint256)"
    )]
    pub struct LooksRare1155Call {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub recipient: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `looksRare721` function with signature `looksRare721(uint256,bytes,address,address,uint256)` and selector `0x9289a6bd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "looksRare721",
        abi = "looksRare721(uint256,bytes,address,address,uint256)"
    )]
    pub struct LooksRare721Call {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub recipient: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `nft20` function with signature `nft20(uint256,bytes)` and selector `0x7bf8e837`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nft20", abi = "nft20(uint256,bytes)")]
    pub struct Nft20Call {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `nftx` function with signature `nftx(uint256,bytes)` and selector `0x9d49b90a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nftx", abi = "nftx(uint256,bytes)")]
    pub struct NftxCall {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `ownerCheck1155` function with signature `ownerCheck1155(address,address,uint256,uint256)` and selector `0x3e148b68`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "ownerCheck1155",
        abi = "ownerCheck1155(address,address,uint256,uint256)"
    )]
    pub struct OwnerCheck1155Call {
        pub owner: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
        pub min_balance: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `ownerCheck721` function with signature `ownerCheck721(address,address,uint256)` and selector `0xe0d26a5a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerCheck721", abi = "ownerCheck721(address,address,uint256)")]
    pub struct OwnerCheck721Call {
        pub owner: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `payPortion` function with signature `payPortion(address,address,uint256)` and selector `0xd5092a81`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "payPortion", abi = "payPortion(address,address,uint256)")]
    pub struct PayPortionCall {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub bips: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `permit2Permit` function with signature `permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)` and selector `0x9d8e5ab3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit2Permit",
        abi = "permit2Permit(((address,uint160,uint48,uint48),address,uint256),bytes)"
    )]
    pub struct Permit2PermitCall {
        pub permit_single: PermitSingle,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `permit2PermitBatch` function with signature `permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)` and selector `0x5b9f25d3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit2PermitBatch",
        abi = "permit2PermitBatch(((address,uint160,uint48,uint48)[],address,uint256),bytes)"
    )]
    pub struct Permit2PermitBatchCall {
        pub permit_batch: PermitBatch,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `permit2TransferFrom` function with signature `permit2TransferFrom(address,address,uint160)` and selector `0xf7223d79`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit2TransferFrom",
        abi = "permit2TransferFrom(address,address,uint160)"
    )]
    pub struct Permit2TransferFromCall {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `permit2TransferFromBatch` function with signature `permit2TransferFromBatch((address,address,uint160,address)[])` and selector `0x812a07a3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "permit2TransferFromBatch",
        abi = "permit2TransferFromBatch((address,address,uint160,address)[])"
    )]
    pub struct Permit2TransferFromBatchCall {
        pub batch_details: ::std::vec::Vec<AllowanceTransferDetails>,
    }
    ///Container type for all input parameters for the `seaport` function with signature `seaport(uint256,bytes)` and selector `0x41cd3002`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "seaport", abi = "seaport(uint256,bytes)")]
    pub struct SeaportCall {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sudoswap` function with signature `sudoswap(uint256,bytes)` and selector `0x9211495f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sudoswap", abi = "sudoswap(uint256,bytes)")]
    pub struct SudoswapCall {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `sweep` function with signature `sweep(address,address,uint256)` and selector `0x62c06767`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sweep", abi = "sweep(address,address,uint256)")]
    pub struct SweepCall {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub amount_min: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `sweepErc1155` function with signature `sweepErc1155(address,address,uint256,uint256)` and selector `0xb930d445`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "sweepErc1155",
        abi = "sweepErc1155(address,address,uint256,uint256)"
    )]
    pub struct SweepErc1155Call {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `sweepErc721` function with signature `sweepErc721(address,address,uint256)` and selector `0x40e2cf43`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sweepErc721", abi = "sweepErc721(address,address,uint256)")]
    pub struct SweepErc721Call {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,address,uint256)` and selector `0xbeabacc8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,address,uint256)")]
    pub struct TransferCall {
        pub token: ::ethers_core::types::Address,
        pub recipient: ::ethers_core::types::Address,
        pub value: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `unwrapWeth` function with signature `unwrapWeth(address,uint256)` and selector `0x521b6390`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unwrapWeth", abi = "unwrapWeth(address,uint256)")]
    pub struct UnwrapWethCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_min: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `v2SwapExactIn` function with signature `v2SwapExactIn(address,uint256,uint256,address[],bool)` and selector `0xdcd5db1f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "v2SwapExactIn",
        abi = "v2SwapExactIn(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactInCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_in: ::ethers_core::types::U256,
        pub amount_out_min: ::ethers_core::types::U256,
        pub path: ::std::vec::Vec<::ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    ///Container type for all input parameters for the `v2SwapExactOut` function with signature `v2SwapExactOut(address,uint256,uint256,address[],bool)` and selector `0xe868b367`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "v2SwapExactOut",
        abi = "v2SwapExactOut(address,uint256,uint256,address[],bool)"
    )]
    pub struct V2SwapExactOutCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_out: ::ethers_core::types::U256,
        pub amount_in_max: ::ethers_core::types::U256,
        pub path: ::std::vec::Vec<::ethers_core::types::Address>,
        pub payer_is_user: bool,
    }
    ///Container type for all input parameters for the `v3SwapExactIn` function with signature `v3SwapExactIn(address,uint256,uint256,bytes,bool)` and selector `0x5a58ec2d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "v3SwapExactIn",
        abi = "v3SwapExactIn(address,uint256,uint256,bytes,bool)"
    )]
    pub struct V3SwapExactInCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_in: ::ethers_core::types::U256,
        pub amount_out_min: ::ethers_core::types::U256,
        pub path: ::ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    ///Container type for all input parameters for the `v3SwapExactOut` function with signature `v3SwapExactOut(address,uint256,uint256,bytes,bool)` and selector `0x14cd4424`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "v3SwapExactOut",
        abi = "v3SwapExactOut(address,uint256,uint256,bytes,bool)"
    )]
    pub struct V3SwapExactOutCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_out: ::ethers_core::types::U256,
        pub amount_in_max: ::ethers_core::types::U256,
        pub path: ::ethers_core::types::Bytes,
        pub payer_is_user: bool,
    }
    ///Container type for all input parameters for the `wrapEth` function with signature `wrapEth(address,uint256)` and selector `0xb3bc1a13`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "wrapEth", abi = "wrapEth(address,uint256)")]
    pub struct WrapEthCall {
        pub recipient: ::ethers_core::types::Address,
        pub amount_min: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `x2y21155` function with signature `x2y21155(uint256,bytes,address,address,uint256,uint256)` and selector `0x108ccf02`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "x2y21155",
        abi = "x2y21155(uint256,bytes,address,address,uint256,uint256)"
    )]
    pub struct X2Y21155Call {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub recipient: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `x2y2721` function with signature `x2y2721(uint256,bytes,address,address,uint256)` and selector `0xc059e681`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "x2y2721", abi = "x2y2721(uint256,bytes,address,address,uint256)")]
    pub struct X2Y2721Call {
        pub value: ::ethers_core::types::U256,
        pub data: ::ethers_core::types::Bytes,
        pub recipient: ::ethers_core::types::Address,
        pub token: ::ethers_core::types::Address,
        pub id: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniversalRouterCommandsCalls {
        Cryptopunks(CryptopunksCall),
        Foundation(FoundationCall),
        LooksRare1155(LooksRare1155Call),
        LooksRare721(LooksRare721Call),
        Nft20(Nft20Call),
        Nftx(NftxCall),
        OwnerCheck1155(OwnerCheck1155Call),
        OwnerCheck721(OwnerCheck721Call),
        PayPortion(PayPortionCall),
        Permit2Permit(Permit2PermitCall),
        Permit2PermitBatch(Permit2PermitBatchCall),
        Permit2TransferFrom(Permit2TransferFromCall),
        Permit2TransferFromBatch(Permit2TransferFromBatchCall),
        Seaport(SeaportCall),
        Sudoswap(SudoswapCall),
        Sweep(SweepCall),
        SweepErc1155(SweepErc1155Call),
        SweepErc721(SweepErc721Call),
        Transfer(TransferCall),
        UnwrapWeth(UnwrapWethCall),
        V2SwapExactIn(V2SwapExactInCall),
        V2SwapExactOut(V2SwapExactOutCall),
        V3SwapExactIn(V3SwapExactInCall),
        V3SwapExactOut(V3SwapExactOutCall),
        WrapEth(WrapEthCall),
        X2Y21155(X2Y21155Call),
        X2Y2721(X2Y2721Call),
    }
    impl ::ethers_core::abi::AbiDecode for IUniversalRouterCommandsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CryptopunksCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Cryptopunks(decoded));
            }
            if let Ok(decoded) = <FoundationCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Foundation(decoded));
            }
            if let Ok(decoded) = <LooksRare1155Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LooksRare1155(decoded));
            }
            if let Ok(decoded) = <LooksRare721Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LooksRare721(decoded));
            }
            if let Ok(decoded) = <Nft20Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nft20(decoded));
            }
            if let Ok(decoded) = <NftxCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nftx(decoded));
            }
            if let Ok(decoded) = <OwnerCheck1155Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerCheck1155(decoded));
            }
            if let Ok(decoded) = <OwnerCheck721Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerCheck721(decoded));
            }
            if let Ok(decoded) = <PayPortionCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayPortion(decoded));
            }
            if let Ok(decoded) = <Permit2PermitCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2Permit(decoded));
            }
            if let Ok(decoded) = <Permit2PermitBatchCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2PermitBatch(decoded));
            }
            if let Ok(decoded) = <Permit2TransferFromCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2TransferFrom(decoded));
            }
            if let Ok(decoded) = <Permit2TransferFromBatchCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Permit2TransferFromBatch(decoded));
            }
            if let Ok(decoded) = <SeaportCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Seaport(decoded));
            }
            if let Ok(decoded) = <SudoswapCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sudoswap(decoded));
            }
            if let Ok(decoded) = <SweepCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sweep(decoded));
            }
            if let Ok(decoded) = <SweepErc1155Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepErc1155(decoded));
            }
            if let Ok(decoded) = <SweepErc721Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SweepErc721(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <UnwrapWethCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnwrapWeth(decoded));
            }
            if let Ok(decoded) = <V2SwapExactInCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V2SwapExactIn(decoded));
            }
            if let Ok(decoded) = <V2SwapExactOutCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V2SwapExactOut(decoded));
            }
            if let Ok(decoded) = <V3SwapExactInCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3SwapExactIn(decoded));
            }
            if let Ok(decoded) = <V3SwapExactOutCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3SwapExactOut(decoded));
            }
            if let Ok(decoded) = <WrapEthCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WrapEth(decoded));
            }
            if let Ok(decoded) = <X2Y21155Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::X2Y21155(decoded));
            }
            if let Ok(decoded) = <X2Y2721Call as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::X2Y2721(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IUniversalRouterCommandsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Cryptopunks(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Foundation(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LooksRare1155(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LooksRare721(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Nft20(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Nftx(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::OwnerCheck1155(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OwnerCheck721(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::PayPortion(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Permit2Permit(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Permit2PermitBatch(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Permit2TransferFrom(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Permit2TransferFromBatch(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Seaport(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Sudoswap(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Sweep(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SweepErc1155(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SweepErc721(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Transfer(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::UnwrapWeth(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V2SwapExactIn(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V2SwapExactOut(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3SwapExactIn(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3SwapExactOut(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::WrapEth(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::X2Y21155(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::X2Y2721(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IUniversalRouterCommandsCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cryptopunks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Foundation(element) => ::core::fmt::Display::fmt(element, f),
                Self::LooksRare1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::LooksRare721(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nft20(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nftx(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerCheck1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerCheck721(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayPortion(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit2PermitBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Permit2TransferFrom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Permit2TransferFromBatch(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Seaport(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sudoswap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sweep(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepErc1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::SweepErc721(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWeth(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2SwapExactIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2SwapExactOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3SwapExactIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3SwapExactOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapEth(element) => ::core::fmt::Display::fmt(element, f),
                Self::X2Y21155(element) => ::core::fmt::Display::fmt(element, f),
                Self::X2Y2721(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CryptopunksCall> for IUniversalRouterCommandsCalls {
        fn from(value: CryptopunksCall) -> Self {
            Self::Cryptopunks(value)
        }
    }
    impl ::core::convert::From<FoundationCall> for IUniversalRouterCommandsCalls {
        fn from(value: FoundationCall) -> Self {
            Self::Foundation(value)
        }
    }
    impl ::core::convert::From<LooksRare1155Call> for IUniversalRouterCommandsCalls {
        fn from(value: LooksRare1155Call) -> Self {
            Self::LooksRare1155(value)
        }
    }
    impl ::core::convert::From<LooksRare721Call> for IUniversalRouterCommandsCalls {
        fn from(value: LooksRare721Call) -> Self {
            Self::LooksRare721(value)
        }
    }
    impl ::core::convert::From<Nft20Call> for IUniversalRouterCommandsCalls {
        fn from(value: Nft20Call) -> Self {
            Self::Nft20(value)
        }
    }
    impl ::core::convert::From<NftxCall> for IUniversalRouterCommandsCalls {
        fn from(value: NftxCall) -> Self {
            Self::Nftx(value)
        }
    }
    impl ::core::convert::From<OwnerCheck1155Call> for IUniversalRouterCommandsCalls {
        fn from(value: OwnerCheck1155Call) -> Self {
            Self::OwnerCheck1155(value)
        }
    }
    impl ::core::convert::From<OwnerCheck721Call> for IUniversalRouterCommandsCalls {
        fn from(value: OwnerCheck721Call) -> Self {
            Self::OwnerCheck721(value)
        }
    }
    impl ::core::convert::From<PayPortionCall> for IUniversalRouterCommandsCalls {
        fn from(value: PayPortionCall) -> Self {
            Self::PayPortion(value)
        }
    }
    impl ::core::convert::From<Permit2PermitCall> for IUniversalRouterCommandsCalls {
        fn from(value: Permit2PermitCall) -> Self {
            Self::Permit2Permit(value)
        }
    }
    impl ::core::convert::From<Permit2PermitBatchCall>
    for IUniversalRouterCommandsCalls {
        fn from(value: Permit2PermitBatchCall) -> Self {
            Self::Permit2PermitBatch(value)
        }
    }
    impl ::core::convert::From<Permit2TransferFromCall>
    for IUniversalRouterCommandsCalls {
        fn from(value: Permit2TransferFromCall) -> Self {
            Self::Permit2TransferFrom(value)
        }
    }
    impl ::core::convert::From<Permit2TransferFromBatchCall>
    for IUniversalRouterCommandsCalls {
        fn from(value: Permit2TransferFromBatchCall) -> Self {
            Self::Permit2TransferFromBatch(value)
        }
    }
    impl ::core::convert::From<SeaportCall> for IUniversalRouterCommandsCalls {
        fn from(value: SeaportCall) -> Self {
            Self::Seaport(value)
        }
    }
    impl ::core::convert::From<SudoswapCall> for IUniversalRouterCommandsCalls {
        fn from(value: SudoswapCall) -> Self {
            Self::Sudoswap(value)
        }
    }
    impl ::core::convert::From<SweepCall> for IUniversalRouterCommandsCalls {
        fn from(value: SweepCall) -> Self {
            Self::Sweep(value)
        }
    }
    impl ::core::convert::From<SweepErc1155Call> for IUniversalRouterCommandsCalls {
        fn from(value: SweepErc1155Call) -> Self {
            Self::SweepErc1155(value)
        }
    }
    impl ::core::convert::From<SweepErc721Call> for IUniversalRouterCommandsCalls {
        fn from(value: SweepErc721Call) -> Self {
            Self::SweepErc721(value)
        }
    }
    impl ::core::convert::From<TransferCall> for IUniversalRouterCommandsCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<UnwrapWethCall> for IUniversalRouterCommandsCalls {
        fn from(value: UnwrapWethCall) -> Self {
            Self::UnwrapWeth(value)
        }
    }
    impl ::core::convert::From<V2SwapExactInCall> for IUniversalRouterCommandsCalls {
        fn from(value: V2SwapExactInCall) -> Self {
            Self::V2SwapExactIn(value)
        }
    }
    impl ::core::convert::From<V2SwapExactOutCall> for IUniversalRouterCommandsCalls {
        fn from(value: V2SwapExactOutCall) -> Self {
            Self::V2SwapExactOut(value)
        }
    }
    impl ::core::convert::From<V3SwapExactInCall> for IUniversalRouterCommandsCalls {
        fn from(value: V3SwapExactInCall) -> Self {
            Self::V3SwapExactIn(value)
        }
    }
    impl ::core::convert::From<V3SwapExactOutCall> for IUniversalRouterCommandsCalls {
        fn from(value: V3SwapExactOutCall) -> Self {
            Self::V3SwapExactOut(value)
        }
    }
    impl ::core::convert::From<WrapEthCall> for IUniversalRouterCommandsCalls {
        fn from(value: WrapEthCall) -> Self {
            Self::WrapEth(value)
        }
    }
    impl ::core::convert::From<X2Y21155Call> for IUniversalRouterCommandsCalls {
        fn from(value: X2Y21155Call) -> Self {
            Self::X2Y21155(value)
        }
    }
    impl ::core::convert::From<X2Y2721Call> for IUniversalRouterCommandsCalls {
        fn from(value: X2Y2721Call) -> Self {
            Self::X2Y2721(value)
        }
    }
    ///`AllowanceTransferDetails(address,address,uint160,address)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllowanceTransferDetails {
        pub from: ::ethers_core::types::Address,
        pub to: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub token: ::ethers_core::types::Address,
    }
    ///`PermitBatch((address,uint160,uint48,uint48)[],address,uint256)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PermitBatch {
        pub details: ::std::vec::Vec<PermitDetails>,
        pub spender: ::ethers_core::types::Address,
        pub sig_deadline: ::ethers_core::types::U256,
    }
    ///`PermitDetails(address,uint160,uint48,uint48)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PermitDetails {
        pub token: ::ethers_core::types::Address,
        pub amount: ::ethers_core::types::U256,
        pub expiration: u64,
        pub nonce: u64,
    }
    ///`PermitSingle((address,uint160,uint48,uint48),address,uint256)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PermitSingle {
        pub details: PermitDetails,
        pub spender: ::ethers_core::types::Address,
        pub sig_deadline: ::ethers_core::types::U256,
    }
}

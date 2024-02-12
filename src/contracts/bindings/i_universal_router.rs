pub use i_universal_router::*;
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
pub mod i_universal_router {
    #[allow(deprecated)]
    fn __abi() -> ::ethers_core::abi::Abi {
        ::ethers_core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers_core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers_core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("params"),
                        kind: ::ethers_core::abi::ethabi::ParamType::Tuple(
                            ::std::vec![
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::Address,
                                ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers_core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct RouterParameters"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("collectRewards"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collectRewards"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("looksRareClaim"),
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
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commands"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commands"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes[]"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "onERC1155BatchReceived",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers_core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RewardsSent"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RewardsSent"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ContractLocked"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ContractLocked"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ETHNotAccepted"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ETHNotAccepted"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExecutionFailed"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExecutionFailed"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandIndex"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("message"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FromAddressIsNotOwner"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FromAddressIsNotOwner",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientETH"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientETH"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientToken"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InsufficientToken"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBips"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidBips"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCommandType"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidCommandType"),
                            inputs: ::std::vec![
                                ::ethers_core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandType"),
                                    kind: ::ethers_core::abi::ethabi::ParamType::Uint(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOwnerERC1155"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOwnerERC1155",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOwnerERC721"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOwnerERC721"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPath"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPath"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReserves"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidReserves"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LengthMismatch"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LengthMismatch"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoSlice"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoSlice"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SliceOutOfBounds"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SliceOutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SliceOverflow"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SliceOverflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ToAddressOutOfBounds"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ToAddressOutOfBounds",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ToAddressOverflow"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ToAddressOverflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ToUint24OutOfBounds"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ToUint24OutOfBounds",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ToUint24Overflow"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ToUint24Overflow"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactionDeadlinePassed"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TransactionDeadlinePassed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnableToClaim"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnableToClaim"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnsafeCast"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("UnsafeCast"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V2InvalidPath"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V2InvalidPath"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V2TooLittleReceived"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "V2TooLittleReceived",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V2TooMuchRequested"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V2TooMuchRequested"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V3InvalidAmountOut"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V3InvalidAmountOut"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V3InvalidCaller"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V3InvalidCaller"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V3InvalidSwap"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V3InvalidSwap"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V3TooLittleReceived"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "V3TooLittleReceived",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("V3TooMuchRequested"),
                    ::std::vec![
                        ::ethers_core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("V3TooMuchRequested"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IUNIVERSALROUTER_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    pub struct IUniversalRouter<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUniversalRouter<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUniversalRouter<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUniversalRouter<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUniversalRouter<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IUniversalRouter))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> IUniversalRouter<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IUNIVERSALROUTER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `collectRewards` (0x709a1cc2) function
        pub fn collect_rewards(
            &self,
            looks_rare_claim: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 154, 28, 194], looks_rare_claim)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x24856bc3) function
        pub fn execute(
            &self,
            commands: ::ethers_core::types::Bytes,
            inputs: ::std::vec::Vec<::ethers_core::types::Bytes>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 133, 107, 195], (commands, inputs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x3593564c) function
        pub fn execute_with_commands_and_inputs(
            &self,
            commands: ::ethers_core::types::Bytes,
            inputs: ::std::vec::Vec<::ethers_core::types::Bytes>,
            deadline: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 147, 86, 76], (commands, inputs, deadline))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::std::vec::Vec<::ethers_core::types::U256>,
            p3: ::std::vec::Vec<::ethers_core::types::U256>,
            p4: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::ethers_core::types::U256,
            p3: ::ethers_core::types::U256,
            p4: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers_core::types::Address,
            p1: ::ethers_core::types::Address,
            p2: ::ethers_core::types::U256,
            p3: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers_core::types::I256,
            amount_1_delta: ::ethers_core::types::I256,
            data: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `RewardsSent` event
        pub fn rewards_sent_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsSentFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RewardsSentFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for IUniversalRouter<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ContractLocked` with signature `ContractLocked()` and selector `0x6f5ffb7e`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ContractLocked", abi = "ContractLocked()")]
    pub struct ContractLocked;
    ///Custom Error type `ETHNotAccepted` with signature `ETHNotAccepted()` and selector `0x1231ae40`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ETHNotAccepted", abi = "ETHNotAccepted()")]
    pub struct ETHNotAccepted;
    ///Custom Error type `ExecutionFailed` with signature `ExecutionFailed(uint256,bytes)` and selector `0x2c4029e9`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ExecutionFailed", abi = "ExecutionFailed(uint256,bytes)")]
    pub struct ExecutionFailed {
        pub command_index: ::ethers_core::types::U256,
        pub message: ::ethers_core::types::Bytes,
    }
    ///Custom Error type `FromAddressIsNotOwner` with signature `FromAddressIsNotOwner()` and selector `0xe7002877`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FromAddressIsNotOwner", abi = "FromAddressIsNotOwner()")]
    pub struct FromAddressIsNotOwner;
    ///Custom Error type `InsufficientETH` with signature `InsufficientETH()` and selector `0x6a12f104`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientETH", abi = "InsufficientETH()")]
    pub struct InsufficientETH;
    ///Custom Error type `InsufficientToken` with signature `InsufficientToken()` and selector `0x675cae38`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientToken", abi = "InsufficientToken()")]
    pub struct InsufficientToken;
    ///Custom Error type `InvalidBips` with signature `InvalidBips()` and selector `0xdeaa01e6`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidBips", abi = "InvalidBips()")]
    pub struct InvalidBips;
    ///Custom Error type `InvalidCommandType` with signature `InvalidCommandType(uint256)` and selector `0xd76a1e9e`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidCommandType", abi = "InvalidCommandType(uint256)")]
    pub struct InvalidCommandType {
        pub command_type: ::ethers_core::types::U256,
    }
    ///Custom Error type `InvalidOwnerERC1155` with signature `InvalidOwnerERC1155()` and selector `0x483a6929`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOwnerERC1155", abi = "InvalidOwnerERC1155()")]
    pub struct InvalidOwnerERC1155;
    ///Custom Error type `InvalidOwnerERC721` with signature `InvalidOwnerERC721()` and selector `0x7dbe7e89`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOwnerERC721", abi = "InvalidOwnerERC721()")]
    pub struct InvalidOwnerERC721;
    ///Custom Error type `InvalidPath` with signature `InvalidPath()` and selector `0x20db8267`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidPath", abi = "InvalidPath()")]
    pub struct InvalidPath;
    ///Custom Error type `InvalidReserves` with signature `InvalidReserves()` and selector `0x7b9c8916`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidReserves", abi = "InvalidReserves()")]
    pub struct InvalidReserves;
    ///Custom Error type `LengthMismatch` with signature `LengthMismatch()` and selector `0xff633a38`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "LengthMismatch", abi = "LengthMismatch()")]
    pub struct LengthMismatch;
    ///Custom Error type `NoSlice` with signature `NoSlice()` and selector `0xcc94a63a`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoSlice", abi = "NoSlice()")]
    pub struct NoSlice;
    ///Custom Error type `SliceOutOfBounds` with signature `SliceOutOfBounds()` and selector `0x3b99b53d`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SliceOutOfBounds", abi = "SliceOutOfBounds()")]
    pub struct SliceOutOfBounds;
    ///Custom Error type `SliceOverflow` with signature `SliceOverflow()` and selector `0x47aaf07a`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SliceOverflow", abi = "SliceOverflow()")]
    pub struct SliceOverflow;
    ///Custom Error type `ToAddressOutOfBounds` with signature `ToAddressOutOfBounds()` and selector `0xa78aa27f`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ToAddressOutOfBounds", abi = "ToAddressOutOfBounds()")]
    pub struct ToAddressOutOfBounds;
    ///Custom Error type `ToAddressOverflow` with signature `ToAddressOverflow()` and selector `0x77146e62`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ToAddressOverflow", abi = "ToAddressOverflow()")]
    pub struct ToAddressOverflow;
    ///Custom Error type `ToUint24OutOfBounds` with signature `ToUint24OutOfBounds()` and selector `0xd9096a3e`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ToUint24OutOfBounds", abi = "ToUint24OutOfBounds()")]
    pub struct ToUint24OutOfBounds;
    ///Custom Error type `ToUint24Overflow` with signature `ToUint24Overflow()` and selector `0x855859b4`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ToUint24Overflow", abi = "ToUint24Overflow()")]
    pub struct ToUint24Overflow;
    ///Custom Error type `TransactionDeadlinePassed` with signature `TransactionDeadlinePassed()` and selector `0x5bf6f916`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "TransactionDeadlinePassed", abi = "TransactionDeadlinePassed()")]
    pub struct TransactionDeadlinePassed;
    ///Custom Error type `UnableToClaim` with signature `UnableToClaim()` and selector `0x7d529919`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnableToClaim", abi = "UnableToClaim()")]
    pub struct UnableToClaim;
    ///Custom Error type `UnsafeCast` with signature `UnsafeCast()` and selector `0xc4bd89a9`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnsafeCast", abi = "UnsafeCast()")]
    pub struct UnsafeCast;
    ///Custom Error type `V2InvalidPath` with signature `V2InvalidPath()` and selector `0xae52ad0c`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V2InvalidPath", abi = "V2InvalidPath()")]
    pub struct V2InvalidPath;
    ///Custom Error type `V2TooLittleReceived` with signature `V2TooLittleReceived()` and selector `0x849eaf98`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V2TooLittleReceived", abi = "V2TooLittleReceived()")]
    pub struct V2TooLittleReceived;
    ///Custom Error type `V2TooMuchRequested` with signature `V2TooMuchRequested()` and selector `0x8ab0bc16`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V2TooMuchRequested", abi = "V2TooMuchRequested()")]
    pub struct V2TooMuchRequested;
    ///Custom Error type `V3InvalidAmountOut` with signature `V3InvalidAmountOut()` and selector `0xd4e0248e`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V3InvalidAmountOut", abi = "V3InvalidAmountOut()")]
    pub struct V3InvalidAmountOut;
    ///Custom Error type `V3InvalidCaller` with signature `V3InvalidCaller()` and selector `0x32b13d91`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V3InvalidCaller", abi = "V3InvalidCaller()")]
    pub struct V3InvalidCaller;
    ///Custom Error type `V3InvalidSwap` with signature `V3InvalidSwap()` and selector `0x316cf0eb`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V3InvalidSwap", abi = "V3InvalidSwap()")]
    pub struct V3InvalidSwap;
    ///Custom Error type `V3TooLittleReceived` with signature `V3TooLittleReceived()` and selector `0x39d35496`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V3TooLittleReceived", abi = "V3TooLittleReceived()")]
    pub struct V3TooLittleReceived;
    ///Custom Error type `V3TooMuchRequested` with signature `V3TooMuchRequested()` and selector `0x739dbe52`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "V3TooMuchRequested", abi = "V3TooMuchRequested()")]
    pub struct V3TooMuchRequested;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniversalRouterErrors {
        ContractLocked(ContractLocked),
        ETHNotAccepted(ETHNotAccepted),
        ExecutionFailed(ExecutionFailed),
        FromAddressIsNotOwner(FromAddressIsNotOwner),
        InsufficientETH(InsufficientETH),
        InsufficientToken(InsufficientToken),
        InvalidBips(InvalidBips),
        InvalidCommandType(InvalidCommandType),
        InvalidOwnerERC1155(InvalidOwnerERC1155),
        InvalidOwnerERC721(InvalidOwnerERC721),
        InvalidPath(InvalidPath),
        InvalidReserves(InvalidReserves),
        LengthMismatch(LengthMismatch),
        NoSlice(NoSlice),
        SliceOutOfBounds(SliceOutOfBounds),
        SliceOverflow(SliceOverflow),
        ToAddressOutOfBounds(ToAddressOutOfBounds),
        ToAddressOverflow(ToAddressOverflow),
        ToUint24OutOfBounds(ToUint24OutOfBounds),
        ToUint24Overflow(ToUint24Overflow),
        TransactionDeadlinePassed(TransactionDeadlinePassed),
        UnableToClaim(UnableToClaim),
        UnsafeCast(UnsafeCast),
        V2InvalidPath(V2InvalidPath),
        V2TooLittleReceived(V2TooLittleReceived),
        V2TooMuchRequested(V2TooMuchRequested),
        V3InvalidAmountOut(V3InvalidAmountOut),
        V3InvalidCaller(V3InvalidCaller),
        V3InvalidSwap(V3InvalidSwap),
        V3TooLittleReceived(V3TooLittleReceived),
        V3TooMuchRequested(V3TooMuchRequested),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers_core::abi::AbiDecode for IUniversalRouterErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ContractLocked as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractLocked(decoded));
            }
            if let Ok(decoded) = <ETHNotAccepted as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ETHNotAccepted(decoded));
            }
            if let Ok(decoded) = <ExecutionFailed as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecutionFailed(decoded));
            }
            if let Ok(decoded) = <FromAddressIsNotOwner as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FromAddressIsNotOwner(decoded));
            }
            if let Ok(decoded) = <InsufficientETH as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientETH(decoded));
            }
            if let Ok(decoded) = <InsufficientToken as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientToken(decoded));
            }
            if let Ok(decoded) = <InvalidBips as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidBips(decoded));
            }
            if let Ok(decoded) = <InvalidCommandType as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCommandType(decoded));
            }
            if let Ok(decoded) = <InvalidOwnerERC1155 as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOwnerERC1155(decoded));
            }
            if let Ok(decoded) = <InvalidOwnerERC721 as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOwnerERC721(decoded));
            }
            if let Ok(decoded) = <InvalidPath as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidPath(decoded));
            }
            if let Ok(decoded) = <InvalidReserves as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidReserves(decoded));
            }
            if let Ok(decoded) = <LengthMismatch as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LengthMismatch(decoded));
            }
            if let Ok(decoded) = <NoSlice as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NoSlice(decoded));
            }
            if let Ok(decoded) = <SliceOutOfBounds as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SliceOutOfBounds(decoded));
            }
            if let Ok(decoded) = <SliceOverflow as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SliceOverflow(decoded));
            }
            if let Ok(decoded) = <ToAddressOutOfBounds as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToAddressOutOfBounds(decoded));
            }
            if let Ok(decoded) = <ToAddressOverflow as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToAddressOverflow(decoded));
            }
            if let Ok(decoded) = <ToUint24OutOfBounds as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToUint24OutOfBounds(decoded));
            }
            if let Ok(decoded) = <ToUint24Overflow as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToUint24Overflow(decoded));
            }
            if let Ok(decoded) = <TransactionDeadlinePassed as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransactionDeadlinePassed(decoded));
            }
            if let Ok(decoded) = <UnableToClaim as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnableToClaim(decoded));
            }
            if let Ok(decoded) = <UnsafeCast as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UnsafeCast(decoded));
            }
            if let Ok(decoded) = <V2InvalidPath as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V2InvalidPath(decoded));
            }
            if let Ok(decoded) = <V2TooLittleReceived as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V2TooLittleReceived(decoded));
            }
            if let Ok(decoded) = <V2TooMuchRequested as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V2TooMuchRequested(decoded));
            }
            if let Ok(decoded) = <V3InvalidAmountOut as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3InvalidAmountOut(decoded));
            }
            if let Ok(decoded) = <V3InvalidCaller as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3InvalidCaller(decoded));
            }
            if let Ok(decoded) = <V3InvalidSwap as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3InvalidSwap(decoded));
            }
            if let Ok(decoded) = <V3TooLittleReceived as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3TooLittleReceived(decoded));
            }
            if let Ok(decoded) = <V3TooMuchRequested as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::V3TooMuchRequested(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IUniversalRouterErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ContractLocked(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ETHNotAccepted(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ExecutionFailed(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::FromAddressIsNotOwner(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientETH(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientToken(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBips(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCommandType(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnerERC1155(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnerERC721(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPath(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserves(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LengthMismatch(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::NoSlice(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::SliceOutOfBounds(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SliceOverflow(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ToAddressOutOfBounds(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ToAddressOverflow(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ToUint24OutOfBounds(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::ToUint24Overflow(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::TransactionDeadlinePassed(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UnableToClaim(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UnsafeCast(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V2InvalidPath(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V2TooLittleReceived(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V2TooMuchRequested(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3InvalidAmountOut(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3InvalidCaller(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3InvalidSwap(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3TooLittleReceived(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::V3TooMuchRequested(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers_core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for IUniversalRouterErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ContractLocked as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ETHNotAccepted as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ExecutionFailed as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FromAddressIsNotOwner as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientETH as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientToken as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBips as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidCommandType as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOwnerERC1155 as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOwnerERC721 as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidPath as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidReserves as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LengthMismatch as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NoSlice as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SliceOutOfBounds as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SliceOverflow as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <ToAddressOutOfBounds as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ToAddressOverflow as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ToUint24OutOfBounds as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ToUint24Overflow as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TransactionDeadlinePassed as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnableToClaim as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <UnsafeCast as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <V2InvalidPath as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <V2TooLittleReceived as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V2TooMuchRequested as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V3InvalidAmountOut as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V3InvalidCaller as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V3InvalidSwap as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <V3TooLittleReceived as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <V3TooMuchRequested as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IUniversalRouterErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::ETHNotAccepted(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecutionFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::FromAddressIsNotOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBips(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCommandType(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOwnerERC1155(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOwnerERC721(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::LengthMismatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoSlice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SliceOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SliceOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToAddressOutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ToAddressOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToUint24OutOfBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ToUint24Overflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionDeadlinePassed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnableToClaim(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnsafeCast(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2InvalidPath(element) => ::core::fmt::Display::fmt(element, f),
                Self::V2TooLittleReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::V2TooMuchRequested(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::V3InvalidAmountOut(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::V3InvalidCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::V3TooLittleReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::V3TooMuchRequested(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IUniversalRouterErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ContractLocked> for IUniversalRouterErrors {
        fn from(value: ContractLocked) -> Self {
            Self::ContractLocked(value)
        }
    }
    impl ::core::convert::From<ETHNotAccepted> for IUniversalRouterErrors {
        fn from(value: ETHNotAccepted) -> Self {
            Self::ETHNotAccepted(value)
        }
    }
    impl ::core::convert::From<ExecutionFailed> for IUniversalRouterErrors {
        fn from(value: ExecutionFailed) -> Self {
            Self::ExecutionFailed(value)
        }
    }
    impl ::core::convert::From<FromAddressIsNotOwner> for IUniversalRouterErrors {
        fn from(value: FromAddressIsNotOwner) -> Self {
            Self::FromAddressIsNotOwner(value)
        }
    }
    impl ::core::convert::From<InsufficientETH> for IUniversalRouterErrors {
        fn from(value: InsufficientETH) -> Self {
            Self::InsufficientETH(value)
        }
    }
    impl ::core::convert::From<InsufficientToken> for IUniversalRouterErrors {
        fn from(value: InsufficientToken) -> Self {
            Self::InsufficientToken(value)
        }
    }
    impl ::core::convert::From<InvalidBips> for IUniversalRouterErrors {
        fn from(value: InvalidBips) -> Self {
            Self::InvalidBips(value)
        }
    }
    impl ::core::convert::From<InvalidCommandType> for IUniversalRouterErrors {
        fn from(value: InvalidCommandType) -> Self {
            Self::InvalidCommandType(value)
        }
    }
    impl ::core::convert::From<InvalidOwnerERC1155> for IUniversalRouterErrors {
        fn from(value: InvalidOwnerERC1155) -> Self {
            Self::InvalidOwnerERC1155(value)
        }
    }
    impl ::core::convert::From<InvalidOwnerERC721> for IUniversalRouterErrors {
        fn from(value: InvalidOwnerERC721) -> Self {
            Self::InvalidOwnerERC721(value)
        }
    }
    impl ::core::convert::From<InvalidPath> for IUniversalRouterErrors {
        fn from(value: InvalidPath) -> Self {
            Self::InvalidPath(value)
        }
    }
    impl ::core::convert::From<InvalidReserves> for IUniversalRouterErrors {
        fn from(value: InvalidReserves) -> Self {
            Self::InvalidReserves(value)
        }
    }
    impl ::core::convert::From<LengthMismatch> for IUniversalRouterErrors {
        fn from(value: LengthMismatch) -> Self {
            Self::LengthMismatch(value)
        }
    }
    impl ::core::convert::From<NoSlice> for IUniversalRouterErrors {
        fn from(value: NoSlice) -> Self {
            Self::NoSlice(value)
        }
    }
    impl ::core::convert::From<SliceOutOfBounds> for IUniversalRouterErrors {
        fn from(value: SliceOutOfBounds) -> Self {
            Self::SliceOutOfBounds(value)
        }
    }
    impl ::core::convert::From<SliceOverflow> for IUniversalRouterErrors {
        fn from(value: SliceOverflow) -> Self {
            Self::SliceOverflow(value)
        }
    }
    impl ::core::convert::From<ToAddressOutOfBounds> for IUniversalRouterErrors {
        fn from(value: ToAddressOutOfBounds) -> Self {
            Self::ToAddressOutOfBounds(value)
        }
    }
    impl ::core::convert::From<ToAddressOverflow> for IUniversalRouterErrors {
        fn from(value: ToAddressOverflow) -> Self {
            Self::ToAddressOverflow(value)
        }
    }
    impl ::core::convert::From<ToUint24OutOfBounds> for IUniversalRouterErrors {
        fn from(value: ToUint24OutOfBounds) -> Self {
            Self::ToUint24OutOfBounds(value)
        }
    }
    impl ::core::convert::From<ToUint24Overflow> for IUniversalRouterErrors {
        fn from(value: ToUint24Overflow) -> Self {
            Self::ToUint24Overflow(value)
        }
    }
    impl ::core::convert::From<TransactionDeadlinePassed> for IUniversalRouterErrors {
        fn from(value: TransactionDeadlinePassed) -> Self {
            Self::TransactionDeadlinePassed(value)
        }
    }
    impl ::core::convert::From<UnableToClaim> for IUniversalRouterErrors {
        fn from(value: UnableToClaim) -> Self {
            Self::UnableToClaim(value)
        }
    }
    impl ::core::convert::From<UnsafeCast> for IUniversalRouterErrors {
        fn from(value: UnsafeCast) -> Self {
            Self::UnsafeCast(value)
        }
    }
    impl ::core::convert::From<V2InvalidPath> for IUniversalRouterErrors {
        fn from(value: V2InvalidPath) -> Self {
            Self::V2InvalidPath(value)
        }
    }
    impl ::core::convert::From<V2TooLittleReceived> for IUniversalRouterErrors {
        fn from(value: V2TooLittleReceived) -> Self {
            Self::V2TooLittleReceived(value)
        }
    }
    impl ::core::convert::From<V2TooMuchRequested> for IUniversalRouterErrors {
        fn from(value: V2TooMuchRequested) -> Self {
            Self::V2TooMuchRequested(value)
        }
    }
    impl ::core::convert::From<V3InvalidAmountOut> for IUniversalRouterErrors {
        fn from(value: V3InvalidAmountOut) -> Self {
            Self::V3InvalidAmountOut(value)
        }
    }
    impl ::core::convert::From<V3InvalidCaller> for IUniversalRouterErrors {
        fn from(value: V3InvalidCaller) -> Self {
            Self::V3InvalidCaller(value)
        }
    }
    impl ::core::convert::From<V3InvalidSwap> for IUniversalRouterErrors {
        fn from(value: V3InvalidSwap) -> Self {
            Self::V3InvalidSwap(value)
        }
    }
    impl ::core::convert::From<V3TooLittleReceived> for IUniversalRouterErrors {
        fn from(value: V3TooLittleReceived) -> Self {
            Self::V3TooLittleReceived(value)
        }
    }
    impl ::core::convert::From<V3TooMuchRequested> for IUniversalRouterErrors {
        fn from(value: V3TooMuchRequested) -> Self {
            Self::V3TooMuchRequested(value)
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "RewardsSent", abi = "RewardsSent(uint256)")]
    pub struct RewardsSentFilter {
        pub amount: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `collectRewards` function with signature `collectRewards(bytes)` and selector `0x709a1cc2`
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
    #[ethcall(name = "collectRewards", abi = "collectRewards(bytes)")]
    pub struct CollectRewardsCall {
        pub looks_rare_claim: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[])` and selector `0x24856bc3`
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
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[])")]
    pub struct ExecuteCall {
        pub commands: ::ethers_core::types::Bytes,
        pub inputs: ::std::vec::Vec<::ethers_core::types::Bytes>,
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes,bytes[],uint256)` and selector `0x3593564c`
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
    #[ethcall(name = "execute", abi = "execute(bytes,bytes[],uint256)")]
    pub struct ExecuteWithCommandsAndInputsCall {
        pub commands: ::ethers_core::types::Bytes,
        pub inputs: ::std::vec::Vec<::ethers_core::types::Bytes>,
        pub deadline: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::std::vec::Vec<::ethers_core::types::U256>,
        pub ::std::vec::Vec<::ethers_core::types::U256>,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::Address,
        pub ::ethers_core::types::U256,
        pub ::ethers_core::types::Bytes,
    );
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
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
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers_core::types::I256,
        pub amount_1_delta: ::ethers_core::types::I256,
        pub data: ::ethers_core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUniversalRouterCalls {
        CollectRewards(CollectRewardsCall),
        Execute(ExecuteCall),
        ExecuteWithCommandsAndInputs(ExecuteWithCommandsAndInputsCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        SupportsInterface(SupportsInterfaceCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers_core::abi::AbiDecode for IUniversalRouterCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CollectRewardsCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollectRewards(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteWithCommandsAndInputsCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithCommandsAndInputs(decoded));
            }
            if let Ok(decoded) = <OnERC1155BatchReceivedCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) = <OnERC1155ReceivedCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for IUniversalRouterCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CollectRewards(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::ExecuteWithCommandsAndInputs(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IUniversalRouterCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CollectRewards(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithCommandsAndInputs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CollectRewardsCall> for IUniversalRouterCalls {
        fn from(value: CollectRewardsCall) -> Self {
            Self::CollectRewards(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for IUniversalRouterCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteWithCommandsAndInputsCall>
    for IUniversalRouterCalls {
        fn from(value: ExecuteWithCommandsAndInputsCall) -> Self {
            Self::ExecuteWithCommandsAndInputs(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for IUniversalRouterCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for IUniversalRouterCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for IUniversalRouterCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for IUniversalRouterCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for IUniversalRouterCalls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
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
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///`RouterParameters(address,address,address,address,address,address,address,address,address,address,address,address,address,address,address,bytes32,bytes32)`
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
    pub struct RouterParameters {
        pub permit_2: ::ethers_core::types::Address,
        pub weth_9: ::ethers_core::types::Address,
        pub seaport: ::ethers_core::types::Address,
        pub nftx_zap: ::ethers_core::types::Address,
        pub x_2y_2: ::ethers_core::types::Address,
        pub foundation: ::ethers_core::types::Address,
        pub sudoswap: ::ethers_core::types::Address,
        pub nft_20_zap: ::ethers_core::types::Address,
        pub cryptopunks: ::ethers_core::types::Address,
        pub looks_rare: ::ethers_core::types::Address,
        pub router_rewards_distributor: ::ethers_core::types::Address,
        pub looks_rare_rewards_distributor: ::ethers_core::types::Address,
        pub looks_rare_token: ::ethers_core::types::Address,
        pub v_2_factory: ::ethers_core::types::Address,
        pub v_3_factory: ::ethers_core::types::Address,
        pub pair_init_code_hash: [u8; 32],
        pub pool_init_code_hash: [u8; 32],
    }
}

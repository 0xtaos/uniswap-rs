use super::{Factory, Library};
use crate::{contracts::bindings::i_uniswap_v2_pair::IUniswapV2Pair, errors::Result, ProtocolType};
use ethers_contract::{
    builders::ContractCall, ContractError, Multicall, MulticallError, MulticallVersion,
};
use ethers_core::{
    abi::{Detokenize, Token},
    types::{Address, Chain, H256,H160}
};
use ethers_providers::Middleware;
use std::{fmt, sync::Arc};

type Tokens = (Address, Address);
type Reserves = (u128, u128, u32);

contract_struct! {
    /// A UniswapV2 liquidity pair, composed of 2 different ERC20 tokens.
    pub struct Pair<M> {
        /// The pair contract.
        contract: IUniswapV2Pair<M>,

        /// The ordered tokens of the pair.
        tokens: Option<Tokens>,

        /// Whether the pair is currently deployed in the client's network.
        deployed: bool,

        /// The token reserves of the pair.
        reserves: Option<Reserves>,

        /// The protocol of the pair.
        pub protocol: ProtocolType,
    }
}

impl<M> fmt::Display for Pair<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let address = self.address();
        if self.tokens.is_none() && self.reserves.is_none() {
            return writeln!(f, "Pair: {address:?}");
        }
        writeln!(f, "Pair:     {address:?}")?;
        if let Some((a, b)) = self.tokens {
            writeln!(f, "Token0:   {a:?}")?;
            write!(f, "Token1:   {b:?}")?;
            if self.reserves.is_some() {
                writeln!(f)?
            };
        }
        if let Some((a, b, _)) = self.reserves {
            writeln!(f, "Reserve0: {a:?}")?;
            write!(f, "Reserve1: {b:?}")?;
        }
        Ok(())
    }
}

impl<M> Pair<M> {
    /// Returns whether the pair has been deployed.
    ///
    /// Note: this will always be false before syncing.
    pub fn deployed(&self) -> bool {
        self.deployed
    }

    /// Returns the addresses of the tokens that make up this pair.
    ///
    /// Note: this will always be None before syncing.
    pub fn tokens(&self) -> Option<Tokens> {
        self.tokens
    }

    /// Returns the reserves of the pair.
    ///
    /// Note: this will always be None before syncing.
    pub fn reserves(&self) -> Option<Reserves> {
        self.reserves
    }

    /// Returns the hash of the pair's deployment code. This can be used to determinalistically
    /// calculate the address of the pair given the addresses of 2 (sorted) tokens.
    ///
    /// Note: `chain` is used only when the pair code hash differs in the same protocol, for example
    /// `Pancakeswap` has two different code hashes for BSC mainnet and testnet.
    pub fn code_hash(&self, chain: Option<Chain>) -> H256 {
        self.protocol.pair_code_hash(chain)
    }
}

impl<M: Middleware> Pair<M> {
    /// Creates a new instance using the provided client and address.
    pub fn new(client: Arc<M>, address: Address, protocol: ProtocolType) -> Self {
        let contract = IUniswapV2Pair::new(address, client);
        Self { contract, tokens: None, deployed: false, reserves: None, protocol }
    }

    /// Creates a new instance using the provided client, factory and tokens' addresses.
    pub fn new_with_factory(
        factory: &Factory<M>,
        token0: Address,
        token1: Address,
    ) -> Result<Self> {
        let (token0, token1) = Library::sort_tokens(token0, token1);
        let address = Library::pair_for(factory, token0, token1);
        let contract = IUniswapV2Pair::new(address, factory.client());

        Ok(Self {
            contract,
            tokens: Some((token0, token1)),
            deployed: false,
            reserves: None,
            protocol: factory.protocol(),
        })
    }

    /// Returns the contract calls for getting the addresses of the pair's tokens.
    pub fn get_tokens(&self) -> (ContractCall<M, Address>, ContractCall<M, Address>) {
        (self.contract.token_0(), self.contract.token_1())
    }

    /// Syncs the tokens and reserves of the pair by querying the blockchain.
    ///
    /// Assumes that any call failure means the pair has not been deployed yet.
    pub async fn sync(&mut self, sync_tokens: bool, sync_reserves: bool) -> Result<&mut Self> {
        // let sync_tokens = self.tokens.is_none() || !self.deployed;
        // let sync_reserves = self.reserves.is_none() || !self.deployed;

        let multicall = Multicall::new(self.client(), None).await?;
        let mut multicall = multicall.version(MulticallVersion::Multicall3);

        if sync_tokens {
            let calls = self.get_tokens();
            multicall.add_call(calls.0, true);
            multicall.add_call(calls.1, true);
        }

        if sync_reserves {
            multicall.add_call(self.contract.get_reserves(), true);
        }

        // Assume any call failure means the contract has not been deployed yet
        let result = multicall.call_raw().await;
        let result = match result {
            Ok(result) => result,
            Err(MulticallError::ContractError(ContractError::DecodingError(_))) => {
                self.tokens = None;
                self.deployed = false;
                return Ok(self);
            }
            Err(e) => return Err(e.into()),
        };

        let result: Vec<Token> = result.into_iter().map(|r| r.unwrap()).collect();

        match (sync_tokens, sync_reserves) {
            (true, true) => {
                if result.len() == 3{
                    let t0 = result[0].clone().into_address();
                    let t1 = result[1].clone().into_address();
                    let tokens = Some((t0.unwrap(), t1.unwrap()));

                    let reserve = result[2].clone();
                    let reserves = match reserve {
                        Token::Tuple(response) => {
                            let reserve0 = response[0].clone().into_uint().unwrap().as_u128();
                            let reserve1 = response[1].clone().into_uint().unwrap().as_u128();
                            let ts = response[2].clone().into_uint().unwrap().as_u32();

                            let reserves = Some((reserve0, reserve1, ts));
                            reserves
                        }
                        _ => { None }
                    };
                    
                    self.tokens = tokens;
                    self.reserves = reserves;
                    self.deployed = true;
                    
                }else{
                    self.tokens = None;
                    self.deployed = false;
                }
            }
            (true, false) => { // tokens on the pair
                if result.len() == 2{
                    let t0 = result[0].clone().into_address();
                    let t1 = result[1].clone().into_address();
                    self.deployed = true;
                    self.tokens = Some((t0.unwrap(), t1.unwrap()));
                }else{
                    self.tokens = None;
                    self.deployed = false;
                }
            }
            (false, true) => { //reserves
                let mut tokens = result.into_iter();
                if let Some(token) = tokens.next() {
                    match token {
                        Token::Tuple(response) => {
                            let reserve0 = response[0].clone().into_uint().unwrap().as_u128();
                            let reserve1 = response[1].clone().into_uint().unwrap().as_u128();
                            let ts = response[2].clone().into_uint().unwrap().as_u32();
    
                            self.deployed = true;
                            self.reserves = Some((reserve0, reserve1, ts));

                            return Ok(self);
                        }
                        _ => {}
                    }
                }

                self.reserves = None;
                self.deployed = false;
                
            }
            (false, false) => {}
        }

        Ok(self)
    }
}

/// Parses (bool, String) from a vector of tokens.
fn parse_errors(tokens: Vec<Token>) -> Vec<Option<String>> {
    type ErrorResult = (bool, String);
    let mut msgs = vec![];

    for token in tokens {
        match ErrorResult::from_tokens(vec![token]) {
            Ok((_, msg)) => msgs.push(Some(msg)),
            Err(_) => msgs.push(None),
        }
    }

    msgs
}

/// Parses a multicall result from a vector of tokens, returning None if the call returned an
/// error.
fn parse_result<D: Detokenize>(tokens: Vec<Token>) -> Result<Option<D>> {
    let res = D::from_tokens(tokens.clone());
    match res {
        Err(e) => {
            // Failed to decode
            let errors = parse_errors(tokens);
            if errors.iter().any(|s| s.is_none()) {
                // Failed to decode errors too
                Err(e.into())
            } else {
                // All calls failed while allowed
                Ok(None)
            }
        }
        Ok(res) => Ok(Some(res)),
    }
}

/// Parses a multicall result of Pair::get_tokens(), returning None if the call returned an
/// error.
fn parse_tokens_result(tokens: Vec<Token>) -> Result<Option<Tokens>> {
    type TokensResult = ((bool, Address), (bool, Address));
    let res: Option<TokensResult> = parse_result(tokens)?;

    match res {
        Some(res) => {
            if res.0 .0 && res.1 .0 {
                Ok(Some((res.0 .1, res.1 .1)))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

// fn parse_multicall_tokens(tokens: Vec<Token>) -> Result<Option<(H160, H160)>> {
//     if tokens.len() != 2{
//         return Ok(None);
//     }else{
//         let t0 = tokens[0].clone().into_address();
//         let t1 = tokens[1].clone().into_address();

//     }

//     type TokensResult = ((bool, Address), (bool, Address));
//     let res: Option<TokensResult> = parse_result(tokens)?;

//     match res {
//         Some(res) => {
//             if res.0 .0 && res.1 .0 {
//                 Ok(Some((res.0 .1, res.1 .1)))
//             } else {
//                 Ok(None)
//             }
//         }
//         None => Ok(None),
//     }
// }

/// Parses a multicall result of Pair::get_reserves(), returning None if the call returned an
/// error.
fn parse_reserves_result(tokens: Vec<Token>) -> Result<Option<Reserves>> {
    type ReservesResult = (bool, Reserves);
    let res: Option<ReservesResult> = parse_result(tokens)?;

    match res {
        Some(res) => {
            if res.0 {
                Ok(Some(res.1))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused)]
    use crate::ProtocolType;
    #[allow(unused)]
    use ethers_providers::{Http, Provider, MAINNET};

    #[cfg(feature = "addresses")]
    fn default_pair() -> Pair<Provider<Http>> {
        let chain = Chain::Mainnet;
        let weth: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap();
        let usdc: Address = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap();
        let provider = MAINNET.provider();
        let client = Arc::new(provider);
        let factory = Factory::new_with_chain(client, chain, ProtocolType::UniswapV2).unwrap();

        Pair::new_with_factory(&factory, weth, usdc).unwrap()
    }

    #[test]
    fn test_parsing() {
        let addresses = (Address::random(), Address::random());
        // let tokens = vec![Token::Address(addresses.0), Token::Address(addresses.1)];
        let reserve_uints = (69u128, 420u128, 1337u32);
        let reserves = vec![
            Token::Uint(reserve_uints.0.into()),
            Token::Uint(reserve_uints.1.into()),
            Token::Uint(reserve_uints.2.into()),
        ];
        let error_message = "Error message".to_string();

        type SuccessResult = ((bool, Address), (bool, Address), (bool, Reserves));
        let success_result: SuccessResult =
            ((true, addresses.0), (true, addresses.1), (true, reserve_uints));
        let success_tokens = vec![
            Token::Tuple(vec![Token::Bool(true), Token::Address(addresses.0)]),
            Token::Tuple(vec![Token::Bool(true), Token::Address(addresses.1)]),
            Token::Tuple(vec![Token::Bool(true), Token::Tuple(reserves)]),
        ];
        type FailureResult = ((bool, String), (bool, String), (bool, String));
        let failure_result: FailureResult = (
            (false, error_message.clone()),
            (false, error_message.clone()),
            (false, error_message.clone()),
        );
        let failure_tokens = vec![
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
            Token::Tuple(vec![Token::Bool(false), Token::String(error_message.clone())]),
        ];

        // parse_errors

        let errors = parse_errors(success_tokens.clone());
        assert_eq!(errors.len(), 3);
        for e in errors {
            assert!(e.is_none());
        }

        let errors = parse_errors(failure_tokens.clone());
        assert_eq!(errors.len(), 3);
        for e in errors {
            assert_eq!(e.unwrap(), error_message.clone());
        }

        // parse_result

        let result = parse_result::<SuccessResult>(success_tokens.clone()).unwrap();
        assert_eq!(result.unwrap(), success_result);

        let result = parse_result::<FailureResult>(failure_tokens.clone()).unwrap();
        assert_eq!(result.unwrap(), failure_result);

        // parse_tokens_result

        let result = parse_tokens_result(success_tokens[0..2].to_vec()).unwrap();
        assert_eq!(result.unwrap(), addresses);

        let result = parse_tokens_result(failure_tokens.clone());
        assert!(result.unwrap().is_none());

        // parse_reserves_result

        let result = parse_reserves_result(success_tokens[2..].to_vec()).unwrap();
        assert_eq!(result.unwrap(), reserve_uints);

        let result = parse_reserves_result(failure_tokens);
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    #[ignore = "async test"]
    #[cfg(feature = "addresses")]
    async fn test_sync() {
        let mut pair = default_pair();

        assert!(!pair.deployed());
        let tokens = pair.tokens().unwrap();
        assert_ne!(tokens.0, Address::zero());
        assert_ne!(tokens.1, Address::zero());

        pair.sync(true, false).await.unwrap();
        assert!(pair.deployed());
        assert!(pair.reserves().is_none());

        pair.sync(false, true).await.unwrap();
        let reserves = pair.reserves().unwrap();
        assert!(pair.deployed());
        assert_ne!(reserves.0, 0);
        assert_ne!(reserves.1, 0);
        assert_ne!(reserves.2, 0);
    }
}

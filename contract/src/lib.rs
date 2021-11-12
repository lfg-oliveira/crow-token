/**
This is a implementation of the DIYnamo(DIYM) fungible token (FT) in the NEAR Protocol blockchain.
This FT is ERC20 compliant. Therefore it implements the following methos:

function name() public view returns (string)
function symbol() public view returns (string)
function decimals() public view returns (uint8)
function totalSupply() public view returns (uint256)
**/

use near_contract_standards::fungible_token::metadata::{FT_METADATA_SPEC,FungibleTokenMetadata,FungibleTokenMetadataProvider};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize, self};
use near_sdk::collections::{LazyOption};
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract{
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey{
    FungibleToken,
    Metadata
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId, total_supply: U128) -> Self{
        Self::new(
            owner_id,
            total_supply,
            FungibleTokenMetadata{
                spec: FT_METADATA_SPEC.to_string(),
                name: "Crow Token".to_string(),
                symbol: "CROW".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 18
            }
        )
    }
    #[init]
    pub fn new(owner_id: AccountId, total_supply: U128, metadata: FungibleTokenMetadata) -> Self{
        assert!(!env::state_exists(), "Already exists");
        metadata.assert_valid();
        let mut this = Self {
            token: FungibleToken::new(StorageKey::FungibleToken),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata))
        };
        this.token.internal_register_account(&owner_id);
        this.token.internal_deposit(&owner_id, total_supply.into());
        this
    }

    pub fn name(&self) -> String {
        self.metadata.get().unwrap().name
    }

    pub fn symbol(&self) -> String{
        self.metadata.get().unwrap().symbol
    }

    pub fn decimals(&self) -> u8 {
        self.metadata.get().unwrap().decimals
    }

    pub fn total_supply(&self) -> u128 {
        self.token.total_supply
    }

    fn on_account_closed(account_id: AccountId, balance: Balance){
        log!("Closed @{} with balance {}", account_id, balance);
    }

    fn on_tokens_burned(account_id: AccountId, amount: Balance){
        log!("Account @{} burned {} tokens", account_id, amount);
    }
}


#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract{
    fn ft_metadata(&self) -> FungibleTokenMetadata{
        self.metadata.get().unwrap()
    }
}


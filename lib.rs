#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod digital_certificate {
    use ink::storage::Mapping;
    use ink::{prelude::vec::Vec};
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct DigitalCertificate {
        issuer: AccountId,
        certificate_authority: Vec<u8>,
        token_uri: Vec<u8>,
        token_owner: Mapping<TokenId, AccountId>,
        candidate_name: Vec<u8>,
        expiration_date: Vec<u8>,
    }
 
    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        OnlyAdminCanIssue,
        TokenExists,
        NotAllowed,
        OnlyAdminCanCheck,
        TokenNotFound,
    }

    pub type TokenId = u32;

    #[ink(event)]
    pub struct Issue {
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: TokenId,
    }

    impl DigitalCertificate {
        #[ink(constructor)]
        pub fn new(issuer: AccountId, certificate_authority: Vec<u8>) -> Self {
            Self {
                issuer,
                certificate_authority,
                token_uri: Default::default(),
                token_owner: Default::default(),
                candidate_name: Default::default(),
                expiration_date: Default::default(),
            }
        }

        #[ink(message)]
        pub fn mint(
            &mut self,
            id: TokenId,
            token_uri: Vec<u8>,
            token_owner: AccountId,
            expiration_date: Vec<u8>,
            candidate_name: Vec<u8>,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if caller != self.issuer {
                return Err(Error::OnlyAdminCanIssue);
            }

            self.token_uri = token_uri;
            self.expiration_date = expiration_date;
            self.candidate_name = candidate_name;

            self.add_token_to(&token_owner, id)?;
            self.env().emit_event(Issue {
                to: Some(token_owner),
                id,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owner.get(&id).clone()
        }

        #[ink(message)]
        pub fn burn(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let owner = self.token_owner.get(&id).ok_or(Error::TokenNotFound)?;
            if owner != caller {
                return Err(Error::NotOwner);
            }

            self.token_owner.take(&id);

            Ok(())
        }

        fn add_token_to(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            if self.token_owner.contains(&id) {
                return Err(Error::TokenExists);
            }
            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed);
            }

            self.token_owner.insert(id, to);

            Ok(())
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "ink-experimental-engine")]
    use crate::digital_certificate::DigitalCertificate;
    fn random_account_id() -> AccountId {
        AccountId::from([0x42; 32])
    }

    #[test]
    fn test_new() {
        let accounts = 
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        let s: &[u8] = "antiersolutions".as_bytes();
        let i = s.to_owned();
        let certificate_authority = i;
        let contract = DigitalCertificate::new(accounts.alice, certificate_authority.clone());
        
        assert_eq!(contract.issuer, accounts.alice);
        assert_eq!(contract.certificate_authority, certificate_authority);
    }

    #[test]
    fn test_mint_certificate() {
        let s: &[u8] = "antiersolutions".as_bytes();
        let i = s.to_owned();
        let certificate_authority = i;
        let accounts = 
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        let mut contract = DigitalCertificate::new(accounts.alice, certificate_authority);
        let token_id = 1;
        let s: &[u8] = "qwerty.com".as_bytes();
        let i = s.to_owned();
        let token_uri = i;
        let token_owner = accounts.bob;
        let s: &[u8] = "23092030".as_bytes();
        let i = s.to_owned();
        let expiration_date = i;
        let s: &[u8] = "JohnDoe".as_bytes();
        let i = s.to_owned();
        let candidate_name = i;

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        ink::env::test::set_callee::<ink::env::DefaultEnvironment>(accounts.bob);
        
        let result = contract.mint(token_id, token_uri, token_owner.clone(), expiration_date, candidate_name);
        assert_eq!(result, Ok(()));
        assert_eq!(contract.owner_of(1), Some(token_owner));
    }

    #[test]
    fn test_mint_certificate_only_admin_can_issue() { 

        // Arrange
        let s: &[u8] = "antiersolutions".as_bytes();
        let i = s.to_owned();
        let certificate_authority = i;
        let accounts = 
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        let mut contract = DigitalCertificate::new(accounts.alice, certificate_authority);        let mut contract = DigitalCertificate::new(random_account_id().clone(), vec![1, 2, 3]);
        let token_id = 1;
        let s: &[u8] = "qwerty.com".as_bytes();
        let i = s.to_owned();
        let token_uri = i;
        let token_owner = accounts.bob;
        let s: &[u8] = "23092030".as_bytes();
        let i = s.to_owned();
        let expiration_date = i;
        let s: &[u8] = "JohnDoe".as_bytes();
        let i = s.to_owned();
        let candidate_name = i;

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        ink::env::test::set_callee::<ink::env::DefaultEnvironment>(accounts.bob);
        
        let result = contract.mint(token_id, token_uri, token_owner.clone(), expiration_date, candidate_name);

        // Assert
        assert_eq!(result, Err(Error::OnlyAdminCanIssue));
        assert_eq!(contract.owner_of(1), None);
    }

    #[test]
    fn test_burn_certificate() {
    
        let s: &[u8] = "antiersolutions".as_bytes();
        let i = s.to_owned();
        let certificate_authority = i;
        let accounts = 
        ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        let contract = DigitalCertificate::new(accounts.alice, certificate_authority);        let mut contract = DigitalCertificate::new(random_account_id().clone(), vec![1, 2, 3]);
        let token_id = 1;
        let s: &[u8] = "qwerty.com".as_bytes();
        let i = s.to_owned();
        let token_uri = i;
        let token_owner = accounts.bob;
        let s: &[u8] = "23092030".as_bytes();
        let i = s.to_owned();
        let expiration_date = i;
        let s: &[u8] = "JohnDoe".as_bytes();
        let i = s.to_owned();
        let candidate_name = i;

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        ink::env::test::set_callee::<ink::env::DefaultEnvironment>(accounts.bob);
        
        let result = contract.mint(token_id, token_uri, token_owner.clone(), expiration_date, candidate_name);
        let result = contract.burn(1);
        // Assert
        assert_eq!(contract.owner_of(1), None);
    }

}

}


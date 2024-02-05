pub use cpv::*;
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
pub mod cpv {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"Opercent\",\"type\":\"uint8\"},{\"internalType\":\"uint8\",\"name\":\"Cpercent\",\"type\":\"uint8\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"ERC721EnumerableForbiddenBatchMint\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"ERC721IncorrectOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ERC721InsufficientApproval\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"approver\",\"type\":\"address\"}],\"name\":\"ERC721InvalidApprover\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"ERC721InvalidOperator\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"ERC721InvalidOwner\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\"}],\"name\":\"ERC721InvalidReceiver\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"sender\",\"type\":\"address\"}],\"name\":\"ERC721InvalidSender\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ERC721NonexistentToken\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"ERC721OutOfBoundsIndex\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Approval\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"ApprovalForAll\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"_fromTokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"_toTokenId\",\"type\":\"uint256\"}],\"name\":\"BatchMetadataUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"_tokenId\",\"type\":\"uint256\"}],\"name\":\"MetadataUpdate\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"Old_Id\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"New_Id\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"Old_Time\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"New_Time\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint8\",\"name\":\"Old_Level\",\"type\":\"uint8\"},{\"indexed\":true,\"internalType\":\"uint8\",\"name\":\"New_Level\",\"type\":\"uint8\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"discord_id\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"role\",\"type\":\"uint256\"}],\"name\":\"New_NFT\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"enter_price\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"request_price\",\"type\":\"uint256\"}],\"name\":\"Price_equal\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"Transfer\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"add_addr\",\"type\":\"address\"}],\"name\":\"AddParceiros\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"}],\"name\":\"Creator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"}],\"name\":\"DelParceiros\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"NFTData\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"expired\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"cargo\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"DiscordId\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"level\",\"type\":\"uint8\"}],\"internalType\":\"struct NFTInfo\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"ParceirosData\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"}],\"name\":\"Plans\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"cargo\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"daysTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"level\",\"type\":\"uint8\"},{\"internalType\":\"uint8\",\"name\":\"acesses\",\"type\":\"uint8\"}],\"internalType\":\"struct Planos[]\",\"name\":\"\",\"type\":\"tuple[]\"},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"True_supply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"cargo\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"daysTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"uint8\",\"name\":\"level\",\"type\":\"uint8\"},{\"internalType\":\"uint8\",\"name\":\"acesses\",\"type\":\"uint8\"}],\"internalType\":\"struct Planos\",\"name\":\"plan\",\"type\":\"tuple\"},{\"internalType\":\"uint256\",\"name\":\"total\",\"type\":\"uint256\"}],\"name\":\"ads\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"}],\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"dustWithdraw\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"}],\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"level\",\"type\":\"uint8\"},{\"internalType\":\"string\",\"name\":\"metadata\",\"type\":\"string\"}],\"name\":\"metadataByLevel\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"level\",\"type\":\"uint8\"}],\"name\":\"metadataOfLevel\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"tokenid\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"discord_id\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"mint\",\"outputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"creator\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"i\",\"type\":\"uint256\"}],\"name\":\"rmv\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\"}],\"name\":\"safeTransferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\"}],\"name\":\"setApprovalForAll\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\"}],\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"}],\"name\":\"tokenOfOwnerByIndex\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"transferFrom\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]\n";
    ///The parsed JSON ABI of the contract.
    pub static CPV_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct cpv<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for cpv<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for cpv<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for cpv<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for cpv<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(cpv)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> cpv<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CPV_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `AddParceiros` (0x5b223828) function
        pub fn add_parceiros(
            &self,
            creator: ::std::string::String,
            add_addr: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([91, 34, 56, 40], (creator, add_addr))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `Creator` (0x4779b65a) function
        pub fn creator(
            &self,
            creator: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([71, 121, 182, 90], creator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DelParceiros` (0x865cdaf0) function
        pub fn del_parceiros(
            &self,
            creator: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([134, 92, 218, 240], creator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NFTData` (0x93607e13) function
        pub fn nft_data(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Nftinfo> {
            self.0
                .method_hash([147, 96, 126, 19], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ParceirosData` (0x36e57cdf) function
        pub fn parceiros_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([54, 229, 124, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `Plans` (0xf52c9a1b) function
        pub fn plans(
            &self,
            creator: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<Planos>, ::std::vec::Vec<::ethers::core::types::U256>),
        > {
            self.0
                .method_hash([245, 44, 154, 27], creator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `True_supply` (0x9e43fa05) function
        pub fn true_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([158, 67, 250, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ads` (0x9bcd12a0) function
        pub fn ads(
            &self,
            creator: ::std::string::String,
            plan: Planos,
            total: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 205, 18, 160], (creator, plan, total))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dustWithdraw` (0xdb631dc3) function
        pub fn dust_withdraw(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 99, 29, 195], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `metadataByLevel` (0x954209f1) function
        pub fn metadata_by_level(
            &self,
            level: u8,
            metadata: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 66, 9, 241], (level, metadata))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `metadataOfLevel` (0x2821ee1c) function
        pub fn metadata_of_level(
            &self,
            level: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([40, 33, 238, 28], level)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x48bc17bf) function
        pub fn mint(
            &self,
            to: ::ethers::core::types::Address,
            creator: ::std::string::String,
            tokenid: ::ethers::core::types::U256,
            discord_id: ::ethers::core::types::U256,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 188, 23, 191], (to, creator, tokenid, discord_id, i))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rmv` (0x7a537d87) function
        pub fn rmv(
            &self,
            creator: ::std::string::String,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 83, 125, 135], (creator, i))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenByIndex` (0x4f6ccce7) function
        pub fn token_by_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function
        pub fn token_of_owner_by_index(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `BatchMetadataUpdate` event
        pub fn batch_metadata_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BatchMetadataUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MetadataUpdate` event
        pub fn metadata_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MetadataUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `New_NFT` event
        pub fn new_nft_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewNFTFilter> {
            self.0.event()
        }
        ///Gets the contract's `Price_equal` event
        pub fn price_equal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceEqualFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, cpvEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for cpv<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC721EnumerableForbiddenBatchMint` with signature `ERC721EnumerableForbiddenBatchMint()` and selector `0x59171fc1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721EnumerableForbiddenBatchMint",
        abi = "ERC721EnumerableForbiddenBatchMint()"
    )]
    pub struct ERC721EnumerableForbiddenBatchMint;
    ///Custom Error type `ERC721IncorrectOwner` with signature `ERC721IncorrectOwner(address,uint256,address)` and selector `0x64283d7b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721IncorrectOwner",
        abi = "ERC721IncorrectOwner(address,uint256,address)"
    )]
    pub struct ERC721IncorrectOwner {
        pub sender: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InsufficientApproval` with signature `ERC721InsufficientApproval(address,uint256)` and selector `0x177e802f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721InsufficientApproval",
        abi = "ERC721InsufficientApproval(address,uint256)"
    )]
    pub struct ERC721InsufficientApproval {
        pub operator: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC721InvalidApprover` with signature `ERC721InvalidApprover(address)` and selector `0xa9fbf51f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidApprover", abi = "ERC721InvalidApprover(address)")]
    pub struct ERC721InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOperator` with signature `ERC721InvalidOperator(address)` and selector `0x5b08ba18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidOperator", abi = "ERC721InvalidOperator(address)")]
    pub struct ERC721InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidOwner` with signature `ERC721InvalidOwner(address)` and selector `0x89c62b64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidOwner", abi = "ERC721InvalidOwner(address)")]
    pub struct ERC721InvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidReceiver` with signature `ERC721InvalidReceiver(address)` and selector `0x64a0ae92`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidReceiver", abi = "ERC721InvalidReceiver(address)")]
    pub struct ERC721InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721InvalidSender` with signature `ERC721InvalidSender(address)` and selector `0x73c6ac6e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721InvalidSender", abi = "ERC721InvalidSender(address)")]
    pub struct ERC721InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC721NonexistentToken` with signature `ERC721NonexistentToken(uint256)` and selector `0x7e273289`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC721NonexistentToken", abi = "ERC721NonexistentToken(uint256)")]
    pub struct ERC721NonexistentToken {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC721OutOfBoundsIndex` with signature `ERC721OutOfBoundsIndex(address,uint256)` and selector `0xa57d13dc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC721OutOfBoundsIndex",
        abi = "ERC721OutOfBoundsIndex(address,uint256)"
    )]
    pub struct ERC721OutOfBoundsIndex {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum cpvErrors {
        ERC721EnumerableForbiddenBatchMint(ERC721EnumerableForbiddenBatchMint),
        ERC721IncorrectOwner(ERC721IncorrectOwner),
        ERC721InsufficientApproval(ERC721InsufficientApproval),
        ERC721InvalidApprover(ERC721InvalidApprover),
        ERC721InvalidOperator(ERC721InvalidOperator),
        ERC721InvalidOwner(ERC721InvalidOwner),
        ERC721InvalidReceiver(ERC721InvalidReceiver),
        ERC721InvalidSender(ERC721InvalidSender),
        ERC721NonexistentToken(ERC721NonexistentToken),
        ERC721OutOfBoundsIndex(ERC721OutOfBoundsIndex),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for cpvErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <ERC721EnumerableForbiddenBatchMint as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721EnumerableForbiddenBatchMint(decoded));
            }
            if let Ok(decoded)
                = <ERC721IncorrectOwner as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721IncorrectOwner(decoded));
            }
            if let Ok(decoded)
                = <ERC721InsufficientApproval as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721InsufficientApproval(decoded));
            }
            if let Ok(decoded)
                = <ERC721InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721InvalidApprover(decoded));
            }
            if let Ok(decoded)
                = <ERC721InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721InvalidOperator(decoded));
            }
            if let Ok(decoded)
                = <ERC721InvalidOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC721InvalidOwner(decoded));
            }
            if let Ok(decoded)
                = <ERC721InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721InvalidReceiver(decoded));
            }
            if let Ok(decoded)
                = <ERC721InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ERC721InvalidSender(decoded));
            }
            if let Ok(decoded)
                = <ERC721NonexistentToken as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721NonexistentToken(decoded));
            }
            if let Ok(decoded)
                = <ERC721OutOfBoundsIndex as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC721OutOfBoundsIndex(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for cpvErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC721EnumerableForbiddenBatchMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721IncorrectOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721NonexistentToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC721OutOfBoundsIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for cpvErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC721EnumerableForbiddenBatchMint as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721IncorrectOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InsufficientApproval as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidApprover as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOperator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidReceiver as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721NonexistentToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC721OutOfBoundsIndex as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for cpvErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC721EnumerableForbiddenBatchMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721IncorrectOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InsufficientApproval(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidApprover(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOperator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidReceiver(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721InvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721NonexistentToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC721OutOfBoundsIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for cpvErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC721EnumerableForbiddenBatchMint> for cpvErrors {
        fn from(value: ERC721EnumerableForbiddenBatchMint) -> Self {
            Self::ERC721EnumerableForbiddenBatchMint(value)
        }
    }
    impl ::core::convert::From<ERC721IncorrectOwner> for cpvErrors {
        fn from(value: ERC721IncorrectOwner) -> Self {
            Self::ERC721IncorrectOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InsufficientApproval> for cpvErrors {
        fn from(value: ERC721InsufficientApproval) -> Self {
            Self::ERC721InsufficientApproval(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidApprover> for cpvErrors {
        fn from(value: ERC721InvalidApprover) -> Self {
            Self::ERC721InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOperator> for cpvErrors {
        fn from(value: ERC721InvalidOperator) -> Self {
            Self::ERC721InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidOwner> for cpvErrors {
        fn from(value: ERC721InvalidOwner) -> Self {
            Self::ERC721InvalidOwner(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidReceiver> for cpvErrors {
        fn from(value: ERC721InvalidReceiver) -> Self {
            Self::ERC721InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC721InvalidSender> for cpvErrors {
        fn from(value: ERC721InvalidSender) -> Self {
            Self::ERC721InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC721NonexistentToken> for cpvErrors {
        fn from(value: ERC721NonexistentToken) -> Self {
            Self::ERC721NonexistentToken(value)
        }
    }
    impl ::core::convert::From<ERC721OutOfBoundsIndex> for cpvErrors {
        fn from(value: ERC721OutOfBoundsIndex) -> Self {
            Self::ERC721OutOfBoundsIndex(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "BatchMetadataUpdate",
        abi = "BatchMetadataUpdate(uint256,uint256)"
    )]
    pub struct BatchMetadataUpdateFilter {
        pub from_token_id: ::ethers::core::types::U256,
        pub to_token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "MetadataUpdate", abi = "MetadataUpdate(uint256)")]
    pub struct MetadataUpdateFilter {
        pub token_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "New_NFT",
        abi = "New_NFT(uint256,uint256,uint256,uint256,uint8,uint8,uint256,uint256)"
    )]
    pub struct NewNFTFilter {
        pub old_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_id: ::ethers::core::types::U256,
        pub old_time: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_time: ::ethers::core::types::U256,
        pub old_level: u8,
        #[ethevent(indexed)]
        pub new_level: u8,
        pub discord_id: ::ethers::core::types::U256,
        pub role: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Price_equal", abi = "Price_equal(uint256,uint256)")]
    pub struct PriceEqualFilter {
        #[ethevent(indexed)]
        pub enter_price: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub request_price: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum cpvEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        BatchMetadataUpdateFilter(BatchMetadataUpdateFilter),
        MetadataUpdateFilter(MetadataUpdateFilter),
        NewNFTFilter(NewNFTFilter),
        PriceEqualFilter(PriceEqualFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for cpvEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(cpvEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(cpvEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = BatchMetadataUpdateFilter::decode_log(log) {
                return Ok(cpvEvents::BatchMetadataUpdateFilter(decoded));
            }
            if let Ok(decoded) = MetadataUpdateFilter::decode_log(log) {
                return Ok(cpvEvents::MetadataUpdateFilter(decoded));
            }
            if let Ok(decoded) = NewNFTFilter::decode_log(log) {
                return Ok(cpvEvents::NewNFTFilter(decoded));
            }
            if let Ok(decoded) = PriceEqualFilter::decode_log(log) {
                return Ok(cpvEvents::PriceEqualFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(cpvEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for cpvEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BatchMetadataUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MetadataUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewNFTFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceEqualFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for cpvEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for cpvEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<BatchMetadataUpdateFilter> for cpvEvents {
        fn from(value: BatchMetadataUpdateFilter) -> Self {
            Self::BatchMetadataUpdateFilter(value)
        }
    }
    impl ::core::convert::From<MetadataUpdateFilter> for cpvEvents {
        fn from(value: MetadataUpdateFilter) -> Self {
            Self::MetadataUpdateFilter(value)
        }
    }
    impl ::core::convert::From<NewNFTFilter> for cpvEvents {
        fn from(value: NewNFTFilter) -> Self {
            Self::NewNFTFilter(value)
        }
    }
    impl ::core::convert::From<PriceEqualFilter> for cpvEvents {
        fn from(value: PriceEqualFilter) -> Self {
            Self::PriceEqualFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for cpvEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `AddParceiros` function with signature `AddParceiros(string,address)` and selector `0x5b223828`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "AddParceiros", abi = "AddParceiros(string,address)")]
    pub struct AddParceirosCall {
        pub creator: ::std::string::String,
        pub add_addr: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `Creator` function with signature `Creator(string)` and selector `0x4779b65a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "Creator", abi = "Creator(string)")]
    pub struct CreatorCall {
        pub creator: ::std::string::String,
    }
    ///Container type for all input parameters for the `DelParceiros` function with signature `DelParceiros(string)` and selector `0x865cdaf0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DelParceiros", abi = "DelParceiros(string)")]
    pub struct DelParceirosCall {
        pub creator: ::std::string::String,
    }
    ///Container type for all input parameters for the `NFTData` function with signature `NFTData(uint256)` and selector `0x93607e13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "NFTData", abi = "NFTData(uint256)")]
    pub struct NftdataCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `ParceirosData` function with signature `ParceirosData()` and selector `0x36e57cdf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ParceirosData", abi = "ParceirosData()")]
    pub struct ParceirosDataCall;
    ///Container type for all input parameters for the `Plans` function with signature `Plans(string)` and selector `0xf52c9a1b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "Plans", abi = "Plans(string)")]
    pub struct PlansCall {
        pub creator: ::std::string::String,
    }
    ///Container type for all input parameters for the `True_supply` function with signature `True_supply()` and selector `0x9e43fa05`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "True_supply", abi = "True_supply()")]
    pub struct TrueSupplyCall;
    ///Container type for all input parameters for the `ads` function with signature `ads(string,(uint256,uint256,uint256,uint8,uint8),uint256)` and selector `0x9bcd12a0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "ads",
        abi = "ads(string,(uint256,uint256,uint256,uint8,uint8),uint256)"
    )]
    pub struct AdsCall {
        pub creator: ::std::string::String,
        pub plan: Planos,
        pub total: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `dustWithdraw` function with signature `dustWithdraw()` and selector `0xdb631dc3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dustWithdraw", abi = "dustWithdraw()")]
    pub struct DustWithdrawCall;
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `metadataByLevel` function with signature `metadataByLevel(uint8,string)` and selector `0x954209f1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "metadataByLevel", abi = "metadataByLevel(uint8,string)")]
    pub struct MetadataByLevelCall {
        pub level: u8,
        pub metadata: ::std::string::String,
    }
    ///Container type for all input parameters for the `metadataOfLevel` function with signature `metadataOfLevel(uint8)` and selector `0x2821ee1c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "metadataOfLevel", abi = "metadataOfLevel(uint8)")]
    pub struct MetadataOfLevelCall {
        pub level: u8,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,string,uint256,uint256,uint256)` and selector `0x48bc17bf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mint", abi = "mint(address,string,uint256,uint256,uint256)")]
    pub struct MintCall {
        pub to: ::ethers::core::types::Address,
        pub creator: ::std::string::String,
        pub tokenid: ::ethers::core::types::U256,
        pub discord_id: ::ethers::core::types::U256,
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rmv` function with signature `rmv(string,uint256)` and selector `0x7a537d87`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rmv", abi = "rmv(string,uint256)")]
    pub struct RmvCall {
        pub creator: ::std::string::String,
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum cpvCalls {
        AddParceiros(AddParceirosCall),
        Creator(CreatorCall),
        DelParceiros(DelParceirosCall),
        Nftdata(NftdataCall),
        ParceirosData(ParceirosDataCall),
        Plans(PlansCall),
        TrueSupply(TrueSupplyCall),
        Ads(AdsCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        DustWithdraw(DustWithdrawCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MetadataByLevel(MetadataByLevelCall),
        MetadataOfLevel(MetadataOfLevelCall),
        Mint(MintCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        Rmv(RmvCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for cpvCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddParceirosCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddParceiros(decoded));
            }
            if let Ok(decoded)
                = <CreatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Creator(decoded));
            }
            if let Ok(decoded)
                = <DelParceirosCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelParceiros(decoded));
            }
            if let Ok(decoded)
                = <NftdataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nftdata(decoded));
            }
            if let Ok(decoded)
                = <ParceirosDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ParceirosData(decoded));
            }
            if let Ok(decoded)
                = <PlansCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Plans(decoded));
            }
            if let Ok(decoded)
                = <TrueSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TrueSupply(decoded));
            }
            if let Ok(decoded)
                = <AdsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ads(decoded));
            }
            if let Ok(decoded)
                = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded)
                = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded)
                = <DustWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DustWithdraw(decoded));
            }
            if let Ok(decoded)
                = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded)
                = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded)
                = <MetadataByLevelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MetadataByLevel(decoded));
            }
            if let Ok(decoded)
                = <MetadataOfLevelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MetadataOfLevel(decoded));
            }
            if let Ok(decoded)
                = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded)
                = <RmvCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rmv(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded)
                = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded)
                = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded)
                = <TokenByIndexCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenOfOwnerByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded)
                = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded)
                = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded)
                = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for cpvCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddParceiros(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Creator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DelParceiros(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nftdata(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ParceirosData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Plans(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrueSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ads(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DustWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MetadataByLevel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MetadataOfLevel(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Rmv(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for cpvCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddParceiros(element) => ::core::fmt::Display::fmt(element, f),
                Self::Creator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelParceiros(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nftdata(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParceirosData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Plans(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrueSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ads(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::DustWithdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetadataByLevel(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetadataOfLevel(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rmv(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddParceirosCall> for cpvCalls {
        fn from(value: AddParceirosCall) -> Self {
            Self::AddParceiros(value)
        }
    }
    impl ::core::convert::From<CreatorCall> for cpvCalls {
        fn from(value: CreatorCall) -> Self {
            Self::Creator(value)
        }
    }
    impl ::core::convert::From<DelParceirosCall> for cpvCalls {
        fn from(value: DelParceirosCall) -> Self {
            Self::DelParceiros(value)
        }
    }
    impl ::core::convert::From<NftdataCall> for cpvCalls {
        fn from(value: NftdataCall) -> Self {
            Self::Nftdata(value)
        }
    }
    impl ::core::convert::From<ParceirosDataCall> for cpvCalls {
        fn from(value: ParceirosDataCall) -> Self {
            Self::ParceirosData(value)
        }
    }
    impl ::core::convert::From<PlansCall> for cpvCalls {
        fn from(value: PlansCall) -> Self {
            Self::Plans(value)
        }
    }
    impl ::core::convert::From<TrueSupplyCall> for cpvCalls {
        fn from(value: TrueSupplyCall) -> Self {
            Self::TrueSupply(value)
        }
    }
    impl ::core::convert::From<AdsCall> for cpvCalls {
        fn from(value: AdsCall) -> Self {
            Self::Ads(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for cpvCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for cpvCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DustWithdrawCall> for cpvCalls {
        fn from(value: DustWithdrawCall) -> Self {
            Self::DustWithdraw(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for cpvCalls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for cpvCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MetadataByLevelCall> for cpvCalls {
        fn from(value: MetadataByLevelCall) -> Self {
            Self::MetadataByLevel(value)
        }
    }
    impl ::core::convert::From<MetadataOfLevelCall> for cpvCalls {
        fn from(value: MetadataOfLevelCall) -> Self {
            Self::MetadataOfLevel(value)
        }
    }
    impl ::core::convert::From<MintCall> for cpvCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<NameCall> for cpvCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for cpvCalls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<RmvCall> for cpvCalls {
        fn from(value: RmvCall) -> Self {
            Self::Rmv(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for cpvCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall> for cpvCalls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for cpvCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for cpvCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for cpvCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for cpvCalls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for cpvCalls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for cpvCalls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for cpvCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for cpvCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for cpvCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `Creator` function with signature `Creator(string)` and selector `0x4779b65a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `NFTData` function with signature `NFTData(uint256)` and selector `0x93607e13`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NftdataReturn(pub Nftinfo);
    ///Container type for all return fields from the `ParceirosData` function with signature `ParceirosData()` and selector `0x36e57cdf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParceirosDataReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `Plans` function with signature `Plans(string)` and selector `0xf52c9a1b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PlansReturn(
        pub ::std::vec::Vec<Planos>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
    );
    ///Container type for all return fields from the `True_supply` function with signature `True_supply()` and selector `0x9e43fa05`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TrueSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `metadataOfLevel` function with signature `metadataOfLevel(uint8)` and selector `0x2821ee1c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MetadataOfLevelReturn(pub ::std::string::String);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerOfReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenOfOwnerByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TokenURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///`Nftinfo(uint256,uint256,uint256,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Nftinfo {
        pub expired: ::ethers::core::types::U256,
        pub cargo: ::ethers::core::types::U256,
        pub discord_id: ::ethers::core::types::U256,
        pub level: u8,
    }
    ///`Planos(uint256,uint256,uint256,uint8,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Planos {
        pub cargo: ::ethers::core::types::U256,
        pub days_time: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub level: u8,
        pub acesses: u8,
    }
}

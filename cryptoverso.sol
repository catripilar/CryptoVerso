pragma solidity ^0.8.21;



struct Planos {
    uint256 cargo;
    uint256 daysTime;
    uint256 amount;
    uint8 level;
    uint8 acesses;
}
struct NFTInfo {
    uint256 expired;
    uint256 cargo;
    uint256 DiscordId;
    uint8 level;
}
contract CryptoVerso is ERC721URIStorage,ERC721Enumerable {
    uint true_supply = 0;
    address owner;
    address[] parceiros;
    uint8 owner_percent;
    uint8 criador_percent;
    
  //modifiers

    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can call this function");
        _;
    }

    modifier creator_exist(string calldata creator) {
        require(criador_addr[creator] != address(0),"creator not exist");
        _;
    }

//overrides

    function _update(address to, uint256 tokenId, address auth) internal virtual override(ERC721,ERC721Enumerable) returns (address) {
        return super._update(to,tokenId,auth);
    }

    function _increaseBalance(address account, uint128 amount) internal virtual override(ERC721, ERC721Enumerable) {
        return super._increaseBalance(account,amount);
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC721Enumerable, ERC721URIStorage) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function tokenURI(uint256 tokenId) public view 
    override(ERC721, ERC721URIStorage) returns (string memory){
        return super.tokenURI(tokenId);
    }

    // User / Creators Data view

    function True_supply() external view returns (uint){
        return true_supply;
    }

    function Plans(string calldata creator) creator_exist(creator) 
    external view returns (Planos[] memory,uint[] memory){
        return (criador_planos[creator],quantity[creator]);
    }

    function Creator(string calldata creator) 
    external view returns (address){
        if (bytes(creator).length == 0){
            return owner;
        }
        return criador_addr[creator];
    }

    function ParceirosData() external view returns (address[] memory) {
        return parceiros;
    }

    function NFTData(uint tokenId) external view returns (NFTInfo memory) {
        return nft_info[tokenId];
    }
///Owner functions
    
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Invalid address");
        owner = newOwner;
    }

    function dustWithdraw() public onlyOwner {
        if (address(this).balance > 0){
            payable(msg.sender).transfer(address(this).balance);
        }else{
            revert("no value to Withdraw.");
        }
    }

    function metadataByLevel(uint8 level,string calldata metadata) public onlyOwner{
        uri[level] = metadata;
    }

    function metadataOfLevel(uint8 level) external view returns(string memory){
       return uri[level];
    }
    //Add Del Parceiros

    function AddParceiros(string memory creator,address add_addr) onlyOwner external {
        
        require(criador_addr[creator] == address(0),"creator exist");
        criador_addr[creator] = add_addr;
        parceiros.push(add_addr);
    }

    function DelParceiros(string calldata creator) creator_exist(creator) onlyOwner external {
        
        for (uint256 i = 0; i < parceiros.length; i++) {
            
            if(criador_addr[creator] == parceiros[i]){

                delete criador_addr[creator];
                uint256 lastIdx = parceiros.length - 1;
                parceiros[i] = parceiros[lastIdx];
                parceiros.pop();

                if(criador_planos[creator].length > 0){

                    for (uint256 id = 0; id < criador_planos[creator].length; id++) {
                        criador_planos[creator].pop();
                        quantity[creator].pop();
                    }
                }

                delete criador_planos[creator];
                break;
            }
        }
    }
    
  //Add Remove Plans

  function ads(
      string calldata creator,
      Planos memory plan,
      uint total
  ) creator_exist(creator) external {
      require(owner == msg.sender || 
      criador_addr[creator] == msg.sender, "not owner..");
      criador_planos[creator].push(plan);
      quantity[creator].push(total);
  }

  function rmv(string calldata creator, uint256 i) creator_exist(creator) external {
      
      require(owner == msg.sender || criador_addr[creator] == msg.sender, "not owner..");
      require(i < criador_planos[creator].length, "Index out of bounds");

      uint256 lastIdx = criador_planos[creator].length - 1;
      criador_planos[creator][i] = criador_planos[creator][lastIdx];
      quantity[creator][i] = quantity[creator][lastIdx];
      criador_planos[creator].pop();
      quantity[creator].pop();
  }

  //NFT Work

  event New_NFT(
      uint Old_Id,
      uint indexed New_Id,
      uint Old_Time,
      uint indexed New_Time,
      uint8 Old_Level,
      uint8 indexed New_Level,
      uint discord_id,
      uint role
      );

event Price_equal(uint indexed enter_price,uint indexed request_price);

function mint(
        address to,
        string calldata creator,
        uint256 tokenid,
        uint256 discord_id,
        uint256 i

    ) creator_exist(creator) external payable {

        require(to != address(0), "address is 0..");
        require(discord_id != 0, "ID err is 0..");
        
        address recive = to;
        uint8 to_level = 0;

        if(tokenid != 0 || balanceOf(to) > 0){
            require(ownerOf(tokenid) == to,"invalid tokenId");
            to_level = nft_info[tokenid].level;
        }

        //Plans Info

        Planos storage action = criador_planos[creator][i];
        address plan_addr = criador_addr[creator];
        uint256 real_price = 0;
        uint8 plan_owners = action.acesses;

        require(plan_owners <= to_level,"user not have nft level");
        require(quantity[creator][i] > 0,"NFT sold out");
        quantity[creator][i]--;
        if (action.amount > 0) {
            real_price = action.amount;
            require(msg.value >= real_price, "insuficient value");

            //owner distribution

            uint256 ownerpart = (real_price/100) * owner_percent;
            uint256 plan_addr_part = (real_price/100) * criador_percent;

            if(parceiros.length-1 == 0){
                plan_addr_part = real_price - ownerpart;
            }
            
            if (payable(plan_addr).send(plan_addr_part) &&
            payable(owner).send(ownerpart)) {

                if(parceiros.length-1 > 0){
                    uint divide = (real_price-(plan_addr_part+ownerpart)) / (parceiros.length-1);
                    for (uint256 ir = 0; ir < parceiros.length; ir++) {

                        if(parceiros[ir] != plan_addr){
                            payable(parceiros[ir]).transfer(divide);
                        }
                    }
                }
            }else {
                revert("payable error");
            }
        }

        emit Price_equal(msg.value,real_price);

        uint256 plan_time = action.daysTime + block.timestamp;
        uint8 plan_level = action.level;
         string memory plan_uri = uri[plan_level];
        uint256 plan_cargo = action.cargo;
        uint tokenId = tokenid;
        uint256 id = discord_id;

        unchecked {
            true_supply++;
        }

        //level update

        if(plan_level <= to_level){
            plan_uri = tokenURI(tokenId);
        }else{
            to_level = plan_level;
        }

        //NFT data update and burn

        if(tokenId != 0){
            NFTInfo memory old_nft = nft_info[tokenId];
            uint256 the_timestamp = old_nft.expired;

            if(the_timestamp < block.timestamp){
                the_timestamp = block.timestamp;
            }
            plan_time = the_timestamp + action.daysTime;
            delete nft_info[tokenId];
            _burn(tokenId);
            emit New_NFT(
                tokenId,
                true_supply,
                old_nft.expired,
                plan_time,
                old_nft.level,
                to_level,
                id,
                plan_cargo
            );
        }else{
            emit New_NFT(0,true_supply,0,plan_time,0,to_level,id,plan_cargo);
        }

        //mint
        NFTInfo memory new_nft = NFTInfo(
            plan_time,
            plan_cargo,
            id,
            to_level
        );

        nft_info[true_supply] = new_nft;
        _setTokenURI(true_supply, plan_uri);
        _safeMint(recive, true_supply);
    }
}

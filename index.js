var balances = 0;
var conectado = false;
var free = false;
const abi = [{"inputs":[{"internalType":"address","name":"orl","type":"address"},{"internalType":"uint8","name":"percent","type":"uint8"}],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[],"name":"ERC721EnumerableForbiddenBatchMint","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721IncorrectOwner","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721InsufficientApproval","type":"error"},{"inputs":[{"internalType":"address","name":"approver","type":"address"}],"name":"ERC721InvalidApprover","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"}],"name":"ERC721InvalidOperator","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721InvalidOwner","type":"error"},{"inputs":[{"internalType":"address","name":"receiver","type":"address"}],"name":"ERC721InvalidReceiver","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"}],"name":"ERC721InvalidSender","type":"error"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721NonexistentToken","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"ERC721OutOfBoundsIndex","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"approved","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"operator","type":"address"},{"indexed":false,"internalType":"bool","name":"approved","type":"bool"}],"name":"ApprovalForAll","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"from","type":"address"},{"indexed":true,"internalType":"address","name":"to","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Transfer","type":"event"},{"inputs":[{"internalType":"address","name":"add_addr","type":"address"}],"name":"AddParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"i","type":"uint256"}],"name":"DelParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"NFTData","outputs":[{"components":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"expired","type":"uint256"},{"internalType":"uint256","name":"minted","type":"uint256"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"cashBack","type":"uint256"},{"internalType":"uint256","name":"DiscordId","type":"uint256"}],"internalType":"struct NFTInfo[]","name":"","type":"tuple[]"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ParceirosData","outputs":[{"internalType":"address[]","name":"","type":"address[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"Price","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"discord_id","type":"uint256"}],"name":"active","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"address","name":"creator_addr","type":"address"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"value","type":"uint256"},{"internalType":"uint256","name":"cashBack","type":"uint256"},{"internalType":"uint256","name":"time","type":"uint256"},{"internalType":"uint8","name":"decimal","type":"uint8"}],"name":"ads","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"approve","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"balanceOf","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"baseURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"burning","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"getApproved","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"address","name":"operator","type":"address"}],"name":"isApprovedForAll","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"discord_id","type":"uint256"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"mint","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ownerOf","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"plans","outputs":[{"components":[{"internalType":"address","name":"owner_p","type":"address"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"timestamp","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"uint256","name":"cashBack","type":"uint256"},{"internalType":"uint8","name":"decimal","type":"uint8"}],"internalType":"struct Planos[]","name":"","type":"tuple[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"rmv","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"bool","name":"approved","type":"bool"}],"name":"setApprovalForAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"_baseURI","type":"string"}],"name":"setBaseURI","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes4","name":"interfaceId","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenOfOwnerByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"tokenURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"transferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"}];
const contrato_addr = "0xc78851a8C20a7829617D4FeDaFe1A7284FdF847A";
async function conectar() {
    if (conectado == false){
        if (typeof window.ethereum === 'undefined') {
            alert('Por favor, instale o Metamask para continuar.');
            return;
        }
        if (typeof window.ethereum !== 'undefined') {
            await window.ethereum.request({ method: 'eth_requestAccounts' });
    
            const web3 = new Web3(window.ethereum);
            const redeAtual = await web3.eth.net.getId();
            if (redeAtual !== 80001/*137*/) {
                alert('Por favor, conecte-se à rede Polygon para continuar.');
                return;
            }
            const contas = await web3.eth.getAccounts();
            const infoParam = getURLParameter("info");
            balances = await web3.eth.getBalance(contas[0]);
            console.log(balances)
            const carteira_string = encurtarString(contas[0],10);
            document.querySelector('.botao').innerHTML = "Conectado: "+carteira_string;
            const contract = await new web3.eth.Contract(abi,contrato_addr);
            conectado = true;
            const baseURI = await contract.methods.baseURI().call();
            fetch(baseURI).then(response => response.json())
            .then(data => {
                document.getElementById("nftimage1").src = data.image
                document.getElementById("nftimage2").src = data.image
            })
            .catch(error => {
                console.error('Erro ao buscar o JSON:', error);
            });
            if (infoParam) {
                const planos = await contract.methods.plans(infoParam).call();
                document.getElementById("walletInput").value = contas[0];
                if (planos[0].owner_p == contas[0]){
                    free = true
                }
                document.getElementById("sec_compra").style.display = "block";
                for (var i = 0; i < planos.length; i++) {
                    const price = await contract.methods.Price(infoParam,i).call();
                    var novoLi = document.createElement("li");
                    novoLi.setAttribute('onclick', 'payable(this)');
                    novoLi.classList.add('submitBtn');
                    novoLi.setAttribute("data-info", `${infoParam}/${i}/${price}/${planos[i].timestamp/86400}`);
                    var texto = document.createTextNode(`${planos[i].timestamp/86400} Dia(s) de plano por ${transformarNumero(price,2)} MATIC`);
                    novoLi.appendChild(texto);
                    var precos = document.getElementById("precos");
                    precos.appendChild(novoLi);
                }
            } else {
                console.log("nao encontrado")
            }
        }
    }
}
async function payable(element) {
    const dataInfo = element.getAttribute("data-info").split("/");
    const carteira = document.getElementById("walletInput").value;
    var id_discord = document.getElementById("discordInput").value;
    if (id_discord == ""){id_discord = "0"}
    const criador = dataInfo[0];
    const plano = dataInfo[1];
    const price = dataInfo[2];
    const time = dataInfo[3];
    if (carteira != ""){
        if (conectado == true){
            const web3 = new Web3(window.ethereum);
            const contas = await web3.eth.getAccounts();
            const contract = await new web3.eth.Contract(abi,contrato_addr);
            if (free == true && time < 7){
                await contract.methods.mint(carteira,criador,id_discord,plano).send({from: contas[0]})
                .then(res => console.log('Success', res))
                .catch(err => console.log(err))  
            }else{
                await contract.methods.mint(carteira,criador,id_discord,plano).send({from: contas[0],value: price})
                .then(res => console.log('Success', res))
                .catch(err => console.log(err))  
            }
        }
    }
}
function encurtarString(str, comprimentoMaximo) {
    if (str.length <= comprimentoMaximo) {
        return str;
    } else {
        const parteInicio = str.slice(0, comprimentoMaximo - 3);
        return parteInicio + '...';
    }
}
function transformarNumero(numero,casasDecimais) {
    const decimal = numero / Math.pow(10, 18);
    const numeroFormatado = decimal.toFixed(18);
    return parseFloat(numeroFormatado).toFixed(casasDecimais);
}
function getURLParameter(name) {
    const urlParams = new URLSearchParams(window.location.search);
    return urlParams.get(name);
}
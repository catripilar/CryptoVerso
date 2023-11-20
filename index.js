const abi = [{"inputs":[{"internalType":"uint8","name":"Opercent","type":"uint8"},{"internalType":"uint8","name":"Cpercent","type":"uint8"}],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[],"name":"ERC721EnumerableForbiddenBatchMint","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721IncorrectOwner","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721InsufficientApproval","type":"error"},{"inputs":[{"internalType":"address","name":"approver","type":"address"}],"name":"ERC721InvalidApprover","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"}],"name":"ERC721InvalidOperator","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721InvalidOwner","type":"error"},{"inputs":[{"internalType":"address","name":"receiver","type":"address"}],"name":"ERC721InvalidReceiver","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"}],"name":"ERC721InvalidSender","type":"error"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721NonexistentToken","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"ERC721OutOfBoundsIndex","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"approved","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"operator","type":"address"},{"indexed":false,"internalType":"bool","name":"approved","type":"bool"}],"name":"ApprovalForAll","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"_fromTokenId","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"_toTokenId","type":"uint256"}],"name":"BatchMetadataUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"_tokenId","type":"uint256"}],"name":"MetadataUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"Old_Id","type":"uint256"},{"indexed":true,"internalType":"uint256","name":"New_Id","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"Old_Time","type":"uint256"},{"indexed":true,"internalType":"uint256","name":"New_Time","type":"uint256"},{"indexed":false,"internalType":"uint8","name":"Old_Level","type":"uint8"},{"indexed":true,"internalType":"uint8","name":"New_Level","type":"uint8"},{"indexed":false,"internalType":"uint256","name":"discord_id","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"role","type":"uint256"}],"name":"New_NFT","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"uint256","name":"enter_price","type":"uint256"},{"indexed":true,"internalType":"uint256","name":"request_price","type":"uint256"}],"name":"Price_equal","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"from","type":"address"},{"indexed":true,"internalType":"address","name":"to","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Transfer","type":"event"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"address","name":"add_addr","type":"address"}],"name":"AddParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"Creator","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"DelParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"NFTData","outputs":[{"components":[{"internalType":"uint256","name":"expired","type":"uint256"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"DiscordId","type":"uint256"},{"internalType":"uint8","name":"level","type":"uint8"}],"internalType":"struct NFTInfo","name":"","type":"tuple"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ParceirosData","outputs":[{"internalType":"address[]","name":"","type":"address[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"Plans","outputs":[{"components":[{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"daysTime","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"uint8","name":"level","type":"uint8"},{"internalType":"uint8","name":"acesses","type":"uint8"}],"internalType":"struct Planos[]","name":"","type":"tuple[]"},{"internalType":"uint256[]","name":"","type":"uint256[]"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"True_supply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"components":[{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"daysTime","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"uint8","name":"level","type":"uint8"},{"internalType":"uint8","name":"acesses","type":"uint8"}],"internalType":"struct Planos","name":"plan","type":"tuple"},{"internalType":"uint256","name":"total","type":"uint256"}],"name":"ads","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"approve","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"balanceOf","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"dustWithdraw","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"getApproved","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"address","name":"operator","type":"address"}],"name":"isApprovedForAll","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint8","name":"level","type":"uint8"},{"internalType":"string","name":"metadata","type":"string"}],"name":"metadataByLevel","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint8","name":"level","type":"uint8"}],"name":"metadataOfLevel","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"tokenid","type":"uint256"},{"internalType":"uint256","name":"discord_id","type":"uint256"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"mint","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ownerOf","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"rmv","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"bool","name":"approved","type":"bool"}],"name":"setApprovalForAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes4","name":"interfaceId","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenOfOwnerByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"tokenURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"transferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"}]
var balances = 0;
var conectado = false;
var free = false;
var tokenId = 0;
var tokenLevel = 0;
var ID = 0;
var discordIDPermit = 0;
const web3 = new Web3(window.ethereum);
//mumbai: 0xc9123Ab13fd0A41658fD25b5199faA674F6a6314 
const contract = new web3.eth.Contract(abi,"0x42D8144dB4e8458406Df60E0408dBC961AAeC0f4");
window.onload = () => {
    const fragment = new URLSearchParams(window.location.hash.slice(1));
    const [accessToken, _] = [fragment.get('access_token'), fragment.get('token_type')];
    if (accessToken) {
        console.log("Usuário autenticado com sucesso!");
        fetch("https://discord.com/api/v10/users/@me", {
            headers: {
                "Authorization": `Bearer ${accessToken}`
            }
        })
        .then(response => response.json())
        .then(data => {
            discordIDPermit = data.id;
            document.getElementById('discord').innerHTML = "Conectado: "+data.username;
        })
        .catch(error => console.error("Erro ao obter informações do usuário:", error));
    }
    if (window.ethereum){
        conectar()
    }
    }
async function conectar() {
    if (conectado == false){
        if (typeof window.ethereum === 'undefined') {
            alert('Por favor, instale o Metamask para continuar.');
            return;
        }
        if (typeof window.ethereum !== 'undefined') {
            const redeAtual = await web3.eth.net.getId();
            /*
            rede
            */
            if (redeAtual !== /*80001*/137) {
                try {
                    await window.ethereum.request({
                        method: 'wallet_switchEthereumChain',
                        params: [{ chainId: '0x89' }],
                    });
                    } catch (error) {
                    alert('Erro ao trocar de rede, por favor, conecte-se à rede Polygon para continuar.');
                        console.error('Erro ao trocar de rede:', error.message);
                        location.reload()
                        return;
                }
            }
            conectado = true;
            if (window.ethereum && window.ethereum.selectedAddress){
                connect_data()
            }
        }
    }
}
async function connect_data() {
    const contas = await web3.eth.getAccounts();
    if(!contas){
        alert('Por favor, conecte-se a uma carteira valida.');
        return;
    }
    const infoParam = getURLParameter("info");
    const carteira_string = encurtarString(contas[0],10);
    var creator_exist = true;
    document.getElementById('wallet').innerHTML = "Conectado: "+carteira_string;
    const the_owner = await contract.methods.Creator("").call();
    await contract.methods.Creator(infoParam).call().then((creator) => {
        if (creator == 0x0000000000000000000000000000000000000000){
            creator_exist = false;
        }
        if (creator == contas[0] && creator_exist == true 
            || the_owner == contas[0] && creator_exist == true){
            tokenLevel = 100;
            document.getElementById("menu").style.display = "block";
            free = true
        }
      }).catch((error) => {
        creator_exist = false;
        console.error('Erro ao obter planos:', error);
    });
    const nfts = await contract.methods.balanceOf(contas[0]).call();
    if(nfts > 0){
        tokenId = await contract.methods.tokenOfOwnerByIndex(contas[0],0).call();
        if(tokenId != 0){
            const nftdata = await contract.methods.NFTData(tokenId).call();
            tokenIduri = await contract.methods.tokenURI(tokenId).call();
            const level = nftdata.level;
            if(tokenLevel < level){
                tokenLevel = level;
            }
            const time = formatUnixTime(nftdata.expired);
            fetch(tokenIduri).then(response => response.json()).then(data => {
                var meunft = document.createElement("div");
                meunft.classList.add('conteiner');
                meunft.innerHTML = 
                `<div class="conteudo">
                    <h2>Sua NFT:</h2>
                    <h2>${data.name} Nível ${level}</h2>
                    <h2>Data de Vencimento: ${time}</h2>
                    <p>${data.description}</p>
                </div>
                <div class="imgbx">
                    <img src=${data.image}>
                </div>`;
                document.getElementById("sec_owner").appendChild(meunft)
                if (nftdata.DiscordId != 0){
                    ID = nftdata.DiscordId;
                }
                if (tokenId != 0){
                    document.getElementById("sec_owner").style.display = "flex";
                }
            })
            .catch(error => {
                console.error('Erro ao buscar o JSON:', error);
            });
        }
    }
    if (infoParam && creator_exist == true) {
        const planos = await contract.methods.Plans(infoParam).call();
        document.getElementById("sec_compra").style.display = "flex";
        for (var i = 0; i < planos[0].length; i++) {
            const price = planos[0][i].amount;
            const level = planos[0][i].level;
            const time = planos[0][i].daysTime/86400;
            const acesses = planos[0][i].acesses;
            const disponiveis = planos[1][i];
            const order = i;
            const uri = await contract.methods.metadataOfLevel(level).call();
            if (tokenLevel >= acesses){
                fetch(uri).then(response => response.json()).then(data => {
                    var remove = "";
                    var buy = "";
                    if(disponiveis > 0){
                        buy = `<div class="botao" style="background-color:#008000;" onclick="payable(this)"
                        data-info="${infoParam}/${order}/${price}/${time}">Comprar agora</div>`
                    }else{
                        buy = `<div class="botao" style="background-color:#808080;">Esgotado</div>`
                    }
                    if(free == true){
                        remove = `<div class="botao" style="background-color: rgb(160, 0, 0);" onclick="remove_plan(this)" data-info="${order}">Remover</div>`
                    }
                    var novaCarta = document.createElement("div");
                    novaCarta.classList.add('card');
                    novaCarta.innerHTML = 
                    `<img src=${data.image}>
                    <div class="card-content">
                        <h3>${data.name} Nível ${level}</h3>
                        <div class="tokenInfo">
                            <div class="price">
                                <ins>$</ins>
                                <p>${transformarNumero(price,2)} MATIC</p>
                            </div>
                            <div class="time">
                                <ins>◷</ins>
                                <p>${time} DIAS</p>
                            </div>
                        </div>
                        ${buy}
                        ${remove}
                        <p>Disponíveis:${disponiveis}</p>
                    </div>`;
                    document.getElementById("sec_compra").appendChild(novaCarta)
                })
                .catch(error => {
                    console.error('Erro ao buscar o JSON:', error);
                });
            }
        }
    } else {
        console.log("nao encontrado")
    }
}
function conectar_discord() {
    const redirectUri = window.location.href;
    window.location.href = `https://discord.com/oauth2/authorize?client_id=1162941546820292768&redirect_uri=${encodeURIComponent(redirectUri)}&response_type=token&scope=identify`;
}
async function remove_plan(element) {
    const infoParam = getURLParameter("info");
    const i = element.getAttribute("data-info");
    if (conectado == true && free == true && infoParam){
        const contas = await web3.eth.getAccounts();
        const contractMethod = contract.methods.rmv(infoParam,i);
        const taxaDeGasAtual = await estimarTaxaDeGas();
        const gasAtual = await contractMethod.estimateGas({from: contas[0]});
        showLoadingPage();
        await contractMethod.send({from: contas[0],gas: gasAtual,gasPrice: taxaDeGasAtual})
        .then(_ => {
            hideLoadingPage();
            alert("Plano removido com sucesso!");
            setTimeout(function() {
                conectar()
            }, 5000);
        })
        .catch(_ => {
            hideLoadingPage();
            alert("erro ao remover seu plano..")
        })
    }
}
async function add_plan() {
    const infoParam = getURLParameter("info");
    const Cargo = document.getElementById("Cargo").value;
    const Valor = parseFloat(document.getElementById("Valor").value);
    const Decimal = parseFloat(document.getElementById("Decimal").value);
    const Dias = document.getElementById("Dias").value;
    const Nivel = document.getElementById("Nivel").value;
    const Acesso = document.getElementById("Acesso").value;
    const Quantidade = document.getElementById("Quantidade").value;
    const uri = await contract.methods.metadataOfLevel(Nivel).call();
    
    if (conectado == true && free == true && 
        infoParam && Dias > 0 && Quantidade > 0 &&
        Nivel > 0 && uri != "" && Valor >= 0 &&
        Decimal >= 0 && Nivel > Acesso){
        const contas = await web3.eth.getAccounts();
        showLoadingPage();
        const contractMethod = contract.methods.ads(infoParam,[Cargo,Dias*86400,adicionarZeros(Valor,Decimal),Nivel,Acesso],Quantidade);
        const taxaDeGasAtual = await estimarTaxaDeGas();
        const gasAtual = await contractMethod.estimateGas({from: contas[0]});
        await contractMethod.send({from: contas[0],gas: gasAtual,gasPrice: taxaDeGasAtual})
        .then(_ => {
            hideLoadingPage();
            alert("Plano adcionado com sucesso!");
            setTimeout(function() {
                conectar()
            }, 5000);
        })
        .catch(_ => {
            hideLoadingPage();
            alert("erro ao adcionar seu plano..")
        })
    }else{
        alert("erro ao ler parâmetros para o novo plano..")
    }
}
async function payable(element) {
    const dataInfo = element.getAttribute("data-info").split("/");
    var id_discord = discordIDPermit;
    if (id_discord == ""){id_discord = "0"}
    const criador = dataInfo[0];
    const plano = dataInfo[1];
    var price = 0;
    if(dataInfo[2] > 0){
        price = Number(dataInfo[2])+(Number(dataInfo[2])/1000);
    }
    const time = dataInfo[3];
    var permit = false;
    if (conectado == true && discordIDPermit != 0){
        const contas = await web3.eth.getAccounts();
        balances = await web3.eth.getBalance(contas[0]);
        if(balances > price){
            permit = true
        }else{
            alert("Fundos insuficientes")
        }
        if (permit){
            const contractMethod = contract.methods.mint(contas[0],criador,tokenId,id_discord,plano);
            const taxaDeGasAtual = await estimarTaxaDeGas();
            const gasAtual = await contractMethod.estimateGas({from: contas[0],value: price});
            showLoadingPage();
            await contractMethod.send({from: contas[0],value: price,gas: gasAtual,gasPrice: taxaDeGasAtual})
            .then(_ => {
                hideLoadingPage();
                alert("NFT comprada com sucesso!");
                setTimeout(function() {
                    conectar()
                }, 5000);
            })
            .catch(_ => {
                hideLoadingPage();
                alert("erro ao comprar sua NFT..")
            })
        }
    }else{
        alert("erro ao conectar seu discord..")
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
function formatUnixTime(unixTime) {
    const date = new Date(unixTime * 1000);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');

    return `${year}/${month}/${day} ás ${hours}h`;
}
function getCurrentUnixTime() {
    return Math.floor(new Date().getTime() / 1000);
}
function avoidDots(input) {
    const value = input.value;
    if (value.includes('.')) {
        input.value = value.slice(0, -1);
    }
}
function exM() {
    var exm = document.querySelector('.exmenu'); exm.classList.toggle('active');
    var nav = document.querySelector('.exnav'); nav.classList.toggle('active');
}
function adicionarZeros(amont, decimal) {
  if (typeof amont !== 'number' || typeof decimal !== 'number') {
    throw new Error('Os argumentos devem ser números');
  }

  const parteDecimalString = '0'.repeat(decimal);
  const resultado = amont + parteDecimalString;

  return resultado;
}
async function estimarTaxaDeGas() {
    try {
        const taxaDeGas = await web3.eth.getGasPrice();
        console.log('Taxa de Gás Atual:', taxaDeGas);
        return taxaDeGas;
    } catch (error) {
        console.error('Erro ao estimar a taxa de gás:', error);
    }
}
function showLoadingPage() {
    const loadingPage = document.getElementById("loading-page");
    loadingPage.style.display = "flex";
}

function hideLoadingPage() {
    const loadingPage = document.getElementById("loading-page");
    loadingPage.style.display = "none";

}

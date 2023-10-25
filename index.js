const abi = [{"inputs":[{"internalType":"address","name":"orl","type":"address"},{"internalType":"uint8","name":"Opercent","type":"uint8"},{"internalType":"uint8","name":"Cpercent","type":"uint8"}],"stateMutability":"nonpayable","type":"constructor"},{"inputs":[],"name":"ERC721EnumerableForbiddenBatchMint","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721IncorrectOwner","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721InsufficientApproval","type":"error"},{"inputs":[{"internalType":"address","name":"approver","type":"address"}],"name":"ERC721InvalidApprover","type":"error"},{"inputs":[{"internalType":"address","name":"operator","type":"address"}],"name":"ERC721InvalidOperator","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"ERC721InvalidOwner","type":"error"},{"inputs":[{"internalType":"address","name":"receiver","type":"address"}],"name":"ERC721InvalidReceiver","type":"error"},{"inputs":[{"internalType":"address","name":"sender","type":"address"}],"name":"ERC721InvalidSender","type":"error"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ERC721NonexistentToken","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"ERC721OutOfBoundsIndex","type":"error"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"approved","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Approval","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"owner","type":"address"},{"indexed":true,"internalType":"address","name":"operator","type":"address"},{"indexed":false,"internalType":"bool","name":"approved","type":"bool"}],"name":"ApprovalForAll","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"_fromTokenId","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"_toTokenId","type":"uint256"}],"name":"BatchMetadataUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"_tokenId","type":"uint256"}],"name":"MetadataUpdate","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"from","type":"address"},{"indexed":true,"internalType":"address","name":"to","type":"address"},{"indexed":true,"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"Transfer","type":"event"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"address","name":"add_addr","type":"address"}],"name":"AddParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"Creator","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"DelParceiros","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"NFTData","outputs":[{"components":[{"internalType":"uint256","name":"expired","type":"uint256"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"DiscordId","type":"uint256"},{"internalType":"uint8","name":"level","type":"uint8"}],"internalType":"struct NFTInfo","name":"","type":"tuple"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"ParceirosData","outputs":[{"internalType":"address[]","name":"","type":"address[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"}],"name":"Plans","outputs":[{"components":[{"internalType":"string","name":"uri","type":"string"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"timestamp","type":"uint256"},{"internalType":"uint256","name":"amount","type":"uint256"},{"internalType":"uint8","name":"decimal","type":"uint8"},{"internalType":"uint8","name":"level","type":"uint8"},{"internalType":"uint8","name":"acesses","type":"uint8"}],"internalType":"struct Planos[]","name":"","type":"tuple[]"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"Price","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"True_supply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"uint256","name":"discord_id","type":"uint256"}],"name":"active","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"string","name":"uri","type":"string"},{"internalType":"uint256","name":"cargo","type":"uint256"},{"internalType":"uint256","name":"value","type":"uint256"},{"internalType":"uint8","name":"nivel","type":"uint8"},{"internalType":"uint8","name":"acesses","type":"uint8"},{"internalType":"uint256","name":"time","type":"uint256"},{"internalType":"uint8","name":"decimal","type":"uint8"}],"name":"ads","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"approve","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"balanceOf","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"getApproved","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"address","name":"operator","type":"address"}],"name":"isApprovedForAll","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"to","type":"address"},{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"tokenid","type":"uint256"},{"internalType":"uint256","name":"discord_id","type":"uint256"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"mint","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"name","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"ownerOf","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"string","name":"creator","type":"string"},{"internalType":"uint256","name":"i","type":"uint256"}],"name":"rmv","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"safeTransferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"operator","type":"address"},{"internalType":"bool","name":"approved","type":"bool"}],"name":"setApprovalForAll","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"bytes4","name":"interfaceId","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"symbol","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"theowner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"owner","type":"address"},{"internalType":"uint256","name":"index","type":"uint256"}],"name":"tokenOfOwnerByIndex","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"tokenURI","outputs":[{"internalType":"string","name":"","type":"string"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"totalSupply","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"tokenId","type":"uint256"}],"name":"transferFrom","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"}];
var balances = 0;
var conectado = false;
var free = false;
var tokenId = 0;
var tokenLevel = 0;
var ID = 0;
var discordIDPermit = 0;
const web3 = new Web3(window.ethereum);
const contract = new web3.eth.Contract(abi,"0xe0a102D8D14035797d0FAeBE8A7aB610e7e10201");
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
            await window.ethereum.request({ method: 'eth_requestAccounts' });
            const redeAtual = await web3.eth.net.getId();
            /*
            
            
            
            
            rede
            rede
            rede
            rede
            rede
            rede
            rede
            
            
            
            
            
            */
            if (redeAtual !== 80001/*137*/) {
                alert('Por favor, conecte-se à rede Polygon Mumbai para continuar.');
                return;
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
    document.getElementById('wallet').innerHTML = "Conectado: "+carteira_string;
    const creator = await contract.methods.Creator(infoParam).call();
    if (creator == contas[0]){
        tokenLevel = 100;
        document.getElementById("menu").style.display = "block";
        document.getElementById("Wallet").style.display = "block";
        document.getElementById("Wallet").value = contas[0];
        free = true
    }
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
                    <h2>${data.name} Nivel ${level}</h2>
                    <h2>Data de Vencimento: ${time}</h2>
                    <p>${data.description}</p>
                    <div class="botao" id="discordIdButton" style="background-color: #4b7bff;" onclick="ativar()">Ativar Discord</div>
                </div>
                <div class="imgbx">
                    <img src=${addhttps(data.image)}>
                </div>`;
                document.getElementById("sec_owner").appendChild(meunft)
                if (nftdata.DiscordId != 0){
                    ID = nftdata.DiscordId;
                    document.getElementById("discordIdButton").style.display = "none";
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
    if (infoParam) {
        const planos = await contract.methods.Plans(infoParam).call();
        document.getElementById("sec_compra").style.display = "flex";
        for (var i = 0; i < planos.length; i++) {
            const price = await contract.methods.Price(infoParam,i).call();
            const level = planos[i].level;
            const time = planos[i].timestamp/86400;
            const order = i;
            if (tokenLevel >= level){
                fetch(planos[order].uri).then(response => response.json()).then(data => {
                    var remove = "";
                    if(free == true){
                        remove = `<div class="botao" style="background-color: rgb(160, 0, 0);" onclick="remove_plan(this)" data-info="${order}">Remover</div>`
                    }
                    var novaCarta = document.createElement("div");
                    novaCarta.classList.add('card');
                    novaCarta.innerHTML = 
                    `<img src=${addhttps(data.image)}>
                    <div class="card-content">
                        <h3>${data.name} Nivel ${level}</h3>
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
                        <div class="botao" style="animation: colorChange 4s linear infinite alternate;" onclick="payable(this)"
                        data-info="${infoParam}/${order}/${price}/${time}">Comprar agora</div>
                        ${remove}
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
async function ativar() {
    const id_discord = discordIDPermit;
    if (conectado == true && tokenId != 0 && ID == 0 && discordIDPermit != 0){
        const contas = await web3.eth.getAccounts();
        await contract.methods.active(tokenId,id_discord).send({from: contas[0]})
        .then(_ => {ID = id_discord;alert("NFT ativada com sucesso!")})
        .catch(_ => {alert("erro ao ativar sua NFT..")})
    }
}
async function remove_plan(element) {
    const infoParam = getURLParameter("info");
    const i = element.getAttribute("data-info");
    if (conectado == true && free == true && infoParam){
        const contas = await web3.eth.getAccounts();
        await contract.methods.rmv(infoParam,i).send({from: contas[0]})
        .then(_ => {alert("Plano removido com sucesso!");location.reload();})
        .catch(_ => {alert("erro ao remover seu plano..")})
    }
}
async function add_plan() {
    const infoParam = getURLParameter("info");
    const URI = document.getElementById("URI").value;
    const Cargo = document.getElementById("Cargo").value;
    const Valor = document.getElementById("Valor").value;
    const Decimal = document.getElementById("Decimal").value;
    const Dias = document.getElementById("Dias").value;
    const Nivel = document.getElementById("Nivel").value;
    const Acesso = document.getElementById("Acesso").value;
    if (conectado == true && free == true && infoParam && URI && Valor > 0){
        const contas = await web3.eth.getAccounts();
        await contract.methods.ads(infoParam,URI,Cargo,Valor,Nivel,Acesso,Dias,Decimal).send({from: contas[0]})
        .then(_ => {alert("Plano adcionado com sucesso!");location.reload();})
        .catch(_ => {alert("erro ao adcionar seu plano..")})
    }
}
async function payable(element) {
    const dataInfo = element.getAttribute("data-info").split("/");
    var id_discord = discordIDPermit;
    if (id_discord == ""){id_discord = "0"}
    const criador = dataInfo[0];
    const plano = dataInfo[1];
    const price = dataInfo[2];
    const time = dataInfo[3];
    var permit = false;
    if (conectado == true && discordIDPermit != 0){
        const contas = await web3.eth.getAccounts();
        var carteira = contas[0];
        balances = await web3.eth.getBalance(contas[0]);
        if(balances > Number(price)){
            permit = true
        }else{
            alert("Fundos insuficientes")
        }
        if (permit){
            if (free == true && time < 7){
                carteira = document.getElementById("Wallet").value;
                if (carteira != contas[0]){
                    id_discord = 0;
                    tokenId = 0;
                }
                await contract.methods.mint(carteira,criador,tokenId,id_discord,plano).send({from: contas[0]})
                .then(_ => {ID = id_discord;alert("NFT do criador mintada com sucesso!")})
                .catch(_ => {alert("erro ao mintar NFT como criador..")})
            }else{
                await contract.methods.mint(carteira,criador,tokenId,id_discord,plano).send({from: contas[0],value: price})
                .then(_ => {
                    ID = id_discord;
                    alert("NFT comprada com sucesso!");
                    location.reload();
                })
                .catch(_ => {alert("erro ao comprar sua NFT..")})
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
function formatUnixTime(unixTime) {
    const date = new Date(unixTime * 1000);
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');

    return `${year}/${month}/${day}:${hours}`;
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
function addhttps(url) {
    if (!url.startsWith("http://") && !url.startsWith("https://")) {
        url = "https://" + url;
    }
    return url;
}
function exM() {
    var exm = document.querySelector('.exmenu'); exm.classList.toggle('active');
    var nav = document.querySelector('.exnav'); nav.classList.toggle('active');
}

use serde_json::{json, Value};
use ethers::prelude::{Http,Provider,Address,U256};
use serenity::prelude::*;
use serenity::model::prelude::{RoleId,Member};
use chrono::Utc;
use serenity::framework::standard::StandardFramework;
mod cpv;

#[tokio::main]
async fn main(){
    let mut jsons = {
        let text = std::fs::read_to_string(&"info.json").unwrap();
        serde_json::from_str::<Value>(&text).unwrap()
    };
    let token: String = serde_json::from_str(&jsons["token"].to_string()).unwrap();
    let rcp: String = serde_json::from_str(&jsons["rcp"].to_string()).unwrap();
    let addr: String = serde_json::from_str(&jsons["addr"].to_string()).unwrap();
    let address:Address = addr.parse().unwrap();
    let guild_id: u64 = serde_json::from_str(&jsons["server"].to_string()).unwrap();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(token, intents)
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");
    println!("Link para adcionar o robô: \n
https://discord.com/api/oauth2/authorize?client_id={}&permissions=8&scope=bot%20applications.commands",
client.cache_and_http.http.get_current_user().await.unwrap().id.as_u64());
    loop{
        match Provider::<Http>::try_from(rcp.as_str()) {
            Ok(provedor) =>{
                std::thread::sleep(core::time::Duration::from_secs(10));
                let cryptoverso = cpv::cpv::cpv::new(address, provedor.into());
                match cryptoverso.true_supply().await {
                    Ok(nuns) =>{
                        let mut users: Vec<String> = serde_json::from_str(&jsons["users"].to_string()).unwrap();
                        let mut home: u128 = jsons["home"].to_string().replace('"',"").parse::<u128>().unwrap();
                        let mut on_nuns: Vec<u128> = vec![];
                        let mut off_nuns: Vec<u128> = vec![];
                        let mut remove_index: u32 = 0;
                        let utc_time: u64 = Utc::now().timestamp() as u64;
                        let http: std::sync::Arc<serenity::http::Http> = client.cache_and_http.http.clone();
                        std::thread::sleep(core::time::Duration::from_secs(10));
                        for n in home..nuns.as_u128()+1{
                            on_nuns.push(n);
                        }
                        if users.len() > 0{
                            for u in 0..users.len(){

                                let unit: Value = serde_json::from_str(&users[u]).unwrap();
                                let ojson: Value = json!(unit);
                                let json_role: u64 = ojson["role"].to_string().replace('"',"").parse::<u64>().unwrap();
                                let json_member: u64 = ojson["menber"].to_string().replace('"',"").parse::<u64>().unwrap();
                                let json_exit: u64 = ojson["exit"].to_string().replace('"',"").parse::<u64>().unwrap();
                                let json_token_id: u128 = ojson["tokenId"].to_string().replace('"',"").parse::<u128>().unwrap();
                                
                                if utc_time > json_exit && json_member > 0{
                                    match http.get_member(guild_id, json_member).await {
                                        Ok(user) =>{
                                            let mut enter: bool = false;
                                            for r in user.clone().roles{
                                                if r.as_u64() == &json_role{
                                                    enter = true;
                                                }
                                            }
                                            if enter == true{
                                                let mut the_user: Member = user;
                                                match the_user.remove_role(http.clone(), RoleId(json_role)).await{
                                                    Ok(_) =>{
                                                        let json_data: String = json!({"tokenId":json_token_id,"menber":0,"role": 0,"exit": 0}).to_string();
                                                        users[u] = json_data;

                                                        jsons["users"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",users).as_str()).unwrap();
                                                        std::fs::write(
                                                            &"info.json",
                                                            serde_json::to_string_pretty(&jsons).unwrap(),
                                                        ).unwrap();
                                                        let sdata: String = format!("<@{}> !
Seu período como Explorador acabou! \n Acesse os Links Oficiais para renovar seus benefícios!",the_user.user.id).to_string();
                                                        match client.cache_and_http.http.send_message(1183625294062555188, &json!({"content": sdata})).await {
                                                            Ok(_) =>{
                                                                println!("rmv {} -> {}",json_role,json_member);
                                                            },Err(e) =>println!("err enviar mensagem {e}")
                                                        }
                                                    },Err(_) =>println!("rmv discord err")
                                                }
                                            }
                                        },Err(e) =>{
                                            println!("usuario {json_member} não encontrado {e}")
                                        }
                                    }
                                }else if json_member == 0 && json_role == 0 && json_exit == 0 && home == json_token_id{
                                    remove_index = remove_index+1;
                                    home = home+1;
                                }
                                off_nuns.push(json_token_id);
                            }
                        }
                        if em_ordem_crescente(&off_nuns) == false{
                           off_nuns.sort();
                           users = reordenar(off_nuns.clone(),users)
                        }
                        let position: Vec<u128> = remove_duplicates(off_nuns, on_nuns);
                        if position.len() > 0{
                            for ids in position{
                                std::thread::sleep(core::time::Duration::from_secs(10));
                                match cryptoverso.nft_data(U256::from(ids)).await {
                                    Ok(data) =>{
                                        let member_id: u64 = data.discord_id.as_u64();
                                        let role_id: u64 = data.cargo.as_u64();
                                        let expire: u64 = data.expired.as_u64();
                                        if member_id > 0 && expire > 0{
                                            match http.get_member(guild_id, member_id).await {
                                                Ok(user) =>{
                                                    let mut enter: bool = true;
                                                    for r in user.clone().roles{
                                                        if r.as_u64() == &role_id{
                                                            enter = false;
                                                        }
                                                    }
                                                    if enter == true{
                                                        let mut the_user: Member = user;
                                                        match the_user.add_role(http.clone(), RoleId(role_id)).await{
                                                            Ok(_) =>{
                                                                let json_data: String = json!({"tokenId":ids,"menber":member_id,"role": role_id,"exit": expire}).to_string();
                                                                users.push(json_data);
                                                                let sdata: String = format!("Parabéns <@{}> !
Você acaba de se tornar um Explorador! \n Acesse o Mapa do CryptoVerso para verificar seus benefícios!",the_user.user.id).to_string();
                                                                match client.cache_and_http.http.send_message(1183625294062555188, &json!({"content": sdata})).await {
                                                                    Ok(_) =>{
                                                                        println!("add {} -> {}",role_id,member_id);
                                                                    },Err(e) =>println!("err enviar mensagem {e}")
                                                                }
                                                            },Err(_) =>println!("add discord err")
                                                        }
                                                    }else {
                                                        let the_user: Member = user;
                                                        let mut exit: u64 = expire;
                                                        (users,exit) = podar(users,member_id,role_id,exit);
                                                        let json_data: String = json!({"tokenId":ids,"menber":member_id,"role": role_id,"exit": exit}).to_string();
                                                        users.push(json_data);
                                                        let sdata: String = format!("Parabéns <@{}> !
Você acaba de se tornar um Explorador! \n Acesse o Mapa do CryptoVerso para verificar seus benefícios!",the_user.user.id).to_string();
                                                        match client.cache_and_http.http.send_message(1183625294062555188, &json!({"content": sdata})).await {
                                                            Ok(_) =>{
                                                                println!("add {} -> {}",role_id,member_id);
                                                            },Err(e) =>println!("err enviar mensagem {e}")
                                                        }
                                                    }
                                                },Err(e) =>{
                                                    println!("{ids} usuario {member_id} não encontrado {e}")
                                                }
                                            }
                                        }else {
                                            if member_id > 0{
                                                let json_data: String = json!({"tokenId":ids,"menber":member_id,"role": role_id,"exit": expire}).to_string();
                                                users.push(json_data);
                                            }
                                        }
                                    },Err(e) =>println!("data err: {e}")
                                }
                            }
                        }
                        if remove_index > 0{
                            for _ in 0..remove_index{
                                users.remove(0);
                            }
                        }
                        let nusers: Vec<String> = serde_json::from_str(&jsons["users"].to_string()).unwrap();
                        let homes: u128 = jsons["home"].to_string().replace('"',"").parse::<u128>().unwrap();
                        if users != nusers || home != homes{
                            jsons["home"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",home).as_str()).unwrap();
                            jsons["users"] = serde_json::from_str::<serde_json::Value>(&format!("{:?}",users).as_str()).unwrap();
                            std::fs::write(
                                &"info.json",
                                serde_json::to_string_pretty(&jsons).unwrap(),
                            ).unwrap();
                        }
                    },Err(e) => println!("total err: {e}")
                }
            },Err(_) => println!("Erro ao fazer a requisição do RCP")
        }
    }
}
fn podar(mut users:Vec<String>,target_menber: u64,target_role: u64,target_expire: u64) -> (Vec<String>,u64){
    let mut members:Vec<u64> = vec![];
    let mut roles:Vec<u64> = vec![];
    let mut token_id:Vec<u128> = vec![];
    let mut expires:Vec<u64> = vec![];
    let mut expire:u64 = target_expire;
    for u in &users{
        let unit: Value = serde_json::from_str(u).unwrap();
        let ojson: Value = json!(unit);
        let json_member: u64 = ojson["menber"].to_string().replace('"',"").parse::<u64>().unwrap();
        let json_role: u64 = ojson["role"].to_string().replace('"',"").parse::<u64>().unwrap();
        let json_token_id: u128 = ojson["tokenId"].to_string().replace('"',"").parse::<u128>().unwrap();
        let json_expire: u64 = ojson["exit"].to_string().replace('"',"").parse::<u64>().unwrap();
        expires.push(json_expire);
        members.push(json_member);
        token_id.push(json_token_id);
        roles.push(json_role);
    }
    let member: Vec<usize> = find_indices(members,target_menber);
    let role: Vec<usize> = find_indices(roles,target_role);
    if member.len() > 0 && role.len() > 0{
        for m in &member{
            for r in &role{
                if m == r{
                    if expire < expires[*m]{
                        expire = expires[*m];
                    }
                    let json_data: String = json!({"tokenId":token_id[*m],"menber":0,"role": 0,"exit": 0}).to_string();
                    users[*m] = json_data;
                }
            }
        }
    }
    return (users,expire);
}
fn reordenar(reorder:Vec<u128>,users:Vec<String>) -> Vec<String>{
    let mut order_users:Vec<String> = vec![];
    let mut token_id:Vec<u128> = vec![];
    for u in &users{
        let unit: Value = serde_json::from_str(u).unwrap();
        let ojson: Value = json!(unit);
        let json_token_id: u128 = ojson["tokenId"].to_string().replace('"',"").parse::<u128>().unwrap();
        token_id.push(json_token_id);
    }
    for r in reorder{
        for t in 0..token_id.len(){
            if r == token_id[t]{
                order_users.push(users[t].clone())
            }
        }
    }
    return order_users;
}
fn remove_duplicates(vec1: Vec<u128>, vec2: Vec<u128>) -> Vec<u128> {
    let mut result: Vec<u128> = Vec::new();

    for value in vec1.iter() {
        if !vec2.contains(value) {
            result.push(*value);
        }
    }

    for value in vec2.iter() {
        if !vec1.contains(value) {
            result.push(*value);
        }
    }

    result
}
fn find_indices(vec: Vec<u64>, target: u64) -> Vec<usize> {
    let mut indices = Vec::new();

    for (index, &value) in vec.iter().enumerate() {
        if value == target {
            indices.push(index);
        }
    }

    indices
}
fn em_ordem_crescente(vetor: &Vec<u128>) -> bool {
    if vetor.is_empty() {
        return true;
    }
    for i in 1..vetor.len() {
        if vetor[i] < vetor[i - 1] {
            return false;
        }
    }
    return true; 
}

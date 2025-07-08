#[test] 
fn base58_to_wallet() { 
println!("Enter your name:"); 
let stdin = io::stdin(); 
let base58 = stdin.lock().lines().next().unwrap().unwrap(); // GGWCWsGAWBZchKFbwxawqeZnrKaEHe4X7t7SeAy9HDGT
 Ck7Vv 
ksUGhPt4SZ8JHVSkt 
let wallet = bs58::decode(base58).into_vec().unwrap(); 
println!("{:?}", wallet); 
} 

#[test] 
fn wallet_to_base58() { 
let wallet: Vec<u8> = 
vec![157,119,18,163,187,190,23,255,210,76,218,13,97,82,81,172,211,231,180,23,33,159,24,110,108,39,52,149,153,252,112,119,226,216,7,234,69,146,96,135,51,242,20,66,58,190,133,107,82,206,246,153,221,142,134,82,37,42,29,214,140,133,148,240]; 
let base58 = bs58::encode(wallet).into_string(); 
println!("{:?}", base58); 
}
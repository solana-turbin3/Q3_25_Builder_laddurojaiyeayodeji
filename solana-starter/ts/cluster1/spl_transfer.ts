import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("D8iHjJm7aUbHEcqdRbKdVvTJWB92RN6shYAhU7BUwHMF");

// Recipient address
const to = new PublicKey("5FaER8yXfEujMT7Q7N7BZ8hDdMgh15Vow7LgQpAeiTTy");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromWallet = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toWallet = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);

        // Transfer the new token to the "toTokenAccount" we just created
        const signature = await transfer(connection, keypair, fromWallet.address, toWallet.address, keypair, 400000)

        console.log(`Transfer transaction: ${signature}`)
        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
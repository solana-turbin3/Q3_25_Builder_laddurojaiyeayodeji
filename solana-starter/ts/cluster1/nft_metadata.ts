import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/B6u1ULGSMWN9kLDmJgXCXQnVA4BE7kSQHkJUHLUeR5dF"
        const metadata = {
            name: "My Rug NFT",
            symbol: "RNFT",
            description: "just a random rug nft",
            image: image,
            attributes: [
                {trait_type: 'rarity', value: 'scarce'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://gateway.irys.xyz/BToamvB58U2yQf92o5cqAowwo3rGv5JWFdsEfAwGVdz7"
                    },
                ]
            },
            creators: [
                {address: keypair.publicKey, share: 100}
            ]
        };
        const myUri = await umi.uploader.uploadJson(metadata)
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// https://gateway.irys.xyz/2G4495woL5xZfcSv3kKZcazBbT2CxKUrQAUKr9HepuJG

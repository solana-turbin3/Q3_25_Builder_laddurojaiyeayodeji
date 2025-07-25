import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"


// Create a devnet connection
// const umi = createUmi('https://api.devnet.solana.com');
const umi = createUmi('https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        const image = await readFile('/root/Turbin3-Classes/Q3_25_Builder_MrLad01/solana-starter/ts/generug.png');

        const genericFile = await createGenericFile(image, "generug.png", {
            contentType: 'image/png'
        });

        const [myUri] = await umi.uploader.upload([genericFile]);      
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();


// https://gateway.irys.xyz/B6u1ULGSMWN9kLDmJgXCXQnVA4BE7kSQHkJUHLUeR5dF
import { Connection, clusterApiUrl, Keypair, PublicKey, TransactionInstruction, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { readFileSync } from "fs";
import path from "path";
import os from "os";

const connection = new Connection(clusterApiUrl('devnet'));


const desktopPath = path.join(os.homedir(), 'Desktop', 'Aether6rFacXQRDMR4x8Z2xbiAVXvf9kmmnb2vML16Hp.json');

const keypair = Keypair.fromSecretKey(
  Uint8Array.from(JSON.parse(readFileSync(desktopPath, "utf-8")))
);

const publicKey = keypair.publicKey;
const PROGRAM_ID = new PublicKey("4Va1dbHzVvLDXUoRxWPrWXZpqZwa9DBPkEfyia8r381p");

const tx = new Transaction;

const ix = new TransactionInstruction({
    keys: [],
    programId: PROGRAM_ID,
});
tx.add(ix);

const send = await sendAndConfirmTransaction(connection, tx, [keypair]);
console.log("Transaction signature: ",send);





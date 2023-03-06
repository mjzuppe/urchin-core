import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EntryProgram } from "../target/types/entry_program";
import { expect, assert } from "chai";

describe("entry-program", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.EntryProgram as Program<EntryProgram>;
    const owner = anchor.web3.Keypair.generate();
    const parent = anchor.web3.Keypair.generate();
    const newKey = anchor.web3.Keypair.generate();
    const fakeSHA256 = "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b";

    it("can create a new entry", async () => {
        await program.methods.createEntry(fakeSHA256, [], false, false).accounts({
            entry: newKey.publicKey,
            payer: anchor.getProvider().wallet.publicKey,
            owner: owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([newKey, owner]).rpc();

        const entryFound = await program.account.entryAccount.fetch(newKey.publicKey);

        assert.equal(entryFound.arweaveId, "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b");
        assert.equal(entryFound.archived, false);
        assert.equal(entryFound.immutable, false);
        assert.equal(entryFound.version, 0);
    });

    it("can retrieve by owner", async () => {
        const foundAccounts = await program.account.entryAccount.all(
            [
                {
                    memcmp: {
                        offset: 8, // Discriminator.
                        bytes: owner.publicKey.toBase58(),
                    }
                }
            ]
        );
        assert.equal(foundAccounts.length, 1);
        const foundAccount: any = foundAccounts[0];
        assert.equal(foundAccount.account.arweaveId, fakeSHA256);
        assert.equal(foundAccount.account.immutable, false);
        assert.equal(foundAccount.taxonomy, undefined);
        assert.equal(foundAccount.account.archived, false);
        assert.equal(foundAccount.account.owner.toString(), owner.publicKey.toString());
    });


    it('can retrieve by a single address', async () => {
        const foundAccounts: any = await program.account.entryAccount.fetchMultiple([newKey.publicKey]);
        assert.equal(foundAccounts.length, 1);
      });

      it("can update a entry", async () => {
        await program.methods.updateEntry(fakeSHA256, true, [newKey.publicKey], true).accounts({
            entry: newKey.publicKey,
            payer: anchor.getProvider().wallet.publicKey,
            owner: owner.publicKey,
        }).signers([owner]).rpc();

        const foundAccount = await program.account.entryAccount.fetch(newKey.publicKey);
        assert.equal(foundAccount.archived, true);
        assert.equal(foundAccount.immutable, true);
      });


});
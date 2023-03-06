import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AssetProgram } from "../target/types/asset_program";
import {expect, assert} from "chai";

describe("asset-program", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.AssetProgram as Program<AssetProgram>;
    const owner = anchor.web3.Keypair.generate();
    const parent = anchor.web3.Keypair.generate();
    const newKey = anchor.web3.Keypair.generate();
    const fakeSHA256 = "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b";

    it("can create a new asset", async () => {
        await program.methods.createAsset(fakeSHA256, false, false,).accounts({
            asset: newKey.publicKey,
            payer: anchor.getProvider().wallet.publicKey,
            owner: owner.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([newKey, owner]).rpc();

        const assetFound = await program.account.assetAccount.fetch(newKey.publicKey);
        assert.equal(assetFound.arweaveId, "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b");
        assert.equal(assetFound.archived, false);
        assert.equal(assetFound.immutable, false);
    });

    it("can retrieve by owner", async () => {
        const foundAccounts = await program.account.assetAccount.all(
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
        assert.equal(foundAccount.account.archived, false);
        assert.equal(foundAccount.account.owner.toString(), owner.publicKey.toString());
      });

      it('can retrieve by a single address', async () => {
        const foundAccounts: any = await program.account.assetAccount.fetchMultiple([newKey.publicKey]);
        assert.equal(foundAccounts.length, 1);
      });

      it("can update a asset", async () => {
        await program.methods.updateAsset(fakeSHA256, true, true).accounts({
            asset: newKey.publicKey,
            payer: anchor.getProvider().wallet.publicKey,
            owner: owner.publicKey,
        }).signers([owner]).rpc();

        const foundAccount = await program.account.assetAccount.fetch(newKey.publicKey);
        assert.equal(foundAccount.archived, true);
        assert.equal(foundAccount.immutable, true);
      });

    //   it("can prevent updating an immutable asset", async () => {
    //     expect(await program.methods.updateAsset(fakeSHA256.split("").reverse().join(""), true, true).accounts({
    //         asset: newKey.publicKey,
    //         payer: anchor.getProvider().wallet.publicKey,
    //         owner: owner.publicKey,
    //     }).signers([owner]).rpc()).to.throw(Error);

 
    //   });


});
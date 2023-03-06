import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TaxonomyProgram } from "../target/types/taxonomy_program";
import * as assert from "assert";

describe("taxonomy-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TaxonomyProgram as Program<TaxonomyProgram>;
  const owner = anchor.web3.Keypair.generate();
  const parent = anchor.web3.Keypair.generate();
  const newKey = anchor.web3.Keypair.generate();

  it("can create a new taxonomy", async () => {
    // Add your test here.
    await program.methods.createTaxonomy("new label", parent.publicKey).accounts({

      taxonomy: newKey.publicKey,
      payer: anchor.getProvider().wallet.publicKey,
      owner: owner.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([newKey, owner]).rpc();

    const taxonomyFound = await program.account.taxonomyAccount.fetch(newKey.publicKey);
    assert.equal(taxonomyFound.label, 'new label');
    // assert.equal(taxonomyFound.parent.toString(), parent.publicKey.toString());

  });

  it('can retrieve by owner', async () => {
    const foundAccounts = await program.account.taxonomyAccount.all(
      [
        {
          memcmp: {
            offset: 8, // Discriminator.
            bytes: owner.publicKey.toBase58(),
          }
        }
      ]
    )
      ;
    assert.equal(foundAccounts.length, 1);
    const foundAccount: any = foundAccounts[0];
    assert.equal(foundAccount.account.label, 'new label');
    if (foundAccount.account.parent !== "null") assert.equal(foundAccount.account.parent.toString(), parent.publicKey.toString());
    assert.equal(foundAccount.account.owner.toString(), owner.publicKey.toString());
  });

  it('can retrieve by a single address', async () => {
    const foundAccounts: any = await program.account.taxonomyAccount.fetchMultiple([newKey.publicKey]);
    assert.equal(foundAccounts.length, 1);
  });

  it('can update a taxonomy', async () => {
    await program.methods.updateTaxonomy("newer label", parent.publicKey).accounts({
      taxonomy: newKey.publicKey,
      payer: anchor.getProvider().wallet.publicKey,
      owner: owner.publicKey,
    }).signers([owner]).rpc();

    const foundAccount = await program.account.taxonomyAccount.fetch(newKey.publicKey);
    assert.equal(foundAccount.label, 'newer label');

  });

});

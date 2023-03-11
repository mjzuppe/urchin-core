import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TemplateProgram } from "../target/types/template_program";
import {assert} from "chai";

describe("template-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TemplateProgram as Program<TemplateProgram>;
  const owner = anchor.web3.Keypair.generate();
  const parent = anchor.web3.Keypair.generate();
  const newKey = anchor.web3.Keypair.generate();

  const fakeSHA256 = "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b"

  it("can create a new template", async () => {
    // Add your test here.
    await program.methods.createTemplate(fakeSHA256, null, false).accounts({
      template: newKey.publicKey,
      payer: anchor.getProvider().wallet.publicKey,
      owner: owner.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([newKey, owner]).rpc();

    const templateFound = await program.account.templateAccount.fetch(newKey.publicKey);
    assert.equal(templateFound.arweaveId, "f0e4c2f76c58916ec258f246851bea091d14d4247a2fc3e18694461b1816e13b");
  });

  it("can retrieve by owner", async () => {
    const foundAccounts = await program.account.templateAccount.all(
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
    assert.equal(foundAccount.account.owner.toString(), owner.publicKey.toString());
  });

  it('can retrieve by a single address', async () => {
    const foundAccounts: any = await program.account.templateAccount.fetchMultiple([newKey.publicKey]);
    assert.equal(foundAccounts.length, 1);
  });

  it("can update a template", async () => {
    await program.methods.updateTemplate(true, 1).accounts({
      template: newKey.publicKey,
      payer: anchor.getProvider().wallet.publicKey,
      owner: owner.publicKey,
    }).signers([owner]).rpc();

    const foundAccount = await program.account.templateAccount.fetch(newKey.publicKey);
    assert.equal(foundAccount.archived, true);
  });

});

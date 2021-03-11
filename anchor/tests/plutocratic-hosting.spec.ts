import * as anchor from '@project-serum/anchor';
import * as serumCmn from '@project-serum/common';
import { TokenInstructions } from '@project-serum/serum';
import * as assert from "assert";

type PublicKey = anchor.web3.PublicKey;
type Account = anchor.web3.Account;

describe('plutocratic-hosting', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.PlutocraticHosting;

  let mint: PublicKey = null;
  let alice: PublicKey = null;
  let bobAccount: Account = null;
  let bob: PublicKey = null;
  let receiver: PublicKey = null;

  it("Sets up initial test state", async () => {
    const [_mint, _alice] = await serumCmn.createMintAndVault(
      program.provider,
      new anchor.BN(1000000)
    );
    mint = _mint;
    alice = _alice;

    receiver = await serumCmn.createTokenAccount(
      program.provider,
      mint,
      program.provider.wallet.publicKey
    );

    bobAccount = new anchor.web3.Account();

    bob = await serumCmn.createTokenAccount(
      program.provider,
      mint,
      bobAccount.publicKey
    );
  });

  const content = new anchor.web3.Account();
  const vault = new anchor.web3.Account();

  let contractSigner: PublicKey = null;

  it("Purchase new content", async () => {
    let [_contractSigner, nonce] = await anchor.web3.PublicKey.findProgramAddress(
      [content.publicKey.toBuffer()],
      program.programId
    );
    contractSigner = _contractSigner;

    await program.rpc.initialize(new anchor.BN(2), "tcontent", nonce, {
      accounts: {
        content: content.publicKey,
        vault: vault.publicKey,
        contractSigner,
        from: alice,
        owner: program.provider.wallet.publicKey,
        tokenProgram: TokenInstructions.TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
      signers: [content, vault],
      instructions: [
        // Initialize contract in instructions, which will be executed before the transaction
        await program.account.contentRecord.createInstruction(content, 300),

        // Initialize vault account.
        ...(await serumCmn.createTokenAccountInstrs(
          program.provider,
          vault.publicKey,
          mint,
          contractSigner
        )),
      ],
    });

    const contentAccount = await program.account.contentRecord(content.publicKey);
    assert.ok(contentAccount.owner.equals(alice));
    assert.ok(contentAccount.price.eq(new anchor.BN(2)));
    assert.ok(contentAccount.content === "tcontent");
    assert.ok(contentAccount.vault.equals(vault.publicKey));
    assert.ok(contentAccount.nonce === nonce);

    let vaultAccount = await serumCmn.getTokenAccount(
      program.provider,
      contentAccount.vault
    );
    assert.ok(vaultAccount.amount.eq(new anchor.BN(2)));
  });
});

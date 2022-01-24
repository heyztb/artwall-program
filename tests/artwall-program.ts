import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { ArtwallProgram } from '../target/types/artwall_program';

describe('artwall-program', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.ArtwallProgram as Program<ArtwallProgram>;
  const account = anchor.web3.Keypair.generate();
  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({
      accounts: {
        artwallAccount: account.publicKey,
        user: anchor.getProvider().wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [account]
    });
    console.log("Your transaction signature", tx);
  });
});

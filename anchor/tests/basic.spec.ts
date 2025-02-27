import * as anchor from '@coral-xyz/anchor';
import { startAnchor } from 'solana-bankrun'
import { BankrunProvider } from 'anchor-bankrun'
import { Program } from '@coral-xyz/anchor';
import { Basic } from '../target/types/basic';
import { Voting } from '../target/types/voting';
import { PublicKey } from '@solana/web3.js';

const IDL = JSON.parse(require('fs').readFileSync('./target/idl/voting.json', 'utf8'))
const votingAddress = new PublicKey("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF")

describe('Voting', () => {
 
  it('Should initialize poll', async () => {
    const context = await startAnchor("", [{ name: "voting", programId: votingAddress}], [])
    const provider = new BankrunProvider(context)

    const votingProgram = new Program<Voting>(
      IDL,
      provider
    )

    await votingProgram.methods.initializePoll(
      new anchor.BN(1),
      "Qual seu cachorro preferido?",
      new anchor.BN(0),
      new anchor.BN(2740615090)
    ).rpc()
  });
});

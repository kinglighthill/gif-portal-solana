// const anchor = require('@project-serum/anchor');

// describe('first-solana-project', () => {

//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.Provider.env());

//   it('Is initialized!', async () => {
//     // Add your test here.
//     const program = anchor.workspace.FirstSolanaProject;
//     const tx = await program.rpc.initialize();
//     console.log("Your transaction signature", tx);
//   });
// });


const anchor = require('@project-serum/anchor')

const { SystemProgram } = anchor.web3

const main = async () => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.FirstSolanaProject

  const baseAccount = anchor.web3.Keypair.generate()
 
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    }, 
    signers: [baseAccount]
  })

  console.log("📝 Your transaction signature", tx)

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log('👀 GIF Count', account.totalGifs.toString())

  await program.rpc.addGif("https://giphy.com/gifs/naruto-shippuden-2y98KScHKeaQM", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey)
  console.log('👀 GIF Count', account.totalGifs.toString())

  console.log('👀 GIF List', account.gifList)
}

const runMain = async () => {
  try {
    await main()
    process.exit(0)
  } catch (error) {
    console.error(error)
    process.exit(1)
  }
}

runMain()
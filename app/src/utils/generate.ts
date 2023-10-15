const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

// Initialize provider and program
const provider = anchor.Provider.local();
const program = new anchor.Program("7hWRveYD6k46EJTxS5W2bnSDh7fCMiRVbBxrLdW12bqJ", "/Users/aswin/Downloads/loansol.json", provider);

// Generate the USDC Mint (or use an existing one)
const usdcMint = anchor.web3.Keypair.generate();

// Generate the lender's keypair (or use an existing one)
const lender = anchor.web3.Keypair.generate();

// Generate the loan escrow address
const loanId = new anchor.BN(1); // or whatever your loan id is
const loan = async() => {
return await anchor.web3.PublicKey.findProgramAddress(
  [Buffer.from("loan_escrow"), loanId.toArrayLike(Buffer, "le", 8)],
  program.programId
);
}
const [loanEscrow, bump] = await loan();

// Generate the loan escrow's associated token account for USDC
const loanEscrowUsdcAta = await token.getAssociatedTokenAddress(
  associatedProgramId, // usually 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
  programId,
  usdcMint,
  loanEscrow,
  true
);

// Generate the lender's associated token account for USDC
const lenderUsdcAta = await token.getAssociatedTokenAddress(
  associatedProgramId, // usually 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
  programId,
  usdcMint,
  lender.publicKey,
  true
);

console.log("usdcMint:", usdcMint.publicKey.toString());
console.log("loanEscrow:", loanEscrow.toString());
console.log("loanEscrowUsdcAta:", loanEscrowUsdcAta.toString());
console.log("lender:", lender.publicKey.toString());
console.log("lenderUsdcAta:", lenderUsdcAta.toString());
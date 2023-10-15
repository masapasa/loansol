const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;
const provider = anchor.Provider.local();
const program = new anchor.Program("7hWRveYD6k46EJTxS5W2bnSDh7fCMiRVbBxrLdW12bqJ", "/Users/aswin/Downloads/loansol.json", provider);

const usdcMint = anchor.web3.Keypair.generate();
const lender = anchor.web3.Keypair.generate();
const loanId = new anchor.BN(1);
const loan = async() => {
return await anchor.web3.PublicKey.findProgramAddress(
  [Buffer.from("loan_escrow"), loanId.toArrayLike(Buffer, "le", 8)],
  program.programId
);
}
const [loanEscrow, bump] = await loan();
const loanEscrowUsdcAta = await token.getAssociatedTokenAddress(
  associatedProgramId,
  programId,
  usdcMint,
  loanEscrow,
  true
);
const lenderUsdcAta = await token.getAssociatedTokenAddress(
  associatedProgramId,
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
export {}
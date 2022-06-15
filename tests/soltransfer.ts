import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Soltransfer } from "../target/types/soltransfer";

describe("soltransfer", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);
	const wallet = provider.wallet;
	const systemProgram = anchor.web3.SystemProgram;
	const program = anchor.workspace.Soltransfer as Program<Soltransfer>;
	const sendWallet = anchor.web3.Keypair.generate();
	const lamps_per_sol = anchor.web3.LAMPORTS_PER_SOL;

	// it("Is sends sol!", async () => {
	//   // Add your test here.
	//   const tx = await program.methods.sendSol(new anchor.BN(0.05 * lamps_per_sol)).accounts({
	//     sender: wallet.publicKey,
	//     reciever: "4vnihPiwkRbNNa5WJCzPbp27exRGz3Lm1dzfaLXEEHcJ",
	//     systemProgram: SystemProgram.programId
	//   }).rpc();

	//   console.log("Your transaction signature", tx);
	//   console.log(sendWallet)
	// });
	// it("Creates PDA for the money to be sent", async () => {
	// 	const [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
	// 		[Buffer.from(anchor.utils.bytes.utf8.encode("sendsol"))],
	// 		program.programId,
	// 	);
	// 	console.log(pda);
	// 	let txn = await program.methods
	// 		.initAccount()
	// 		.accounts({
	// 			authority: wallet.publicKey,
	// 			programAccount: pda,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();
	// });
	// it("Sends money to the pda and check balance", async () => {
	// 	const [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
	// 		[Buffer.from(anchor.utils.bytes.utf8.encode("sendsol"))],
	// 		program.programId,
	// 	);
	// 	console.log(pda.toString());
	// 	let txn = await program.methods
	// 		.sendSolToPda(new anchor.BN(0.05 * lamps_per_sol))
	// 		.accounts({
	// 			sender: wallet.publicKey,
	// 			programAccount: pda,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();
	// });
	// it("Withdraws money", async () => {
	// 	const [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
	// 		[Buffer.from(anchor.utils.bytes.utf8.encode("sendsol"))],
	// 		program.programId,
	// 	);
	// 	console.log(pda.toString());
	// 	let txn = await program.methods
	// 		.withdrawFromPda(new anchor.BN(0.05 * lamps_per_sol))
	// 		.accounts({
	// 			programAccount: pda,
	// 			reciever: wallet.publicKey,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();
	// });
	it("Is sends sol to a system pda!", async () => {
		// Add your test here.
		const [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(anchor.utils.bytes.utf8.encode("newseed"))],
			program.programId,
		);
		const tx = await program.methods
			.sendSysPdaSol(new anchor.BN(2 * lamps_per_sol))
			.accounts({
				senderAccount: wallet.publicKey,
				programAccount: pda,
				systemProgram: SystemProgram.programId,
			})
			.rpc();

		console.log("Your transaction signature", tx);
		// console.log(sendWallet);
	});
	it("Is withdraws sol from a system pda!", async () => {
		// Add your test here.
		const [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from(anchor.utils.bytes.utf8.encode("newseed"))],
			program.programId,
		);
		const tx = await program.methods
			.withdrawSolSysPda(new anchor.BN(0.5 * lamps_per_sol))
			.accounts({
				recieverAccount: wallet.publicKey,
				programAccount: pda,
				systemProgram: SystemProgram.programId,
			})
			.rpc();

		console.log("Your transaction signature", tx);
		// console.log(sendWallet);
	});
});

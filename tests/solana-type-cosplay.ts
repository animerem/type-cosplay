import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { TypeCosplay } from "../target/types/type_cosplay"
import { expect } from "chai"

describe("type-cosplay", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)

  const program = anchor.workspace.TypeCosplay as Program<TypeCosplay>

  const userAccount = anchor.web3.Keypair.generate()
  const newAdmin = anchor.web3.Keypair.generate()

  it("Initialize User Account", async () => {
    await program.methods
      .initializeUser()
      .accounts({
        newAccount: userAccount.publicKey,
      })
      .signers([userAccount])
      .rpc()
  })

  it("Invoke update admin instruction with user account", async () => {
    await program.methods
      .updateAdmin()
      .accounts({
        adminConfig: userAccount.publicKey,
        newAdmin: newAdmin.publicKey,
      })
      .rpc()
  })
})

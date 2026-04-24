import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solanavic } from "../target/types/solanavic";

describe("solanavic", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solanavic as Program<Solanavic>;

  it("¡Crea un folio de servicio con éxito!", async () => {
    const serviceId = new anchor.BN(1); // Folio #1
    
    // Derivamos la dirección única (PDA) para este folio
    const [servicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("service"),
        provider.wallet.publicKey.toBuffer(),
        serviceId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Llamamos a la función del contrato
    await program.methods
      .createService(serviceId, "Cliente Ejemplo", "Soporte Técnico", new anchor.BN(500))
      .accounts({
        serviceRecord: servicePda,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    console.log("Servicio creado en la dirección:", servicePda.toBase58());
  });
});
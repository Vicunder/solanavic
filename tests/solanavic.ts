// 1. IMPORTACIONES
// Traemos las librerías necesarias. Anchor actúa como un "traductor" 
// entre el mundo de JavaScript/TypeScript y el mundo de Rust/Solana.
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solanavic } from "../target/types/solanavic"; // Importamos los tipos generados de tu contrato

describe("Suite de Pruebas: Service Ledger", () => {
  // 2. CONFIGURACIÓN DEL ENTORNO (PROVIDER)
  // El Provider lee el archivo de configuración de tu computadora (~/.config/solana/id.json)
  // Esto simula que el usuario ha conectado su "Phantom Wallet" a tu aplicación.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 3. CONEXIÓN AL PROGRAMA
  // 'workspace' carga tu programa compilado usando el archivo IDL (el mapa de tus funciones).
  // Así TypeScript sabe que existe una función llamada 'createService'.
  const program = anchor.workspace.Solanavic as Program<Solanavic>;

  it("Debe crear un folio nuevo y luego leer sus datos desde la Blockchain", async () => {
    
    // 4. PREPARACIÓN DE DATOS
    // Generamos un ID aleatorio para evitar el error de "Cuenta ya en uso" si corres el test varias veces.
    // Usamos 'BN' (BigNumber) porque la blockchain usa números más grandes de los que JS puede manejar nativamente.
    const randomId = Math.floor(Math.random() * 100000);
    const serviceId = new anchor.BN(randomId); 
    
    // 5. DERIVACIÓN DE LA PDA (Program Derived Address)
    // ¡Ojo! Aquí NO estamos creando la cuenta, solo estamos "calculando" matemáticamente
    // cuál SERÁ su dirección usando las mismas "semillas" (seeds) que definimos en Rust.
    const [servicePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("service"), // Semilla 1: La palabra "service" en bytes
        provider.wallet.publicKey.toBuffer(), // Semilla 2: La llave pública de tu wallet en bytes
        serviceId.toArrayLike(Buffer, "le", 8), // Semilla 3: El número de folio en formato Little-Endian
      ],
      program.programId // La dirección de tu contrato inteligente
    );

    console.log(`\n--- INICIANDO SIMULACIÓN PARA EL FOLIO #${randomId} ---`);
    console.log("Dirección PDA calculada:", servicePda.toBase58());

    // ==========================================
    // OPERACIÓN 1: CREATE (Escribir en Blockchain)
    // ==========================================
    console.log("\n[1/2] Enviando transacción a la red (Create)...");
    
    // .methods accede a las funciones de Rust.
    // Pasamos los argumentos exactamente en el orden que los pide 'create_service' en lib.rs
    await program.methods
      .createService(serviceId, "Cliente VIP Grudisa", "Migración de Servidor Linux", new anchor.BN(4500))
      .accounts({
        serviceRecord: servicePda, // Le decimos dónde guardar los datos (la dirección que calculamos)
        admin: provider.wallet.publicKey, // Quién firma y paga
      })
      .rpc(); // .rpc() es el "botón de enviar". Empaqueta todo y lo manda a la red de Solana.

    console.log("¡Transacción confirmada! Registro guardado de forma inmutable.");

    // ==========================================
    // OPERACIÓN 2: READ (Leer de Blockchain)
    // ==========================================
    console.log("\n[2/2] Consultando la red (Read)...");
    
    // En lugar de llamar a una función (.methods), leemos directamente el estado de la cuenta.
    // .fetch() descarga los bytes de la dirección 'servicePda' y los convierte a texto legible.
    const accountData = await program.account.serviceRecord.fetch(servicePda);

    console.log("Datos extraídos directamente de la cuenta:");
    console.log("---------------------------------------------------");
    console.log("NÚMERO DE FOLIO: ", accountData.serviceId.toString());
    console.log("CLIENTE:         ", accountData.clientName);
    console.log("TIPO DE TRABAJO: ", accountData.serviceType);
    console.log("ESTATUS ACTUAL:  ", accountData.status);
    console.log("PRECIO ACORDADO: ", accountData.price.toString(), "MXN");
    console.log("---------------------------------------------------");
  });
});
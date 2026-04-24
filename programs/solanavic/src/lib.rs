// 1. IMPORTACIONES: Traemos las herramientas de Anchor que necesitamos.
// Anchor pre-empaqueta muchas funciones complejas de Solana para hacernos la vida fácil.
use anchor_lang::prelude::*;

// 2. IDENTIFICADOR DEL PROGRAMA (PROGRAM ID)
// Esta es la "matrícula" pública de tu contrato inteligente en la red. 
// ¡AQUÍ DEBES PONER TU DIRECCIÓN REAL QUE OBTUVISTE CON SOLANA ADDRESS!
declare_id!("5KJ8hjNBAQH8Ert7QKwW17d5jqroNy1qotjgVfBhj25n");

// 3. EL MÓDULO PRINCIPAL (LA LÓGICA DEL PROGRAMA)
// #[program] es un "macro" (una etiqueta mágica de Anchor) que le dice al compilador: 
// "Lo que está aquí adentro son las funciones que la gente puede ejecutar".
#[program]
pub mod solanavic {
    use super::*;

    // --- FUNCION 1: CREAR EL REGISTRO (CREATE) ---
    // Esta función inicializa el "folio" del servicio.
    pub fn create_service(
        ctx: Context<CreateService>, // Contexto: trae las cuentas involucradas
        service_id: u64,             // Un número único de folio (ej. 1, 2, 3...)
        client_name: String,         // El nombre de tu cliente
        service_type: String,        // Ej: "Instalación CCTV", "Soporte Técnico"
        price: u64,                  // Precio del servicio
    ) -> Result<()> {
        
        // Tomamos la cuenta vacía que Anchor acaba de crear para nosotros
        let service_record = &mut ctx.accounts.service_record;

        // Llenamos la cuenta con los datos que nos enviaron:
        // Guardamos quién es el administrador (Tú)
        service_record.admin = *ctx.accounts.admin.key; 
        // Guardamos el folio
        service_record.service_id = service_id;
        // Guardamos el nombre del cliente
        service_record.client_name = client_name;
        // Guardamos de qué trató el servicio
        service_record.service_type = service_type;
        // Al crear, el estatus siempre inicia en "Pendiente" por defecto
        service_record.status = String::from("Pendiente");
        // Guardamos el precio
        service_record.price = price;
        // Guardamos el "bump" (un número de seguridad matemático que usa la PDA)
        service_record.bump = ctx.bumps.service_record;

        // Confirmamos que todo salió bien
        Ok(())
    }

    // --- FUNCION 2: ACTUALIZAR EL ESTATUS (UPDATE) ---
    // Cuando terminas el trabajo, usas esto para cambiar a "Completado".
    pub fn update_status(
        ctx: Context<UpdateService>, 
        new_status: String // El nuevo estado, ej: "Completado"
    ) -> Result<()> {
        
        // Accedemos al registro que ya existe
        let service_record = &mut ctx.accounts.service_record;
        
        // Reemplazamos el estatus viejo por el nuevo
        service_record.status = new_status;

        Ok(())
    }

    // --- FUNCION 3: BORRAR EL REGISTRO (DELETE) ---
    // Elimina el registro de la blockchain y te devuelve la renta (SOL).
    // Nota: La lógica real de borrado ocurre en la estructura "DeleteService" de abajo,
    // por eso la función aquí adentro casi no lleva código extra.
    pub fn delete_service(_ctx: Context<DeleteService>) -> Result<()> {
        // Al ejecutar esta función, Anchor automáticamente cierra la cuenta 
        // y envía el dinero al admin gracias a la etiqueta "close = admin" de abajo.
        Ok(())
    }
}

// -------------------------------------------------------------------------
// 4. ESTRUCTURAS DE CONTEXTO (VALIDACIONES Y SEGURIDAD)
// Aquí le decimos a Solana qué cuentas pueden ejecutar cada función y cómo
// se deben crear las direcciones derivadas (PDAs).
// -------------------------------------------------------------------------

// Contexto para Crear:
#[derive(Accounts)]
// Necesitamos que pasen el folio (service_id) para generar la dirección única.
#[instruction(service_id: u64)] 
pub struct CreateService<'info> {
    // Aquí definimos la magia de la PDA (Program Derived Address).
    // init: Crea la cuenta por primera vez.
    // payer = admin: El admin (tú) paga la renta inicial.
    // space = 200: Reservamos 200 bytes de espacio para guardar los textos y números.
    // seeds: Los ingredientes para crear la dirección única. Usamos la palabra "service", tu llave pública y el folio.
    // bump: Anchor calcula automáticamente el número de seguridad de la semilla.
    #[account(
        init,
        payer = admin,
        space = 200, 
        seeds = [b"service", admin.key().as_ref(), &service_id.to_le_bytes()],
        bump
    )]
    pub service_record: Account<'info, ServiceRecord>,

    // Esta es tu cuenta, debe firmar la transacción (mut significa mutable, porque se restará dinero para pagar la renta).
    #[account(mut)]
    pub admin: Signer<'info>,

    // Herramienta interna de Solana necesaria para crear cuentas nuevas.
    pub system_program: Program<'info, System>,
}

// Contexto para Actualizar:
#[derive(Accounts)]
pub struct UpdateService<'info> {
    // mut: Significa que vamos a modificar (mutar) los datos de esta cuenta.
    // has_one = admin: ¡SEGURIDAD! Valida que la persona que intenta actualizar 
    // sea exactamente la misma persona que creó el registro.
    #[account(mut, has_one = admin)]
    pub service_record: Account<'info, ServiceRecord>,

    pub admin: Signer<'info>,
}

// Contexto para Borrar:
#[derive(Accounts)]
pub struct DeleteService<'info> {
    // mut: Vamos a modificarla (para cerrarla).
    // close = admin: ¡LA MAGIA DEL BORRADO! Le dice a Anchor que destruya la cuenta 
    // y le regrese los SOL (Lamports) sobrantes de la renta al "admin".
    // has_one = admin: Solo el creador puede borrarla.
    #[account(mut, close = admin, has_one = admin)]
    pub service_record: Account<'info, ServiceRecord>,

    pub admin: Signer<'info>,
}

// -------------------------------------------------------------------------
// 5. LA ESTRUCTURA DE DATOS (EL ESTADO / LA BASE DE DATOS)
// Así es como se ve "físicamente" nuestro registro guardado en la blockchain.
// -------------------------------------------------------------------------
#[account]
pub struct ServiceRecord {
    pub admin: Pubkey,         // 32 bytes - La dirección de tu wallet
    pub service_id: u64,       // 8 bytes  - El número de folio
    pub client_name: String,   // ~50 bytes - Nombre del cliente
    pub service_type: String,  // ~50 bytes - Tipo (CCTV, Redes, etc.)
    pub status: String,        // ~20 bytes - Estado (Pendiente / Completado)
    pub price: u64,            // 8 bytes  - Precio del servicio
    pub bump: u8,              // 1 byte   - Seguridad de la PDA
}
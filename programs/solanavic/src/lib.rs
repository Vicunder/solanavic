// 1. IMPORTACIONES BÁSICAS
// Traemos todas las herramientas y macros (atajos de código) del framework Anchor.
// Esto incluye funciones para manejar cuentas, claves públicas y errores.
use anchor_lang::prelude::*;

// 2. IDENTIFICADOR DEL PROGRAMA (PROGRAM ID)
// Esta es la "dirección pública" de tu contrato inteligente en la red Devnet.
// Reemplázala con tu ID real (ej. )
declare_id!("5KJ8hjNBAQH8Ert7QKwW17d5jqroNy1qotjgVfBhj25n");
             

// 3. MÓDULO PRINCIPAL DEL PROGRAMA (LA LÓGICA)
// La macro #[program] le dice a Anchor que aquí viven las instrucciones (funciones)
// que los usuarios podrán ejecutar desde el exterior (como desde tu futuro Frontend).
#[program]
pub mod solanavic {
    use super::*;

    // --- C (CREATE): Crear un folio de servicio ---
    // ctx: Contiene las cuentas involucradas en esta transacción.
    // Los demás parámetros son los datos que el usuario manda desde la web.
    pub fn create_service(
        ctx: Context<CreateService>, 
        service_id: u64,             
        client_name: String,         
        service_type: String,        
        price: u64,                  
    ) -> Result<()> {
        // Accedemos a la cuenta recién creada usando una referencia mutable (&mut)
        // porque vamos a escribir datos dentro de ella.
        let service_record = &mut ctx.accounts.service_record;

        // Asignamos los datos recibidos a los campos de nuestra estructura (nuestra "tabla")
        service_record.admin = *ctx.accounts.admin.key;       // Guardamos la llave de quien paga/crea
        service_record.service_id = service_id;               // Número de folio (ej. 101)
        service_record.client_name = client_name;             // "Cliente Grudisa", etc.
        service_record.service_type = service_type;           // "Mantenimiento CCTV", etc.
        service_record.status = String::from("Pendiente");    // Por defecto, todo servicio inicia así
        service_record.price = price;                         // Costo en unidades (o MXN simulado)
        
        // Guardamos el "bump". Es un número (0-255) que Solana usa matemáticamente 
        // para asegurar que esta dirección PDA no tenga una clave privada (es in-hackeable).
        service_record.bump = ctx.bumps.service_record;       

        // msg! imprime un registro en la blockchain (útil para auditorías en el explorador)
        msg!("Servicio IT Creado: Folio {}", service_id); 
        Ok(()) // Retornamos Ok() para indicar que la transacción fue exitosa
    }

    // --- R (READ): Consultar el servicio ---
    // Aunque la lectura se hace directamente viendo la cuenta, tener esta función
    // permite crear un endpoint formal. No modificamos nada, solo validamos que exista.
    pub fn get_service(_ctx: Context<GetService>) -> Result<()> {
        msg!("Consulta de folio exitosa en el sistema");
        Ok(())
    }

    // --- U (UPDATE): Actualizar estado ---
    // Permite cambiar el estatus de "Pendiente" a "Completado" o "Cancelado" o "En Proceso".
    pub fn update_status(ctx: Context<UpdateService>, new_status: String) -> Result<()> {
        let service_record = &mut ctx.accounts.service_record;
        service_record.status = new_status; // Sobrescribimos el valor anterior
        msg!("Folio actualizado a estatus: {}", service_record.status);
        Ok(())
    }

    // --- D (DELETE): Eliminar registro ---
    // No necesitamos escribir lógica aquí. La magia ocurre en la estructura DeleteService,
    // donde la instrucción 'close' le dice a Solana que borre la cuenta y devuelva el dinero.
    pub fn delete_service(_ctx: Context<DeleteService>) -> Result<()> {
        msg!("Folio cerrado exitosamente. Renta de SOL recuperada.");
        Ok(())
    }
}

// 4. ESTRUCTURAS DE VALIDACIÓN (CONTEXTOS)
// Aquí definimos QUÉ cuentas se necesitan para cada función y sus REGLAS de seguridad.

// Reglas para Crear (Create)
#[derive(Accounts)]
#[instruction(service_id: u64)] // Pasamos el service_id para usarlo en la semilla de la PDA
pub struct CreateService<'info> {
    #[account(
        init, // Indica que Anchor debe crear esta cuenta desde cero
        payer = admin, // ¿Quién paga el costo de espacio en la red (renta)? El admin.
        // space = 8 (discriminador obligatorio de Anchor) + tamaño de los datos
        // 32(Pubkey) + 8(u64) + 60(String) + 60(String) + 20(String) + 8(u64) + 1(u8)
        space = 8 + 32 + 8 + 60 + 60 + 20 + 8 + 1, 
        // seeds: La "receta" única para generar la dirección PDA de este folio específico.
        // Si alguien intenta crear el mismo folio 2 veces, Solana lo bloqueará.
        seeds = [b"service", admin.key().as_ref(), &service_id.to_le_bytes()], 
        bump // Anchor calcula y valida el bump automáticamente
    )]
    pub service_record: Account<'info, ServiceRecord>, // La cuenta donde se guardará la info
    
    #[account(mut)] // mut = mutable. El saldo del admin cambiará porque pagará la transacción
    pub admin: Signer<'info>, // Debe firmar la transacción con su wallet privada
    
    pub system_program: Program<'info, System>, // Programa base de Solana que crea las cuentas físicas
}

// Reglas para Leer (Read)
#[derive(Accounts)]
pub struct GetService<'info> {
    // Solo leemos, por lo que NO usamos 'init' ni 'mut'.
    pub service_record: Account<'info, ServiceRecord>,
}

// Reglas para Actualizar (Update)
#[derive(Accounts)]
pub struct UpdateService<'info> {
    #[account(
        mut, // Necesitamos modificar los datos dentro de la cuenta
        has_one = admin // SEGURIDAD CRÍTICA: Verifica que la wallet que intenta actualizar sea la misma que creó el registro.
    )] 
    pub service_record: Account<'info, ServiceRecord>,
    pub admin: Signer<'info>, // El admin debe firmar para autorizar el cambio
}

// Reglas para Borrar (Delete)
#[derive(Accounts)]
pub struct DeleteService<'info> {
    #[account(
        mut, 
        close = admin, // CIERRE AUTOMÁTICO: Destruye la cuenta y transfiere el SOL de reserva al 'admin'
        has_one = admin // Solo el dueño puede borrarlo
    )] 
    pub service_record: Account<'info, ServiceRecord>,
    pub admin: Signer<'info>,
}

// 5. DEFINICIÓN DE LA CUENTA (EL "MODELO DE DATOS")
// Así es como se estructuran los bytes físicamente dentro de la blockchain.
#[account]
pub struct ServiceRecord {
    pub admin: Pubkey,         // Llave pública del creador (32 bytes)
    pub service_id: u64,       // ID del folio, u64 es un número entero sin signo (8 bytes)
    pub client_name: String,   // Texto dinámico para el nombre del cliente
    pub service_type: String,  // Texto para clasificar el trabajo técnico
    pub status: String,        // Estado actual del trabajo
    pub price: u64,            // Valor del servicio (en números enteros)
    pub bump: u8,              // Número verificador de la PDA (1 byte)
}
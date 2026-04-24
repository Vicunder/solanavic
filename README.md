# Service Ledger 🛠️

Este es el backend de un sistema de gestión de servicios IT desarrollado para la red de Solana. 
Permite crear folios de servicio inmutables, actualizar su estado y gestionar el ciclo de vida técnico.

## 🚀 Características Técnicas
- **Framework:** Anchor v0.30.1
- **Lenguaje:** Rust
- **Lógica de Datos:** Implementación de **PDA** (Program Derived Addresses) para organizar folios por usuario.
- **Operaciones CRUD:**
  - `create_service`: Inicializa un folio con datos del cliente y precio.
  - `update_status`: Permite cambiar el estado (ej. de Pendiente a Completado).
  - `delete_service`: Cierra la cuenta y recupera el SOL de la renta.

## 🛠️ Cómo probarlo
1. Clonar el repositorio.
2. Ejecutar `anchor build`.
3. Ejecutar `anchor test`.

**Program ID en Devnet:** 5KJ8hJNBAQH8Ert7QKww17d5jqroNy1qotjgVfBhj25n
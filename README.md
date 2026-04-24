# Service Ledger - Solana CRUD con PDA

Este proyecto es un Sistema de Gestión de Folios de Servicio desarrollado en la blockchain de Solana utilizando el framework **Anchor**. Permite a una entidad técnica registrar, consultar, actualizar y finalizar órdenes de servicio de forma inmutable y transparente.

## Arquitectura Técnica

El programa implementa un ciclo de vida **CRUD** completo, optimizado para la eficiencia en almacenamiento y seguridad.

### Uso de PDAs (Program Derived Addresses)
En lugar de usar cuentas generadas al azar, este proyecto utiliza **PDAs** para localizar los registros de servicio de manera determinista.
* **Semillas utilizadas:** `[b"service", admin_pubkey, service_id_bytes]`
* **Beneficio:** Esto garantiza que cada `service_id` generado por el administrador tenga una dirección única y recuperable sin necesidad de guardar un mapeo externo.

### Funcionalidades (CRUD)
1. **Create (Crear):** Inicializa una cuenta de servicio en la red, calculando el espacio exacto en bytes para optimizar el pago de renta (Lamports).
2. **Read (Leer):** Recupera la información estructurada desde la blockchain (Cliente, Tipo de Servicio, Estatus, Precio).
3. **Update (Actualizar):** Permite modificar el estatus del folio (ej. de "Pendiente" a "Completado") validando que solo el `admin` autorizado pueda realizar cambios.
4. **Delete (Borrar/Cerrar):** Implementa la instrucción `close`, la cual elimina la cuenta de la red y reembolsa los Lamports de la renta al administrador, manteniendo la blockchain limpia.

## Tecnologías Utilizadas
* **Rust & Anchor Framework:** Lógica del Smart Contract.
* **TypeScript:** Suite de pruebas y simulación de transacciones.
* **Solana Devnet:** Entorno de despliegue y pruebas reales.

## Pruebas y Despliegue
El proyecto ha sido validado exitosamente en la **Devnet** de Solana.

Para replicar las pruebas:
1. Sincronizar Program ID en `lib.rs` y `Anchor.toml`.
2. Ejecutar `anchor build`.
3. Ejecutar `anchor deploy` para subir el código a la nube.
4. Ejecutar `anchor test --skip-deploy` para verificar el ciclo CRUD.

**Program ID en Devnet:** 5KJ8hJNBAQH8Ert7QKww17d5jqroNy1qotjgVfBhj25n
---
*Desarrollado por Vic - Edo. de México - 2026*

//! Camada de boot: a única parte do kernel que conhece a API do
//! bootloader (hoje, o Limine).

use limine::BaseRevision;

/// Requisição de "base revision" exigida pelo protocolo de boot do Limine
/// v6 — é o pedido que o bootloader lê para saber que versão do protocolo
/// este kernel fala.
///
/// - `#[used]` evita que o compilador remova esta variável achando que
///   ela não é usada.
/// - `#[link_section = ".limine_requests"]` coloca a variável na seção do
///   binário reservada pelo `linker.ld` para os pedidos do Limine.
#[used]
#[link_section = ".limine_requests"]
static BASE_REVISION: BaseRevision = BaseRevision::new();

/// Verifica se o bootloader que carregou o kernel suporta a revisão do
/// protocolo que pedimos acima.
///
/// Retorna `true` se o boot pode prosseguir normalmente, ou `false` caso
/// o bootloader seja incompatível.
pub fn is_supported() -> bool {
    BASE_REVISION.is_supported()
}

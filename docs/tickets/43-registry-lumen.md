# Especificação Normativa — Ticket #43
## Migração das Dependências Satélites do Joaju para o Registro Soberano Lumen e Publicação

**Norma de Referência:** GOST v1.5.0 (Perfil Lumen — Seção 9)  
**Status:** Aprovado para Implementação  
**Data:** 2026-10-20  

---

### 1. Escopo e Objetivos

Esta especificação define a transição arquitetural do compilador orquestrador **`joaju`** para consumir todas as suas dependências satélites através do registro soberano de pacotes Cargo no Forgejo central (`https://gambi.2lp.in/api/packages/lumen/cargo/`):

1. **Desacoplamento Completo de Caminhos Locais:**
   - Remoção de todos os `path = "../..."` no `Cargo.toml`.
   - Vinculação ao registro soberano `lumen`:
     ```toml
     joaju-pedigree = { version = "0.1.0", registry = "lumen" }
     joaju-csv = { version = "0.1.0", registry = "lumen" }
     joaju-table = { version = "0.1.0", registry = "lumen" }
     joaju-unfurl = { version = "0.1.0", registry = "lumen" }
     ```
2. **Resolução Hermética de Dependências:**
   - Verificação de compilação sem necessidade de diretórios irmãos locais.
3. **Publicação no Registro Soberano:**
   - Publicação da versão `0.1.0` da crate orquestradora `joaju` no registro `lumen`.

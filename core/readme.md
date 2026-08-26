# core/

Núcleo específico do projeto.

Tudo que for próprio do domínio (código, modelos, schemas, componentes, ativos, firmware, CAD, etc.) deve viver aqui — sempre que isso não contrariar o padrão da tecnologia.

## Exemplos por domínio

| Domínio | Subpastas possíveis |
|---|---|
| Software | `src/`, `tests/`, `api/`, `packages/` |
| Hardware | `cad/`, `bom/`, `schematics/`, `firmware/` |
| Sistema operacional | `profiles/`, `policies/`, `services/` |
| Impressão 3D | `models/`, `slicing/`, `materials/` |
| Híbrido | combinação das acima |

## Regra

A governança do repositório fica na raiz; o produto/domínio fica em `/core`.

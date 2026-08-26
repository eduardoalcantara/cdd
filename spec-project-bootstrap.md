# spec-root-repo-build.md

## Objetivo

Este documento define a fundação universal para criação de qualquer novo repositório no Cursor.

Ele serve para projetos de software, hardware, configuração de sistema operacional, automação, impressão 3D, documentação técnica, pesquisa, ferramentas, bibliotecas, protótipos ou qualquer combinação desses domínios.

A meta é padronizar a raiz do repositório com estrutura, governança, contexto operacional e orientação suficiente para que o Cursor consiga trabalhar com consistência desde o primeiro momento.

---

## 1. Princípios centrais

### 1.1 Repositório nasce documentado
Todo projeto deve começar com uma estrutura mínima de contexto, não com arquivos soltos sem governança.

### 1.2 A raiz é a fonte de orientação
Os arquivos da raiz não são enfeites; cada um tem função operacional específica.

### 1.3 O agente precisa de contexto explícito
O Cursor deve encontrar no repositório as regras, o fluxo, o estado, o histórico e a orientação de trabalho.

### 1.4 O formato é universal, o conteúdo é adaptável
A espinha dorsal se mantém; o interior das pastas muda conforme o domínio do projeto.

### 1.5 Existe uma pasta `/core`
Todo projeto deve conter uma pasta `/core`, que concentra as pastas, arquivos e artefatos específicos do projeto.

### Regra obrigatória
Tudo que for específico do projeto deve viver sob `/core` sempre que isso não contrariar o padrão da tecnologia ou do domínio.

### 1.6 Existe um arquivo `.prompt-status`
Todo projeto deve conter um arquivo `.prompt-status` na raiz para rastrear a execução de cada prompt/tarefa do Cursor AI.

### 1.7 Rastreamento de execução é obrigatório
Antes de executar um prompt, o Cursor AI deve ler `.prompt-status`; ao iniciar e ao concluir a tarefa, deve atualizar esse arquivo com o estado atual, o LLM em uso e o resultado obtido.

---

## 2. Estrutura raiz padrão

Todo novo repositório deve possuir, sempre que aplicável, os seguintes itens na raiz:

- `.gitignore`
- `readme.md`
- `spec-root.md`
- `flow.md`
- `rules.md`
- `status.md`
- `timeline.md`
- `setup.md`
- `tools-linux.md`
- `tools-windows.md`
- `.cursorrules`
- `.prompt-status`
- `spec-template.md`
- `rules-scripts.md`
- `docs/`
- `ideas/`
- `specs/`
- `references/`
- `scripts/`
- `reports/`
- `prompts/`
- `resources/`
- `core/`

### Regra
Pastas opcionais podem não existir em todos os projetos, mas a intenção estrutural deve permanecer clara.

---

## 3. Função dos arquivos da raiz

### 3.1 `.gitignore`
Define o que não deve entrar no versionamento.

### 3.2 `readme.md`
Explica o projeto para humanos: propósito, visão, escopo e uso inicial.

### 3.3 `spec-root.md`
É a autoridade arquitetural máxima do repositório.

### 3.4 `flow.md`
Define o fluxo operacional do agente: o que ler, em que ordem agir e como validar.

### 3.5 `rules.md`
Centraliza regras permanentes de governança, escopo, arquitetura e qualidade.

### 3.6 `status.md`
Registra o estado atual do projeto como snapshot mutável.

### 3.7 `timeline.md`
Registra o histórico do projeto em ordem cronológica decrescente, como log evolutivo.

### 3.8 `setup.md`
Documenta preparação do ambiente, instalação, bootstrap e caminhos de suporte.

### 3.9 `tools-linux.md`
Lista ferramentas, pacotes e comandos relevantes para Linux.

### 3.10 `tools-windows.md`
Lista ferramentas, pacotes e comandos relevantes para Windows.

### 3.11 `.cursorrules`
Define comportamento, honestidade, limites e formato de resposta do agente no repositório.

### 3.12 `.prompt-status`
Rastreia o prompt em andamento, o último prompt concluído, o LLM utilizado, a duração e os acumulados de execução.

### 3.13 `spec-template.md`
Fornece o molde para novas specs do projeto.

### 3.14 `rules-scripts.md`
Define a governança de criação e execução de scripts, incluindo organização, input, cabeçalho padronizado e colorido, autodetecção da raiz, UX operacional (inclusive modos de automação) e desinstalação reversa.

### 3.15 `docs/`
Armazena documentação técnica, operacional, normativa e de produto.

### 3.16 `ideas/`
Armazena hipóteses, rascunhos, propostas e ideias ainda não formalizadas.

### 3.17 `specs/`
Armazena especificações formais do projeto, com sub-pastas `to-do/` e `done/`.

### 3.18 `references/`
Armazena referências locais, engenharia reversa, exemplos e materiais de estudo.

### 3.19 `scripts/`
Armazena scripts de automação, bootstrap, validação e apoio operacional.

### 3.20 `reports/`
Armazena relatórios de implementação, auditoria, validação e entrega.

### 3.21 `prompts/`
Armazena prompts reutilizáveis, colas e instruções mestre.

### 3.22 `resources/`
Armazena downloads, binários, imagens, apps auxiliares e materiais de apoio.

### 3.23 `core/`
Armazena o conteúdo específico do projeto, incluindo pastas de domínio, arquivos-fonte, ativos, modelos, componentes e artefatos operacionais próprios do repositório.

---

## 4. Estrutura correta de cada arquivo

### 4.1 `.gitignore`
- Ignorar artefatos temporários.
- Ignorar saídas de build, caches, logs e arquivos locais.
- Ignorar segredos, credenciais e arquivos específicos do ambiente.
- Não ignorar arquivos de governança do projeto.

### 4.2 `readme.md`
- Nome do projeto.
- Resumo do que ele faz.
- Público e objetivo.
- Estrutura de alto nível.
- Como iniciar.
- Documentação relacionada.
- Estado atual.

### 4.3 `spec-root.md`
- Propósito do projeto.
- Visão geral.
- Princípios fundacionais.
- Escopo.
- Fora de escopo.
- Arquitetura de alto nível.
- Contratos centrais.
- Regras permanentes.
- Critérios de sucesso.
- Conclusão normativa.

### 4.4 `flow.md`
- Ordem de leitura obrigatória.
- Fluxo de execução do agente.
- Ordem de decisões.
- Gate de confirmação.
- Checklist de execução.
- Passos de validação.
- Passos de encerramento.
- Leitura e atualização de `.prompt-status` no início e no fim de cada prompt.

### 4.5 `rules.md`
- Hierarquia normativa.
- Regras gerais permanentes.
- Regras de escopo.
- Regras de qualidade.
- Regras de documentação.
- Regras de validação.
- Regras de bloqueio.
- Regras de atualização.
- **CCIA**: Continuidade de Contexto para IA (Logs diários do chat na pasta `prompts/`).

### 4.6 `status.md`
- Data da última atualização.
- Resumo do estado atual.
- Tarefas concluídas.
- Tarefas pendentes.
- Riscos.
- Próximos passos.
- Mudanças recentes.

### 4.7 `timeline.md`
- Histórico cronológico reverso.
- Entrada por evento/entrega.
- Impacto.
- Arquivos afetados.
- Data.
- Observações relevantes.

### 4.8 `setup.md`
- Pré-requisitos.
- Instalação.
- Bootstrap.
- Variáveis de ambiente.
- Verificações.
- Execução inicial.
- Solução de problemas.

### 4.9 `tools-linux.md`
- Ferramentas necessárias.
- Comandos úteis.
- Dependências do sistema.
- Uso recomendado.
- Observações específicas de Linux.

### 4.10 `tools-windows.md`
- Ferramentas necessárias.
- Comandos úteis.
- Dependências do sistema.
- Uso recomendado.
- Observações específicas de Windows.

### 4.11 `.cursorrules`
- Perfil do agente.
- Comportamento esperado.
- Honestidade e bloqueio.
- Regras de resposta (incluindo rodapé obrigatório com metadados do `.prompt-status`).
- Regras de validação.
- Regras de escopo.
- CCIA: Continuidade de Contexto para IA (obrigação de registrar chat em `prompts/chat-YYYY-MM-DD-<hostname>.md`).
- Regras de atualização documental.
- Regras de dados mínimos por resposta.
- Regras de scripts e input.
- Regras de leitura e atualização de `.prompt-status`.

### 4.12 `.prompt-status`
- Prompt em andamento.
- Último prompt concluído.
- LLM atual e LLM anterior, quando relevante.
- Datas e horários de início e fim.
- Duração da tarefa.
- Status da execução.
- Resumo curto da tarefa.
- Acumulados de prompts e tempo.

### 4.13 `spec-template.md`
- Título.
- Objetivo.
- Dependências.
- Escopo.
- Fora de escopo.
- Requisitos.
- Resultado esperado.
- Critérios de aceite.
- Impacto.
- Arquivos afetados.
- Validações.
- Riscos.
- Próxima spec.
- Se aplicável, impacto sobre `.prompt-status` e sobre os fluxos de rastreio do Cursor AI.

### 4.14 `rules-scripts.md`
- Propósito e escopo.
- Limpeza inicial da tela.
- Cabeçalho operacional tabelado, obrigatoriamente renderizado em ciano usando caracteres ASCII/OEM 437.
- Organização por categoria macro.
- Descoberta da raiz do repositório.
- Parâmetros mínimos, input interativo numerado (`0`/`1`).
- Modos não-interativo e remoto (`--quiet`, `--force`, `--ssh`, `--log`).
- Uso de caminhos internos.
- Mensagens e UX operacional.
- Equivalência comportamental entre Linux e Windows.
- Scripts de instalação e desinstalação.
- Atualização documental e Regra final.

### 4.15 `docs/`
- Documentação técnica.
- Guias de uso.
- Decisões arquiteturais.
- Referências de implementação.
- Mudanças normativas.

### 4.16 `ideas/`
- Ideias soltas.
- Hipóteses.
- Propostas.
- Rascunhos.
- Explorações ainda não aprovadas.

### 4.17 `specs/`
- Specs formais.
- Cada spec com escopo e critério de aceite.
- Relacionamento explícito com `spec-root.md`.
- Ordem por domínio ou grupo.

### 4.18 `references/`
- Links, estudos, comparativos.
- Material de engenharia reversa.
- Arquivos de apoio e análise.
- Conteúdo consultivo, não normativo.

### 4.19 `scripts/`
- Scripts de setup.
- Scripts de validação.
- Scripts de automação.
- Scripts de manutenção.
- Estrutura por categoria e por sistema operacional.

### 4.20 `reports/`
- Relatórios de implementação.
- Relatórios de validação.
- Relatórios de auditoria.
- Relatórios de entrega.
- Evidências e impacto.

### 4.21 `prompts/`
- Prompts reutilizáveis.
- Colas de bootstrap.
- Prompts de revisão.
- Prompts de grupo e de fluxo.

### 4.22 `resources/`
- Downloads.
- Binários.
- Imagens.
- Apps auxiliares.
- Materiais de apoio.

### 4.23 `core/`
- Código ou conteúdo central do projeto.
- Subpastas específicas do domínio.
- Modelos, schemas, componentes, ativos e dados centrais.
- Tudo que é específico do projeto e não da governança do repositório.

---

## 5. Regras de `.cursorrules`

A `.cursorrules` deve deixar claro, no mínimo, os seguintes pontos:

### 5.1 Comportamento do agente
- Ser objetivo.
- Ser consistente.
- Ser disciplinado com o repositório.
- Não inventar requisitos.
- Não extrapolar escopo.

### 5.2 Honestidade
- Dizer quando não sabe.
- Distinguir fato de suposição.
- Não afirmar validação que não ocorreu.
- Não fingir que entendeu algo que não está documentado.

### 5.3 Dados que a resposta deve mostrar
Toda resposta relevante deve privilegiar:
- o que foi alterado;
- o que foi validado;
- o que ficou pendente;
- quais arquivos foram impactados;
- qual o próximo passo;
- quais documentos justificam a ação.
- OBRIGATÓRIO: Terminar a resposta com o resumo do `.prompt-status` na última linha: `[Prompt #X | LLM: <Nome> | Tempo: <Tempo>]` (se durar > 60s, exiba em minutos e segundos, ex: `1m 30s`)

### 5.4 Leitura obrigatória
Antes de agir, o agente deve ler os documentos-raiz relevantes.

### 5.5 Regras de bloqueio
Se faltar contexto, houver contradição ou escopo indefinido, o agente deve pausar e pedir esclarecimento.

### 5.6 Regras de documentação
Mudanças relevantes devem refletir em documentação, status e timeline.

### 5.7 Regras de scripts de instalação
Todo script de instalação deve suportar, quando aplicável, um parâmetro `--uninstall` que execute a remoção reversa do que o script instalou, preservando o comportamento esperado e evitando efeitos colaterais não documentados.

### 5.8 Regras de input interativo
Sempre que o usuário precisar tomar uma escolha, o script deve exibir uma lista numerada de opções e aguardar que ele digite o número correspondente.

### 5.9 Convenção de escolha
- `0` significa sempre `não`, `false` ou `no`.
- `1` significa sempre `sim`, `true` ou `yes`.
- Pressionar `Enter` sem digitar nada deve selecionar a opção default documentada.
- A entrada deve ser validada antes de continuar.
- Entrada inválida deve gerar novo prompt claro.

### 5.10 Regras de `.prompt-status`
- Ler `.prompt-status` antes de iniciar qualquer prompt.
- Atualizar `.prompt-status` no início e no fim de cada prompt.
- Registrar o LLM usado na execução atual.
- Registrar o tempo de início, fim e duração.
- Registrar o status final e um resumo curto da tarefa.
- Preservar o histórico recente sem sobrescrever o que já foi concluído.

### 5.11 CCIA: Continuidade de Contexto para IA
Para lidar com o problema de fragmentação de histórico (diferentes instâncias do Cursor nas máquinas ou Cloud Agents), o agente deve SEMPRE:
- Salvar a pergunta do usuário e um resumo executivo da sua resposta (decisões, códigos modificados, bugs arrumados).
- Utilizar o arquivo correspondente ao dia e máquina: `prompts/chat-YYYY-MM-DD-<hostname>.md` (anexando ao final do arquivo).
- Assim, outra instância da IA poderá ser instruída a ler estes arquivos para recuperar todo o histórico de decisões e pensamentos, superando a barreira de contexto local de cada IDE.

---

## 6. Regras de `flow.md`

O `flow.md` deve orientar a sequência de trabalho no repositório.

### Estrutura mínima
1. Ler `spec-root.md`.
2. Ler `rules.md`.
3. Ler `.cursorrules`.
4. Ler `.prompt-status`.
5. Ler `rules-scripts.md` quando a tarefa envolver scripts.
6. Ler `status.md` e `timeline.md` para contexto atual.
7. Ler `specs/` e `docs/` relevantes.
8. Planejar a entrega.
9. Implementar ou documentar somente o escopo confirmado.
10. Atualizar `.prompt-status` no início da execução.
11. Validar o que foi feito.
12. Atualizar `status.md`.
13. Atualizar `timeline.md`.
14. Atualizar `.prompt-status` ao concluir a execução.
15. Produzir relatório de entrega.
16. Registrar próximos passos.

### Regras
- Não pular leitura obrigatória.
- Não começar implementação sem contexto.
- Não misturar grupos ou temas sem autorização.
- Encerrar cada tarefa com validação e atualização documental.
- Nenhum prompt deve ser tratado como concluído sem atualizar `.prompt-status`.

---

## 7. Regras adicionais para scripts

Todo script de instalação deve oferecer um modo de desinstalação reversa, preferencialmente por `--uninstall` ou equivalente documentado.

### Regras de Reversão
- O modo de desinstalação deve desfazer exclusivamente as alterações realizadas pelo script.
- O modo de desinstalação deve preservar conteúdo que não foi criado por ele.
- O modo de desinstalação deve falhar de forma segura se não puder reverter algo integralmente.
- Se o script instalar, configurar, registrar ou copiar algo, precisa existir caminho documentado para desfazer isso.
- A reversão deve ser previsível, explícita e segura.

### Visualização
Todo script interativo deve iniciar limpando a tela (`clear`) e renderizando um cabeçalho de tabela estruturada em caracteres ASCII/OEM (code page 437) colorida em Ciano, exibindo metadados operacionais fixos separados por linha horizontal de informações em tempo de execução.

### Regra de input do usuário
Sempre que o usuário precisar tomar uma escolha, o script deve exibir uma lista numerada de opções e aguardar que ele digite o número correspondente. (NUNCA solicite `s/n` ou texto aberto sem necessidade explícita).

### Convenção obrigatória de seleção
- `0` significa sempre `não`, `false`, `no` ou `abortar`.
- `1` significa sempre `sim`, `true`, `yes` ou `continuar`.
- Pressionar `Enter` sem digitar nada deve selecionar a opção default documentada.
- O script deve validar a opção escolhida antes de seguir.
- Se a entrada for inválida, o script deve repetir a solicitação de forma clara.

### Suporte a automação
- Todo script interativo de alteração sistêmica DEVE permitir o modo automático via opções padrão de linha de comando: `--quiet` (suprime input), `--force` (autoriza ações destrutivas quando silencioso) e `--ssh` + `--log` (execução segura por agentes e sem TTY, espelhando a saída da tela num arquivo).

---

## 8. Regras de adaptação por tipo de projeto

### 8.1 Software
Pode incluir `src/`, `tests/`, `build/`, `packages/`, `migrations/`, `api/` e similares dentro de `/core` ou de subestruturas apropriadas.

### 8.2 Hardware
Pode incluir `cad/`, `bom/`, `schematics/`, `firmware/`, `manufacturing/` dentro de `/core`.

### 8.3 Sistema operacional
Pode incluir `profiles/`, `policies/`, `scripts/`, `debs/`, `rpm/`, `services/` dentro de `/core`.

### 8.4 Impressão 3D
Pode incluir `models/`, `slicing/`, `profiles/`, `materials/`, `test-parts/` dentro de `/core`.

### 8.5 Projetos híbridos
Podem combinar estruturas, desde que a raiz documental permaneça consistente e `/core` concentre o conteúdo específico do projeto.

---

## 9. Regras para o Cursor ao criar o repositório

O Cursor deve:

- reconhecer a natureza do projeto;
- criar a raiz documental mínima;
- preencher os arquivos com wireframes adequados ao domínio;
- manter consistência entre `spec-root.md`, `rules.md`, `.cursorrules`, `.prompt-status`, `flow.md` e `rules-scripts.md`;
- não confundir arquivo de visão com arquivo operacional;
- manter `status.md` e `timeline.md` vivos desde o início;
- colocar tudo que é específico do projeto sob `/core` sempre que aplicável;
- inicializar `.prompt-status` antes do primeiro prompt executado no repositório.

---

## 10. Critério de completude

Um novo repositório está realmente pronto quando o Cursor consegue responder, sem ambiguidade:

- o que o projeto é;
- quais são as regras;
- como operar;
- como validar;
- como documentar progresso;
- onde ficam as referências;
- onde fica o núcleo específico do projeto;
- como o agente deve se comportar;
- como rastrear o início, o fim e o estado de cada prompt em `.prompt-status`.

---

## 11. Template de `.prompt-status`

O arquivo `.prompt-status` deve usar um formato simples de pares `chave = valor`, com seções para andamento, último prompt e acumulados.

### 11.1 Seção de prompt em andamento
- `current_prompt_number`
- `current_prompt_start_time`
- `current_prompt_status`
- `current_prompt_llm`
- `current_prompt_summary`

### 11.2 Seção de último prompt concluído
- `last_prompt_number`
- `last_prompt_start_time`
- `last_prompt_end_time`
- `last_prompt_duration_seconds`
- `last_prompt_status`
- `last_prompt_llm`
- `last_prompt_summary`

### 11.3 Seção de acumulados
- `total_prompts_tracked`
- `total_execution_seconds`

### 11.4 Regras do template
- Os horários devem refletir a hora real do sistema.
- O resumo deve ser curto e objetivo.
- O status deve ser explícito, como `running`, `success`, `blocked` ou `failed`.
- O campo de LLM deve identificar claramente o modelo usado na tarefa.
- O arquivo deve ser atualizado sem perder o histórico recente da execução anterior.

---

## 12. Resumo normativo

`spec-root-repo-build.md` deve ser a raiz universal para criação de qualquer novo repositório no Cursor, com estrutura documental completa, wireframes por arquivo, regras de comportamento do agente, honestidade operacional, suporte a scripts reversíveis, regra de input numerado, fluxo de trabalho explícito, uso obrigatório de `.prompt-status` e centralização do conteúdo específico do projeto em `/core`.

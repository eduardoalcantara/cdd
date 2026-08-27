# Prompt Base: Bootstrap de Personalização de Projeto

Ao usar o template de fundação em um novo repositório derivado de `spec-project-bootstrap.md`, utilize este prompt (ou peça ao Cursor para rodá-lo) a fim de substituir os placeholders genéricos para a essência real do seu projeto.

## Ação Solicitada à IA

```markdown
Por favor, personalize a fundação documental deste projeto derivado do meu template.
Estamos criando um novo sistema/comando com as seguintes características:

1. **Nome do Projeto**: [INSERIR NOME]
2. **Propósito (Uma frase)**: [INSERIR PROPÓSITO]
3. **Público e Objetivo**: [INSERIR PÚBLICO E OBJETIVO PRINCIPAL]
4. **Escopo Principal**: [O QUE ESTÁ DENTRO DO ESCOPO]

Execute as seguintes ações:
1. Altere todos os cabeçalhos e placeholders `[NOME DO PROJETO]`, `[RESUMO]` etc., em `readme.md` e `spec-root.md`.
2. Inclua o escopo inicial em `spec-root.md`.
3. Escreva a primeira documentação de especificações sobre os detalhes técnicos discutidos acima em `specs/to-do/SPEC-core.md`.
4. Atualize o `status.md` (removendo as tarefas sobre a criação do template genérico e começando as do novo projeto).
5. Faça um lançamento de marco cronológico no `timeline.md`.
6. Após as alterações, atualize `.prompt-status` **somente no início** (número, hora de início, LLM, resumo) e siga os procedimentos CCIA anexando as respostas aos logs `prompts/chat...` da sessão de hoje. Não grave fim nem duração do prompt atual nesse arquivo.
```
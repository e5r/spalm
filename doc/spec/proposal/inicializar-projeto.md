
# INICIALIZAR PROJETO

* [x] Proposta
* [ ] Protótipo _(não há)_
* [ ] Implementacao _(não implementado)_
* [ ] Especificação _(não especificado)_

## Resumo
[summary]: #summary

Inicializa um diretório com estrutura básica de um projeto de especificação.

## Motivação
[motivation]: #motivation

Criar diretórios e subdiretórios, bem como arquivos chaves principais torna-se uma
tarefa bastante chata, além de, apesar de simples, podem ocorrer erros com nomes de
arquivos ou esquecimento de algum diretório.

É possível simplesmente copiar uma estrutura da internet e descompactá-la. Mas mesmo
assim, ainda tem o trabalho de descompactar, e às vezes renomear um ou outro arquivo.

Será muito mais produtivo ter uma funcionalidade que faça isso automaticamente.

## Detalhamento
[detailing]: #detailing

Um comando que receba a localização de um diretório e crie-o caso não exista.
Seguindo, cria alguns arquivos e subdiretórios conforme sugestão abaixo:

```
./
  doc/
    tutorials/
      .empty
    spec/
      proposal/
        .empty
  	  specification/
        .empty
      refused/
        .empty
    index.md
    project-description.md
```

#### `./`
Este é o diretório informado para inicializar o projeto.

#### `./doc/`
Neste subdiretório fica toda a estrutura do projeto de especificação.

#### `./doc/tutorials/`
Subdiretório reservado para tutoriais e guias do usuário.

#### `./doc/spec/proposal`
Subdiretório reservado para propostas de evolução do software, as `SEP`
(Software Evolution Proposal).

#### `./doc/spec/specification/`
Subdiretório reservado para especificações de funcionalidades, ou seja,
as propostas aprovadas.

#### `./doc/spec/refused/`
Subdiretório reservado para propostas recusadas.

#### `./doc/spec/**/.empty`
Um arquivo vazio que deve ser removido assim que o primeiro arquivo real for
adicionado.

Isso é necessário para os sistemas de controle de versão como
*[Git][GitLink]*, que não guarda diretórios vazios, então faz-se necessário um arquivo
qualquer para que o diretório seja submetido.

### `./doc/index.md`
Arquivo com informação do projeto e links para conteúdos da especificação
(esse deve ser gerado automaticamente por outro comando).

#### `./doc/project-description.md`
Arquivo com título e descrição do projeto (este irá auxiliar outro comando que
monta o conteúdo de `./doc/index.md`).

### Algumas notas

* Não usar arquivos mágicos para identificação de diretório já inicializado.
* Caso um diretório já esteja inicializado, porém falte algum subdiretório/aquivo,
  este é simplesmente adicionado.
* Um diretório é considerado já inicializado caso não precise criar nenhum
  subdiretório/arquivo.

## Contras
[drawbacks]: #drawbacks

Não há contras.

## Alternativas
[alternatives]: #alternatives

Sem essa funcionalidade, o desenvolvedor pode inicializar sua estrutura de diretórios
manualmente, ou baixando um arquivo compactado de modelo para isso.

Caso a funcionalidade não seja implementada, o impacto é pequeno, somente o tempo
para inicializar um projeto vai ser maior, mas é questão de minutos.

## Questões não resolvidas
[unresolved]: #unresolved

Não há questões não resolvidas.

## Links úteis
[links]: #links

* [SEP - (Software Evolution Proposal)][SEPLink]

[GitLink]: https://www.git-scm.com
[SEPLink]: https://github.com/e5r/alm/blob/master/doc/draft/SEP.md
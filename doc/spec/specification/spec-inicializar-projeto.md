INICIALIZAR PROJETO
===================

* [x] Proposta: [inicializar-projeto][ORIGEMLink]
* [x] Especificação: `2019-01-08`
* [ ] Revisão
* [ ] Aceite
* [ ] Implementação
* [ ] Teste
* [ ] Liberação: [v0.0.0][TAGLink]

Permite inicializar um diretório qualquer com a estrutura base de um projeto
de especificação de software, conforme modelo proposto pela equipe de
desenvolvimento E5R.

[Saiba mais sobre o modelo proposto pelo time E5R aqui.][E5R-ALM]

A funcionalidade é acionada pelo seguinte comando:

```console
$ spalm init
Usage: spalm init [OPTIONS] PATH

Options:
    -h, --help     Exibe uma mensagem de ajuda da funcionalidade

Path:              Caminho do diretório para inicializar.
                   É opcional. Se não informado assume o diretório
                   atual do processo.
```

## Especificação formal

### Cenário 1: Quando o diretório informado não existe
```gherkin
@RN01, @MS01
Dado um caminho de diretório que não existe
Quando acionar o comando "init"
Então o diretório será criado
Então subdiretório "doc" será criado no caminho indicado
Então os subdiretórios "spec" e "tutorials" serão criados em "doc"
Então os subdiretórios "proposal", "refused" e "specification" serão criados em "doc/spec"
Então um arquivo ".empty" será criado em cada subdiretório "proposal", "refused" e "specification"
Então os arquivos "index.md" e "project.md" serão criados em "doc"
Então a mensagem "{MS01}" é exibida
Então o programa encessa sucesso
```

...

## Funcionalidades relacionadas

* Não há funcionalidades relacionadas

## Equipe envolvida

Revisores:
* Erlimar Silva Campos, erlimar@gmail.com

Aceite:
* Não aceita ainda

Implementadores:
* Não implementado ainda

Testadores:
* Não testado ainda

## Regras de Negócio
[rn]: #rn

### RN01: Diretório inexistente deve ser criado

Quando o usuário informar um caminho de diretório que não existe. O mesmo deve ser
criado antes que e o projeto seja inicializado.

## Mensagens

- *MS01* - Projeto de especificação inicializado com sucesso em {a}
  - `{a}` - Caminho completo do diretório "doc" criado

## Protótipos
[prototype]: #prototype

Não há protótipos.

#### Regras de Apresentação

Não há regras de apresentação.

## Links úteis
[links]: #links

Não existem outros links.

[ORIGEMLink]: ../proposal/inicializar-projeto.md
[FEATURE-A]: ../link/to/feature-a.md
[TAGLink]: http://git.control/tag/x
[E5R-ALM]: https://github.com/e5r/alm
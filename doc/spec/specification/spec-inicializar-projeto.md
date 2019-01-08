INICIALIZAR PROJETO
===================

* [x] Proposta: [inicializar-projeto][ORIGEMLink]
* [x] Especificação: `2019-01-08`
* [x] Revisão: `2019-01-08`
* [ ] Aprovação
* [ ] Implementação
* [ ] Teste
* [ ] Liberação

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
                   Opcional! Se não informado assume o diretório
                   atual do processo.
```

## Especificação formal

### Cenário 1: Diretório informado não existe
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
Então o programa encerra com sucesso
```

### Cenário 2: Diretório informado já existe
```gherkin
@MS01
Dado um caminho de diretório que já existe
Quando acionar o comando "init"
Então subdiretório "doc" será criado no caminho indicado
Então os subdiretórios "spec" e "tutorials" serão criados em "doc"
Então os subdiretórios "proposal", "refused" e "specification" serão criados em "doc/spec"
Então um arquivo ".empty" será criado em cada subdiretório "proposal", "refused" e "specification"
Então os arquivos "index.md" e "project.md" serão criados em "doc"
Então a mensagem "{MS01}" é exibida
Então o programa encerra com sucesso
```

### Cenário 3: Diretório informado já contém "doc"
```gherkin
@MS01
Dado um caminho de diretório que já contém o subdiretório "doc"
E que também contenha o arquivo "index.md"
Quando acionar o comando "init"
Então os subdiretórios "spec" e "tutorials" serão criados em "doc"
Então os subdiretórios "proposal", "refused" e "specification" serão criados em "doc/spec"
Então um arquivo ".empty" será criado em cada subdiretório "proposal", "refused" e "specification"
Então o arquivo "project.md" será criado em "doc"
Mas o subdiretório "doc" não será criado no caminho indicado
Mas o arquivo "index.md" não será criado em "doc"
Então a mensagem "{MS01}" é exibida
Então o programa encerra com sucesso
```

### Cenário 4: Diretório informado completamente inicializado
```gherkin
@RN02, @ME01
Dado um caminho de diretório que já esteja completamente inicializado
Quando acionar o comando "init"
Então nenhum arquivo ou subdiretório é criado
Então a mensagem "{ME01}" é exibida
Então o programa encerra com código de falha 1
```

**Nota!** Um diretório completamente inicializado é um diretório que contém
          todos os arquivos e diretórios listados abaixo:

```
./
  doc/
    tutorials/
    spec/
      proposal/
  	  specification/
      refused/
    index.md
    project.md
```

### Cenário 5: Diretório não informado
```gherkin
@RN03, @MS01
Dado uma execução sem informar o caminho do diretório
Quando acionar o comando "init"
Então subdiretório "doc" será criado no diretório atual do processo
Então os subdiretórios "spec" e "tutorials" serão criados em "doc"
Então os subdiretórios "proposal", "refused" e "specification" serão criados em "doc/spec"
Então um arquivo ".empty" será criado em cada subdiretório "proposal", "refused" e "specification"
Então os arquivos "index.md" e "project.md" serão criados em "doc"
Então a mensagem "{MS01}" é exibida
Então o programa encerra com sucesso
```

## Funcionalidades relacionadas

* Não há funcionalidades relacionadas.

## Equipe

Analistas:
* Erlimar Silva Campos, erlimar@gmail.com

Revisores:
* Erlimar Silva Campos, erlimar@gmail.com

Aprovadores:
* Erlimar Silva Campos, erlimar@gmail.com

Implementadores:
* `Não implementado ainda`

Testadores:
* `Não testado ainda`

## Regras de Negócio
[rn]: #rn

### RN01: Diretório inexistente deve ser criado

Quando o usuário informar um caminho de diretório que não existe. O mesmo deve ser
criado antes que e o projeto seja inicializado.

### RN02: Arquivos e diretórios já existentes não devem ser sobrescritos

Quando o usuário informar um caminho de diretório que já contenha algum dos subdiretórios
ou arquivos do modelo proposto. Este subdiretório ou arquivo não deve ser sobrescrito.

### RN03: Diretório atual do processo é o padrão

Caso um caminho de diretório não seja informado, o diretório atual do processo em
execução deve ser considerado como o caminho.

## Mensagens de sucesso
- **MS01** - Projeto de especificação inicializado com sucesso em "{a}"!
  - `{a}` - Caminho completo do subdiretório "doc" criado

## Mensagens de erro
- **ME01** - O projeto de especificação em "{a}" já estava inicializado.
  - `{a}` - Caminho completo do subdiretório "doc" criado

## Protótipos
[prototype]: #prototype

Não há protótipos.

#### Regras de Apresentação

Não há regras de apresentação.

## Links úteis
[links]: #links

Não há links.

[ORIGEMLink]: ../proposal/inicializar-projeto.md
[FEATURE-A]: ../link/to/feature-a.md
[TAGLink]: http://git.control/tag/x
[E5R-ALM]: https://github.com/e5r/alm
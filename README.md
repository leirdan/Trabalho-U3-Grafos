# Trabalho Unidade 3 Grafos

## Estrutura do repositório em Rust 🦀

```bash
Trabalho-U3-Grafos/
├── README.md
├── Cargo.lock
├── Cargo.toml
├── latex # Código fonte do documento latex
│   ├── chapters/
│   ├── CS_report.sty   # Definições e import de biblioteca
│   ├── figures         # Imagens usadas no documento
│   ├── main.tex        # Entry point do código fonte
│   ├── Dockerfile      # Receita de uma imagem docker para compilar o documento
│   ├── Makefile        # Para compilar o documento
│   └── references.bib  # Referências usadas no texto
└── src # Código fonte da implementação dos algoritmos
    ├── graph.rs                 # Traços relacionadas a grafos (Grafo, Grafo não direcionado, Grafo ponderado)
    ├── lib.rs                   # Módulos exportados pela biblioteca
    └── traversal.rs             # Algoritmos de travessia no grafo (DFS, BFS, etc)
    ...
```

## Desenvolvimento

### Pré-requisitos

- [Cargo 1.90.0](https://rust-lang.org/learn/get-started/)
- [Texlive (full)](https://tug.org/texlive/) e Texlive-lang-portuguese: pode ser encontrado nos gerenciadores de pacote comuns.
- [Docker](https://www.docker.com/): Alternativa para compilar o $\LaTeX$, caso não queira instalar o `texlive`
- [Graphviz](https://www.graphviz.org/download/): Para converter os arquivos `.dot` em imagens `.png`

### Compilação e testes

É possível fazer isso via cargo ou Makefile.

- **cargo**

  ```bash
  # Compila o projeto
  cargo br

  # Executa binários na pasta examples/
  cargo er [example]

  # Executa testes unitários
  cargo tr

  # Verifica o código usando o clippy
  cargo clippy

  # Formata o código
  cargo fmt
  ```

- **Makefile**

  ```bash
  $ make help
  Available targets:
    build           Builds the program with release mode
    check           Performs a cargo check with release mode
    clean           Cleans cargo generated artifacts
    clippy          Runs clippy
    example         Runs a given a example e.g. `make example -- example1`
    fmt_check       Check if the code is formatted
    fmt             Formats the code
    help            Show this help message
    run             Runs main in release mode
    test            Runs all tests
  ```

#### $\LaTeX$

Na pasta `latex/`:

```bash
# Exibe receitas disponíveis
make help

# Compila pdf no diretório output/
make

# Limpa arquivos auxiliares
make clean

# Limpa todos os arquivos (incluindo pdf)
make distclean

# Limpa e compila novamente
make rebuild
```

#### $\LaTeX$ com Docker

```bash
# Cria a imagem docker
docker build -t latex-compiler latex/

# Compila a imagem e executa o container criando o pdf.
# --rm automaticamente deleta o container e o volume
docker run --rm latex-compiler > main.pdf
```

Existe uma imagem compilada em `vleema/latex-compiler` (não garantimos que esteja atualizada). Podes substituir `docker build...` por

```bash
docker pull vleema/latex-compiler:latest
```

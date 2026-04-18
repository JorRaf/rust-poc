# Tetris Rust

<img width="662" height="493" alt="image" src="https://github.com/user-attachments/assets/09363a02-ce14-477d-ace2-1a8e6ad2d4f4" />

Um Tetris em Rust rodando no terminal, com controles por teclado, pontuacao, limpeza de linhas e arquitetura modular para facilitar evolucao futura para interface grafica.

## Funcionalidades

- Tetrominos aleatorios
- Movimento lateral
- Rotacao
- Soft drop e hard drop
- Limpeza de linhas
- Pontuacao
- Preview da proxima peca
- Game over
- CI com formatacao, lint, testes e build da imagem Docker

## Requisitos

- Rust toolchain instalado
- Terminal com suporte basico a `crossterm`

## Como rodar

```bash
cargo run
```

## Controles

- `Left`: mover para a esquerda
- `Right`: mover para a direita
- `Up`: rotacionar
- `Down`: soft drop
- `Space`: hard drop
- `q`: sair

## Qualidade

Rodar localmente:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Docker

Build da imagem:

```bash
docker build -t tetris-rust .
```

Execucao:

```bash
docker run --rm -it tetris-rust
```

## Estrutura

```text
src/
  constants.rs  # constantes de jogo
  game.rs       # estado e regras do jogo
  piece.rs      # pecas e tetrominos
  render.rs     # renderizacao no terminal
  terminal.rs   # setup e restauracao do terminal
  main.rs       # loop principal e testes
```

## Arquitetura

O projeto foi separado para reduzir acoplamento entre regras do jogo e terminal:

- `game.rs` concentra estado e comandos do dominio
- `piece.rs` define pecas, formatos e transformacoes
- `render.rs` conhece apenas como desenhar o estado atual
- `main.rs` traduz input do terminal em comandos de jogo

Isso deixa o codigo melhor preparado para uma futura GUI.

## CI

O workflow em `.github/workflows/rust.yml` executa:

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `docker build -t tetris-rust .`

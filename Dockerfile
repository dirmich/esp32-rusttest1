FROM espressif/idf-rust:all_latest

WORKDIR /workspace

ENV CARGO_TERM_COLOR=always

RUN rustup target add --toolchain esp riscv32imc-esp-espidf

CMD ["cargo", "build"]

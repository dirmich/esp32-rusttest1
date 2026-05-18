FROM espressif/idf-rust:all_latest

WORKDIR /workspace

ENV CARGO_TERM_COLOR=always

CMD ["cargo", "build"]

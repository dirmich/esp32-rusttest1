FROM espressif/idf-rust:all_latest

WORKDIR /workspace

ENV CARGO_TERM_COLOR=always

RUN git config --global --add safe.directory '*'

CMD ["cargo", "build"]

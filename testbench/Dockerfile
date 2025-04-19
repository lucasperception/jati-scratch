FROM rust:1-bookworm

ARG COMMIT_HASH

RUN apt-get update && apt-get -yqq --no-install-recommends install git && apt-get clean && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/tools

ENV GIT_URL=https://github.com/acg-team/rust-phylo.git

RUN git clone "$GIT_URL" rust-phylo && \
    cd rust-phylo && \
    if [ -n "$COMMIT_HASH" ]; then \
        git checkout "$COMMIT_HASH"; \
    else \
        git checkout main; \
    fi

 WORKDIR /opt/tools/rust-phylo/phylo

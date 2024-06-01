FROM rust:1.78-bookworm

ARG USER
ARG WORKSPACES
ARG HOME

SHELL [ "/bin/sh", "-c" ]

RUN useradd -m ${USER} -u 1000 -s /bin/bash

USER root
RUN apt-get update \
    && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends sudo git curl tree musl-tools \
    && rustup target add x86_64-unknown-linux-musl

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir -p /commandhistory ${HOME}/.vscode-server ${HOME}/.cargo /usr/local/rustup /usr/local/cargo/registry \
    && touch /commandhistory/.bash_history \
    && chown -R ${USER}:${USER} /commandhistory ${HOME}/.vscode-server ${HOME}/.cargo /usr/local/rustup /usr/local/cargo/registry \
    && echo $SNIPPET >> ${HOME}/.bashrc

RUN sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --yes \
    && echo 'eval "$(starship init bash)"' >> ${HOME}/.bashrc

USER ${USER}
WORKDIR ${HOME}
RUN { \
    echo 'alias ll="ls -l"'; \
    echo 'alias la="ls -a"'; \
    echo 'alias lla="ls -l -a"'; \
    } >> ${HOME}/.bashrc

RUN cargo install cargo-watch \
    && rustup component add clippy rustfmt

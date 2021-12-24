FROM node:16-bullseye

ARG USER
ARG WORKSPACES
ARG HOME=/home/${USER}

SHELL [ "/bin/sh", "-c" ]

RUN userdel -r node && useradd -m ${USER} -u 1000 -s /bin/bash

WORKDIR ${HOME}

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir -p /commandhistory /home/${USER}/.vscode-server/extensions ${WORKSPACES}/node_modules \
    && touch /commandhistory/.bash_history \
    && chown -R ${USER}:${USER} /commandhistory /home/${USER}/.vscode-server ${WORKSPACES}/node_modules \
    && echo $SNIPPET >> "/home/${USER}/.bashrc"

RUN curl -fsSL https://get.pulumi.com | sh

RUN sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --yes \
    && echo 'eval "$(starship init bash)"' >> .bashrc

ENV PATH=$PATH:${HOME}/.yarn/bin:${HOME}/.pulumi/bin

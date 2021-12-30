FROM node:16-bullseye

ARG USER
ARG WORKSPACES
ARG HOME=/home/${USER}

SHELL [ "/bin/sh", "-c" ]

RUN userdel -r node && useradd -m ${USER} -u 1000 -s /bin/bash

RUN echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] http://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list \
    && curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | apt-key --keyring /usr/share/keyrings/cloud.google.gpg  add - \
    && apt-get update -y \
    && apt-get install google-cloud-sdk -y

RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir -p /commandhistory ${HOME}/.vscode-server/extensions ${HOME}/.config/gcloud ${WORKSPACES}/node_modules \
    && touch /commandhistory/.bash_history \
    && chown -R ${USER}:${USER} /commandhistory ${HOME}/.vscode-server ${HOME}/.config ${WORKSPACES}/node_modules \
    && echo $SNIPPET >> ${HOME}/.bashrc

RUN sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --yes \
    && echo 'eval "$(starship init bash)"' >> ${HOME}/.bashrc

USER ${USER}
WORKDIR ${HOME}
RUN curl -fsSL https://get.pulumi.com | sh

ENV PATH=$PATH:${HOME}/.yarn/bin:${HOME}/.pulumi/bin

FROM registry.access.redhat.com/ubi9/ubi:latest

ARG USERNAME=rxegy
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN dnf config-manager \
    --add-repo https://download.opensuse.org/repositories/shells:/fish:/release:/3/CentOS-9_Stream/shells:fish:release:3.repo

RUN yum -y install \
    clang-devel \
    cmake \
    dbus-tools \
    fish \
    gcc \
    gcc-c++ \
    git \
    make \
    openssl-devel \
    python3-pip \
    sudo \
    && yum clean all

# Setup Username
# Create the user
RUN set -x \
    && groupadd --gid "${USER_GID}" "${USERNAME}" \
    && useradd --uid "${USER_UID}" --gid "${USER_GID}" -m "${USERNAME}" \
    && echo "${USERNAME} ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}" \
    && chmod 0440 "/etc/sudoers.d/${USERNAME}" \
    && mkdir /workspaces \
    && chown -R "${USERNAME}:${USERNAME}" /workspaces

# Install pre-commit
RUN pip3 install pipx

ENV HOME "/home/${USERNAME}"
USER "${USERNAME}"

# Install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && echo 'source $HOME/.cargo/env' >> "$HOME/.bashrc" \
    && source "$HOME/.cargo/env" \
    && mkdir -p ~/.config/fish/completions \
    && rustup completions fish > ~/.config/fish/completions/rustup.fish \
    && rustup component add clippy rls rust-analyzer rust-src rustfmt \
    && rustup install 1.85.0 \
    && rustup default 1.85.0

# Install pre-commit
RUN pipx install pre-commit

# Install starship
RUN source "$HOME/.cargo/env" \
    && cargo install cargo-binstall \
    && cargo binstall starship

ENTRYPOINT [ "sleep", "infinity" ]
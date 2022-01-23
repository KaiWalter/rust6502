## update and install 1st level of packages
apt-get update
apt-get install -y \
curl \
git \
gnupg2 \
jq \
sudo \
zsh \
build-essential \
libssl-dev libncurses5-dev libncursesw5-dev \
openssl \
unzip

## update and install 2nd level of packages
curl -fsSL https://deb.nodesource.com/setup_17.x | sudo -E bash -

apt-get update
apt-get install -y \
pkg-config \
nodejs

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y

export PATH=/root/.cargo/bin:$PATH

rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit

cargo install wasm-pack
cargo install cargo-generate

## setup git
git config --global core.editor "code --wait"

## setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc
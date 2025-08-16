#!/bin/bash

# source <(curl --proto '=https' --tlsv1.2 -sSf https://gojo-cli.github.io/gojo/install.sh)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "linux detected"
    # Install dependencies

    which -s git
    if [[ $? != 0 ]] ; then
      echo "downloading git"
      apt install -y git
    fi

    which -s g++
    if [[ $? != 0 ]] ; then
      echo "downloading g++"
      apt install -y g++
    fi

    which -s curl
    if [[ $? != 0 ]] ; then
      echo "downloading curl"
      apt install -y curl
    fi

    which -s cmake
    if [[ $? != 0 ]] ; then
      echo "downloading cmake"
      apt install -y cmake
    fi

    which -s cppcheck
    if [[ $? != 0 ]] ; then
      echo "downloading cppcheck"
      apt install -y cppcheck
    fi

    which -s pipx
    if [[ $? != 0 ]] ; then
      echo "downloading pipx"
      apt install -y pipx
      pipx ensurepath
    fi
    pipx install cpplint

    which -s cargo
    if [[ $? != 0 ]] ; then
      echo "downloading cargo"
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
    fi

    source ~/.bashrc
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macos detected"
    # Install dependencies

    xcode-select -p 1>/dev/null
    if [[ $? != 2 ]] ; then
      xcode-select --install
    fi

    which -s brew
    if [[ $? != 0 ]] ; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        brew update
    fi

    echo "export PATH=\"/opt/homebrew/bin:\$PATH\"" >> ~/.zshenv
    source ~/.zshenv

    which -s git
    if [[ $? != 0 ]] ; then
      brew install -y git
    fi

    which -s cmake
    if [[ $? != 0 ]] ; then
      brew install -y cmake
    fi

    which -s cppcheck
    if [[ $? != 0 ]] ; then
      brew install -y cppcheck
    fi

    which -s pipx
    if [[ $? != 0 ]] ; then
      brew install -y pipx
    fi
    pipx install cpplint

    which -s cargo
    if [[ $? != 0 ]] ; then
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | source -s -- -y
    fi

    pipx ensurepath
    source ~/.zshenv
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

# Create gojo directory.
echo "making gojo directory"
mkdir -p ~/.gojo
mkdir -p ~/.gojo/repos
mkdir ~/.gojo/include
mkdir ~/.gojo/lib
mkdir ~/.gojo/bin

# Clone gojo repo and build from source.
echo "cloning gojo"
cd ~/.gojo/repos
git clone https://github.com/gojo-cli/gojo.git
cd gojo
~/.cargo/bin/cargo build --release
strip target/release/gojo
mv target/release/gojo ~/.gojo/bin
rm -rf target

# Clone gojo++ repo and build from source.
echo "cloning gojo++"
cd ~/.gojo/repos
git clone https://github.com/gojo-cli/gojo-updater.git
#cd gojo-updater
#~/.cargo/bin/cargo build --release
#strip target/release/gojo-updater
#mv target/release/gojo-updater target/release/gojo++
#mv target/release/gojo++ ~/.gojo/bin

# Clone packages repository
cd ~/.gojo/repos
git clone https://github.com/gojo-cli/packages.git

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "adding gojo to bashrc"
    echo "export PATH=\"$HOME/.gojo/bin:\$PATH\"" >> ~/.bashrc
    source ~/.bashrc
    echo ""
    echo "gojo installed successfully"
fi
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "export PATH=\"$HOME/.gojo/bin:\$PATH\"" >> ~/.zshenv
    source ~/.zshenv
    echo ""
    echo "gojo installed successfully"
fi

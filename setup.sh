#!/usr/bin/env bash

#some setup stuff for the dev environment
#install nodejs
curl -fsSL https://deb.nodesource.com/setup_19.x | sudo -E bash - &&\
sudo apt-get install -y nodejs

#install GitHub Copilot CLI
npm install -g @githubnext/github-copilot-cli

#authenticate with GitHub Copilot
github-copilot-cli auth

#Upgrade
#npm install -g @githubnext/github-copilot-cli
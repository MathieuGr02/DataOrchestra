#!/bin/bash

echo "Setting up ssh client docker"

current_path = pwd

if [current_path -neq "/"]
then
    cd /
fi

echo "apt-get update"
apt-get update

echo "Installing openssh-server"
apt-get install -y openssh-server

mkdir /var/run/sshd

echo "root:password" | chpasswd

echo "PermitRootLogin yes" >>/etc/ssh/sshd_config

echo "Running sshd"
/usr/sbin/sshd -D

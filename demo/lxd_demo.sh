lxc launch ubuntu: myubuntu

# creating directorys
lxc exec myubuntu -- mkdir /home/sandbox

# push files into /sandbox
lxc file push safe-box myubuntu/home/sandbox/safe-box
lxc file push ip_permission.conf myubuntu/home/sandbox/ip_permission.conf
lxc file push file_permission.conf myubuntu/home/sandbox/file_permission.conf

# lxc exec myubuntu -- /bin/bash
# cd /home/sandbox
# ./safe-box git clone https://github.com/codius/rust-ptrace.git
# ./safe-box wget www.baidu.com

# ./safe-box -aa -ip git clone https://github.com/codius/rust-ptrace.git
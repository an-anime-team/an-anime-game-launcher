sandbox = 沙盒
sandbox-description = 在隔离环境中运行游戏，阻止其对个人数据的访问

enable-sandboxing = 启用沙盒
enable-sandboxing-description = 在根文件系统的只读副本中运行游戏

hide-home-directory = 隐藏家目录
hide-home-directory-description = 将 /home、 /var/home/$USER 和 $HOME 目录与游戏隔离

hostname = 主机名
additional-arguments = 额外参数

private-directories = 隐私目录
private-directories-description = 这些目录将会被空的虚拟文件系统（tmpfs）替代，其中的原始内容不可被沙盒中的游戏访问

path = 路径

shared-directories = 共享目录
shared-directories-description = 这些目录将会被软链接到主机系统上的目录

original-path = 原路径
new-path = 新路径

read-only = 只读
read-only-description = 禁止游戏向此目录写入任何数据

symlinks = 软链接
symlinks-description = 软链接原始路径到沙盒里的新路径

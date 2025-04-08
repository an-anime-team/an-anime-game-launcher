sandbox = 沙盒
sandbox-description = 在隔離環境中運行遊戲，阻止其對個人數據的訪問

enable-sandboxing = 啟用沙盒
enable-sandboxing-description = 在根文件系統的只讀副本中運行遊戲

hide-home-directory = 隱藏家目錄
hide-home-directory-description = 將 /home、 /var/home/$USER 和 $HOME 目錄與遊戲隔離

hostname = 主機名
additional-arguments = 額外參數

private-directories = 隱私目錄
private-directories-description = 這些目錄將會被空的虛擬文件系統（tmpfs）替代，其中的原始內容不可被沙盒中的遊戲訪問

path = 路徑

shared-directories = 共享目錄
shared-directories-description = 這些目錄將會被軟鏈接到主機系統上的目錄

original-path = 原路徑
new-path = 新路徑

read-only = 只讀
read-only-description = 禁止遊戲向此目錄寫入任何數據

symlinks = 軟鏈接
symlinks-description = 軟鏈接原始路徑到沙盒裡的新路徑

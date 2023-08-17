sandbox = Sandbox
sandbox-description = Rode o jogo em um ambiente isolado, prevenindo-o de acessar seus dados

enable-sandboxing = Habilitar sandboxing
enable-sandboxing-description = Rode o jogo em uma cópia somente leitura do seu sistema de arquivo root

hide-home-directory = Esconder pasta home
hide-home-directory-description = Isolar as suas pastas /home, /var/home/$USER e $HOME do jogo

hostname = Hostname
additional-arguments = Argumentos adicionais

private-directories = Pastas privadas
private-directories-description = Essas pastas serão substituidas por um sistema virtual (tmpfs), e seu conteúdo original não será disponível ao jogo

path = Caminho

shared-directories = Pastas compartilhadas
shared-directories-description = Essas pastas serão symlinkadas à pastas nos seu sistema host

original-path = Caminho original
new-path = Novo caminho

read-only = Apenas leitura
read-only-description = Proibir o jogo de escrever qualquer dado nessa pasta

symlinks = Symlinks
symlinks-description = Symlink o caminho original para o novo dentro da sandbox

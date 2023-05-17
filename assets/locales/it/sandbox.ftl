sandbox = Sandbox
sandbox-description = Esegui il gioco in un ambiente isolato, impedendogli di accedere ai tuoi dati personali

enable-sandboxing = Abilita il sandboxing
enable-sandboxing-description = Esegui il gioco in una copia di sola lettura della radice del tuo filesystem

hide-home-directory = Nascondi la cartella home
hide-home-directory-description = Isola le tue cartelle /home, /var/home/$USER, e $HOME dal gioco

hostname = Hostname
additional-arguments = Ulteriori argomenti

private-directories = Cartelle private
private-directories-description = Queste cartelle verranno rimpiazzate da un filesystem virtuale vuoto (tmpfs) e il loro contenuto originale non sarà disponibile al gioco nella sandbox

path = Percorso

shared-directories = Cartelle condivise
shared-directories-description = Queste cartelle verranno collegate simbolicamente a delle cartelle sul tuo sistema ospitante

original-path = Percorso originale
new-path = Nuovo percorso

read-only = Sola lettura
read-only-description = Impedisci al gioco di scrivere dati in questa cartella

symlinks = Collegamenti simbolici
symlinks-description = Crea un collegamento simbolico dal percorso originale a quello nuovo all'interno della tua sandbox

sandbox = Aislamiento
sandbox-description = Ejecutar el juego en un entorno aislado, previniendo el acceso a datos personales

enable-sandboxing = Activar aislamiento
enable-sandboxing-description = Ejecutar el juego en una copia de sólo lectura de tu sistema de archivos

hide-home-directory = Esconder el directorio home
hide-home-directory-description = Aisla las carpetas /home, /var/home/$USER, y $HOME del juego

hostname = Nombre del host
additional-arguments = Additional arguments

private-directories = Directorios privados
private-directories-description = Estas carpetas serán reemplazadas por un sistema de archivos virtual (tmpfs) vacío, y su contenido real no será accesible al juego aislado

path = Ruta

shared-directories = Directorios compartidos
shared-directories-description = Estos directorios serán enlazados a directorios de tu sistema anfitrión

original-path = Ruta original
new-path = Nueva ruta

read-only = Sólo lectura
read-only-description = Le prohibe al juego escribir datos en este directorio

symlinks = Symlinks
symlinks-description = Symlink original path to the new one inside of your sandbox

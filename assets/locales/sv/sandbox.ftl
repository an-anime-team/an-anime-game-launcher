sandbox = Sandlåda
sandbox-description = Kör spelet i en isolerad miljö, vilket förhindra det från att komma åt dina personliga data

enable-sandboxing = Aktivera sandlådeläge
enable-sandboxing-description = Kör spelet i en skrivskyddad kopia av ditt rotfilsystem

hide-home-directory = Dölj hemkatalog
hide-home-directory-description = Isolera mapparna /home, /var/home/$USER, och $HOME från spelet

hostname = Värdnamn
additional-arguments = Ytterligare argument

private-directories = Privata kataloger
private-directories-description = Dessa mappar kommer att ersättas av ett tomt virtuellt filsystem (tmpfs), och deras ursprungliga innehåll kommer inte att vara tillgängligt för spelet i sandlådeläge

path = Sökväg

shared-directories = Delade kataloger
shared-directories-description = Dessa kataloger kommer att symlänkas till kataloger i ditt värdsystem

original-path = Ursprunglig sökväg
new-path = Ny sökväg

read-only = Skrivskyddad
read-only-description = Förbjud spelet att skriva data till denna katalog

symlinks = Symlänkar
symlinks-description = Symlänka ursprungliga sökvägen till den nya inuti din sandlåda

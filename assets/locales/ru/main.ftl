custom = Свое значение
none = Нет
default = По умолчанию
details = Подробнее
options = Опции

width = Ширина
height = Высота

# Menu items

launcher-folder = Папка лаунчера
game-folder = Папка игры
config-file = Файл настроек
debug-file = Журнал отладки
wish-url = История молитв
about = О программе


close = { $form ->
    [verb] Закрываться
    *[noun] Закрыть
}

hide = { $form ->
    [verb] Скрываться
    *[noun] Скрыть
}

nothing = Ничего
save = Сохранить
continue = Продолжить
resume = Возобновить
exit = Выйти
check = Проверить
restart = Перезапустить
agree = Подтвердить


loading-data = Загрузка данных
downloading-background-picture = Загрузка фонового изображения
updating-components-index = Обновление индекса компонентов
loading-game-version = Загрузка версии игры
loading-patch-status = Загрузка статуса патча
loading-launcher-state = Загрузка статуса лаунчера
loading-launcher-state--game = Загрузка статуса лаунчера: проверка версии игры
loading-launcher-state--voice = Загрузка статуса лаунчера: проверка {$locale ->
    [English] английского
    [Japanese] японского
    [Korean] корейского
    [Chinese] китайского
    *[other] $locale
} языкового пакета
loading-launcher-state--patch = Загрузка статуса лаунчера: проверка установленного патча


checking-free-space = Проверка свободного места
downloading = Загрузка
unpacking = Распаковка
verifying-files = Проверка файлов
repairing-files = Починка файлов
migrating-folders = Перемещение папок
applying-hdiff = Применение патчей hdiff
removing-outdated = Удаление устаревших файлов


components-index-updated = Индекс компонентов был обновлен


launch = Запустить
migrate-folders = Переместить папки
migrate-folders-tooltip = Обновить структуру файлов игры
apply-patch = Применить патч
disable-telemetry = Отключить телеметрию
download-wine = Установить Wine
create-prefix = Создать префикс
update = Обновить
download = Установить
predownload-update = Предустановить обновление {$version} ({$size})

main-window--patch-unavailable-tooltip = Серверы патча недоступны и лаунчер не может проверить статус патча игры. Вам разрешено запустить игру на ваш страх и риск
main-window--patch-outdated-tooltip = Патч устарел или находится в процессе разработки, поэтому не может быть применен. Возвращайтесь позже чтобы проверить его статус
main-window--version-outdated-tooltip = Версия слишком стара и не может быть обновлена


preferences = Настройки
general = Основное
enhancements = Улучшения

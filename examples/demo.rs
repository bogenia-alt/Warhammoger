use warhammoger::logger;

fn main() {
    logger::trace("Разведчик замечен на периметре");
    logger::debug("Загрузка боевого устава ордена");
    logger::info("some_text");
    logger::warn("Пробит щит генератора");
    logger::error("Контакт с силами Хаоса потерян");
}

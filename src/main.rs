mod app;
mod engine;
mod i18n;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();

    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);

    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(300.0)
            .min_height(450.0)
            .max_width(600.0)
            .max_height(900.0),
    );

    cosmic::app::run::<app::Calculator>(settings, ())
}

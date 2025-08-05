use iced::{
    Background, Border, Color, Gradient, Radians, Shadow, color,
    gradient::{ColorStop, Linear},
};

struct Palette {
    gradient_background: Background,
}

impl Palette {
    pub const LIGHT: Palette = Palette {
        gradient_background: Background::Gradient(Gradient::Linear(Linear {
            angle: Radians(2.355),
            stops: [
                Some(ColorStop {
                    offset: 0.0,
                    color: color!(0x3026F1),
                }),
                Some(ColorStop {
                    offset: 1.0,
                    color: color!(0xFF9292),
                }),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
        })),
    };
}

fn gradient_style(_: &iced::Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        text_color: Some(Color::WHITE),
        background: Some(Palette::LIGHT.gradient_background),
        border: Border::default(),
        shadow: Shadow::default(),
        snap: true,
    }
}

#[derive(Clone, Debug)]
enum Message {
    DoNothing,
}

#[derive(Default)]
struct IcedGradientExample;

impl IcedGradientExample {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    pub fn view(&self) -> iced::Element<Message> {
        let empty_text = iced::widget::Text::new("");
        iced::widget::Container::new(empty_text)
            .style(gradient_style)
            .width(100.0)
            .height(100.0)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(
        IcedGradientExample::default,
        IcedGradientExample::update,
        IcedGradientExample::view,
    )
    .run()
}

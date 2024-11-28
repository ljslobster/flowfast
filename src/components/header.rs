use adw::prelude::*;
use relm4::*;

#[derive(Debug)]
pub enum HeaderMsg {}

#[derive(Debug)]
pub struct HeaderModel {
    title: String,
}

#[relm4::component(pub)]
impl SimpleComponent for HeaderModel {
    type Init = String;
    type Input = HeaderMsg;
    type Output = ();
    type Widgets = HeaderWidgets;

    view! {
        adw::HeaderBar {
            #[wrap(Some)]
            set_title_widget = &adw::WindowTitle {
                set_title: model.title.as_str(),
            }
        }
    }

    fn init(
        title: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = HeaderModel { title };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

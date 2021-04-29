fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Data::Xml::Dom::XmlDocument,
        Windows::Data::Xml::Dom::XmlElement,
        Windows::Data::Xml::Dom::XmlAttribute,
        Windows::Data::Xml::Dom::XmlCDataSection,
        Windows::UI::Notifications::ToastNotification,
        Windows::UI::Notifications::ToastNotifier,
        Windows::UI::Notifications::ToastNotificationManager
    );
}

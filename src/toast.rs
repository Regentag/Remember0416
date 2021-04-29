use bindings::{
	Windows::UI::Notifications::*,
	Windows::Data::Xml::Dom::XmlDocument
};
use windows::Result;

#[allow(dead_code)]
pub enum ToastDuration {
	Short, Long
}

#[allow(dead_code)]
pub enum ToastAudio {
	Default,
	IM,
	Mail,
	Reminder,
	SMS,
	LoopingAlarm,
	LoopingAlarm2,
	LoopingAlarm3,
	LoopingAlarm4,
	LoopingAlarm5,
	LoopingAlarm6,
	LoopingAlarm7,
	LoopingAlarm8,
	LoopingAlarm9,
	LoopingAlarm10,
	LoopingCall,
	LoopingCall2,
	LoopingCall3,
	LoopingCall4,
	LoopingCall5,
	LoopingCall6,
	LoopingCall7,
	LoopingCall8,
	LoopingCall9,
	LoopingCall10,
	Silent
}

pub struct Notification<'a> {
	pub app_id: &'a str,
	pub title: &'a str,
	pub message: &'a str,
	pub body_image: &'a str,
	pub icon: &'a str,
	pub activation_type: &'a str,
	pub activation_argument: &'a str,
	pub audio: ToastAudio,
	pub audio_loop: bool,
	pub duration: ToastDuration,
	pub actions: Vec<ToastAction<'a>>,
}

pub struct ToastAction<'a> {
	pub action_type:	&'a str,
	pub label:			&'a str,
	pub arguments:		&'a str,
}

impl<'a> Notification<'a> {
	pub fn new() -> Notification<'a> {
		Notification {
			app_id: "TestApp",
			title: "Noti Test",
			message: "Message",
			body_image: "",
			icon: "",
			activation_type: "",
			activation_argument: "",
			audio: ToastAudio::Default,
			audio_loop: false,
			duration: ToastDuration::Short,
			actions: Vec::new(),
		}
	}

	fn apply_default(&mut self) {
		if self.activation_type == "" {
			self.activation_type = "protocol";
		}
	}

	fn build_xml(&self) -> XmlDocument {
		let doc = XmlDocument::new().unwrap();
		let root = doc.CreateElement("toast").unwrap();
		root.SetAttribute("activationType", self.activation_type).ok();
		root.SetAttribute("launch", self.activation_argument).ok();
		root.SetAttribute("duration", self.duration.get_str()).ok();

		// visual
		let visual = doc.CreateElement("visual").unwrap();
		{
			let bind = doc.CreateElement("binding").unwrap();
			{
				bind.SetAttribute("template", "ToastGeneric").ok();

				if self.icon != "" {
					let icon = doc.CreateElement("image").unwrap();
					icon.SetAttribute("placement", "appLogoOverride").ok();
					icon.SetAttribute("src", self.icon).ok();
					bind.AppendChild(icon).ok();
				}

				if self.title != "" {
					let text = doc.CreateElement("text").unwrap();
					let cdata = doc.CreateCDataSection(self.title).unwrap();
					text.AppendChild(cdata).ok();
					bind.AppendChild(text).ok();
				}

				if self.message != "" {
					let text = doc.CreateElement("text").unwrap();
					let cdata = doc.CreateCDataSection(self.message).unwrap();
					text.AppendChild(cdata).ok();
					bind.AppendChild(text).ok();
				}

				if self.body_image != "" {
					let body_image = doc.CreateElement("image").unwrap();
					body_image.SetAttribute("src", self.body_image).ok();
					bind.AppendChild(body_image).ok();
				}
			}
			visual.AppendChild(bind).ok();
		}
		root.AppendChild(visual).ok();


		// audio
		let audio = doc.CreateElement("audio").unwrap();
		match &self.audio {
			ToastAudio::Silent => {
				audio.SetAttribute( "silent", "true" ).ok();
			},
			_ => {
				audio.SetAttribute( "src", self.audio.get_str() ).ok();
				audio.SetAttribute( "loop",
					if self.audio_loop { "true" } else { "false" } ).ok();
			}
		}
		root.AppendChild(audio).ok();

		// action
		if self.actions.len() > 0 {
			let actions = doc.CreateElement("actions").unwrap();
			for act in self.actions.iter() {
				let action = doc.CreateElement("action").unwrap();
				action.SetAttribute("activationType", act.action_type).ok();
				action.SetAttribute("content", act.label).ok();
				action.SetAttribute("arguments", act.arguments).ok();

				actions.AppendChild(action).ok();
			}

			root.AppendChild(actions).ok();
		}

		doc.AppendChild( root ).ok();

		doc
	}

	pub fn push(&mut self) -> Result<()> {
		&self.apply_default();

		let xml = self.build_xml();
		//println!("Xml={}", xml.GetXml().unwrap());
		let noti = ToastNotification::CreateToastNotification(xml)
			.unwrap();

		ToastNotificationManager::CreateToastNotifierWithId(self.app_id)
			.unwrap()
			.Show(noti)
	}
}

impl<'a> ToastAction<'a> {
	pub fn new( action_type: &'a str, label: &'a str, arguments: &'a str )
	-> ToastAction<'a> {
		ToastAction {
			action_type: action_type,
			label: label,
			arguments: arguments
		}
	}
}

impl ToastAudio {
	pub fn get_str(&self) -> &'static str {
		match self {
			ToastAudio::Default         => "ms-winsoundevent:Notification.Default",
			ToastAudio::IM              => "ms-winsoundevent:Notification.IM",
			ToastAudio::Mail            => "ms-winsoundevent:Notification.Mail",
			ToastAudio::Reminder        => "ms-winsoundevent:Notification.Reminder",
			ToastAudio::SMS             => "ms-winsoundevent:Notification.SMS",
			ToastAudio::LoopingAlarm    => "ms-winsoundevent:Notification.Looping.Alarm",
			ToastAudio::LoopingAlarm2   => "ms-winsoundevent:Notification.Looping.Alarm2",
			ToastAudio::LoopingAlarm3   => "ms-winsoundevent:Notification.Looping.Alarm3",
			ToastAudio::LoopingAlarm4   => "ms-winsoundevent:Notification.Looping.Alarm4",
			ToastAudio::LoopingAlarm5   => "ms-winsoundevent:Notification.Looping.Alarm5",
			ToastAudio::LoopingAlarm6   => "ms-winsoundevent:Notification.Looping.Alarm6",
			ToastAudio::LoopingAlarm7   => "ms-winsoundevent:Notification.Looping.Alarm7",
			ToastAudio::LoopingAlarm8   => "ms-winsoundevent:Notification.Looping.Alarm8",
			ToastAudio::LoopingAlarm9   => "ms-winsoundevent:Notification.Looping.Alarm9",
			ToastAudio::LoopingAlarm10  => "ms-winsoundevent:Notification.Looping.Alarm10",
			ToastAudio::LoopingCall     => "ms-winsoundevent:Notification.Looping.Call",
			ToastAudio::LoopingCall2    => "ms-winsoundevent:Notification.Looping.Call2",
			ToastAudio::LoopingCall3    => "ms-winsoundevent:Notification.Looping.Call3",
			ToastAudio::LoopingCall4    => "ms-winsoundevent:Notification.Looping.Call4",
			ToastAudio::LoopingCall5    => "ms-winsoundevent:Notification.Looping.Call5",
			ToastAudio::LoopingCall6    => "ms-winsoundevent:Notification.Looping.Call6",
			ToastAudio::LoopingCall7    => "ms-winsoundevent:Notification.Looping.Call7",
			ToastAudio::LoopingCall8    => "ms-winsoundevent:Notification.Looping.Call8",
			ToastAudio::LoopingCall9    => "ms-winsoundevent:Notification.Looping.Call9",
			ToastAudio::LoopingCall10   => "ms-winsoundevent:Notification.Looping.Call10",
			ToastAudio::Silent          => "silent",
		}
	}
}

impl ToastDuration {
	pub fn get_str(&self) -> &'static str {
		match self {
			ToastDuration::Long => "long",
			ToastDuration::Short => "short",
		}
	}
}

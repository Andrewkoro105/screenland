use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use glam::Vec2;
use iced::{Point, Task, exit, window};
use iced_layershell::to_layer_message;
use tracing::debug;

use crate::app::{
    Mode, Screenland,
    edit_object::{
        self, EditObjectSettings,
        custom_object::{self, param::channel},
    },
    end::End,
    selection,
    settings::edit_object_base_settings,
};

#[to_layer_message(multi)]
#[derive(Clone, Debug)]
pub enum Message {
    Exit,
    AutoExit,
    SetMode(Mode),
    MoveMouse(Point, window::Id),
    TouchStart,
    TouchEnd,
    End(End),
    ReloadShaderObjects,
    ReindexingObjects,
    SelectionUpdate(Option<selection::Message>),
    EditObjectBaseSettings(edit_object_base_settings::Message),
    AddObject(edit_object::CreateObjects),
    UpdateEditObject((usize, edit_object::Message)),
    CustomObjectsChannelUpdate {
        i: usize,
        index: usize,
        channel_type: channel::ChannelType,
        data: Vec<u8>,
    },
    None,
}

impl Screenland {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => exit(),
            Message::AutoExit => {
                if self.auto_exit {
                    Task::done(Message::Exit)
                } else {
                    Task::none()
                }
            }
            Message::SetMode(mode) => {
                debug!("Set mode {mode:?}");
                self.mode = mode;
                Task::none()
            }
            Message::MoveMouse(point, id) => {
                self.mouse_pos = Vec2 {
                    x: point.x,
                    y: point.y,
                } + self
                    .windows_data
                    .get(&id)
                    .map(|window_data| Vec2 {
                        x: window_data.pos.0 as _,
                        y: window_data.pos.1 as _,
                    })
                    .unwrap_or(Vec2 { x: 0., y: 0. });

                match &self.mode {
                    Mode::Base => Task::none(),
                    Mode::Move(message) => Task::done(*message.clone()),
                    Mode::Transparency => Task::none(),
                }
            }
            Message::TouchStart => {
                let double_click = self.mouse_touch_time.elapsed() < Duration::from_millis(500);
                self.mouse_touch_time = Instant::now();

                if double_click {
                    match self.mode {
                        Mode::Base => {
                            if let Some(new_current_object) = self.get_object_in_which_mouse() {
                                let object = self.objects.remove(new_current_object);
                                self.objects.push(object);
                                self.current_object = Some(self.objects.len() - 1);
                                Task::done(Message::ReindexingObjects)
                            } else {
                                Task::none()
                            }
                        }
                        Mode::Move(_) => Task::none(),
                        Mode::Transparency => Task::none(),
                    }
                } else {
                    match self.mode {
                        Mode::Base => {
                            let object_messages = self
                                .current_object
                                .map(|current_object| {
                                    self.objects[current_object].get_messages(&self.mouse_pos)
                                })
                                .unwrap_or_default();

                            if let Some(reload) = object_messages {
                                reload
                                    .map(|ui_messages| {
                                        ui_messages.get_task(|message| {
                                            Message::SetMode(Mode::Move(Box::new(message)))
                                        })
                                    })
                                    .get_task()
                            } else if let Some(new_current_object) =
                                self.get_object_in_which_mouse()
                            {
                                self.current_object = Some(new_current_object);
                                Task::none()
                            } else {
                                let selection_messages =
                                    self.selection.get_messages(&self.mouse_pos);
                                if let Some(reload) = selection_messages {
                                    reload
                                        .messages_map(Some)
                                        .messages_map(Message::SelectionUpdate)
                                        .map(|ui_messages| {
                                            ui_messages.get_task(|message| {
                                                Message::SetMode(Mode::Move(Box::new(message)))
                                            })
                                        })
                                        .get_task()
                                } else {
                                    Task::none()
                                }
                            }
                        }
                        Mode::Move(_) => Task::none(),
                        Mode::Transparency => Task::none(),
                    }
                }
            }
            Message::TouchEnd => match self.mode {
                Mode::Base => Task::none(),
                Mode::Move(_) => {
                    self.mode = Mode::Base;
                    Task::done(Message::SelectionUpdate(None)).chain(
                        self.current_object
                            .map(|current_object| {
                                Task::done(Message::UpdateEditObject((
                                    current_object,
                                    edit_object::Message::Custom(custom_object::Message::Point(
                                        None,
                                    )),
                                )))
                            })
                            .unwrap_or(Task::none()),
                    )
                }
                Mode::Transparency => Task::none(),
            },
            Message::End(end) => {
                self.auto_exit = false;
                let selection = self.selection;
                let windows_data = self.windows_data.clone();
                let settings = self.settings.clone();
                Task::done(Message::SetMode(Mode::Transparency)).chain(
                    Task::future(async move {
                        sleep(Duration::from_millis(50));
                        let screen = Self::screenshot(selection, &settings.color_format);

                        let mut windows_task = Task::<Message>::none();

                        for (id, _) in windows_data.iter() {
                            windows_task = windows_task.chain(window::close(*id));
                        }
                        windows_task.chain(
                            Task::future(async move {
                                end.end(&settings, screen);
                            })
                            .discard(),
                        )
                    })
                    .then(|task| task)
                    .chain(exit()),
                )
            }
            Message::ReloadShaderObjects => {
                self.reload_shader_objects();
                Task::none()
            }
            Message::ReindexingObjects => {
                self.reindexing_objects();
                Task::done(Message::ReloadShaderObjects)
            }
            Message::SelectionUpdate(message) => {
                let reload = self.selection.update(self.mouse_pos, message);
                reload
                    .task_map(Some)
                    .task_map(Message::SelectionUpdate)
                    .get_task()
            }
            Message::EditObjectBaseSettings(message) => {
                let task = self.settings.edit_object_base_settings.update(message);
                self.settings.save();
                task.map(Message::EditObjectBaseSettings)
            }
            Message::AddObject(create_objects) => {
                match create_objects {
                    edit_object::CreateObjects::Custom(i) => {
                        self.current_object = Some(self.objects.len());
                        self.objects
                            .push(Box::new(self.settings.custom_objects[i].get_object(
                                self.objects.len(),
                                &self.settings.edit_object_base_settings.clone().into(),
                            )))
                    }
                }
                Task::done(Message::ReloadShaderObjects)
            }
            Message::UpdateEditObject((i, message)) => {
                self.objects[i].update(self.mouse_pos, message)
            }
            Message::CustomObjectsChannelUpdate {
                i,
                index,
                channel_type,
                data,
            } => {
                if data.len() != channel_type.get_size() {
                    panic!(
                        "The `Message::CustomObjectsChannelUpdate` contains {} bytes, which exceeds the size of the `{:?}` type. The type in this channel is {} bytes in size",
                        data.len(),
                        channel_type,
                        channel_type.get_size(),
                    )
                }
                self.custom_objects_channel.update(
                    &channel_type,
                    if let edit_object::ShaderObjects::Custom(custom_shader_object) = &self.shader_objects[i] {
                        &custom_shader_object.channel_index
                    } else {
                        unreachable!("In Message::CustomObjectsChannelUpdate, a message for the wrong object was sent.")
                    },
                    index,
                    data,
                );
                Task::none()
            }
            Message::None => Task::none(),
            _ => unreachable!(),
        }
    }
}

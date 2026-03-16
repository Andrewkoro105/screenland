use std::{thread::sleep, time::Duration};

use glam::Vec2;
use iced::{Point, Task, exit, window};

use crate::app::{
    Mode, Screenland,
    edit_object::{self, EditObject, EditObjectSettings, custom_object::CustomObject},
    end::End,
    selection,
    settings::{self, edit_object_base_settings},
};

#[derive(Clone)]
pub enum Message {
    Exit,
    AutoExit,
    SetMode(Mode),
    MoveMouse(Point, window::Id),
    TouchStart,
    TouchEnd,
    End(End),
    SelectionUpdate(selection::Message),
    EditObjectBaseSettings(edit_object_base_settings::Message),
    AddObject(edit_object::CreateObjects),
    UpdateEditObject((usize, edit_object::Message)),
    SetF32InCustomObjectsChenel { i: usize, index: usize, value: f32 },
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
                    Mode::Move(message) => self.update(Message::SelectionUpdate(message.clone())),
                    Mode::Selection => {
                        self.selection.end = self.mouse_pos;
                        Task::none()
                    }
                    Mode::Transparency => Task::none(),
                }
            }
            Message::TouchStart => match self.mode {
                Mode::Base => {
                    let select_message = self.selection.get_messages(&self.mouse_pos);
                    if !select_message.is_empty() {
                        self.mode = Mode::Move(select_message[0].clone());
                    } else {
                        self.mode = Mode::Selection;
                        self.selection.start = self.mouse_pos;
                        self.selection.end = self.mouse_pos;
                    }
                    Task::none()
                }
                Mode::Move(_) => Task::none(),
                Mode::Selection => {
                    self.mode = Mode::Selection;
                    self.selection.start = self.mouse_pos;
                    self.selection.end = self.mouse_pos;
                    Task::none()
                }
                Mode::Transparency => Task::none(),
            },
            Message::TouchEnd => match self.mode {
                Mode::Base => Task::none(),
                Mode::Move(_) => {
                    self.mode = Mode::Base;
                    Task::none()
                }
                Mode::Selection => {
                    if let Some(end) = &self.settings.base_end {
                        Task::done(Message::End(end.clone()))
                    } else {
                        self.mode = Mode::Base;
                        Task::none()
                    }
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
                        sleep(Duration::from_millis(10));
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
            Message::SelectionUpdate(message) => self
                .selection
                .update(self.mouse_pos, message)
                .map(Message::SelectionUpdate),
            Message::EditObjectBaseSettings(message) => {
                let task = self.settings.edit_object_base_settings.update(message);
                self.settings.save();
                task.map(Message::EditObjectBaseSettings)
            }
            Message::AddObject(create_objects) => {
                match create_objects {
                    edit_object::CreateObjects::Custom(i) => {
                        self.objects.push(
                            edit_object::Objects::Custom(
                                self.settings.custom_objects[i].get_object(
                                    self.objects.len(),
                                    &self.settings.edit_object_base_settings.clone().into()
                                )
                            )
                        )
                    },
                }
                self.reload_shader_objects();
                Task::none()
            }
            Message::UpdateEditObject((i, message)) => {
                match &mut self.objects[i] {
                    edit_object::Objects::Custom(custom_object) => custom_object.update(
                        self.mouse_pos,
                        if let edit_object::Message::Custom(message) = message {
                            message
                        } else {
                            panic!("In Message::UpdateEditObject, a message for the wrong object was sent.")
                        },
                    ),
                }.map(Into::into)
            }
            Message::SetF32InCustomObjectsChenel { i, index, value } => {
                self.custom_objects_chenel.set_f32(
                    if let edit_object::ShaderObjects::Custom(custom_shader_object) = &self.shader_objects[i] {
                        custom_shader_object.channel_index
                    } else {
                        panic!("In Message::SetF32InCustomObjectsChenel, a message for the wrong object was sent.")
                    },
                    index,
                    value,
                );
                Task::none()
            },
            Message::None => Task::none(),
        }
    }
}

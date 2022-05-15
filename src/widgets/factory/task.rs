use relm4::{CommandFuture, gtk, ShutdownReceiver};
use relm4::{Sender, view, WidgetPlus};
use relm4::factory::{DynamicIndex, FactoryComponent};
use relm4::gtk::prelude::{
    BoxExt, ButtonExt, CheckButtonExt, EditableExt, ListBoxRowExt, OrientableExt, ToggleButtonExt,
    WidgetExt,
};

use crate::models::task::{Task, TaskStatus};
use crate::widgets::component::content::ContentInput;

pub enum TaskInput {
    SetCompleted(bool),
    Favorite(bool),
    ModifyTitle(String),
}

pub enum TaskOutput {
    Remove(DynamicIndex),
}

pub struct TaskWidgets {
    entry: gtk::Entry,
}

impl FactoryComponent<gtk::Box, ContentInput> for Task {
    type Command = ();
    type CommandOutput = ();
    type Input = TaskInput;
    type Output = TaskOutput;
    type InitParams = Task;
    type Root = gtk::ListBoxRow;
    type Widgets = TaskWidgets;

    fn output_to_parent_msg(output: Self::Output) -> Option<ContentInput> {
        Some(match output {
            TaskOutput::Remove(index) => ContentInput::Remove(index)
        })
    }

    fn init_model(
        params: Self::InitParams,
        index: &DynamicIndex,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Self {
        params
    }

    fn init_root() -> Self::Root {
        view! {
            root = &gtk::ListBoxRow {
                set_selectable: false,
            }
        }
        root
    }

    fn init_widgets(
        &mut self,
        index: &DynamicIndex,
        root: &Self::Root,
        returned_widget: &gtk::Widget,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Self::Widgets {
        view! {
            container = &gtk::Box {
                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    set_margin_start: 10,
                    set_margin_end: 10,
                    append = &gtk::CheckButton {
                        set_active: self.status.as_bool(),
                        connect_toggled[input] => move |checkbox| {
                            input.send(TaskInput::SetCompleted(checkbox.is_active()));
                        }
                    },
                    append = &gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 15,
                        append: entry = &gtk::Entry {
                            add_css_class: "flat",
                            add_css_class: "no-border",
                            set_hexpand: true,
                            set_text: &self.title,
                            connect_activate[input] => move |entry| {
                                let buffer = entry.buffer();
                                input.send(TaskInput::ModifyTitle(buffer.text()));
                            },
                            connect_changed[input] => move |entry| {
                                let buffer = entry.buffer();
                                input.send(TaskInput::ModifyTitle(buffer.text()));
                            }
                        },
                        append: favorite = &gtk::ToggleButton {
                            add_css_class: "opaque",
                            add_css_class: "circular",
                            set_class_active: track!(self.changed(Task::favorite()), "favorite", self.favorite),
                            set_active: track!(self.changed(Task::favorite()), self.favorite),
                            set_icon_name: "starred-symbolic",
                            connect_toggled[input] => move |button| {
                                if button.is_active() {
                                    button.add_css_class("favorite");
                                } else {
                                    button.remove_css_class("favorite");
                                }
                                input.send(TaskInput::Favorite(button.is_active()));
                            }
                        },
                        append: delete = &gtk::Button {
                            add_css_class: "destructive-action",
                            add_css_class: "circular",
                            set_icon_name: "user-trash-symbolic",
                            connect_clicked[output, index] => move |_| {
                                output.send(TaskOutput::Remove(index))
                            }
                        }
                    }
                }
            }
        }
        root.set_child(Some(&container));

        TaskWidgets { entry }
    }

    fn update(
        &mut self,
        message: Self::Input,
        input: &Sender<Self::Input>,
        output: &Sender<Self::Output>,
    ) -> Option<Self::Command> {
        match message {
            TaskInput::SetCompleted(completed) => {
                self.status = if completed {
                    TaskStatus::Completed
                } else {
                    TaskStatus::NotStarted
                }
            }
            TaskInput::Favorite(favorite) => self.favorite = favorite,
            TaskInput::ModifyTitle(title) => self.title = title,
        }
        None
    }
}